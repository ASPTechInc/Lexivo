# [v1.0.0](https://github.com/ASPTechInc/Lexivo/tree/v1.0.0) (2026-09-09)

Full stable release of Lexivo, a cross-platform word puzzle game built with Rust and egui. Significant updates to user experience, accessibility and platform integration.

### Features

- **Cross-platform word puzzles**: Play seamlessly across Android, Web (PWA) and Desktop (Linux/Windows/macOS).
- **Core gameplay loop**: Engaging linguistic puzzles where you transform words through single-letter changes and anagrams.
- **Daily challenges**: Compete in daily puzzle sets with stable seeds for consistent global difficulty.
- **Game modes**:
  - **Quick play**: Endless word fun with randomised puzzles.
  - **Daily challenge**: A curated set of puzzles to test your skills every day.
- **Difficulty Levels**: Choose between Easy, Medium and Hard puzzle pools to match your vocabulary level.
- **Score & streak tracking**: Monitor your progress with point bonuses, high scores and win streak tracking.
- **Hint system**: Strategic gameplay with the ability to reveal letters when you get stuck, at the cost of points.
- **Local persistence**: Automatic saving of game state, settings and leaderboard data using platform-native storage.
- **Modern UI**: A lightweight, reactive and minimalist interface powered by `egui` and `eframe`.
- **Developer experience**:
  - Clean Rust-based engine with shared logic across all platforms.
  - Automated build system using Gradle and `cargo-ndk` for Android deployment.
  - Comprehensive unit testing for core game logic.
- **FOSS readiness**: Fully open-source under the MIT licence with clear contributing guidelines.
- **Dynamic theming**: Full support for Light, Dark and System theme switching with a custom-built palette system for optimal readability.
- **Enhanced accessibility**: New text size settings (Small, Normal, Large) to improve the gameplay experience across different screen densities.
- **In-app update system**: Added automated update checks and a seamless installation flow for Android devices.
- **Platform-native feedback**: Integrated haptic and audio feedback for answer validation on Android via a new JNI bridge.
- **ChromeOS verification**: Officially verified support for x86_64 architecture, providing a native experience on ChromeOS devices.
- **Session management**: Improved game state persistence and added confirmation modals for back-navigation to prevent accidental progress loss.
- **Expanded puzzle pool**: Substantially increased the number of word puzzles across all difficulty levels.
- **Advanced settings**: New technical build information window and developer support options.

### Technical Improvements

- **JNI Architecture**: Robust implementation of Java Native Interface bridges for system-level Android features.
- **UI Performance**: Optimised the reactive `egui` interface for smoother animations and better battery efficiency.
- **Asset management**: Streamlined the loading of embedded puzzle data for faster initialisation.