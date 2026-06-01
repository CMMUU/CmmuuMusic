# Cmmuu Music 사용 문서

[English](../README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [한국어](README.ko.md)

Cmmuu Music은 Tauri v2, Vue 3, TypeScript, Pinia, Vite로 구축된 크로스 플랫폼 데스크톱 음악 플레이어입니다. 가벼운 데스크톱 셸, Rust 백엔드, 플러그인 중심의 음악 데이터 접근 구조를 제공합니다.

> 이 프로젝트는 음악 원본 파일을 제공, 호스팅 또는 배포하지 않습니다. 사용자는 합법적이고 적절한 플러그인과 콘텐츠 소스를 사용할 책임이 있습니다.

## 주요 기능

- Linux, macOS, Windows 데스크톱 플랫폼 지원
- Tauri v2 백엔드와 Vue 3 프론트엔드 기반
- TypeScript 기반 프론트엔드 개발
- Pinia 상태 관리와 Vue Router 라우팅
- 플러그인 중심의 음악 데이터 접근 구조
- 로컬 개발, 테스트, 데스크톱 앱 번들링 스크립트 제공

## 기술 스택

- 데스크톱 런타임: Tauri v2
- 백엔드: Rust
- 프론트엔드: Vue 3 + TypeScript
- 빌드 도구: Vite
- 상태 관리: Pinia
- 라우터: Vue Router
- 패키지 매니저: pnpm
- 테스트: Vitest

## 요구 사항

프로젝트를 실행하기 전에 다음 도구 체인을 설치하세요.

- Node.js LTS
- pnpm 10+
- Rust stable toolchain
- 사용 중인 운영체제에 필요한 Tauri v2 시스템 의존성

플랫폼별 Tauri 의존성은 Tauri v2 공식 설정 가이드를 따라 구성하세요.

## 시작하기

저장소를 클론하고 의존성을 설치합니다.

```bash
git clone https://github.com/CMMUU/CmmuuMusic.git
cd CmmuuMusic
pnpm install
```

프론트엔드 개발 서버만 실행합니다.

```bash
pnpm dev
```

Tauri 데스크톱 개발 앱을 실행합니다.

```bash
pnpm tauri:dev
```

테스트를 실행합니다.

```bash
pnpm test
```

프론트엔드를 빌드합니다.

```bash
pnpm build
```

데스크톱 앱을 빌드합니다.

```bash
pnpm tauri:build
```

## Linux 사용 방법

1. Node.js LTS, pnpm, Rust, Linux용 Tauri v2 필수 패키지를 설치합니다.
2. 저장소 루트에서 `pnpm install`을 실행합니다.
3. `pnpm tauri:dev`로 로컬 데스크톱 개발 앱을 실행합니다.
4. `pnpm tauri:build`로 Linux 데스크톱 번들을 생성합니다.

생성된 패키지는 일반적으로 `src-tauri/target/release/bundle/`에 출력됩니다.

## macOS 사용 방법

1. Node.js LTS, pnpm, Rust, Xcode Command Line Tools를 설치합니다.
2. 저장소 루트에서 `pnpm install`을 실행합니다.
3. `pnpm tauri:dev`로 로컬 데스크톱 개발 앱을 실행합니다.
4. `pnpm tauri:build`로 macOS 앱 번들을 생성합니다.

생성된 앱 번들은 일반적으로 `src-tauri/target/release/bundle/`에 출력됩니다.

## Windows 사용 방법

1. Node.js LTS, pnpm, Rust, Windows용 Tauri v2 빌드 도구를 설치합니다.
2. 저장소 루트에서 `pnpm install`을 실행합니다.
3. `pnpm tauri:dev`로 로컬 데스크톱 개발 앱을 실행합니다.
4. `pnpm tauri:build`로 Windows 설치 파일 또는 패키지를 생성합니다.

생성된 설치 파일과 패키지는 일반적으로 `src-tauri/target/release/bundle/`에 출력됩니다.

## 자주 사용하는 명령어

| 명령어 | 설명 |
| --- | --- |
| `pnpm dev` | Vite 프론트엔드 개발 서버 실행 |
| `pnpm build` | 타입 검사 후 프론트엔드 빌드 |
| `pnpm preview` | 빌드된 프론트엔드 미리 보기 |
| `pnpm tauri:dev` | Tauri 데스크톱 개발 앱 실행 |
| `pnpm tauri:build` | Tauri로 데스크톱 번들 빌드 |
| `pnpm test` | Vitest 테스트 실행 |

## 관련 문서

- [소프트웨어 설계 문서](../CmmuuMusic_TauriV2_SDD.md)
- [English README](../README.md)
- [简体中文文档](README.zh-CN.md)
- [日本語ドキュメント](README.ja.md)

## 라이선스

[LICENSE](../LICENSE)를 참고하세요.
