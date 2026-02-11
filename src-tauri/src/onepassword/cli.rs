use std::process::{Command, Stdio};

use super::types::{OpItem, OpItemDetail, OpStatus, OpVault};

/// Create `op` Command with stdin closed to prevent interactive prompts
fn op_cmd() -> Command {
    let mut cmd = Command::new("op");
    cmd.stdin(Stdio::null());
    cmd
}

/// Check if `op` CLI is installed and get its status
pub fn check_op_status() -> OpStatus {
    // Check CLI exists using which crate (no subprocess)
    let cli_installed = which::which("op").is_ok();

    let cli_version = if cli_installed {
        op_cmd()
            .arg("--version")
            .output()
            .ok()
            .and_then(|out| String::from_utf8(out.stdout).ok())
            .map(|v| v.trim().to_string())
    } else {
        None
    };

    // Don't call `op account list` or `op vault list` here — they may trigger
    // interactive prompts via /dev/tty when no accounts are configured.
    // Sign-in status is checked lazily when the user actually lists vaults.

    OpStatus {
        cli_installed,
        cli_version,
        signed_in: false,
        accounts: vec![],
    }
}

/// List all vaults
pub fn list_vaults() -> Result<Vec<OpVault>, String> {
    let output = op_cmd()
        .args(["vault", "list", "--format", "json"])
        .output()
        .map_err(|e| format!("Failed to execute op CLI: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("op vault list failed: {}", stderr));
    }

    let stdout =
        String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 output: {}", e))?;

    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse vault list: {}", e))
}

/// List items in a vault
pub fn list_vault_items(vault_id: &str) -> Result<Vec<OpItem>, String> {
    let output = op_cmd()
        .args(["item", "list", "--vault", vault_id, "--format", "json"])
        .output()
        .map_err(|e| format!("Failed to execute op CLI: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("op item list failed: {}", stderr));
    }

    let stdout =
        String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 output: {}", e))?;

    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse item list: {}", e))
}

/// Get a specific item detail (including fields/values)
pub fn get_item_detail(vault_id: &str, item_id: &str) -> Result<OpItemDetail, String> {
    let output = op_cmd()
        .args([
            "item", "get", item_id, "--vault", vault_id, "--format", "json",
        ])
        .output()
        .map_err(|e| format!("Failed to execute op CLI: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("op item get failed: {}", stderr));
    }

    let stdout =
        String::from_utf8(output.stdout).map_err(|e| format!("Invalid UTF-8 output: {}", e))?;

    serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse item detail: {}", e))
}

/// Read a secret by its reference (e.g., "op://vault/item/field")
pub fn read_secret(reference: &str) -> Result<String, String> {
    let output = op_cmd()
        .args(["read", reference])
        .output()
        .map_err(|e| format!("Failed to execute op CLI: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("op read failed: {}", stderr));
    }

    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("Invalid UTF-8 output: {}", e))
}

/// Read a field from a vault item - tries multiple strategies
pub fn read_item_field(vault: &str, item: &str, field: &str) -> Result<String, String> {
    // Strategy 1: Try op:// reference
    let reference = format!("op://{}/{}/{}", vault, item, field);
    if let Ok(value) = read_secret(&reference) {
        if !value.is_empty() {
            return Ok(value);
        }
    }

    // Strategy 2: Get item detail and search fields
    let detail = get_item_detail(vault, item)?;
    for f in &detail.fields {
        if f.label.to_lowercase() == field.to_lowercase()
            || f.id.to_lowercase() == field.to_lowercase()
        {
            if !f.value.is_empty() {
                return Ok(f.value.clone());
            }
        }
    }

    // Strategy 3: For Secure Notes, the "notesPlain" field contains the content
    if field == "mnemonic" || field == "notes" {
        for f in &detail.fields {
            if f.id == "notesPlain" || f.label == "notesPlain" {
                if !f.value.is_empty() {
                    return Ok(f.value.clone());
                }
            }
        }
    }

    Err(format!(
        "Field '{}' not found in item '{}' vault '{}'",
        field, item, vault
    ))
}
