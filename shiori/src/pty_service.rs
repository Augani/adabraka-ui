use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use thiserror::Error;

pub const DEFAULT_PTY_COLS: u16 = 80;
pub const DEFAULT_PTY_ROWS: u16 = 24;

#[derive(Error, Debug)]
pub enum PtyError {
    #[error("Failed to create PTY: {0}")]
    CreateFailed(String),
    #[error("Failed to spawn shell: {0}")]
    SpawnFailed(String),
    #[error("Failed to write to PTY: {0}")]
    WriteFailed(String),
    #[error("Failed to read from PTY: {0}")]
    ReadFailed(String),
    #[error("Failed to resize PTY: {0}")]
    ResizeFailed(String),
    #[error("PTY not running")]
    NotRunning,
}

pub struct PtyService {
    pty_pair: Option<PtyPair>,
    writer: Option<Box<dyn Write + Send>>,
    reader_thread: Option<thread::JoinHandle<()>>,
    output_sender: flume::Sender<Vec<u8>>,
    output_receiver: flume::Receiver<Vec<u8>>,
    is_running: Arc<Mutex<bool>>,
    working_directory: PathBuf,
    cols: u16,
    rows: u16,
}

impl PtyService {
    pub fn new() -> Self {
        let (output_sender, output_receiver) = flume::unbounded();
        Self {
            pty_pair: None,
            writer: None,
            reader_thread: None,
            output_sender,
            output_receiver,
            is_running: Arc::new(Mutex::new(false)),
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
            cols: DEFAULT_PTY_COLS,
            rows: DEFAULT_PTY_ROWS,
        }
    }

    pub fn with_working_directory(mut self, path: PathBuf) -> Self {
        self.working_directory = path;
        self
    }

    pub fn with_size(mut self, cols: u16, rows: u16) -> Self {
        self.cols = cols;
        self.rows = rows;
        self
    }

    pub fn is_running(&self) -> bool {
        self.is_running.lock().map(|guard| *guard).unwrap_or(false)
    }

    pub fn output_receiver(&self) -> &flume::Receiver<Vec<u8>> {
        &self.output_receiver
    }

    pub fn working_directory(&self) -> &PathBuf {
        &self.working_directory
    }

    pub fn set_working_directory(&mut self, path: PathBuf) {
        self.working_directory = path;
    }

    pub fn start(&mut self) -> Result<(), PtyError> {
        if self.is_running() {
            return Ok(());
        }

        let pty_system = native_pty_system();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows: self.rows,
                cols: self.cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| PtyError::CreateFailed(e.to_string()))?;

        let shell = get_default_shell();

        let mut cmd = CommandBuilder::new(&shell);
        cmd.cwd(&self.working_directory);

        if let Ok(term) = std::env::var("TERM") {
            cmd.env("TERM", term);
        } else {
            cmd.env("TERM", "xterm-256color");
        }

        let _child = pty_pair
            .slave
            .spawn_command(cmd)
            .map_err(|e| PtyError::SpawnFailed(e.to_string()))?;

        let writer = pty_pair
            .master
            .take_writer()
            .map_err(|e| PtyError::CreateFailed(e.to_string()))?;

        let mut reader = pty_pair
            .master
            .try_clone_reader()
            .map_err(|e| PtyError::CreateFailed(e.to_string()))?;

        *self.is_running.lock().unwrap() = true;
        let is_running = Arc::clone(&self.is_running);
        let output_sender = self.output_sender.clone();

        let reader_thread = thread::spawn(move || {
            let mut buffer = [0u8; 4096];
            loop {
                if !*is_running.lock().unwrap() {
                    break;
                }

                match reader.read(&mut buffer) {
                    Ok(0) => {
                        *is_running.lock().unwrap() = false;
                        break;
                    }
                    Ok(n) => {
                        let data = buffer[..n].to_vec();
                        if output_sender.send(data).is_err() {
                            break;
                        }
                    }
                    Err(e) => {
                        if e.kind() != std::io::ErrorKind::WouldBlock {
                            *is_running.lock().unwrap() = false;
                            break;
                        }
                    }
                }
            }
        });

        self.pty_pair = Some(pty_pair);
        self.writer = Some(writer);
        self.reader_thread = Some(reader_thread);

        Ok(())
    }

    pub fn stop(&mut self) {
        *self.is_running.lock().unwrap() = false;
        self.writer = None;
        self.pty_pair = None;
        if let Some(handle) = self.reader_thread.take() {
            let _ = handle.join();
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), PtyError> {
        if let Some(writer) = &mut self.writer {
            writer
                .write_all(data)
                .map_err(|e| PtyError::WriteFailed(e.to_string()))?;
            writer
                .flush()
                .map_err(|e| PtyError::WriteFailed(e.to_string()))?;
            Ok(())
        } else {
            Err(PtyError::NotRunning)
        }
    }

    pub fn write_str(&mut self, s: &str) -> Result<(), PtyError> {
        self.write(s.as_bytes())
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> Result<(), PtyError> {
        self.cols = cols;
        self.rows = rows;

        if let Some(pty_pair) = &self.pty_pair {
            pty_pair
                .master
                .resize(PtySize {
                    rows,
                    cols,
                    pixel_width: 0,
                    pixel_height: 0,
                })
                .map_err(|e| PtyError::ResizeFailed(e.to_string()))?;
        }
        Ok(())
    }

    pub fn drain_output(&self) -> Vec<u8> {
        let mut output = Vec::new();
        while let Ok(data) = self.output_receiver.try_recv() {
            output.extend(data);
        }
        output
    }
}

impl Default for PtyService {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for PtyService {
    fn drop(&mut self) {
        self.stop();
    }
}

fn get_default_shell() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
    }

    #[cfg(target_os = "macos")]
    {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string())
    }

    #[cfg(target_os = "linux")]
    {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string())
    }
}

pub mod key_codes {
    pub const ENTER: &[u8] = b"\r";
    pub const TAB: &[u8] = b"\t";
    pub const BACKSPACE: &[u8] = b"\x7f";
    pub const ESCAPE: &[u8] = b"\x1b";
    pub const DELETE: &[u8] = b"\x1b[3~";

    pub const UP: &[u8] = b"\x1b[A";
    pub const DOWN: &[u8] = b"\x1b[B";
    pub const RIGHT: &[u8] = b"\x1b[C";
    pub const LEFT: &[u8] = b"\x1b[D";

    pub const HOME: &[u8] = b"\x1b[H";
    pub const END: &[u8] = b"\x1b[F";
    pub const PAGE_UP: &[u8] = b"\x1b[5~";
    pub const PAGE_DOWN: &[u8] = b"\x1b[6~";

    pub const CTRL_C: &[u8] = b"\x03";
    pub const CTRL_D: &[u8] = b"\x04";
    pub const CTRL_Z: &[u8] = b"\x1a";
    pub const CTRL_L: &[u8] = b"\x0c";
}
