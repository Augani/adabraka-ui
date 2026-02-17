use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use serde_json::Value;

use super::config::ServerConfig;
use super::types::FileDiagnostics;

#[derive(Debug)]
pub enum TransportError {
    SpawnFailed(std::io::Error),
    WriteFailed(std::io::Error),
    ServerExited,
    ParseError(String),
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SpawnFailed(e) => write!(f, "Failed to spawn LSP server: {}", e),
            Self::WriteFailed(e) => write!(f, "Failed to write to LSP server: {}", e),
            Self::ServerExited => write!(f, "LSP server exited"),
            Self::ParseError(e) => write!(f, "Failed to parse LSP message: {}", e),
        }
    }
}

pub struct LspTransport {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    child: Arc<Mutex<Child>>,
    reader_thread: Option<thread::JoinHandle<()>>,
    response_rx: flume::Receiver<Value>,
    diagnostics_rx: flume::Receiver<FileDiagnostics>,
    pending_requests: Arc<Mutex<HashMap<i64, flume::Sender<Value>>>>,
    next_id: Arc<Mutex<i64>>,
    is_running: Arc<Mutex<bool>>,
}

impl LspTransport {
    pub fn spawn(config: &ServerConfig) -> Result<Self, TransportError> {
        let mut cmd = Command::new(&config.command);
        cmd.args(&config.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if config.command.contains("rust-analyzer") {
            cmd.env("RUSTUP_TOOLCHAIN", "stable");
        }

        let mut child = cmd.spawn().map_err(TransportError::SpawnFailed)?;

        if let Some(stderr) = child.stderr.take() {
            let cmd_name = config.command.clone();
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    match line {
                        Ok(l) => eprintln!("[LSP:{}] {}", cmd_name, l),
                        Err(_) => break,
                    }
                }
            });
        }

        let stdin = child.stdin.take().ok_or(TransportError::ServerExited)?;
        let stdout = child.stdout.take().ok_or(TransportError::ServerExited)?;

        let writer: Arc<Mutex<Box<dyn Write + Send>>> = Arc::new(Mutex::new(Box::new(stdin)));
        let pending_requests: Arc<Mutex<HashMap<i64, flume::Sender<Value>>>> =
            Arc::new(Mutex::new(HashMap::new()));
        let is_running = Arc::new(Mutex::new(true));

        let (response_tx, response_rx) = flume::unbounded();
        let (diagnostics_tx, diagnostics_rx) = flume::unbounded();

        let pending_clone = pending_requests.clone();
        let running_clone = is_running.clone();

        let reader_thread = thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                {
                    if !*running_clone.lock().unwrap() {
                        break;
                    }
                }

                match read_message(&mut reader) {
                    Ok(msg) => {
                        if let Some(id) = msg.get("id") {
                            if msg.get("method").is_none() {
                                if let Some(id_num) = id.as_i64() {
                                    let sender = pending_clone.lock().unwrap().remove(&id_num);
                                    if let Some(tx) = sender {
                                        let _ = tx.send(msg);
                                        continue;
                                    }
                                }
                            }
                        }

                        if let Some(method) = msg.get("method").and_then(|m| m.as_str()) {
                            if method == "textDocument/publishDiagnostics" {
                                if let Some(params) = msg.get("params") {
                                    if let Some(diags) = parse_diagnostics(params) {
                                        let _ = diagnostics_tx.send(diags);
                                        continue;
                                    }
                                }
                            }
                        }

                        let _ = response_tx.send(msg);
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self {
            writer,
            child: Arc::new(Mutex::new(child)),
            reader_thread: Some(reader_thread),
            response_rx,
            diagnostics_rx,
            pending_requests,
            next_id: Arc::new(Mutex::new(1)),
            is_running,
        })
    }

    pub fn send_request(
        &self,
        method: &str,
        params: Value,
    ) -> Result<flume::Receiver<Value>, TransportError> {
        let id = {
            let mut next = self.next_id.lock().unwrap();
            let id = *next;
            *next += 1;
            id
        };

        let (tx, rx) = flume::bounded(1);
        self.pending_requests.lock().unwrap().insert(id, tx);

        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });

        self.write_message(&msg)?;
        Ok(rx)
    }

    pub fn send_notification(&self, method: &str, params: Value) -> Result<(), TransportError> {
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        self.write_message(&msg)
    }

    fn write_message(&self, msg: &Value) -> Result<(), TransportError> {
        let body =
            serde_json::to_string(msg).map_err(|e| TransportError::ParseError(e.to_string()))?;
        let header = format!("Content-Length: {}\r\n\r\n", body.len());

        let mut writer = self.writer.lock().unwrap();
        writer
            .write_all(header.as_bytes())
            .map_err(TransportError::WriteFailed)?;
        writer
            .write_all(body.as_bytes())
            .map_err(TransportError::WriteFailed)?;
        writer.flush().map_err(TransportError::WriteFailed)?;
        Ok(())
    }

    pub fn diagnostics_rx(&self) -> &flume::Receiver<FileDiagnostics> {
        &self.diagnostics_rx
    }

    #[allow(dead_code)]
    pub fn response_rx(&self) -> &flume::Receiver<Value> {
        &self.response_rx
    }

    pub fn stop(&mut self) {
        *self.is_running.lock().unwrap() = false;
        if let Ok(mut child) = self.child.lock() {
            let _ = child.kill();
            let _ = child.wait();
        }
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for LspTransport {
    fn drop(&mut self) {
        self.stop();
    }
}

fn read_message(reader: &mut BufReader<impl Read>) -> Result<Value, TransportError> {
    let mut content_length: usize = 0;
    loop {
        let mut line = String::new();
        let bytes_read = reader
            .read_line(&mut line)
            .map_err(|_| TransportError::ServerExited)?;
        if bytes_read == 0 {
            return Err(TransportError::ServerExited);
        }
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        if let Some(val) = trimmed.strip_prefix("Content-Length: ") {
            content_length = val
                .parse()
                .map_err(|_| TransportError::ParseError("Invalid Content-Length".into()))?;
        }
    }

    if content_length == 0 {
        return Err(TransportError::ParseError("Missing Content-Length".into()));
    }

    let mut body = vec![0u8; content_length];
    reader
        .read_exact(&mut body)
        .map_err(|_| TransportError::ServerExited)?;

    serde_json::from_slice(&body).map_err(|e| TransportError::ParseError(e.to_string()))
}

fn parse_diagnostics(params: &Value) -> Option<FileDiagnostics> {
    let uri = params.get("uri")?.as_str()?;
    let path = url::Url::parse(uri)
        .ok()
        .and_then(|u| u.to_file_path().ok())?;

    let diags_arr = params.get("diagnostics")?.as_array()?;
    let mut diagnostics = Vec::with_capacity(diags_arr.len());

    for diag in diags_arr {
        let parsed = (|| -> Option<super::types::Diagnostic> {
            let range = diag.get("range")?;
            let start = range.get("start")?;
            let end = range.get("end")?;

            let severity = match diag.get("severity").and_then(|s| s.as_u64()) {
                Some(1) => super::types::DiagnosticSeverity::Error,
                Some(2) => super::types::DiagnosticSeverity::Warning,
                Some(3) => super::types::DiagnosticSeverity::Information,
                _ => super::types::DiagnosticSeverity::Hint,
            };

            let message = diag.get("message")?.as_str()?.to_string();
            let source = diag
                .get("source")
                .and_then(|s| s.as_str())
                .map(String::from);
            let code = diag.get("code").and_then(|c| {
                c.as_str()
                    .map(String::from)
                    .or_else(|| c.as_u64().map(|n| n.to_string()))
            });

            Some(super::types::Diagnostic {
                range_start_line: start.get("line")?.as_u64()? as u32,
                range_start_col: start.get("character")?.as_u64()? as u32,
                range_end_line: end.get("line")?.as_u64()? as u32,
                range_end_col: end.get("character")?.as_u64()? as u32,
                severity,
                message,
                source,
                code,
            })
        })();

        if let Some(d) = parsed {
            diagnostics.push(d);
        }
    }

    eprintln!(
        "[LSP] publishDiagnostics: {} diags for {}",
        diagnostics.len(),
        path.file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_default()
    );

    Some(FileDiagnostics { path, diagnostics })
}
