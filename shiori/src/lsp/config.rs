use adabraka_ui::components::editor::Language;

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub command: String,
    pub args: Vec<String>,
}

pub fn default_server_for(language: Language) -> Option<ServerConfig> {
    match language {
        Language::Rust => Some(ServerConfig {
            command: "rust-analyzer".into(),
            args: vec![],
        }),
        Language::TypeScript | Language::JavaScript => Some(ServerConfig {
            command: "typescript-language-server".into(),
            args: vec!["--stdio".into()],
        }),
        Language::Python => Some(ServerConfig {
            command: "pyright-langserver".into(),
            args: vec!["--stdio".into()],
        }),
        Language::Go => Some(ServerConfig {
            command: "gopls".into(),
            args: vec![],
        }),
        Language::C | Language::Cpp => Some(ServerConfig {
            command: "clangd".into(),
            args: vec![],
        }),
        Language::Lua => Some(ServerConfig {
            command: "lua-language-server".into(),
            args: vec![],
        }),
        Language::Zig => Some(ServerConfig {
            command: "zls".into(),
            args: vec![],
        }),
        Language::Bash => Some(ServerConfig {
            command: "bash-language-server".into(),
            args: vec!["start".into()],
        }),
        Language::Java => Some(ServerConfig {
            command: "jdtls".into(),
            args: vec![],
        }),
        Language::Ruby => Some(ServerConfig {
            command: "solargraph".into(),
            args: vec!["stdio".into()],
        }),
        Language::Css => Some(ServerConfig {
            command: "css-languageserver".into(),
            args: vec!["--stdio".into()],
        }),
        Language::Html => Some(ServerConfig {
            command: "html-languageserver".into(),
            args: vec!["--stdio".into()],
        }),
        _ => None,
    }
}

pub fn discover_server(language: Language) -> Option<ServerConfig> {
    let config = default_server_for(language)?;
    if which::which(&config.command).is_ok() {
        Some(config)
    } else {
        try_fallback(language)
    }
}

fn try_fallback(language: Language) -> Option<ServerConfig> {
    let fallbacks: &[(&str, &[&str])] = match language {
        Language::Python => &[("pylsp", &[]), ("python-lsp-server", &[])],
        Language::TypeScript | Language::JavaScript => &[("vtsls", &["--stdio"])],
        _ => return None,
    };

    for (cmd, args) in fallbacks {
        if which::which(cmd).is_ok() {
            return Some(ServerConfig {
                command: cmd.to_string(),
                args: args.iter().map(|s| s.to_string()).collect(),
            });
        }
    }
    None
}
