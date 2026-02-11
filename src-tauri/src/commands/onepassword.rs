use crate::onepassword::cli;
use crate::onepassword::types::OpStatus;

use serde::Serialize;

/// Serializable vault for frontend
#[derive(Serialize)]
pub struct FrontendVault {
    pub id: String,
    pub name: String,
}

/// Serializable vault item for frontend
#[derive(Serialize)]
pub struct FrontendVaultItem {
    pub id: String,
    pub title: String,
    pub category: String,
    pub vault_id: String,
    pub vault_name: String,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[tauri::command]
pub fn check_op_status() -> Result<OpStatus, String> {
    Ok(cli::check_op_status())
}

#[tauri::command]
pub fn list_vaults() -> Result<Vec<FrontendVault>, String> {
    let vaults = cli::list_vaults()?;
    Ok(vaults
        .into_iter()
        .map(|v| FrontendVault {
            id: v.id,
            name: v.name,
        })
        .collect())
}

#[tauri::command]
pub fn list_vault_items(vault_id: String) -> Result<Vec<FrontendVaultItem>, String> {
    let items = cli::list_vault_items(&vault_id)?;
    Ok(items
        .into_iter()
        .map(|item| FrontendVaultItem {
            id: item.id,
            title: item.title,
            category: item.category,
            vault_id: item.vault.id,
            vault_name: item.vault.name,
            tags: item.tags,
            created_at: item.created_at,
            updated_at: item.updated_at,
        })
        .collect())
}
