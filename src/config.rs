// Application Configuration
// Contains global constants and configuration settings for the game.

/// Global configuration constants for the application.
pub struct AppConfig;

impl AppConfig {
    /// The name of the application displayed in the UI and window title.
    pub const APP_NAME: &'static str = "Lexivo";
    /// The current version of the application, pulled from Cargo.toml.
    pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub const DONATE_URL: &'static str = "https://buymeacoffee.com/asptechinc";
    pub const GITHUB_REPO: &'static str = "https://github.com/ASPTechInc/Lexivo";
    pub const GITHUB_ISSUES: &'static str = "https://github.com/ASPTechInc/Lexivo/issues";
    pub const GITHUB_RELEASES: &'static str = "https://api.github.com/repos/ASPTechInc/Lexivo/releases/latest";

    pub const ROUND_TIME_SECS: f32 = 60.0;
    pub const LETTER_REVEAL_COST_POINTS: u32 = 5;
}
