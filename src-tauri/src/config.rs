use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Manager;

/// 앱 설정 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MurmurConfig {
    pub stt_mode: String,
    pub openai_api_key: String,
    pub local_server_url: String,
    pub hotkey_activate: String,
    pub hotkey_insert: String,
    pub language: String,
}

impl Default for MurmurConfig {
    fn default() -> Self {
        Self {
            stt_mode: "openai".to_string(),
            openai_api_key: String::new(),
            local_server_url: "http://localhost:8000".to_string(),
            hotkey_activate: "Ctrl+Shift+W".to_string(),
            hotkey_insert: "Ctrl+Shift+Q".to_string(),
            language: "ko".to_string(),
        }
    }
}

/// 설정 파일 경로 반환
pub fn config_path(app: &tauri::AppHandle) -> PathBuf {
    let config_dir = app.path().app_config_dir().expect("앱 설정 디렉토리를 찾을 수 없음");
    config_dir.join("config.json")
}

/// 설정 로드 (파일 없으면 기본값 생성 후 저장)
pub fn load_config(app: &tauri::AppHandle) -> MurmurConfig {
    let path = config_path(app);

    if path.exists() {
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                match serde_json::from_str::<MurmurConfig>(&content) {
                    Ok(config) => return config,
                    Err(e) => {
                        log::warn!("설정 파일 파싱 실패, 기본값 사용: {}", e);
                    }
                }
            }
            Err(e) => {
                log::warn!("설정 파일 읽기 실패, 기본값 사용: {}", e);
            }
        }
    }

    let config = MurmurConfig::default();
    if let Err(e) = save_config(app, &config) {
        log::warn!("기본 설정 저장 실패: {}", e);
    }
    config
}

/// 설정 저장
pub fn save_config(app: &tauri::AppHandle, config: &MurmurConfig) -> Result<(), String> {
    let path = config_path(app);

    // 디렉토리가 없으면 생성
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("설정 디렉토리 생성 실패: {}", e))?;
    }

    let json = serde_json::to_string_pretty(config)
        .map_err(|e| format!("설정 직렬화 실패: {}", e))?;

    std::fs::write(&path, json)
        .map_err(|e| format!("설정 파일 쓰기 실패: {}", e))?;

    Ok(())
}
