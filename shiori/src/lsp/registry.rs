use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::thread;

use adabraka_ui::components::editor::Language;

use super::client::LspClient;
use super::config::{discover_server, ServerConfig};
use super::types::FileDiagnostics;
use crate::settings::ShioriSettings;

struct PendingOpen {
    path: PathBuf,
    language_id: String,
    text: String,
}

pub struct LspRegistry {
    clients: HashMap<Language, LspClient>,
    root_path: Option<PathBuf>,
    diagnostics_tx: flume::Sender<FileDiagnostics>,
    diagnostics_rx: flume::Receiver<FileDiagnostics>,
    failed_languages: HashMap<Language, std::time::Instant>,
    pending_starts: HashMap<Language, ()>,
    ready_rx: flume::Receiver<(Language, Result<LspClient, String>)>,
    ready_tx: flume::Sender<(Language, Result<LspClient, String>)>,
    queued_opens: HashMap<Language, Vec<PendingOpen>>,
}

const RETRY_COOLDOWN: std::time::Duration = std::time::Duration::from_secs(60);

impl LspRegistry {
    pub fn new() -> Self {
        let (diagnostics_tx, diagnostics_rx) = flume::unbounded();
        let (ready_tx, ready_rx) = flume::unbounded();
        Self {
            clients: HashMap::new(),
            root_path: None,
            diagnostics_tx,
            diagnostics_rx,
            failed_languages: HashMap::new(),
            pending_starts: HashMap::new(),
            ready_rx,
            ready_tx,
            queued_opens: HashMap::new(),
        }
    }

    pub fn set_root(&mut self, path: PathBuf) {
        self.root_path = Some(path);
    }

    pub fn poll_ready(&mut self) {
        while let Ok((language, result)) = self.ready_rx.try_recv() {
            self.pending_starts.remove(&language);
            match result {
                Ok(client) => {
                    eprintln!("[LSP] Server ready for {:?}", language);
                    self.clients.insert(language, client);
                    if let Some(opens) = self.queued_opens.remove(&language) {
                        if let Some(client) = self.clients.get(&language) {
                            for open in opens {
                                let _ = client.did_open(&open.path, &open.language_id, &open.text);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[LSP] Server failed for {:?}: {}", language, e);
                    self.failed_languages
                        .insert(language, std::time::Instant::now());
                    self.queued_opens.remove(&language);
                }
            }
        }
    }

    pub fn ensure_client_for(&mut self, language: Language, settings: &ShioriSettings) {
        if self.clients.contains_key(&language) || self.pending_starts.contains_key(&language) {
            return;
        }

        if let Some(failed_at) = self.failed_languages.get(&language) {
            if failed_at.elapsed() < RETRY_COOLDOWN {
                return;
            }
            self.failed_languages.remove(&language);
        }

        let root = match self.root_path.clone() {
            Some(r) => r,
            None => {
                eprintln!("[LSP] No root path set for {:?}", language);
                return;
            }
        };
        let config = match self.resolve_config(language, settings) {
            Some(c) => c,
            None => {
                eprintln!("[LSP] No config resolved for {:?}", language);
                return;
            }
        };

        eprintln!(
            "[LSP] Starting {} for {:?} (async)",
            config.command, language
        );
        self.pending_starts.insert(language, ());

        let tx = self.ready_tx.clone();
        thread::spawn(move || {
            let result = match LspClient::start(&config, &root) {
                Ok(mut client) => match client.initialize() {
                    Ok(()) => Ok(client),
                    Err(e) => {
                        client.stop();
                        Err(format!("initialize failed: {}", e))
                    }
                },
                Err(e) => Err(format!("spawn failed: {}", e)),
            };
            let _ = tx.send((language, result));
        });
    }

    pub fn client_for(&self, language: Language) -> Option<&LspClient> {
        self.clients.get(&language)
    }

    pub fn drain_diagnostics(&self) -> Vec<FileDiagnostics> {
        let mut all = Vec::new();
        for client in self.clients.values() {
            while let Ok(diag) = client.diagnostics_rx().try_recv() {
                all.push(diag);
            }
        }
        while let Ok(diag) = self.diagnostics_rx.try_recv() {
            all.push(diag);
        }
        all
    }

    pub fn notify_did_open(
        &mut self,
        language: Language,
        path: &Path,
        text: &str,
        settings: &ShioriSettings,
    ) {
        self.ensure_client_for(language, settings);

        if let Some(client) = self.clients.get(&language) {
            let lang_id = language_id_str(language);
            let _ = client.did_open(path, lang_id, text);
        } else if self.pending_starts.contains_key(&language) {
            self.queued_opens
                .entry(language)
                .or_default()
                .push(PendingOpen {
                    path: path.to_path_buf(),
                    language_id: language_id_str(language).to_string(),
                    text: text.to_string(),
                });
        }
    }

    pub fn notify_did_change(&self, language: Language, path: &Path, text: &str, version: i32) {
        if let Some(client) = self.clients.get(&language) {
            let _ = client.did_change(path, text, version);
        }
    }

    pub fn notify_did_save(&self, language: Language, path: &Path) {
        if let Some(client) = self.clients.get(&language) {
            let _ = client.did_save(path);
        }
    }

    pub fn notify_did_close(&self, language: Language, path: &Path) {
        if let Some(client) = self.clients.get(&language) {
            let _ = client.did_close(path);
        }
    }

    pub fn has_client_for(&self, language: Language) -> bool {
        self.clients.contains_key(&language)
    }

    pub fn is_starting(&self, language: Language) -> bool {
        self.pending_starts.contains_key(&language)
    }

    pub fn active_languages(&self) -> Vec<Language> {
        self.clients.keys().copied().collect()
    }

    pub fn pending_languages(&self) -> Vec<Language> {
        self.pending_starts.keys().copied().collect()
    }

    pub fn stop_all(&mut self) {
        for (_, mut client) in self.clients.drain() {
            client.stop();
        }
        self.failed_languages.clear();
        self.pending_starts.clear();
        self.queued_opens.clear();
    }

    pub fn restart_language(&mut self, language: Language, settings: &ShioriSettings) {
        if let Some(mut client) = self.clients.remove(&language) {
            client.stop();
        }
        self.failed_languages.remove(&language);
        self.pending_starts.remove(&language);
        self.ensure_client_for(language, settings);
    }

    fn resolve_config(
        &self,
        language: Language,
        settings: &ShioriSettings,
    ) -> Option<ServerConfig> {
        let lang_key = language_key(language);

        if let Some(user_config) = settings.server_config_for(lang_key) {
            if which::which(&user_config.command).is_ok() {
                return Some(ServerConfig {
                    command: user_config.command.clone(),
                    args: user_config.args.clone(),
                });
            }
        }

        discover_server(language)
    }
}

impl Drop for LspRegistry {
    fn drop(&mut self) {
        self.stop_all();
    }
}

fn language_id_str(lang: Language) -> &'static str {
    match lang {
        Language::Rust => "rust",
        Language::JavaScript => "javascript",
        Language::TypeScript => "typescript",
        Language::Python => "python",
        Language::Go => "go",
        Language::C => "c",
        Language::Cpp => "cpp",
        Language::Java => "java",
        Language::Ruby => "ruby",
        Language::Bash => "shellscript",
        Language::Css => "css",
        Language::Html => "html",
        Language::Json => "json",
        Language::Toml => "toml",
        Language::Yaml => "yaml",
        Language::Markdown => "markdown",
        Language::Lua => "lua",
        Language::Zig => "zig",
        Language::Scala => "scala",
        Language::Php => "php",
        Language::OCaml => "ocaml",
        Language::Sql => "sql",
        Language::Plain => "plaintext",
    }
}

pub fn language_key(lang: Language) -> &'static str {
    match lang {
        Language::Rust => "rust",
        Language::JavaScript => "javascript",
        Language::TypeScript => "typescript",
        Language::Python => "python",
        Language::Go => "go",
        Language::C | Language::Cpp => "c",
        Language::Java => "java",
        Language::Ruby => "ruby",
        Language::Bash => "bash",
        Language::Css => "css",
        Language::Html => "html",
        Language::Lua => "lua",
        Language::Zig => "zig",
        _ => "other",
    }
}
