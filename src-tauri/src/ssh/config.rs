use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

use ssh2_config::{ParseRule, SshConfig};

use super::types::{SshHost, SshHostGroup};

/// Get the path to the user's SSH config file
pub fn ssh_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("Cannot determine home directory");
    home.join(".ssh").join("config")
}

/// Parse ~/.ssh/config and return a list of SSH hosts
pub fn parse_ssh_config() -> Result<Vec<SshHost>, String> {
    let config_path = ssh_config_path();
    if !config_path.exists() {
        return Err(format!(
            "SSH config not found at: {}",
            config_path.display()
        ));
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read SSH config: {}", e))?;

    // Also read to check for 1Password agent globally
    let has_global_1password_agent =
        content.contains("agent.sock") && content.contains("1password");

    // Parse using ssh2-config crate
    let mut reader = BufReader::new(content.as_bytes());
    let config = SshConfig::default()
        .parse(&mut reader, ParseRule::ALLOW_UNKNOWN_FIELDS)
        .map_err(|e| format!("Failed to parse SSH config: {}", e))?;

    // We also manually parse to get host names since ssh2-config
    // only provides query-based access
    let hosts = parse_host_entries(&content, &config, has_global_1password_agent);

    Ok(hosts)
}

/// Manually parse host entries from config content
/// since ssh2-config doesn't expose the host list directly
fn parse_host_entries(
    content: &str,
    _config: &SshConfig,
    has_global_1password_agent: bool,
) -> Vec<SshHost> {
    let mut hosts = Vec::new();
    let mut current_host: Option<String> = None;
    let mut current_hostname = String::new();
    let mut current_port: u16 = 22;
    let mut current_user = String::new();
    let mut current_identity_file: Option<String> = None;
    let mut current_proxy_jump: Option<String> = None;
    let mut current_has_1p_agent = false;

    let home = dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Handle Include directives - skip for now
        if trimmed.to_lowercase().starts_with("include") {
            continue;
        }

        if trimmed.to_lowercase().starts_with("host ")
            && !trimmed.to_lowercase().starts_with("hostname")
        {
            // Save previous host if any
            if let Some(name) = current_host.take() {
                if name != "*" && !name.contains('*') && !name.contains('?') {
                    hosts.push(build_ssh_host(
                        &name,
                        &current_hostname,
                        current_port,
                        &current_user,
                        current_identity_file.take(),
                        current_proxy_jump.take(),
                        current_has_1p_agent || has_global_1password_agent,
                    ));
                }
            }

            // Start new host
            let host_name = trimmed[5..].trim().to_string();
            current_host = Some(host_name.clone());
            current_hostname = host_name;
            current_port = 22;
            current_user = String::new();
            current_identity_file = None;
            current_proxy_jump = None;
            current_has_1p_agent = false;
        } else if let Some(ref _host) = current_host {
            let lower = trimmed.to_lowercase();
            if lower.starts_with("hostname") {
                current_hostname = extract_value(trimmed);
            } else if lower.starts_with("port") {
                current_port = extract_value(trimmed).parse().unwrap_or(22);
            } else if lower.starts_with("user ") {
                current_user = extract_value(trimmed);
            } else if lower.starts_with("identityfile") {
                let path = extract_value(trimmed).replace('~', &home);
                current_identity_file = Some(path);
            } else if lower.starts_with("proxyjump") {
                current_proxy_jump = Some(extract_value(trimmed));
            } else if lower.starts_with("identityagent") && lower.contains("1password") {
                current_has_1p_agent = true;
            }
        } else {
            // Global config (Host *)
            let lower = trimmed.to_lowercase();
            if lower.starts_with("user ") && current_user.is_empty() {
                // Will apply to all subsequent hosts without explicit user
            }
        }
    }

    // Save last host
    if let Some(name) = current_host.take() {
        if name != "*" && !name.contains('*') && !name.contains('?') {
            hosts.push(build_ssh_host(
                &name,
                &current_hostname,
                current_port,
                &current_user,
                current_identity_file,
                current_proxy_jump,
                current_has_1p_agent || has_global_1password_agent,
            ));
        }
    }

    hosts
}

fn extract_value(line: &str) -> String {
    // Handle both "Key Value" and "Key=Value" formats
    let parts: Vec<&str> = if line.contains('=') {
        line.splitn(2, '=').collect()
    } else {
        line.splitn(2, char::is_whitespace).collect()
    };

    if parts.len() > 1 {
        parts[1].trim().trim_matches('"').to_string()
    } else {
        String::new()
    }
}

fn build_ssh_host(
    name: &str,
    hostname: &str,
    port: u16,
    user: &str,
    identity_file: Option<String>,
    proxy_jump: Option<String>,
    is_1password_agent: bool,
) -> SshHost {
    let group = determine_group(name, hostname, &proxy_jump);

    SshHost {
        name: name.to_string(),
        hostname: hostname.to_string(),
        port,
        user: user.to_string(),
        identity_file,
        proxy_jump,
        is_1password_agent,
        group,
    }
}

fn determine_group(name: &str, hostname: &str, proxy_jump: &Option<String>) -> SshHostGroup {
    let lower_name = name.to_lowercase();
    let lower_host = hostname.to_lowercase();

    if lower_name.contains("github") || lower_host.contains("github") {
        SshHostGroup::Github
    } else if proxy_jump.is_some() {
        SshHostGroup::Proxy
    } else if lower_host.ends_with(".local")
        || lower_host.starts_with("192.168.")
        || lower_host.starts_with("10.")
        || lower_host == "localhost"
        || lower_host == "127.0.0.1"
    {
        SshHostGroup::Local
    } else {
        SshHostGroup::Direct
    }
}
