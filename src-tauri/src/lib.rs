mod config;
mod hotkey;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // 로그 플러그인 (디버그 모드)
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let handle = app.handle().clone();

            // 설정 로드
            let murmur_config = config::load_config(&handle);
            log::info!("설정 로드 완료: stt_mode={}", murmur_config.stt_mode);

            // 시스템 트레이 생성
            tray::create_tray(&handle)?;
            log::info!("시스템 트레이 생성 완료");

            // 글로벌 단축키 등록
            hotkey::register_hotkeys(&handle, &murmur_config)?;
            log::info!("글로벌 단축키 등록 완료");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
