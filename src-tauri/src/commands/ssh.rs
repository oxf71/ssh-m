use crate::settings::{self, AppSettings};
use crate::ssh::config::{parse_ssh_config, ssh_config_path};
use crate::ssh::types::SshHost;
use glob::glob;
use serde::Serialize;
use ssh2_config::{ParseRule, SshConfig};
use std::collections::HashSet;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Clone)]
pub struct SshConfigFile {
    /// Display name (relative to ~/.ssh/)
    pub name: String,
    /// Absolute path
    pub path: String,
    /// Whether this is the main config file
    pub is_main: bool,
    /// Number of Host entries in this file
    pub host_count: usize,
}

/// Expand an Include pattern relative to ~/.ssh/
fn expand_include_pattern(pattern: &str) -> Vec<PathBuf> {
    let home = dirs::home_dir().unwrap_or_default();
    let ssh_dir = home.join(".ssh");

    let full_pattern = if pattern.starts_with('/') || pattern.starts_with('~') {
        pattern.replace('~', &home.to_string_lossy())
    } else {
        ssh_dir.join(pattern).to_string_lossy().to_string()
    };

    let mut files = Vec::new();
    if let Ok(paths) = glob(&full_pattern) {
        for entry in paths.flatten() {
            if entry.is_file() {
                files.push(entry);
            }
        }
    }
    files.sort();
    files
}

/// Parse Include directives from a config file and return all referenced files recursively
fn collect_config_files(path: &Path, visited: &mut HashSet<PathBuf>) -> Vec<PathBuf> {
    let canonical = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    if visited.contains(&canonical) || !path.exists() {
        return Vec::new();
    }
    visited.insert(canonical);

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let mut result = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.to_lowercase().starts_with("include") {
            let parts: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
            if parts.len() == 2 {
                let pattern = parts[1].trim();
                for inc_path in expand_include_pattern(pattern) {
                    result.push(inc_path.clone());
                    // Recurse into included files
                    result.extend(collect_config_files(&inc_path, visited));
                }
            }
        }
    }
    result
}

/// Count Host entries in a file
fn count_hosts(content: &str) -> usize {
    content
        .lines()
        .filter(|l| {
            let t = l.trim().to_lowercase();
            t.starts_with("host ") && !t.starts_with("hostname")
        })
        .count()
}

#[tauri::command]
pub fn list_ssh_config_files() -> Result<Vec<SshConfigFile>, String> {
    let main_path = ssh_config_path();
    if !main_path.exists() {
        return Err(format!("SSH config not found at: {}", main_path.display()));
    }

    let home = dirs::home_dir().unwrap_or_default();
    let ssh_dir = home.join(".ssh");
    let make_name = |p: &Path| -> String {
        p.strip_prefix(&ssh_dir)
            .map(|r| r.to_string_lossy().to_string())
            .unwrap_or_else(|_| p.to_string_lossy().to_string())
    };

    let mut files = Vec::new();

    // Main config
    let main_content =
        fs::read_to_string(&main_path).map_err(|e| format!("Failed to read SSH config: {}", e))?;
    files.push(SshConfigFile {
        name: "config".to_string(),
        path: main_path.to_string_lossy().to_string(),
        is_main: true,
        host_count: count_hosts(&main_content),
    });

    // Collect included files
    let mut visited = HashSet::new();
    let includes = collect_config_files(&main_path, &mut visited);
    for inc_path in includes {
        let content = fs::read_to_string(&inc_path).unwrap_or_default();
        files.push(SshConfigFile {
            name: make_name(&inc_path),
            path: inc_path.to_string_lossy().to_string(),
            is_main: false,
            host_count: count_hosts(&content),
        });
    }

    Ok(files)
}

#[tauri::command]
pub fn list_ssh_hosts() -> Result<Vec<SshHost>, String> {
    parse_ssh_config()
}

#[tauri::command]
pub fn refresh_ssh_config() -> Result<Vec<SshHost>, String> {
    parse_ssh_config()
}

#[tauri::command]
pub fn open_ssh_terminal(host: String, terminal: Option<String>) -> Result<(), String> {
    let terminal = terminal.unwrap_or_else(|| "terminal".to_string());

    #[cfg(target_os = "macos")]
    {
        if terminal == "warp" {
            // Warp doesn't support AppleScript `do script`, and using
            // System Events keystroke requires Accessibility permissions.
            // Use a self-deleting .command file instead.
            let tmp_path = format!("/tmp/ssh-m-connect-{}.command", std::process::id());
            let script_content = format!("#!/bin/bash\nexec ssh {}\n", host);
            std::fs::write(&tmp_path, &script_content)
                .map_err(|e| format!("Failed to create temp script: {}", e))?;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))
                    .map_err(|e| format!("Failed to set script permissions: {}", e))?;
            }
            std::process::Command::new("open")
                .args(["-a", "Warp", &tmp_path])
                .spawn()
                .map_err(|e| format!("Failed to open Warp: {}", e))?;
        } else {
            let script = match terminal.as_str() {
                "iterm" => format!(
                    r#"
                    tell application "iTerm"
                        activate
                        create window with default profile command "ssh {}"
                    end tell
                    "#,
                    host
                ),
                _ => format!(
                    r#"
                    tell application "Terminal"
                        activate
                        do script "ssh {}"
                    end tell
                    "#,
                    host
                ),
            };

            std::process::Command::new("osascript")
                .arg("-e")
                .arg(&script)
                .spawn()
                .map_err(|e| format!("Failed to open terminal: {}", e))?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        let _ = &terminal; // suppress unused warning
        let terminals = ["gnome-terminal", "konsole", "xterm", "x-terminal-emulator"];
        let mut launched = false;

        for term in &terminals {
            if std::process::Command::new(term)
                .args(["--", "ssh", &host])
                .spawn()
                .is_ok()
            {
                launched = true;
                break;
            }
        }

        if !launched {
            return Err("No terminal emulator found".to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        let _ = &terminal; // suppress unused warning
        std::process::Command::new("cmd")
            .args(["/c", "start", "ssh", &host])
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn read_ssh_config(path: Option<String>) -> Result<String, String> {
    let config_path = match path {
        Some(p) => PathBuf::from(p),
        None => ssh_config_path(),
    };
    if !config_path.exists() {
        return Err(format!(
            "SSH config not found at: {}",
            config_path.display()
        ));
    }
    // Security: only allow reading files under ~/.ssh/
    let home = dirs::home_dir().unwrap_or_default();
    let ssh_dir = home.join(".ssh");
    let canonical = config_path
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    if !canonical.starts_with(&ssh_dir) {
        return Err("Only files under ~/.ssh/ can be read".to_string());
    }
    fs::read_to_string(&canonical).map_err(|e| format!("Failed to read SSH config: {}", e))
}

#[tauri::command]
pub fn validate_ssh_config(content: String) -> Result<Vec<String>, String> {
    let mut warnings = Vec::new();

    // Known SSH config keywords (case-insensitive)
    let known_keywords: HashSet<&str> = [
        "host",
        "match",
        "hostname",
        "user",
        "port",
        "identityfile",
        "identitiesonly",
        "forwardagent",
        "proxyjump",
        "proxycommand",
        "localforward",
        "remoteforward",
        "serveraliveinterval",
        "serveralivecountmax",
        "stricthostkeychecking",
        "userknownhostsfile",
        "loglevel",
        "compression",
        "connecttimeout",
        "connectionattempts",
        "addkeystoagent",
        "identityagent",
        "pubkeyauthentication",
        "preferredauthentications",
        "batchmode",
        "checkhostip",
        "ciphers",
        "controlmaster",
        "controlpath",
        "controlpersist",
        "dynamicforward",
        "escapechar",
        "exitonforwardfailure",
        "fingerprinthash",
        "gatewayports",
        "globalknownhostsfile",
        "gssapiauthentication",
        "gssapidelegatecredentials",
        "hashknownhosts",
        "hostbasedauthentication",
        "hostkeyalgorithms",
        "hostkeyalias",
        "include",
        "ipqos",
        "kbdinteractiveauthentication",
        "kexalgorithms",
        "localcommand",
        "macs",
        "numberofpasswordprompts",
        "passwordauthentication",
        "permitlocalcommand",
        "pkcs11provider",
        "protocol",
        "pubkeyacceptedalgorithms",
        "rekeylimit",
        "requesttty",
        "sendenv",
        "setenv",
        "tcpkeepalive",
        "tunnel",
        "tunneldevice",
        "updatehostkeys",
        "verifyhostkeydns",
        "visualhostkey",
        "xauthlocation",
        "canonicaldomains",
        "canonicalizefallbacklocal",
        "canonicalizehostname",
        "canonicalizemaxdots",
        "canonicalizepermittedcnames",
        "casignaturealgorithms",
        "certificatefile",
        "addressfamily",
        "bindaddress",
        "bindinterface",
        "canonicaldomains",
        "nohostauthenticationforlocalhost",
        "permitremoteopen",
        "proxyusefdpass",
        "revokedhostkeys",
        "securitykeyprovider",
        "streamlocalbindmask",
        "streamlocalbindunlink",
        "syslogfacility",
        "tag",
        "ignorehostkeys",
    ]
    .iter()
    .copied()
    .collect();

    let bool_keywords: HashSet<&str> = [
        "forwardagent",
        "identitiesonly",
        "compression",
        "batchmode",
        "checkhostip",
        "exitonforwardfailure",
        "gatewayports",
        "gssapiauthentication",
        "gssapidelegatecredentials",
        "hashknownhosts",
        "hostbasedauthentication",
        "passwordauthentication",
        "permitlocalcommand",
        "pubkeyauthentication",
        "tcpkeepalive",
        "visualhostkey",
        "canonicalizefallbacklocal",
        "nohostauthenticationforlocalhost",
        "streamlocalbindunlink",
    ]
    .iter()
    .copied()
    .collect();

    // Line-by-line validation
    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Split on whitespace or '='
        let parts: Vec<&str> = trimmed
            .splitn(2, |c: char| c.is_whitespace() || c == '=')
            .collect();
        let keyword = parts[0];
        let keyword_lower = keyword.to_lowercase();

        // Check if keyword is known
        if !known_keywords.contains(keyword_lower.as_str()) {
            warnings.push(format!("第 {} 行: 未知指令 \"{}\"", i + 1, keyword));
        }

        // Check for missing value
        let value = parts.get(1).map(|v| v.trim()).unwrap_or("");
        if value.is_empty() && !keyword_lower.eq("match") {
            warnings.push(format!("第 {} 行: \"{}\" 缺少值", i + 1, keyword));
            continue;
        }

        // Validate Port range
        if keyword_lower == "port" {
            match value.parse::<u16>() {
                Ok(0) => warnings.push(format!("第 {} 行: Port 不能为 0", i + 1)),
                Err(_) => warnings.push(format!(
                    "第 {} 行: Port 值 \"{}\" 不是有效端口号 (1-65535)",
                    i + 1,
                    value
                )),
                _ => {}
            }
        }

        // Validate ServerAliveInterval / ServerAliveCountMax / ConnectTimeout
        if [
            "serveraliveinterval",
            "serveralivecountmax",
            "connecttimeout",
            "connectionattempts",
            "numberofpasswordprompts",
        ]
        .contains(&keyword_lower.as_str())
        {
            if value.parse::<u32>().is_err() {
                warnings.push(format!(
                    "第 {} 行: {} 值 \"{}\" 应为正整数",
                    i + 1,
                    keyword,
                    value
                ));
            }
        }

        // Validate boolean fields
        if bool_keywords.contains(keyword_lower.as_str()) {
            if !["yes", "no"].contains(&value.to_lowercase().as_str()) {
                warnings.push(format!(
                    "第 {} 行: {} 值 \"{}\" 应为 yes 或 no",
                    i + 1,
                    keyword,
                    value
                ));
            }
        }

        // Validate LogLevel
        if keyword_lower == "loglevel" {
            let valid_levels = [
                "quiet", "fatal", "error", "info", "verbose", "debug", "debug1", "debug2", "debug3",
            ];
            if !valid_levels.contains(&value.to_lowercase().as_str()) {
                warnings.push(format!(
                    "第 {} 行: LogLevel \"{}\" 不是有效级别",
                    i + 1,
                    value
                ));
            }
        }

        // Validate AddKeysToAgent
        if keyword_lower == "addkeystoagent" {
            let valid = ["yes", "no", "confirm", "ask"];
            if !valid.contains(&value.to_lowercase().as_str()) && value.parse::<u32>().is_err() {
                warnings.push(format!(
                    "第 {} 行: AddKeysToAgent \"{}\" 应为 yes/no/confirm/ask 或秒数",
                    i + 1,
                    value
                ));
            }
        }

        // Validate StrictHostKeyChecking
        if keyword_lower == "stricthostkeychecking" {
            let valid = ["yes", "no", "ask", "accept-new", "off"];
            if !valid.contains(&value.to_lowercase().as_str()) {
                warnings.push(format!(
                    "第 {} 行: StrictHostKeyChecking \"{}\" 应为 yes/no/ask/accept-new/off",
                    i + 1,
                    value
                ));
            }
        }

        // Validate RequestTTY
        if keyword_lower == "requesttty" {
            let valid = ["yes", "no", "force", "auto"];
            if !valid.contains(&value.to_lowercase().as_str()) {
                warnings.push(format!(
                    "第 {} 行: RequestTTY \"{}\" 应为 yes/no/force/auto",
                    i + 1,
                    value
                ));
            }
        }

        // Validate ControlMaster
        if keyword_lower == "controlmaster" {
            let valid = ["yes", "no", "ask", "auto", "autoask"];
            if !valid.contains(&value.to_lowercase().as_str()) {
                warnings.push(format!(
                    "第 {} 行: ControlMaster \"{}\" 应为 yes/no/ask/auto/autoask",
                    i + 1,
                    value
                ));
            }
        }
    }

    // Also parse with ssh2-config for structural validation
    let mut reader = BufReader::new(content.as_bytes());
    match SshConfig::default().parse(&mut reader, ParseRule::ALLOW_UNKNOWN_FIELDS) {
        Err(e) => {
            let msg = format!("{}", e);
            // Add as error - structural parse failure is fatal
            return Err(format!("SSH 配置结构错误: {}", msg));
        }
        Ok(_) => {}
    }

    Ok(warnings)
}

#[tauri::command]
pub fn save_ssh_config(content: String, path: Option<String>) -> Result<Vec<String>, String> {
    // Validate first
    let warnings = validate_ssh_config(content.clone())?;

    let config_path = match path {
        Some(p) => PathBuf::from(p),
        None => ssh_config_path(),
    };

    // Security: only allow writing files under ~/.ssh/
    let home = dirs::home_dir().unwrap_or_default();
    let ssh_dir = home.join(".ssh");
    let canonical = config_path
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;
    if !canonical.starts_with(&ssh_dir) {
        return Err("Only files under ~/.ssh/ can be saved".to_string());
    }

    // Create backup
    if config_path.exists() {
        let backup_name = format!(
            "{}.bak",
            config_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );
        let backup = config_path.with_file_name(backup_name);
        fs::copy(&config_path, &backup).map_err(|e| format!("Failed to create backup: {}", e))?;
    }

    fs::write(&canonical, &content).map_err(|e| format!("Failed to save SSH config: {}", e))?;

    Ok(warnings)
}

#[tauri::command]
pub fn save_app_settings(default_terminal: String, ssh_config_path: String) -> Result<(), String> {
    let s = AppSettings {
        default_terminal,
        ssh_config_path,
    };
    settings::save_settings_to_file(&s)
}

#[tauri::command]
pub fn get_app_settings() -> Result<AppSettings, String> {
    Ok(settings::load_settings())
}
