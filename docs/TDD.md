# Murmur (속삭임) — 프로젝트 설계

음성을 텍스트로 변환하여 커서 위치에 삽입하는 입력 도우미 프로그램.

## 기술 스택

| 항목 | 선택 |
|------|------|
| 프레임워크 | Tauri (Rust + WebView) |
| 프론트엔드 | HTML / CSS / JS |
| STT 엔진 | OpenAI Whisper API (기본) / 로컬 Whisper 서버 (설정 전환) |
| UI | 미니멀 팝업 + 메모장 수준 텍스트 편집 |
| 상주 방식 | 시스템 트레이 |

## 디렉토리 구조

```
murmur/
├── src-tauri/                # Rust 백엔드
│   ├── src/
│   │   ├── main.rs           # 엔트리포인트
│   │   ├── hotkey.rs         # 글로벌 단축키 관리
│   │   ├── clipboard.rs      # 커서 위치 기억 & 텍스트 삽입
│   │   ├── stt.rs            # Whisper API / 로컬 서버 호출
│   │   ├── audio.rs          # 마이크 녹음
│   │   ├── config.rs         # 설정 관리 (API키, 단축키, 서버URL)
│   │   └── tray.rs           # 시스템 트레이
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                      # 프론트엔드 (WebView)
│   ├── index.html            # 메인 팝업
│   ├── settings.html         # 설정 화면
│   ├── css/
│   │   └── style.css
│   └── js/
│       ├── app.js            # 팝업 로직
│       ├── editor.js         # 텍스트 편집기
│       └── settings.js       # 설정 로직
├── docs/                     # 설계 문서
├── package.json
└── README.md
```

## 모듈별 역할

| 모듈 | 역할 | 핵심 라이브러리 |
|------|------|----------------|
| `hotkey.rs` | 글로벌 단축키 등록/감지 | `tauri-plugin-global-shortcut` |
| `clipboard.rs` | 포커스 저장, 텍스트 삽입 (시뮬레이트 붙여넣기) | `enigo` or Windows API |
| `audio.rs` | 마이크 입력 → WAV/WebM 녹음 | `cpal` or WebView `MediaRecorder` |
| `stt.rs` | 오디오 → 텍스트 변환 (API 호출) | `reqwest` |
| `config.rs` | 설정 파일 읽기/쓰기 | `serde_json`, 로컬 JSON 파일 |
| `tray.rs` | 트레이 아이콘, 메뉴 (설정/종료) | `tauri::SystemTray` |
| `editor.js` | `<textarea>` 기반 편집기 | 순수 JS |

## 동작 시나리오

```
1. 앱 실행 → 시스템 트레이에 상주
2. 어떤 앱에서 글 쓰다가 Ctrl+Shift+W → 커서 위치 기억
3. Murmur 팝업 등장 → 녹음 시작
4. 말하기 종료 → Whisper API로 텍스트 변환
5. 팝업에서 텍스트 확인/수정 (메모장 수준 편집)
6. Ctrl+Shift+Q (또는 Enter) → 원래 커서 위치에 텍스트 삽입
7. 팝업 닫힘
```

## 데이터 흐름

```
[단축키 감지] ──→ [현재 포커스 윈도우/커서 저장]
      │
      ▼
[팝업 표시] ──→ [녹음 시작 (마이크)]
      │
      ▼
[녹음 종료] ──→ [오디오 전송]
      │              │
      │    ┌─────────┴──────────┐
      │    ▼                    ▼
      │  [OpenAI API]    [로컬 서버]
      │    └─────────┬──────────┘
      │              ▼
      │         [텍스트 반환]
      ▼
[편집기에 표시] ──→ [사용자 수정]
      │
      ▼
[삽입 단축키] ──→ [저장된 윈도우로 포커스 복원]
      │
      ▼
[클립보드에 복사 → Ctrl+V 시뮬레이트] ──→ [원래 위치에 삽입]
```

## 텍스트 삽입 방식

커서 위치에 삽입하는 건 OS 레벨 트릭이 필요하다:

1. 원래 클립보드 내용 백업
2. 변환된 텍스트를 클립보드에 복사
3. 저장해둔 윈도우로 포커스 전환
4. `Ctrl+V` 키 시뮬레이트
5. 클립보드 원래 내용 복원

이 방식이 가장 범용적이고 안정적이다.

## 설정 파일 (`config.json`)

```json
{
  "stt_mode": "openai",
  "openai_api_key": "",
  "local_server_url": "http://localhost:8000",
  "hotkey_activate": "Ctrl+Shift+W",
  "hotkey_insert": "Ctrl+Shift+Q",
  "language": "ko"
}
```

## 단축키

| 동작 | 기본 단축키 | 비고 |
|------|------------|------|
| 호출 (팝업 열기) | `Ctrl+Shift+W` | 설정에서 변경 가능 |
| 삽입 (텍스트 붙여넣기) | `Ctrl+Shift+Q` | 설정에서 변경 가능 |
