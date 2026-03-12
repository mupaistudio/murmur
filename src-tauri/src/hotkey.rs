use tauri::Emitter;
use tauri::Manager;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

/// 글로벌 단축키 등록
pub fn register_hotkeys(
    app: &tauri::AppHandle,
    config: &crate::config::MurmurConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let activate_shortcut: Shortcut = config
        .hotkey_activate
        .parse()
        .map_err(|e| format!("활성화 단축키 파싱 실패 '{}': {:?}", config.hotkey_activate, e))?;

    let insert_shortcut: Shortcut = config
        .hotkey_insert
        .parse()
        .map_err(|e| format!("삽입 단축키 파싱 실패 '{}': {:?}", config.hotkey_insert, e))?;

    app.global_shortcut().on_shortcuts(
        [activate_shortcut, insert_shortcut],
        move |app, shortcut, event| {
            if event.state != ShortcutState::Pressed {
                return;
            }

            if shortcut == &activate_shortcut {
                log::info!("활성화 단축키 감지");
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
                let _ = app.emit("shortcut-activate", ());
            } else if shortcut == &insert_shortcut {
                log::info!("삽입 단축키 감지");
                let _ = app.emit("shortcut-insert", ());
            }
        },
    )?;

    log::info!(
        "단축키 등록 완료: 활성화={}, 삽입={}",
        config.hotkey_activate,
        config.hotkey_insert
    );

    Ok(())
}
