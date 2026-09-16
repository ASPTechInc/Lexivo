// Shared Data Types
// Defines the core enums and structs used throughout the Lexivo application.

use serde::{Deserialize, Serialize};

/// Defines the user interface theme mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ThemeMode {
    System,
    Light,
    Dark,
}

pub fn default_theme_mode() -> ThemeMode {
    ThemeMode::System
}

pub fn default_true() -> bool {
    true
}

/// Represents the different text size
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TextSize {
    Small,
    Normal,
    Large,
}

pub fn default_text_size() -> TextSize {
    TextSize::Normal
}

/// Represents the different screens/views in the application.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Screen {
    Start,
    Game,
    Leaderboard,
    Settings,
}

/// The available game modes for Lexivo.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum GameMode {
    QuickPlay,
    DailyChallenge,
}

/// Status of the application update check.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum UpdateStatus {
    #[default]
    Idle,
    Checking,
    UpToDate,
    Available { version: String, url: String },
    Error(String),
}

/// Difficulty levels that affect the puzzle pool and transformation logic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn as_mode_key(self) -> &'static str {
        match self {
            Self::Easy => "easy",
            Self::Medium => "medium",
            Self::Hard => "hard",
        }
    }

    pub fn file_name(self) -> &'static str {
        match self {
            Self::Easy => "puzzles-easy.json",
            Self::Medium => "puzzles-medium.json",
            Self::Hard => "puzzles-hard.json",
        }
    }
}

/// Represents a single word puzzle.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Puzzle {
    /// The starting word displayed to the user.
    pub source: String,
    /// A textual hint describing the answer.
    pub hint: String,
    /// The target word the user must guess.
    pub answer: String,
}
