# Murmur CLAUDE.md

## 프로젝트 개요

음성을 텍스트로 변환하여 커서 위치에 삽입하는 입력 도우미 프로그램.
시스템 트레이에 상주하며, 단축키로 호출/삽입.

## 기술 스택

| 항목 | 선택 |
|------|------|
| 프레임워크 | Tauri v2 (Rust + WebView) |
| 프론트엔드 | HTML / CSS / JS (Vanilla) |
| STT 엔진 | OpenAI Whisper API (기본) / 로컬 Whisper 서버 (설정 전환) |
| UI | 미니멀 팝업 + 메모장 수준 텍스트 편집 |
| 상주 방식 | 시스템 트레이 |

## 디렉토리 구조

```
murmur/
├── src-tauri/           # Rust 백엔드
│   └── src/
│       ├── main.rs      # 엔트리포인트
│       └── lib.rs       # Tauri 앱 설정
├── src/                 # 프론트엔드 (WebView)
│   ├── index.html
│   ├── css/style.css
│   └── js/app.js
├── docs/DESIGN.md       # 설계 문서
└── CLAUDE.md
```

## 빌드/실행

```bash
# 개발 모드
npm run dev

# 빌드
npm run build

# Tauri CLI 직접 사용
npx tauri dev
npx tauri build
```

## 주요 Rust 의존성

- `tauri` 2.10.x — 앱 프레임워크
- `tauri-plugin-global-shortcut` 2.x — 글로벌 단축키
- `cpal` 0.17.x — 마이크 녹음
- `reqwest` 0.13.x — HTTP (Whisper API 호출)
- `enigo` 0.6.x — 키 입력 시뮬레이션 (붙여넣기)
- `serde_json` — 설정 파일 처리

## 코드 컨벤션

- Rust: snake_case, rustfmt 기본
- JS: camelCase, 세미콜론 사용
- CSS: BEM 불필요 (소규모), 간결하게
- 한국어 주석 허용

## 설정 파일

앱 설정은 `config.json`으로 관리 (stt_mode, api_key, hotkey 등)
상세: `docs/DESIGN.md` 참조
