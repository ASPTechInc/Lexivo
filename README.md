<a id="top"></a>

# Lexivo

<img alt="Logo" src="assets/images/app-logo.png" width="120" height="120" />

[![Open Source](https://img.shields.io/badge/Open%20Source-MIT%20License-green.svg)](LICENSE)

![Continuous Integration](https://github.com/ASPTechInc/Lexivo/actions/workflows/ci.yml/badge.svg)

## Table of contents

- [Overview](#overview)
- [Features](#features)
- [Screenshots](#screenshots)
- [Game concept](#game-concept)
- [Project structure](#project-structure)
- [Requirements](#requirements)
- [Development setup](#development-setup)
- [Running the application](#how-to-run)
- [Testing the application](#how-to-test)
- [Releasing the application](#how-to-release)
- [Game architecture](#game-architecture)
- [F-Droid metadata](#f-droid-metadata)
- [Licence](#licence)
- [Contributing](#contributing)
- [Developer notes](#developer-notes)
- [Other projects](#other-projects)
- [Support](#support)

---

[⬇️ Go to bottom](#bottom)

## Overview

Lexivo is a free open source (FOSS) cross-platform word puzzle game built with Rust and [egui/eframe](https://github.com/emilk/egui/). It challenges players to solve linguistic puzzles by transforming words through single-letter, double-letter or triple-letter changes and anagrams.

---

## Features

|     | Feature               | Description                                                                       |
| --- | :-------------------- | :-------------------------------------------------------------------------------- |
| ✅  | **Cross-platform**    | Play seamlessly on **Android**, **Web (PWA)** and **Desktop** (Linux/macOS/Win).  |
| ✅  | **Core gameplay**     | Engaging puzzles: transform words via single-letter changes and anagrams.         |
| ✅  | **Daily challenges**  | Compete in daily puzzle sets with stable global seeds for fair competition.       |
| ✅  | **Quick play**        | Jump into endless word fun with randomised puzzles from the pool.                 |
| ✅  | **Difficulty levels** | Choose between **Easy**, **Medium** and **Hard** to match your vocabulary skills. |
| ✅  | **Score tracking**    | Monitor your progress with point bonuses, high scores and win streaks.            |
| ✅  | **Hint system**       | Strategic letter reveals to help when you're stuck (at the cost of points).       |
| ✅  | **Local persistence** | Automatically saves game state, settings and leaderboards to your device.         |
| ✅  | **Modern UI**         | A lightweight, reactive interface powered by the **egui** Rust toolkit.           |
| ✅  | **ChromeOS support**  | Native **x86_64** support for optimised performance on Chromebooks.               |
| ✅  | **Offline-first**     | No tracking, no ads and no internet connection required for core gameplay.        |
| ✅  | **Open source**       | Fully transparent, MIT-licensed code for the community to audit and contribute.   |

---

## Screenshots

### Dark mode

| **Start screen** | **Game screen** | **Leaderboard** | **Settings (1)** |
| :---: | :---: | :---: | :---: |
| <img src="assets/screenshots/dark-theme/start-screen-dark.png" /> | <img src="assets/screenshots/dark-theme/game-screen-dark.png" /> | <img src="assets/screenshots/dark-theme/leaderboard-screen-dark.png" /> | <img src="assets/screenshots/dark-theme/settings-screen1-dark.png" /> |

| **Settings (2)** | **Settings (3)** | **How to play** | **How to answer** |
| :---: | :---: | :---: | :---: |
| <img src="assets/screenshots/dark-theme/settings-screen2-dark.png" /> | <img src="assets/screenshots/dark-theme/settings-screen3-dark.png" /> | <img src="assets/screenshots/dark-theme/how-to-play-guide-dialogue-dark.png" /> | <img src="assets/screenshots/dark-theme/how-to-answer-guide-dialogue-dark.png" /> |

| **Changelog** | **App updater** | **Build info** | **Support** |
| :---: | :---: | :---: | :---: |
| <img src="assets/screenshots/dark-theme/changelog-dialogue-dark.png" /> | <img src="assets/screenshots/dark-theme/app-updater-dialogue-dark.png" /> | <img src="assets/screenshots/dark-theme/app-build-info-dialogue-dark.png" /> | <img src="assets/screenshots/dark-theme/app-support-dialogue-dark.png" /> |

| **Reset progress** |
| :---: |
| <img src="assets/screenshots/dark-theme/reset-progress-dialogue-dark.png" width="300"  /> |

### Light mode

| **Start screen** | **Game screen** | **Leaderboard** | **Settings (1)** |
| :---: | :---: | :---: | :---: |
| <img src="assets/screenshots/light-theme/start-screen-light.png" /> | <img src="assets/screenshots/light-theme/game-screen-light.png" /> | <img src="assets/screenshots/light-theme/leaderboard-screen-light.png" /> | <img src="assets/screenshots/light-theme/settings-screen1-light.png" /> |

| **Settings (2)** | **Settings (3)** | **How to play** | **How to answer** |
| :---: | :---: | :---: | :---: |
| <img src="assets/screenshots/light-theme/settings-screen2-light.png" /> | <img src="assets/screenshots/light-theme/settings-screen3-light.png" /> | <img src="assets/screenshots/light-theme/how-to-play-guide-dialogue-light.png" /> | <img src="assets/screenshots/light-theme/how-to-answer-guide-dialogue-light.png" /> |

| **Changelog** | **App updater** | **Build info** | **Support** |
| :---: | :---: | :---: | :---: |
| <img src="assets/screenshots/light-theme/changelog-dialogue-light.png" /> | <img src="assets/screenshots/light-theme/app-updater-dialogue-light.png" /> | <img src="assets/screenshots/light-theme/app-build-info-dialogue-light.png" /> | <img src="assets/screenshots/light-theme/app-support-dialogue-light.png" /> |

| **Reset progress** |
| :---: |
| <img src="assets/screenshots/light-theme/reset-progress-dialogue-light.png" width="300" /> |



---

## Game concept

The core gameplay loop is simple yet challenging:

1. **Start with a word**: You are given a source word.
2. **Change letters**: Transform the word by swapping characters. The exact
   number of letters to change depends on the difficulty level that you
   are playing the game at.

   _Easy mode_: Change 1 letter, then, rearrange to form a new word matching the clue.

   _Medium mode_: Change 2 letters then, rearrange.

   _Hard mode_: Change 3 letters then, rearrange.

3. **Rearrange (Anagram)**: Rearrange the new set of letters to match a specific hint.

### Example

- **Mode**: `easy`
- **Source**: `SERUM`
- **Hint**: _A medical professional_
- **Answer**: `NURSE` (Change `M` to `N`, then rearrange `S E R U N` to `N U R S E`)

---

## Project structure

Lexivo is designed to run natively on Desktop (Linux/Windows/macOS), as an Android application and a Web PWA.

```text
Lexivo/
├── src/                # Shared Rust source code
│   ├── app.rs          # UI and Game logic
│   ├── lib.rs          # Platform entry points (Android/Web)
│   └── main.rs         # Native Desktop entry point
├── android/            # Android Studio project & Gradle config
├── assets/             # Game assets (puzzles-easy.json, puzzles-medium.json, puzzles-hard.json, icons, PWA manifest)
├── Cargo.toml          # Rust dependencies and build profiles
└── Trunk.toml          # Web build configuration
```

## Requirements

- **Rust**: Latest stable toolchain (`rustup`).
- **Android Studio**: SDK and NDK (required for Android builds).
- **Trunk**: Web build tool and local server (required for Web/PWA).
- **cargo-ndk**: For compiling Rust code into Android native libraries.
- **WebAssembly Target**: `wasm32-unknown-unknown` for web support.

---

## Development setup

### 1. Install prerequisites

#### Rust

Install Rust using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify installation: `rustup --version`, `cargo --version`.

#### Android environment

- Install **Android Studio** for the SDK, NDK and Emulators.
- Install Android targets for Rust:

  ```bash
  rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android
  ```

- Install `cargo-ndk`: `cargo install cargo-ndk`

#### Web environment

- Install WASM target: `rustup target add wasm32-unknown-unknown`
- Install **Trunk**: `cargo install --locked trunk`

---

## How to run

### Native desktop (fastest for testing)

Run directly on your computer:

```bash
cargo run --release
```

_Note: Linux users may need to install `libxcb-render0-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`, `libxkbcommon-dev` and `libssl-dev`._

### Android application

1. Open the `android/` folder in **Android Studio**.
2. Connect a device or start an emulator.
3. Click **Run**. Gradle will automatically invoke `cargo-ndk` to compile the Rust logic into the APK.

For faster local development builds, you can compile only one ABI:

```bash
cd android
./gradlew :app:assembleDebug -PsingleAbiDev=true
```

You can also force a specific ABI manually:

```bash
cd android

# For arm64-v8a (modern physical devices)
./gradlew :app:assembleDebug -PsingleAbiDev=true -PdevAbi=arm64-v8a

# For x86_64 (ChromeOS / Emulators)
./gradlew :app:assembleDebug -PsingleAbiDev=true -PdevAbi=x86_64
```

### Web / PWA

Serve locally to test in a browser:

```bash
trunk serve
```

Open `http://127.0.0.1:8080` in your browser. The app is configured as a PWA and can be installed for offline play.

---

## How to test

### Run unit tests

Execute the following command to run all unit tests:

```bash
cargo test
```

### Run integration tests

Integration tests are located in the `tests/` directory. Run them with:

```bash
cargo test --test <test_name>
```

Replace `<test_name>` with the name of the specific test file you would like to run.

---

## How to format code (locally)

### Typos check

`cargo install typos-cli && typos`

#### Full local CI script

`check.sh`

This covers Rust check, wasm check, Android target check, fmt check, clippy, tests, doc tests and trunk build.

### Rust format and lint

#### Check format only

`cargo fmt --all -- --check`

#### Auto format

`cargo fmt --all`

#### Lint

`cargo clippy --workspace --all-targets --all-features -- -D warnings`

### Rust syntax and compile checks

#### Workspace:

`cargo check --workspace --all-targets`

#### Wasm target

`cargo check --workspace --all-features --lib --target wasm32-unknown-unknown`

#### Android target:

`cargo check --workspace --all-features --lib --target aarch64-linux-android`

### Rust tests

#### Unit and integration:

`cargo test --workspace --all-targets --all-features`

#### Doc tests

`cargo test --workspace --doc`

### Web build validation

`trunk build`

### Android checks

#### Build

`cd android && ./gradlew assembleDebug`

#### Unit tests

`cd android && ./gradlew testDebugUnitTest`

#### Android lint

`cd android && ./gradlew lintDebug`

#### Kotlin lint check

`cd android && ./gradlew ktlintCheck`

#### Kotlin auto format

`cd android && ./gradlew ktlintFormat`

---

## How to release

This project uses GitHub Actions for automated releases. Pushing a tag matching `v*` to the `main`
branch will trigger a build and create a draft release with signed APKs.

Example of creating a tag:

```bash
git tag v1.0.0

git push origin v1.0.0
```

**Releasing the app via the terminal**

```bash
cd android && ./gradlew :app:assembleRelease

# or

cd android && ./gradlew :app:bundleRelease
```

---

## Game architecture

Lexivo follows a clean separation between UI and state:

- **Game state**: Managed in `LexivoApp` (src/app.rs).
- **Puzzle data**: Loaded from `assets/data/puzzles-easy.json`, `assets/data/puzzles-medium.json` and `assets/data/puzzles-hard.json`.
- **Cross-platform**: The game logic is completely platform-agnostic, allowing for easy testing and rapid iteration on desktop before deploying to mobile or web.

---

## F-Droid metadata

This project includes [Fastlane](https://docs.fastlane.tools/getting-started/android/metadata/)
compatible metadata. This ensures that the app's listing, descriptions and changelogs are
maintained directly within source control for FOSS repository compatibility.

Location: `fastlane/metadata/android/en-GB/`

---

## Licence

This project is licensed under the [MIT Licence](LICENSE).

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on code
style, testing and PR submission.

New puzzles can be added to `assets/data/puzzles-easy.json`, `assets/data/puzzles-medium.json` or `assets/data/puzzles-hard.json`. Each puzzle requires a `mode` level, `source` word, a `hint` and the target `answer`.

---

## Developer notes

For a detailed breakdown of the technical concepts, Rust patterns and project architecture, see
the [DEVELOPER_NOTES.md](docs/DEVELOPER_NOTES.md) file.

---

## Other projects

View the [other projects by ASPTechnologies Incorporation](docs/FOSS_PROJECTS.md) on GitHub.

## Support

Developed by [**ASPTechnologies Incorporation**](https://github.com/ASPTechInc 'GitHub ASPTechInc').
If you find Lexivo useful,
consider [Supporting the Developer](https://www.buymeacoffee.com/asptechinc). Lexivo is and will
always be ad-free.

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://buymeacoffee.com/asptechinc)

[🔝 Back to top](#top)

<a id="bottom"></a>
