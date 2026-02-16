use portable_pty::{native_pty_system, CommandBuilder, PtyPair, PtySize};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
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

        cmd.env("TERM", "xterm-256color");
        cmd.env("COLORTERM", "truecolor");
        cmd.env("TERM_PROGRAM", "Shiori");
        cmd.env("TERM_PROGRAM_VERSION", "0.1.0");

        if let Ok(lang) = std::env::var("LANG") {
            cmd.env("LANG", lang);
        } else {
            cmd.env("LANG", "en_US.UTF-8");
        }
        if let Ok(lc) = std::env::var("LC_ALL") {
            cmd.env("LC_ALL", lc);
        }

        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", &home);
        }
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
        if let Ok(user) = std::env::var("USER") {
            cmd.env("USER", user);
        }
        if let Ok(shell_env) = std::env::var("SHELL") {
            cmd.env("SHELL", shell_env);
        }

        setup_shell_prompt(&mut cmd, &shell);

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

fn setup_shell_prompt(cmd: &mut CommandBuilder, shell: &str) {
    let dir = std::env::temp_dir().join("shiori_shell");
    if std::fs::create_dir_all(&dir).is_err() {
        return;
    }

    if shell.contains("zsh") {
        setup_zsh_prompt(cmd, &dir);
    } else if shell.contains("bash") {
        setup_bash_prompt(cmd, &dir);
    }
}

fn setup_zsh_prompt(cmd: &mut CommandBuilder, dir: &Path) {
    let zshenv = dir.join(".zshenv");
    let _ = std::fs::write(
        &zshenv,
        r#"# Shiori: source user's zshenv
[[ -f "$HOME/.zshenv" ]] && ZDOTDIR="$HOME" source "$HOME/.zshenv"
"#,
    );

    let zshrc = dir.join(".zshrc");
    let _ = std::fs::write(&zshrc, ZSH_PROMPT_INIT);

    cmd.env("ZDOTDIR", dir.to_string_lossy().as_ref());
}

fn setup_bash_prompt(cmd: &mut CommandBuilder, dir: &Path) {
    let bashrc = dir.join(".bashrc");
    let _ = std::fs::write(&bashrc, BASH_PROMPT_INIT);
    cmd.env("SHIORI_BASH_INIT", bashrc.to_string_lossy().as_ref());
}

const ZSH_PROMPT_INIT: &str = r#"# Shiori Terminal - Custom Shell Theme
# Source user's zshrc first
_shiori_zdotdir="$ZDOTDIR"
ZDOTDIR="$HOME"
[[ -f "$HOME/.zshrc" ]] && source "$HOME/.zshrc"
ZDOTDIR="$_shiori_zdotdir"
unset _shiori_zdotdir

# ── Shiori Prompt ──────────────────────────────────────────
autoload -Uz vcs_info
precmd_functions+=(vcs_info)

zstyle ':vcs_info:git:*' formats '%b'
zstyle ':vcs_info:*' enable git

setopt PROMPT_SUBST

_shiori_git_info() {
  if [[ -n "$vcs_info_msg_0_" ]]; then
    local branch="$vcs_info_msg_0_"
    local git_status=""
    local dirty=$(command git status --porcelain 2>/dev/null | head -1)
    if [[ -n "$dirty" ]]; then
      git_status=" %F{#e0af68}✦%f"
    fi
    echo " %F{#565f89}on%f %F{#bb9af7}⎇ ${branch}%f${git_status}"
  fi
}

PROMPT=$'%F{#565f89}╭─%f %F{#7aa2f7}%~%f$(_shiori_git_info)\n%F{#565f89}╰─%f%(?.%F{#7dcfff}.%F{#f7768e})❯%f '
RPROMPT=''
"#;

const BASH_PROMPT_INIT: &str = r#"# Shiori Terminal - Custom Shell Theme
# Source user's bashrc first
[[ -f "$HOME/.bashrc" ]] && source "$HOME/.bashrc"

# ── Shiori Prompt ──────────────────────────────────────────
__shiori_prompt() {
  local exit_code=$?
  local dim='\[\e[38;2;86;95;137m\]'
  local blue='\[\e[38;2;122;162;247m\]'
  local purple='\[\e[38;2;187;154;247m\]'
  local yellow='\[\e[38;2;224;175;104m\]'
  local cyan='\[\e[38;2;125;207;255m\]'
  local red='\[\e[38;2;247;118;142m\]'
  local reset='\[\e[0m\]'

  local git_info=""
  local branch=$(git branch --show-current 2>/dev/null)
  if [[ -n "$branch" ]]; then
    local dirty=$(git status --porcelain 2>/dev/null | head -1)
    local status_icon=""
    [[ -n "$dirty" ]] && status_icon=" ${yellow}✦${reset}"
    git_info=" ${dim}on${reset} ${purple}⎇ ${branch}${reset}${status_icon}"
  fi

  local arrow=$cyan
  [[ $exit_code -ne 0 ]] && arrow=$red

  PS1="\n${dim}╭─${reset} ${blue}\w${reset}${git_info}\n${dim}╰─${reset}${arrow}❯${reset} "
}

PROMPT_COMMAND=__shiori_prompt
"#;

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
}
