#[allow(unused_imports)]
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::config::ServerConfig;
use super::transport::{LspTransport, TransportError};
use super::types::{
    FileDiagnostics, HoverInfo, LocationInfo, LspCompletionItem, LspCompletionKind,
};

pub struct LspClient {
    transport: LspTransport,
    root_uri: String,
    server_capabilities: Option<Value>,
}

impl LspClient {
    pub fn start(config: &ServerConfig, root_path: &Path) -> Result<Self, TransportError> {
        let transport = LspTransport::spawn(config)?;
        let root_uri = url::Url::from_file_path(root_path)
            .map(|u| u.to_string())
            .unwrap_or_else(|_| format!("file://{}", root_path.display()));

        Ok(Self {
            transport,
            root_uri,
            server_capabilities: None,
        })
    }

    pub fn initialize(&mut self) -> Result<(), TransportError> {
        let params = json!({
            "processId": std::process::id(),
            "rootUri": self.root_uri,
            "capabilities": {
                "textDocument": {
                    "completion": {
                        "completionItem": {
                            "snippetSupport": false,
                            "labelDetailsSupport": true,
                        },
                        "contextSupport": true,
                    },
                    "hover": {
                        "contentFormat": ["plaintext", "markdown"],
                    },
                    "publishDiagnostics": {
                        "relatedInformation": false,
                    },
                    "definition": {},
                    "synchronization": {
                        "didSave": true,
                        "willSave": false,
                        "willSaveWaitUntil": false,
                    },
                },
                "workspace": {
                    "workspaceFolders": false,
                },
            },
            "initializationOptions": Value::Null,
        });

        let rx = self.transport.send_request("initialize", params)?;
        match rx.recv_timeout(std::time::Duration::from_secs(30)) {
            Ok(response) => {
                self.server_capabilities = response.get("result").cloned();
                self.transport.send_notification("initialized", json!({}))?;
                Ok(())
            }
            Err(_) => Err(TransportError::ServerExited),
        }
    }

    pub fn did_open(
        &self,
        path: &Path,
        language_id: &str,
        text: &str,
    ) -> Result<(), TransportError> {
        let uri = path_to_uri(path);
        self.transport.send_notification(
            "textDocument/didOpen",
            json!({
                "textDocument": {
                    "uri": uri,
                    "languageId": language_id,
                    "version": 1,
                    "text": text,
                }
            }),
        )
    }

    pub fn did_change(&self, path: &Path, text: &str, version: i32) -> Result<(), TransportError> {
        let uri = path_to_uri(path);
        self.transport.send_notification(
            "textDocument/didChange",
            json!({
                "textDocument": {
                    "uri": uri,
                    "version": version,
                },
                "contentChanges": [{
                    "text": text,
                }]
            }),
        )
    }

    pub fn did_save(&self, path: &Path) -> Result<(), TransportError> {
        let uri = path_to_uri(path);
        self.transport.send_notification(
            "textDocument/didSave",
            json!({
                "textDocument": {
                    "uri": uri,
                }
            }),
        )
    }

    pub fn did_close(&self, path: &Path) -> Result<(), TransportError> {
        let uri = path_to_uri(path);
        self.transport.send_notification(
            "textDocument/didClose",
            json!({
                "textDocument": {
                    "uri": uri,
                }
            }),
        )
    }

    pub fn completion(
        &self,
        path: &Path,
        line: u32,
        col: u32,
    ) -> Result<flume::Receiver<Value>, TransportError> {
        let uri = path_to_uri(path);
        self.transport.send_request(
            "textDocument/completion",
            json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": col },
            }),
        )
    }

    pub fn hover(
        &self,
        path: &Path,
        line: u32,
        col: u32,
    ) -> Result<flume::Receiver<Value>, TransportError> {
        let uri = path_to_uri(path);
        self.transport.send_request(
            "textDocument/hover",
            json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": col },
            }),
        )
    }

    pub fn goto_definition(
        &self,
        path: &Path,
        line: u32,
        col: u32,
    ) -> Result<flume::Receiver<Value>, TransportError> {
        let uri = path_to_uri(path);
        self.transport.send_request(
            "textDocument/definition",
            json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": col },
            }),
        )
    }

    pub fn shutdown(&mut self) -> Result<(), TransportError> {
        let rx = self.transport.send_request("shutdown", Value::Null)?;
        let _ = rx.recv_timeout(std::time::Duration::from_secs(5));
        self.transport.send_notification("exit", Value::Null)?;
        Ok(())
    }

    pub fn stop(&mut self) {
        let _ = self.shutdown();
        self.transport.stop();
    }

    pub fn diagnostics_rx(&self) -> &flume::Receiver<FileDiagnostics> {
        self.transport.diagnostics_rx()
    }

    pub fn parse_completion_response(response: &Value) -> Vec<LspCompletionItem> {
        let result = match response.get("result") {
            Some(r) => r,
            None => return Vec::new(),
        };

        let items = if let Some(arr) = result.as_array() {
            arr
        } else if let Some(arr) = result.get("items").and_then(|i| i.as_array()) {
            arr
        } else {
            return Vec::new();
        };

        items
            .iter()
            .filter_map(|item| {
                let label = item.get("label")?.as_str()?.to_string();
                let detail = item
                    .get("detail")
                    .and_then(|d| d.as_str())
                    .map(String::from);

                let insert_text = item
                    .get("insertText")
                    .and_then(|t| t.as_str())
                    .map(|t| strip_snippet_syntax(t))
                    .unwrap_or_else(|| {
                        item.get("textEdit")
                            .and_then(|te| te.get("newText"))
                            .and_then(|t| t.as_str())
                            .map(|t| strip_snippet_syntax(t))
                            .unwrap_or_else(|| label.clone())
                    });

                let kind = item
                    .get("kind")
                    .and_then(|k| k.as_i64())
                    .map(|k| LspCompletionKind::from_lsp_i32(k as i32))
                    .unwrap_or(LspCompletionKind::Other);

                let sort_text = item
                    .get("sortText")
                    .and_then(|s| s.as_str())
                    .map(String::from);

                Some(LspCompletionItem {
                    label,
                    detail,
                    insert_text,
                    kind,
                    sort_text,
                })
            })
            .collect()
    }

    pub fn parse_hover_response(response: &Value) -> Option<HoverInfo> {
        let result = response.get("result")?;
        if result.is_null() {
            return None;
        }

        let contents = result.get("contents")?;
        let text = if let Some(s) = contents.as_str() {
            s.to_string()
        } else if let Some(obj) = contents.as_object() {
            obj.get("value")?.as_str()?.to_string()
        } else if let Some(arr) = contents.as_array() {
            arr.iter()
                .filter_map(|item| {
                    item.as_str()
                        .map(String::from)
                        .or_else(|| item.get("value").and_then(|v| v.as_str()).map(String::from))
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        } else {
            return None;
        };

        if text.is_empty() {
            return None;
        }

        let range = result.get("range").and_then(|r| {
            let start = r.get("start")?;
            let end = r.get("end")?;
            Some((
                start.get("line")?.as_u64()? as u32,
                start.get("character")?.as_u64()? as u32,
                end.get("line")?.as_u64()? as u32,
                end.get("character")?.as_u64()? as u32,
            ))
        });

        Some(HoverInfo {
            contents: text,
            range,
        })
    }

    pub fn parse_definition_response(response: &Value) -> Vec<LocationInfo> {
        let result = match response.get("result") {
            Some(r) if !r.is_null() => r,
            _ => return Vec::new(),
        };

        let locations: Vec<&Value> = if let Some(arr) = result.as_array() {
            arr.iter().collect()
        } else if result.is_object() {
            vec![result]
        } else {
            return Vec::new();
        };

        locations
            .into_iter()
            .filter_map(|loc| {
                let uri_str = loc
                    .get("uri")
                    .or_else(|| loc.get("targetUri"))
                    .and_then(|u| u.as_str())?;

                let path = url::Url::parse(uri_str)
                    .ok()
                    .and_then(|u| u.to_file_path().ok())?;

                let range = loc
                    .get("range")
                    .or_else(|| loc.get("targetSelectionRange"))?;
                let start = range.get("start")?;
                let line = start.get("line")?.as_u64()? as u32;
                let col = start.get("character")?.as_u64()? as u32;

                Some(LocationInfo { path, line, col })
            })
            .collect()
    }
}

fn path_to_uri(path: &Path) -> String {
    url::Url::from_file_path(path)
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("file://{}", path.display()))
}

fn strip_snippet_syntax(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '$' {
            if chars.peek() == Some(&'{') {
                chars.next();
                let mut depth = 1;
                let mut placeholder = String::new();
                let mut past_colon = false;
                while let Some(c) = chars.next() {
                    if c == '{' {
                        depth += 1;
                    } else if c == '}' {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    } else if c == ':' && !past_colon {
                        past_colon = true;
                        continue;
                    }
                    if past_colon {
                        placeholder.push(c);
                    }
                }
                result.push_str(&placeholder);
            } else if chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                while chars.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                    chars.next();
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    result
}
