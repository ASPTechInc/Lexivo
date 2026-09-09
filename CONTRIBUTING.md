# Contributing to Lexivo

Thanks for your interest in contributing to **Lexivo**!
Lexivo is a free open source (FOSS) Android application and your contributions help make it better
for everyone. ❤️

---

## How to Contribute

1. **Fork the repository** to your own GitHub account.
2. **Clone your fork** to your local machine.
3. **Create a new branch** for your work:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. **Make your changes**. Ensure you follow the project's code style and architecture.
5. **Run tests** to ensure no regressions were introduced (
   see [Testing](/README.md#how-to-test)).
6. **Commit your changes** with clear and descriptive messages.
7. **Push to your fork**:
   ```bash
   git push origin feature/your-feature-name
   ```
8. **Create a pull request** from your fork's branch to the `main` branch of the original
   repository.

---

## Technical Stack

Lexivo is built with the Rust ecosystem and cross-platform desktop/web/mobile UI tooling:

- **Language**: [Rust](https://www.rust-lang.org/)
- **UI framework**: [egui](https://github.com/emilk/egui) via [eframe](https://github.com/emilk/egui/tree/master/crates/eframe)
- **Build tool**: [Cargo](https://doc.rust-lang.org/cargo/)
- **Web delivery**: [Trunk](https://trunkrs.dev/)
- **Android integration**: Rust + Android NDK via `cargo-ndk`
- **Data**: JSON-based puzzle loading from `assets/puzzles-easy.json`, 
  `assets/puzzles-medium.json` and `assets/puzzles-hard.json`

---

## Code Style

To keep the project consistent and maintainable:

- **Rust Conventions**: Follow standard Rust style and prefer clear, idiomatic code.
- **Formatting**: Run `cargo fmt` before committing.
- **Linting**: Keep Clippy warnings in check when possible.
- **Keep PRs focused**: Small, single-purpose changes are easier to review and safer to merge.
- **Documentation**: Add brief comments where logic is non-obvious, especially around puzzle 
  generation, scoring and challenge flow.

---

## Testing

We value reliable gameplay and regression coverage. Please add or update tests when changing 
game logic.

### Run the test suite

```bash
cargo test -- --nocapture
```

### Useful checks

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

---

## Reporting Issues

If you find a bug or have a feature request, please [open an issue](https://github.com/ASPTechInc/Lexivo/issues) and include:

- a clear title and description
- steps to reproduce (for bugs)
- expected vs actual behaviour
- screenshots, recordings or logs if helpful
- the platform affected (desktop, Android or web)

---

## Pull Request Checklist

Before submitting your PR, please ensure:

- [ ] Your code compiles successfully.
- [ ] You ran `cargo fmt`.
- [ ] Relevant tests pass with `cargo test`.
- [ ] Your PR description clearly explains the change.
- [ ] You kept the scope focused and avoided unrelated edits.

---

## Questions or Help

If you have questions about setup, architecture or gameplay changes, open a discussion or issue 
in the repository and we’ll help.

---

## Questions or Help

If you have questions or need help setting up the environment, please
use [GitHub Discussions](https://github.com/ASPTechInc/Lexivo/discussions).
