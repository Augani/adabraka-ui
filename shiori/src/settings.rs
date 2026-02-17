use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShioriSettings {
    #[serde(default)]
    pub lsp_enabled: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default)]
    pub language_servers: HashMap<String, LanguageServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageServerConfig {
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_theme() -> String {
    "Island Dark".into()
}

fn default_true() -> bool {
    true
}

impl Default for ShioriSettings {
    fn default() -> Self {
        Self {
            lsp_enabled: false,
            theme: default_theme(),
            language_servers: default_language_servers(),
        }
    }
}

fn default_language_servers() -> HashMap<String, LanguageServerConfig> {
    let mut map = HashMap::new();
    map.insert(
        "rust".into(),
        LanguageServerConfig {
            command: "rust-analyzer".into(),
            args: vec![],
            enabled: true,
        },
    );
    map.insert(
        "typescript".into(),
        LanguageServerConfig {
            command: "typescript-language-server".into(),
            args: vec!["--stdio".into()],
            enabled: true,
        },
    );
    map.insert(
        "python".into(),
        LanguageServerConfig {
            command: "pyright-langserver".into(),
            args: vec!["--stdio".into()],
            enabled: true,
        },
    );
    map.insert(
        "go".into(),
        LanguageServerConfig {
            command: "gopls".into(),
            args: vec![],
            enabled: true,
        },
    );
    map.insert(
        "c".into(),
        LanguageServerConfig {
            command: "clangd".into(),
            args: vec![],
            enabled: true,
        },
    );
    map.insert(
        "lua".into(),
        LanguageServerConfig {
            command: "lua-language-server".into(),
            args: vec![],
            enabled: true,
        },
    );
    map.insert(
        "zig".into(),
        LanguageServerConfig {
            command: "zls".into(),
            args: vec![],
            enabled: true,
        },
    );
    map.insert(
        "bash".into(),
        LanguageServerConfig {
            command: "bash-language-server".into(),
            args: vec!["start".into()],
            enabled: true,
        },
    );
    map
}

impl ShioriSettings {
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|d| d.join("shiori"))
    }

    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|d| d.join("settings.json"))
    }

    pub fn load() -> Self {
        let path = match Self::config_path() {
            Some(p) => p,
            None => return Self::default(),
        };

        match std::fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) {
        let dir = match Self::config_dir() {
            Some(d) => d,
            None => return,
        };
        let path = match Self::config_path() {
            Some(p) => p,
            None => return,
        };

        if let Err(e) = std::fs::create_dir_all(&dir) {
            eprintln!("Failed to create config dir: {}", e);
            return;
        }

        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&path, json) {
                    eprintln!("Failed to save settings: {}", e);
                }
            }
            Err(e) => eprintln!("Failed to serialize settings: {}", e),
        }
    }

    pub fn server_config_for(&self, language_key: &str) -> Option<&LanguageServerConfig> {
        self.language_servers
            .get(language_key)
            .filter(|c| c.enabled)
    }
}
