use serde::Serialize;

/// Represents a parsed SSH host entry from ~/.ssh/config
#[derive(Debug, Clone, Serialize)]
pub struct SshHost {
    pub name: String,
    pub hostname: String,
    pub port: u16,
    pub user: String,
    pub identity_file: Option<String>,
    pub proxy_jump: Option<String>,
    pub is_1password_agent: bool,
    pub group: SshHostGroup,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SshHostGroup {
    Direct,
    Proxy,
    Local,
    Github,
}
