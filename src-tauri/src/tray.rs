use tauri::{
    image::Image,
    menu::{MenuBuilder, SubmenuBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager, Wry,
};

use crate::ssh::config::parse_ssh_config;
use crate::ssh::types::SshHostGroup;

/// Build and attach the system tray icon with SSH host menu.
/// Called once during app setup.
pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let tray_menu = build_tray_menu(app)?;

    TrayIconBuilder::with_id("ssh-m-tray")
        .icon(load_tray_icon()?)
        .menu(&tray_menu)
        .tooltip("SSH-M – 快速连接")
        .show_menu_on_left_click(true)
        .on_menu_event(move |app: &AppHandle<Wry>, event| {
            let id = event.id().as_ref();
            if id == "quit" {
                app.exit(0);
            } else if id == "show" {
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            } else if id == "refresh" {
                if let Ok(menu) = build_tray_menu(app) {
                    if let Some(tray) = app.tray_by_id("ssh-m-tray") {
                        let _ = tray.set_menu(Some(menu));
                    }
                }
            } else if id.starts_with("ssh:") {
                let host_name = &id[4..];
                let _ = open_ssh_from_tray(host_name);
            }
        })
        .build(app)?;

    Ok(())
}

/// Build the tray context menu with SSH hosts grouped by category.
fn build_tray_menu(
    app: &AppHandle,
) -> Result<tauri::menu::Menu<tauri::Wry>, Box<dyn std::error::Error>> {
    let hosts = parse_ssh_config().unwrap_or_default();

    // Group hosts
    let mut direct: Vec<_> = Vec::new();
    let mut proxy: Vec<_> = Vec::new();
    let mut local: Vec<_> = Vec::new();
    let mut github: Vec<_> = Vec::new();

    for h in &hosts {
        match h.group {
            SshHostGroup::Direct => direct.push(h),
            SshHostGroup::Proxy => proxy.push(h),
            SshHostGroup::Local => local.push(h),
            SshHostGroup::Github => github.push(h),
        }
    }

    let mut menu_builder = MenuBuilder::new(app);

    // Title
    menu_builder = menu_builder.text("title", format!("SSH-M ({} 台主机)", hosts.len()));
    menu_builder = menu_builder.separator();

    // Helper to add a group submenu
    let groups: Vec<(&str, &str, &Vec<&crate::ssh::types::SshHost>)> = vec![
        ("direct", "直连", &direct),
        ("proxy", "代理跳板", &proxy),
        ("local", "本地/隧道", &local),
        ("github", "GitHub", &github),
    ];

    for (_, label, group_hosts) in &groups {
        if group_hosts.is_empty() {
            continue;
        }

        let mut submenu = SubmenuBuilder::new(app, format!("{} ({})", label, group_hosts.len()));

        for host in *group_hosts {
            let detail = if host.user.is_empty() || host.user == "root" {
                format!("{}  →  {}", host.name, host.hostname)
            } else {
                format!("{}  →  {}@{}", host.name, host.user, host.hostname)
            };
            submenu = submenu.text(format!("ssh:{}", host.name), detail);
        }

        menu_builder = menu_builder.item(&submenu.build()?);
    }

    // If there are few hosts total (≤ 8), also list them flat at top level
    if hosts.len() <= 8 {
        menu_builder = menu_builder.separator();
        for host in &hosts {
            let detail = if host.user.is_empty() {
                host.name.clone()
            } else {
                format!("{} ({}@{})", host.name, host.user, host.hostname)
            };
            menu_builder = menu_builder.text(format!("ssh:{}", host.name), detail);
        }
    }

    menu_builder = menu_builder.separator();
    menu_builder = menu_builder.text("refresh", "↻ 刷新主机列表");
    menu_builder = menu_builder.text("show", "显示窗口");
    menu_builder = menu_builder.separator();
    menu_builder = menu_builder.text("quit", "退出 SSH-M");

    Ok(menu_builder.build()?)
}

/// Open an SSH connection in the system terminal (same logic as the command).
fn open_ssh_from_tray(host: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let script = format!(
            r#"
            tell application "Terminal"
                activate
                do script "ssh {}"
            end tell
            "#,
            host
        );
        std::process::Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        let terminals = ["gnome-terminal", "konsole", "xterm"];
        let mut launched = false;
        for term in &terminals {
            if std::process::Command::new(term)
                .args(["--", "ssh", host])
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
        std::process::Command::new("cmd")
            .args(["/c", "start", "ssh", host])
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
    }

    Ok(())
}

/// Load a small PNG icon for the tray.
/// Uses the app's 32x32 icon from the icons directory.
fn load_tray_icon() -> Result<Image<'static>, Box<dyn std::error::Error>> {
    let icon_bytes: &[u8] = include_bytes!("../icons/32x32.png");
    let img = Image::from_bytes(icon_bytes)?;
    Ok(img)
}
