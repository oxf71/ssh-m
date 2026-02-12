mod blockchain;
mod commands;
mod crypto;
mod onepassword;
mod ssh;
mod tray;

use commands::blockchain::{derive_accounts, get_default_chain_configs, query_balances};
use commands::onepassword::{check_op_status, list_vault_items, list_vaults};
use commands::ssh::{
    list_ssh_config_files, list_ssh_hosts, open_ssh_terminal, read_ssh_config, refresh_ssh_config,
    save_ssh_config, validate_ssh_config,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // SSH commands
            list_ssh_hosts,
            refresh_ssh_config,
            open_ssh_terminal,
            read_ssh_config,
            validate_ssh_config,
            save_ssh_config,
            list_ssh_config_files,
            // 1Password commands
            check_op_status,
            list_vaults,
            list_vault_items,
            // Blockchain commands
            derive_accounts,
            query_balances,
            get_default_chain_configs,
        ])
        .setup(|app| {
            // Setup system tray
            tray::setup_tray(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            // 关闭主窗口时只隐藏到后台，不退出应用
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
