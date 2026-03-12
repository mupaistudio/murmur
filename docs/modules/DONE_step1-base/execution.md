# step1-base — 구현 기록
> 시작일: 2026-03-12
> plan: C:/Users/mupai/.claude/plans/swift-painting-willow.md

## 구현 환경
| 항목 | 값 |
|------|-----|
| 작업 브랜치 | step1-base |
| worktree | C:/Users/mupai/murmur-wt-step1-base |

## 진행 기록

### Phase 1: 백엔드 모듈 (config + tray + hotkey + lib.rs)
**상태**: 완료

| 파일 | 변경 유형 | 내용 |
|------|-----------|------|
| config.rs | 신규 | MurmurConfig 구조체, load/save, Default 구현 |
| tray.rs | 신규 | TrayIconBuilder + 메뉴(설정/종료) |
| hotkey.rs | 신규 | GlobalShortcutExt, Ctrl+Shift+W/Q 등록 |
| lib.rs | 수정 | mod 선언, 플러그인 등록, setup 통합 |
| Cargo.toml | 수정 | tauri features 추가 (tray-icon, image-png) |

### Phase 2: 설정 파일
**상태**: 완료

| 파일 | 변경 유형 | 내용 |
|------|-----------|------|
| capabilities/default.json | 수정 | global-shortcut:default 권한 추가 |
| tauri.conf.json | 수정 | withGlobalTauri: true, visible: false |

### Phase 3: 프론트엔드
**상태**: 완료

| 파일 | 변경 유형 | 내용 |
|------|-----------|------|
| index.html | 수정 | 상태 표시 #status, 단축키 안내 추가 |
| app.js | 수정 | shortcut-activate/insert 이벤트 리스너 |

**핵심 결정**: cargo check 통과 확인 완료

## 변경 파일 요약
- 신규 3개: config.rs, tray.rs, hotkey.rs
- 수정 6개: lib.rs, Cargo.toml, Cargo.lock, capabilities/default.json, tauri.conf.json, index.html, app.js

## 교훈
- Tauri v2에서 `tauri_plugin_global_shortcut::init()` 없음 → `Builder::new().build()` 사용
- `Image::from_bytes()`에 `image-png` feature 필요, 트레이는 `tray-icon` feature 필요
- `ShortcutState::Pressed` 필터링 필수 (Pressed/Released 둘 다 콜백 호출됨)
