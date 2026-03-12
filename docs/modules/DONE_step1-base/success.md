# step1-base — 성공 보고서
> 완료일: 2026-03-12
> 커밋: ebb3d8f feat(core): 기반 모듈 구현 — 트레이 상주 + 글로벌 단축키 반응 뼈대

## 결과 요약
Tauri v2 앱이 시스템 트레이에 상주하고 글로벌 단축키(Ctrl+Shift+W/Q)에 반응하는 기반 구조 완성.
설정 파일 자동 생성/로드, 트레이 메뉴(설정/종료), 프론트엔드 이벤트 수신까지 동작.

## 변경 파일
| 파일 | 변경 유형 | 설명 |
|------|-----------|------|
| src-tauri/src/config.rs | 신규 | MurmurConfig 구조체, load_config/save_config |
| src-tauri/src/tray.rs | 신규 | TrayIconBuilder + 메뉴(설정/종료) |
| src-tauri/src/hotkey.rs | 신규 | GlobalShortcutExt, Shortcut 파싱 + 이벤트 emit |
| src-tauri/src/lib.rs | 수정 | mod 선언, 플러그인 등록, setup 통합 |
| src-tauri/Cargo.toml | 수정 | tauri features (tray-icon, image-png) 추가 |
| src-tauri/capabilities/default.json | 수정 | global-shortcut:default 권한 |
| src-tauri/tauri.conf.json | 수정 | withGlobalTauri: true, visible: false |
| src/index.html | 수정 | 상태 표시 + 단축키 안내 |
| src/js/app.js | 수정 | shortcut-activate/insert 이벤트 리스너 |

## 핵심 결정과 이유
| 결정 | 이유 | 대안 |
|------|------|------|
| global-shortcut::Builder::new().build() | Tauri v2에 init() 없음 | init() (v1 패턴, 동작 안 함) |
| include_bytes! 아이콘 임베딩 | 상대경로 from_path 불안정 | from_path (런타임 실패 위험) |
| visible: false 기본 | 트레이 상주 앱, 시작 시 창 불필요 | visible: true (UX 불일치) |

## 문제 → 해결 (검색용)
| 문제 (키워드) | 원인 | 해결 |
|---------------|------|------|
| tauri tray-icon feature | Image::from_bytes 컴파일 에러 | Cargo.toml에 features = ["tray-icon", "image-png"] 추가 |
| global-shortcut init 없음 | Tauri v2 API 변경 | Builder::new().build() 패턴 사용 |
| ShortcutState 중복 호출 | Pressed + Released 둘 다 콜백 | ShortcutState::Pressed 필터링 |

## 재사용 가능 패턴
- Tauri v2 트레이 생성: TrayIconBuilder + MenuBuilder + on_menu_event 패턴
- Tauri v2 글로벌 단축키: GlobalShortcutExt + on_shortcuts + ShortcutState 필터링
- config 로드/저장: app.path().app_config_dir() + serde_json + Default 트레이트

## 교훈
- Tauri v2 플러그인 API는 v1과 크게 다름. 공식 문서/crate 소스 확인 필수
- include_bytes! 매크로가 아이콘 로딩에 가장 안정적 (빌드타임 임베딩)
- ShortcutState::Pressed 필터링을 빠뜨리면 단축키 이벤트가 2번 발생

## git log 검색 키워드
`git log --grep="기반 모듈"` `git log --grep="tray"` `git log --grep="hotkey"`
