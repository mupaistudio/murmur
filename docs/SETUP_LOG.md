# Murmur — 초기 셋업 작업 로그

**작업일:** 2026-03-12
**실행 스킬:** /dao-init

---

## 1. Phase 1: DESIGN.md 분석

`docs/DESIGN.md`에서 프로젝트 정보 추출 완료.

| 항목 | 값 |
|------|-----|
| project_name | Murmur (속삭임) |
| tech_stack | Tauri v2 (Rust + WebView), HTML/CSS/JS, OpenAI Whisper API |
| source_path | `src-tauri/` (백엔드), `src/` (프론트엔드) |
| build_command | `npm run build` |
| 상주 방식 | 시스템 트레이 |

---

## 2. Phase 2: 기술 환경 셋업

### 2-1. 도구 설치 상태

| 도구 | 버전 | 상태 |
|------|------|------|
| Node.js | v24.13.1 | ✅ 기존 설치 |
| npm | 11.8.0 | ✅ 기존 설치 |
| Rust (rustc) | 1.94.0 | ✅ 신규 설치 (rustup) |
| Cargo | 1.94.0 | ✅ 신규 설치 |
| VS 2022 Community | MSVC v14.40 | ⚠️ link.exe 있으나 Windows SDK 미설치 |
| Windows 10/11 SDK | - | ❌ 미설치 (빌드 차단 원인) |

### 2-2. 최신 버전 조사 결과

| 패키지 | 유형 | 버전 |
|--------|------|------|
| @tauri-apps/cli | npm | 2.10.1 |
| @tauri-apps/api | npm | 2.10.1 |
| tauri-plugin-global-shortcut | Rust+npm | 2.2.0 |
| cpal | Rust crate | 0.17.3 |
| reqwest | Rust crate | 0.13.2 |
| enigo | Rust crate | 0.6.1 |
| serde_json | Rust crate | 1.0.149 |

### 2-3. 프로젝트 초기화

1. `npm init -y` → `package.json` 생성
2. `npm install @tauri-apps/cli @tauri-apps/api` → Tauri 프론트엔드 패키지
3. `npx tauri init` → `src-tauri/` 디렉토리 생성 (Cargo.toml, tauri.conf.json, src/)
4. `git init` → Git 저장소 초기화

### 2-4. 설정 파일 커스터마이즈

**Cargo.toml** — 프로젝트명 `murmur`, 의존성 추가:
- `tauri-plugin-global-shortcut = "2"` (글로벌 단축키)
- `reqwest = { version = "0.13", features = ["json", "multipart"] }` (Whisper API)
- `cpal = "0.17"` (마이크 녹음)
- `enigo = "0.6"` (키 입력 시뮬레이션)
- `tokio = { version = "1", features = ["full"] }` (비동기 런타임)

**tauri.conf.json** — 윈도우 크기 400x300, 중앙 배치, 트레이 아이콘 설정

**package.json** — `npm run dev`, `npm run build` 스크립트 추가

### 2-5. 프론트엔드 뼈대 파일

| 파일 | 역할 |
|------|------|
| `src/index.html` | 메인 HTML (Vanilla) |
| `src/css/style.css` | 기본 스타일 (다크 테마) |
| `src/js/app.js` | 앱 엔트리포인트 |

### 2-6. 빌드 시도 및 차단 이슈

```
LINK : fatal error LNK1181: cannot open input file 'kernel32.lib'
```

**원인:** Visual Studio 2022에 MSVC 컴파일러(link.exe)는 있으나, Windows 10/11 SDK가 설치되어 있지 않아 `kernel32.lib` 등 시스템 라이브러리를 찾을 수 없음.

**해결 방법:** Visual Studio Installer → Community 2022 → 수정 → "C++를 사용한 데스크톱 개발" 워크로드에서 **Windows 11 SDK** 포함 설치.

---

## 3. Phase 3: AI 환경 셋업

### 3-1. 생성된 파일

| 파일 | 역할 | 상태 |
|------|------|------|
| `CLAUDE.md` | 프로젝트 전용 AI 지침서 | ✅ |
| `.claude/agents/dao-worker.md` | 구현 전문 에이전트 | ✅ |
| `.claude/agents/dao-analyst.md` | 분석 전문 에이전트 | ✅ |
| `.claude/agents/dao-tester.md` | 검증 전문 에이전트 | ✅ |
| `.claude/agents/dao-scribe.md` | 기록/학습 전문 에이전트 | ✅ |
| 프로젝트 MEMORY.md | 작업 메모리 인덱스 | ✅ |

### 3-2. 에이전트 커스터마이즈 내용

모든 에이전트의 `프로젝트 컨텍스트` 섹션을 Murmur 정보로 교체:
- `project_name: "Murmur"`
- `tech_stack: "Tauri v2, Rust, HTML/CSS/JS, OpenAI Whisper API"`
- `source_path`, `build_command`, `key_directories` 등 프로젝트 실정에 맞게 설정

---

## 4. Phase 4: 검증 — ⏳ 대기 중

Windows SDK 설치 후 진행 예정:
- [ ] `cargo check` 성공
- [ ] `npm run build` 성공
- [ ] 앱 실행 확인

---

## 5. 현재 프로젝트 구조

```
murmur/
├── .claude/
│   └── agents/
│       ├── dao-worker.md
│       ├── dao-analyst.md
│       ├── dao-tester.md
│       └── dao-scribe.md
├── .git/
├── .gitignore
├── CLAUDE.md
├── docs/
│   ├── DESIGN.md (→ TDD.md)
│   └── SETUP_LOG.md          ← 이 파일
├── node_modules/
├── package.json
├── package-lock.json
├── src/
│   ├── index.html
│   ├── css/style.css
│   └── js/app.js
└── src-tauri/
    ├── .gitignore
    ├── build.rs
    ├── capabilities/
    ├── Cargo.toml
    ├── icons/
    ├── src/
    │   ├── main.rs
    │   └── lib.rs
    └── tauri.conf.json
```

---

## 6. 다음 단계

1. **Windows 11 SDK 설치** → 빌드 차단 해소
2. **빌드 검증** (Phase 4 완료)
3. **초기 커밋** 생성
4. **구현 시작** — DESIGN.md 모듈별 순서대로 진행
