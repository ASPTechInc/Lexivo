// Lexivo - A word puzzle game where players unscramble and transform words.
// This module contains the main application state and core game logic.

use crate::config::AppConfig;
use crate::types::{
    Difficulty, GameMode, Puzzle, Screen, TextSize, ThemeMode, default_text_size,
    default_theme_mode,
};
use crate::utils::{
    daily_seed_for_difficulty, normalise_answer, scramble_word, seed_from_time, shuffle_slice,
};
use anyhow::{Context as _, Result};
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// The main application state for Lexivo.
#[derive(Debug, Deserialize, Serialize)]
pub struct LexivoApp {
    /// The list of puzzles loaded for the current difficulty.
    pub(crate) puzzles: Vec<Puzzle>,
    /// Index of the current puzzle in the `puzzles` list.
    pub(crate) current_index: usize,
    /// The current source word after being scrambled.
    pub(crate) scrambled_source: String,
    /// Current text entered by the user in the input field.
    pub(crate) user_input: String,
    /// Current session score.
    pub(crate) score: u32,
    /// Highest score achieved in the current session.
    pub(crate) best_score: u32,
    /// Current consecutive correct answers.
    pub(crate) streak: u32,
    /// Highest streak achieved in the current session.
    pub(crate) best_streak: u32,
    /// List of top scores for the leaderboard.
    pub(crate) leaderboard: Vec<u32>,
    /// Feedback message displayed to the user (e.g., "Correct!").
    pub(crate) message: String,
    /// Whether the input field should automatically grab focus.
    pub(crate) auto_focus: bool,
    /// State for the pseudo-random number generator.
    pub(crate) rng_state: u64,
    /// Remaining time for the current round in seconds.
    pub(crate) time_left: f32,
    /// Whether the current game or round has ended.
    pub(crate) game_over: bool,
    /// The currently active screen.
    pub(crate) screen: Screen,
    /// The previous screen visited, used for back-navigation.
    #[serde(default)]
    pub(crate) previous_screen: Option<Screen>,
    /// Whether the back-navigation confirmation modal is visible.
    #[serde(default)]
    pub(crate) show_back_confirm: bool,
    /// Pending target screen after confirming back-navigation.
    #[serde(default)]
    pub(crate) pending_back_target: Option<Screen>,
    /// Whether the reset-progress confirmation modal is visible.
    #[serde(default)]
    pub(crate) show_reset_confirm: bool,
    /// Toggle for UI animations.
    pub(crate) animations_enabled: bool,
    /// Toggle for sound effects and answer feedback.
    pub(crate) sound_enabled: bool,
    /// Toggle for the game timer.
    pub(crate) timer_enabled: bool,
    /// Total number of questions in a challenge.
    pub(crate) challenge_total: usize,
    /// Number of questions completed in the current challenge.
    pub(crate) challenge_progress: usize,
    /// Whether a daily challenge is currently being played.
    pub(crate) daily_challenge_active: bool,
    /// Number of questions selected for the challenge.
    pub(crate) selected_question_count: usize,
    /// The selected game mode (`QuickPlay` or `DailyChallenge`).
    pub(crate) selected_mode: GameMode,
    /// The current difficulty setting.
    pub(crate) current_difficulty: Difficulty,
    /// Indices of letters in the answer that have been revealed via hints.
    pub(crate) revealed_answer_indices: Vec<usize>,
    /// The user's preferred theme mode.
    #[serde(default = "default_theme_mode")]
    pub(crate) theme_mode: ThemeMode,
    /// The user's preferred text size.
    #[serde(default = "default_text_size")]
    pub(crate) text_size: TextSize,
    /// Toggle to show technical build information.
    #[serde(default)]
    pub(crate) show_build_info: bool,
    /// Toggle to show game instructions in settings.
    #[serde(default = "crate::types::default_true")]
    pub(crate) show_game_instructions: bool,
    /// Toggle to check for application updates.
    #[serde(default)]
    pub(crate) check_app_for_updates: bool,
    /// Toggle to show the changelog.
    #[serde(default)]
    pub(crate) show_changelog: bool,
    /// Toggle to support development.
    #[serde(default)]
    pub(crate) support_development: bool,
    /// Status of the last update check.
    #[serde(skip)]
    pub(crate) update_status: crate::types::UpdateStatus,
    /// Receiver for background update checks.
    #[serde(skip)]
    pub(crate) update_rx: Option<std::sync::mpsc::Receiver<crate::types::UpdateStatus>>,
    /// Currently focused answer slot index for manual keyboard input.
    #[serde(skip)]
    pub(crate) focused_slot: Option<usize>,
    /// Tracks the last theme applied to the context to avoid redundant updates.
    #[serde(skip)]
    pub(crate) last_applied_theme: Option<(ThemeMode, egui::Theme)>,
    /// Tracks the last text size applied to the context.
    #[serde(skip)]
    pub(crate) last_applied_text_size: Option<TextSize>,
    /// Timer for transient feedback messages (toasts).
    #[serde(skip)]
    pub(crate) message_timer: f32,
}

impl Default for LexivoApp {
    fn default() -> Self {
        let mut app = Self {
            puzzles: Vec::new(),
            current_index: 0,
            scrambled_source: String::new(),
            user_input: String::new(),
            score: 0,
            best_score: 0,
            streak: 0,
            best_streak: 0,
            leaderboard: Vec::new(),
            message: String::new(),
            message_timer: 0.0,
            auto_focus: true,
            rng_state: seed_from_time(),
            time_left: AppConfig::ROUND_TIME_SECS,
            game_over: false,
            screen: Screen::Start,
            previous_screen: None,
            show_back_confirm: false,
            pending_back_target: None,
            show_reset_confirm: false,
            animations_enabled: false,
            sound_enabled: true,
            timer_enabled: true,
            challenge_total: 10,
            challenge_progress: 0,
            daily_challenge_active: false,
            selected_question_count: 10,
            selected_mode: GameMode::QuickPlay,
            current_difficulty: Difficulty::Easy,
            revealed_answer_indices: Vec::new(),
            theme_mode: ThemeMode::System,
            text_size: TextSize::Normal,
            show_build_info: false,
            show_game_instructions: true,
            check_app_for_updates: false,
            update_status: crate::types::UpdateStatus::Idle,
            update_rx: None,
            show_changelog: false,
            support_development: false,
            focused_slot: None,
            last_applied_theme: None,
            last_applied_text_size: None,
        };

        if let Ok(puzzles) = Self::load_puzzles_for_difficulty(Difficulty::Easy) {
            app.puzzles = puzzles;
        } else {
            // Provide a hardcoded fallback puzzle if assets are missing or corrupted.
            app.puzzles = vec![Puzzle {
                source: "LEXIVO".to_owned(),
                hint: "The name of this app".to_owned(),
                answer: "LEXIVO".to_owned(),
            }];
        }

        let mut rng_state = app.rng_state;
        shuffle_slice(&mut app.puzzles, &mut rng_state);
        app.rng_state = rng_state;
        app.current_index = 0;
        app.scrambled_source =
            scramble_word(&app.puzzles[app.current_index].source, &mut app.rng_state);
        app.apply_question_count_timer_policy();

        app
    }
}

impl LexivoApp {
    fn question_count_time_limit_secs(question_count: usize) -> Option<f32> {
        match question_count {
            10 => Some(60.0),
            20 => Some(120.0),
            30 => Some(180.0),
            0 => None,
            _ => Some(AppConfig::ROUND_TIME_SECS),
        }
    }

    fn current_time_limit_secs(&self) -> f32 {
        Self::question_count_time_limit_secs(self.selected_question_count)
            .unwrap_or(AppConfig::ROUND_TIME_SECS)
    }

    fn apply_question_count_timer_policy(&mut self) {
        if let Some(seconds) = Self::question_count_time_limit_secs(self.selected_question_count) {
            self.time_left = seconds;
            self.timer_enabled = true;
        } else {
            self.time_left = 0.0;
            self.timer_enabled = false;
        }
    }

    /// Creates a new application instance, attempting to load saved state from storage.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = if let Some(storage) = cc.storage {
            // Attempt to restore the application state from persistent storage.
            eframe::get_value(storage, "lexivo_state").unwrap_or_else(Self::default)
        } else {
            Self::default()
        };

        // Initialise the font
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);

        // Apply theme and text size immediately to the context
        match app.theme_mode {
            ThemeMode::System => cc.egui_ctx.set_theme(egui::ThemePreference::System),
            ThemeMode::Light => cc.egui_ctx.set_theme(egui::ThemePreference::Light),
            ThemeMode::Dark => cc.egui_ctx.set_theme(egui::ThemePreference::Dark),
        }
        crate::theme::apply_to_ctx(&cc.egui_ctx);

        let zoom = match app.text_size {
            TextSize::Small => 0.8,
            TextSize::Normal => 1.0,
            TextSize::Large => 1.2,
        };
        cc.egui_ctx.set_zoom_factor(zoom);

        // Reset transient UI state after loading.
        app.user_input.clear();
        app.message.clear();
        app.auto_focus = true;
        app.apply_question_count_timer_policy();
        app.show_back_confirm = false;
        app.pending_back_target = None;
        app.show_reset_confirm = false;
        app.update_rx = None;
        app.focused_slot = None;
        app.last_applied_theme = None;
        app.last_applied_text_size = Some(app.text_size);

        if app.check_app_for_updates {
            app.check_for_updates();
        }

        app
    }

    /// Determines the absolute path for an asset file across different platforms and environments.
    fn asset_path(file_name: &str) -> Result<std::path::PathBuf> {
        let cwd = std::env::current_dir().context("Failed to get current working directory")?;
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(std::path::Path::to_path_buf))
            .unwrap_or_else(|| cwd.clone());

        let candidates = [
            cwd.join("assets").join("data").join(file_name),
            cwd.join("..").join("assets").join("data").join(file_name),
            exe_dir.join("assets").join("data").join(file_name),
            exe_dir
                .join("..")
                .join("assets")
                .join("data")
                .join(file_name),
        ];

        candidates
            .into_iter()
            .find(|path| path.exists())
            .context(format!(
                "Could not find asset file '{file_name}' in the project assets/data/ folder"
            ))
    }

    /// Loads the puzzle set corresponding to the specified difficulty level.
    fn load_puzzles_for_difficulty(level: Difficulty) -> Result<Vec<Puzzle>> {
        let file_name = level.file_name();

        if let Ok(path) = Self::asset_path(file_name) {
            let json = std::fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}", path.display()))?;
            return serde_json::from_str(&json)
                .with_context(|| format!("Failed to parse {}", path.display()));
        }

        // Fallback for packaged targets (e.g. Android/web) where assets are not plain files.
        let embedded_json = match level {
            Difficulty::Easy => include_str!("../assets/data/puzzles-easy.json"),
            Difficulty::Medium => include_str!("../assets/data/puzzles-medium.json"),
            Difficulty::Hard => include_str!("../assets/data/puzzles-hard.json"),
        };

        serde_json::from_str(embedded_json)
            .with_context(|| format!("Failed to parse embedded {file_name}"))
    }

    /// Updates the application state to use a new difficulty level and reloads puzzles.
    ///
    /// # Errors
    ///
    /// Returns an error if the puzzle set for the specified difficulty cannot be loaded or parsed.
    pub fn load_difficulty(&mut self, level: Difficulty) -> Result<()> {
        let puzzles = Self::load_puzzles_for_difficulty(level)?;
        self.puzzles = puzzles;
        self.current_difficulty = level;
        self.current_index = 0;
        self.user_input.clear();

        if self.puzzles.is_empty() {
            self.scrambled_source.clear();
            return Ok(());
        }

        let mut rng_state = self.rng_state;
        shuffle_slice(&mut self.puzzles, &mut rng_state);
        self.rng_state = rng_state;
        self.scrambled_source = scramble_word(
            &self.puzzles[self.current_index].source,
            &mut self.rng_state,
        );

        Ok(())
    }

    /// Sanitses and normalises user input for comparison against puzzle answers.
    fn normalise_input(value: &str) -> String {
        normalise_answer(value)
    }

    fn current_answer_chars(&self) -> Vec<char> {
        if self.puzzles.is_empty() {
            return Vec::new();
        }

        self.puzzles[self.current_index].answer.chars().collect()
    }

    pub(crate) fn is_revealed_answer_index(&self, index: usize) -> bool {
        self.revealed_answer_indices.contains(&index)
    }

    pub(crate) fn max_manual_input_len(&self) -> usize {
        let answer_len = self.current_answer_chars().len();
        answer_len.saturating_sub(self.revealed_answer_indices.len())
    }

    pub(crate) fn answer_slots(&self) -> Vec<Option<char>> {
        let answer_chars = self.current_answer_chars();
        let mut manual_chars = self.user_input.chars();

        answer_chars
            .iter()
            .enumerate()
            .map(|(idx, answer_ch)| {
                if self.is_revealed_answer_index(idx) {
                    Some(*answer_ch)
                } else {
                    manual_chars.next()
                }
            })
            .collect()
    }

    fn manual_input_index_for_slot(&self, slot_index: usize) -> Option<usize> {
        if self.is_revealed_answer_index(slot_index) {
            return None;
        }

        let manual_index = (0..slot_index)
            .filter(|idx| !self.is_revealed_answer_index(*idx))
            .count();

        Some(manual_index)
    }

    pub(crate) fn remove_manual_input_for_slot(&mut self, slot_index: usize) -> bool {
        let Some(manual_index) = self.manual_input_index_for_slot(slot_index) else {
            return false;
        };

        let count = self.user_input.chars().count();
        if manual_index >= count {
            return false;
        }

        self.user_input = Self::remove_char_at(&self.user_input, manual_index);
        true
    }

    fn composed_answer_from_slots(&self) -> Option<String> {
        let slots = self.answer_slots();
        if slots.iter().any(|slot| slot.is_none()) {
            return None;
        }

        Some(slots.into_iter().flatten().collect())
    }

    /// Validates if a character can be appended to the current user input.
    /// Keyboard input is free-form, limited only by the answer length.
    pub(crate) fn can_append_char(&self, _tapped_char: char) -> bool {
        let max_len = self.max_manual_input_len();
        self.user_input.chars().count() < max_len
    }

    pub(crate) fn remove_char_at(value: &str, index: usize) -> String {
        value
            .chars()
            .enumerate()
            .filter_map(|(idx, ch)| (idx != index).then_some(ch))
            .collect()
    }

    pub(crate) fn sync_manual_input(&mut self) {
        let max_len = self.max_manual_input_len();
        let mut clean = String::with_capacity(self.user_input.len());

        for c in self.user_input.to_uppercase().chars() {
            if clean.chars().count() >= max_len {
                break;
            }
            if c.is_alphanumeric() {
                clean.push(c);
            }
        }
        self.user_input = clean;
    }

    /// Configures the number of questions to be included in the next challenge.
    pub(crate) fn set_question_count(&mut self, count: usize) {
        self.selected_question_count = count;
        self.challenge_total = if count == 0 || count >= self.puzzles.len() {
            self.puzzles.len().max(1)
        } else {
            count
        };
        self.apply_question_count_timer_policy();
    }

    pub(crate) fn set_mode(&mut self, mode: GameMode) {
        self.selected_mode = mode;
    }

    pub(crate) fn go_to_screen(&mut self, target: Screen) {
        if self.screen != target {
            self.previous_screen = Some(self.screen);
            self.screen = target;
        }
    }

    fn back_target(&self) -> Screen {
        match self.previous_screen {
            Some(previous) if previous != self.screen => previous,
            _ => Screen::Start,
        }
    }

    pub(crate) fn request_back_navigation(&mut self) {
        if self.screen == Screen::Game && !self.game_over {
            self.pending_back_target = Some(Screen::Start);
            self.show_back_confirm = true;
            return;
        }

        let target = self.back_target();
        self.show_back_confirm = false;
        self.pending_back_target = None;
        self.go_to_screen(target);
    }

    pub(crate) fn confirm_back_navigation(&mut self) {
        let target = self.pending_back_target.take().unwrap_or(Screen::Start);
        self.show_back_confirm = false;
        self.go_to_screen(target);
    }

    pub(crate) fn cancel_back_navigation(&mut self) {
        self.show_back_confirm = false;
        self.pending_back_target = None;
    }

    pub(crate) fn request_reset_progress(&mut self) {
        self.show_reset_confirm = true;
    }

    pub(crate) fn confirm_reset_progress(&mut self) {
        self.show_reset_confirm = false;
        self.reset_progress();
    }

    pub(crate) fn cancel_reset_progress(&mut self) {
        self.show_reset_confirm = false;
    }

    /// Displays a temporary feedback message (toast) to the user.
    pub(crate) fn set_message(&mut self, text: impl Into<String>) {
        self.message = text.into();
        self.message_timer = 3.0;
    }

    /// Show game help instructions
    pub(crate) fn show_game_instructions_guide(&mut self, ctx: &egui::Context) {
        let mut open = self.show_game_instructions;
        let mut close_clicked = false;
        let theme = crate::theme::theme_for_ctx(ctx);
        let metrics = theme.metrics;

        egui::Window::new("How to Play")
            .open(&mut open)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(
                        metrics.dialogue_content_size,
                        egui::FontFamily::Proportional,
                    ),
                );

                ui.label(
                    "Unscramble the tiles at the top to match the word described by the hint.",
                );
                ui.add_space(8.0);
                ui.label("• There are four challenge lengths available: 10, 20, 30 and all.");
                ui.label("• There are three difficulty levels available: Easy, Medium and Hard.");
                ui.label(
                    "• Easy: change 1 letter. Medium: change 2 letters. Hard: change 3 letters.",
                );
                ui.label("• Tap a tile to move it into the answer slots.");
                ui.label("• Tap a filled slot to remove that letter.");
                ui.label("• Revealed letters cost 5 points and are locked in place.");
                ui.label("• Revealed letters are highlighted and filled automatically.");
                ui.label("• Correct answer: 10 points + seconds left (min 1).");
                ui.add_space(12.0);
                ui.vertical_centered(|ui| {
                    let button_label = if self.screen == Screen::Game {
                        "Let's Play!"
                    } else {
                        "Close"
                    };

                    if ui
                        .add_sized(egui::vec2(120.0, 40.0), egui::Button::new(button_label))
                        .clicked()
                    {
                        close_clicked = true;
                    }
                });
            });

        if close_clicked {
            open = false;
        }
        self.show_game_instructions = open;
    }

    /// Displays the application changelog in a scrollable window.
    /// The changelog is parsed from CHANGELOG.md at runtime using `include_str`!.
    pub(crate) fn show_changelog_window(&mut self, ctx: &egui::Context) {
        let mut open = self.show_changelog;
        let mut close_clicked = false;
        // ... (omitting for brevity in this step, but I'll update the whole file if needed)

        let theme = crate::theme::theme_for_ctx(ctx);
        let metrics = theme.metrics;

        egui::Window::new("What's New")
            .open(&mut open)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(metrics.settings_group_min_width)
            .show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(
                        metrics.dialogue_content_size,
                        egui::FontFamily::Proportional,
                    ),
                );

                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() * 0.7)
                    .show(ui, |ui| {
                        let content = include_str!("../CHANGELOG.md");
                        let entries = crate::changelog::parse_changelog(content);

                        if entries.is_empty() {
                            ui.label("No changelog entries found.");
                        }

                        for entry in entries {
                            ui.group(|ui| {
                                ui.set_width(ui.available_width());
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new(&entry.version)
                                                .strong()
                                                .size(metrics.changelog_version_size),
                                        );
                                        if let Some(date) = &entry.date {
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    ui.label(egui::RichText::new(date).weak());
                                                },
                                            );
                                        }
                                    });
                                    ui.add_space(4.0);
                                    for segments in entry.changes {
                                        ui.horizontal_wrapped(|ui| {
                                            ui.spacing_mut().item_spacing.x = 0.0;
                                            ui.label("• ");
                                            for segment in segments {
                                                match segment {
                                                    crate::changelog::TextSegment::Plain(text) => {
                                                        ui.label(text);
                                                    }
                                                    crate::changelog::TextSegment::Bold(text) => {
                                                        ui.label(
                                                            egui::RichText::new(text).strong(),
                                                        );
                                                    }
                                                    crate::changelog::TextSegment::Italic(text) => {
                                                        ui.label(
                                                            egui::RichText::new(text).italics(),
                                                        );
                                                    }
                                                }
                                            }
                                        });
                                    }
                                });
                            });
                            ui.add_space(10.0);
                        }
                    });

                ui.add_space(12.0);
                ui.vertical_centered(|ui| {
                    if ui
                        .add_sized(egui::vec2(120.0, 40.0), egui::Button::new("Close"))
                        .clicked()
                    {
                        close_clicked = true;
                    }
                });
            });

        if close_clicked {
            open = false;
        }
        self.show_changelog = open;
    }

    /// Displays the support development dialog.
    pub(crate) fn show_support_development_window(&mut self, ctx: &egui::Context) {
        let mut open = self.support_development;
        let mut close_clicked = false;

        let theme = crate::theme::theme_for_ctx(ctx);
        let metrics = theme.metrics;

        egui::Window::new("Enjoying the app?")
            .open(&mut open)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(metrics.settings_group_min_width)
            .show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(metrics.dialogue_content_size, egui::FontFamily::Proportional),
                );

                ui.vertical(|ui| {
                    ui.label("This project is developed and maintained in my spare time. If you’d like to help support its continued development, you can make an optional contribution.");
                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        ui.label(egui_phosphor::regular::HEART);
                        ui.hyperlink_to("Sponsor development", AppConfig::DONATE_URL);
                        ui.label("- ongoing support");
                    });

                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.label(egui_phosphor::regular::CODE);
                        ui.hyperlink_to("Contribute code", AppConfig::GITHUB_REPO);
                        ui.label("- GitHub contributions");
                    });

                    ui.add_space(4.0);

                    ui.horizontal(|ui| {
                        ui.label(egui_phosphor::regular::BUG);
                        ui.hyperlink_to("Report an issue", AppConfig::GITHUB_ISSUES);
                        ui.label("- non-financial contribution");
                    });
                });

                ui.add_space(16.0);
                ui.vertical_centered(|ui| {
                    if ui
                        .add_sized(egui::vec2(120.0, 40.0), egui::Button::new("Close"))
                        .clicked()
                    {
                        close_clicked = true;
                    }
                });
            });

        if close_clicked {
            open = false;
        }
        self.support_development = open;
    }

    /// Displays the build information dialog.
    pub(crate) fn show_build_info_window(&mut self, ctx: &egui::Context) {
        let mut open = self.show_build_info;
        let mut close_clicked = false;

        let metrics = crate::theme::theme_for_ctx(ctx).metrics;

        egui::Window::new("Build information")
            .open(&mut open)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(
                        metrics.dialogue_content_size,
                        egui::FontFamily::Proportional,
                    ),
                );

                ui.label(format!("App name: {}", crate::config::AppConfig::APP_NAME));
                ui.label(format!(
                    "App version: {}",
                    crate::config::AppConfig::APP_VERSION
                ));
                ui.label(format!("Architecture: {}", std::env::consts::ARCH));
                ui.label(format!("OS: {}", std::env::consts::OS));
                #[cfg(target_os = "android")]
                ui.label("Platform: Android (x86_64 supported for ChromeOS)");
                #[cfg(not(target_os = "android"))]
                ui.label(format!("Platform: {}", std::env::consts::FAMILY));
                ui.add_space(12.0);
                ui.vertical_centered(|ui| {
                    if ui
                        .add_sized(egui::vec2(120.0, 40.0), egui::Button::new("Close"))
                        .clicked()
                    {
                        close_clicked = true;
                    }
                });
            });

        if close_clicked {
            open = false;
        }
        self.show_build_info = open;
    }

    /// Displays reset progress dialog.
    pub(crate) fn show_reset_progress_window(&mut self, ctx: &egui::Context) {
        let theme = crate::theme::theme_for_ctx(ctx);
        let metrics = theme.metrics;

        egui::Window::new("Reset progress?")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(
                        metrics.dialogue_content_size,
                        egui::FontFamily::Proportional,
                    ),
                );

                ui.label("This will reset progress data:");
                ui.label("- Leaderboard");
                ui.label("- Current streak");
                ui.label("- Best score and best streak");
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    if ui
                        .add_sized(
                            egui::vec2(96.0, metrics.nav_button_height),
                            egui::Button::new("Cancel"),
                        )
                        .clicked()
                    {
                        self.cancel_reset_progress();
                    }

                    if ui
                        .add_sized(
                            egui::vec2(120.0, metrics.nav_button_height),
                            egui::Button::new("Reset now"),
                        )
                        .clicked()
                    {
                        self.confirm_reset_progress();
                    }
                });
            });
    }

    /// Displays the update check dialog.
    pub(crate) fn show_update_window(&mut self, ctx: &egui::Context) {
        let mut open = self.check_app_for_updates;
        let mut close_clicked = false;

        let theme = crate::theme::theme_for_ctx(ctx);
        let metrics = theme.metrics;

        // Clone the status to avoid borrow conflicts when calling mutable methods of self
        let status = self.update_status.clone();

        egui::Window::new("Check for Updates")
            .open(&mut open)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .default_width(metrics.settings_group_min_width)
            .show(ctx, |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(
                        metrics.dialogue_content_size,
                        egui::FontFamily::Proportional,
                    ),
                );

                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Current version:");
                        ui.label(egui::RichText::new(AppConfig::APP_VERSION).strong());
                    });
                    ui.add_space(8.0);

                    match &status {
                        crate::types::UpdateStatus::Idle => {
                            if ui.button("Check for updates").clicked() {
                                self.check_for_updates();
                            }
                        }
                        crate::types::UpdateStatus::Checking => {
                            ui.horizontal(|ui| {
                                ui.spinner();
                                ui.label("Checking for updates...");
                            });
                        }
                        crate::types::UpdateStatus::UpToDate => {
                            ui.label("App is up to date!");
                            ui.add_space(4.0);
                            if ui.button("Check again").clicked() {
                                self.check_for_updates();
                            }
                        }
                        crate::types::UpdateStatus::Available { version, url } => {
                            ui.label(
                                egui::RichText::new(format!("Update available: {version}"))
                                    .color(theme.palette.success),
                            );
                            ui.add_space(8.0);
                            if ui
                                .add_sized(
                                    egui::vec2(ui.available_width(), 40.0),
                                    egui::Button::new("Update Now"),
                                )
                                .clicked()
                            {
                                self.install_update(url);
                            }
                        }
                        crate::types::UpdateStatus::Error(err) => {
                            ui.label(
                                egui::RichText::new(format!("Error: {err}"))
                                    .color(theme.palette.error),
                            );
                            ui.add_space(4.0);
                            if ui.button("Try again").clicked() {
                                self.check_for_updates();
                            }
                        }
                    }
                });

                ui.add_space(16.0);
                ui.vertical_centered(|ui| {
                    if ui
                        .add_sized(egui::vec2(120.0, 40.0), egui::Button::new("Close"))
                        .clicked()
                    {
                        close_clicked = true;
                    }
                });
            });

        if close_clicked {
            open = false;
        }
        self.check_app_for_updates = open;
    }

    /// Triggers a background check for application updates.
    /// On Android, it invokes the JNI bridge to handle system-level networking.
    /// On other platforms, it uses a background thread to fetch the latest release from GitHub.
    pub(crate) fn check_for_updates(&mut self) {
        self.update_status = crate::types::UpdateStatus::Checking;

        #[cfg(target_arch = "wasm32")]
        {
            // Update checking not supported on web yet.
            self.update_status = crate::types::UpdateStatus::UpToDate;
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            // ...

            #[cfg(target_os = "android")]
            {
                crate::platform_feedback::trigger_update_check();
            }

            let (tx, rx) = std::sync::mpsc::channel();
            self.update_rx = Some(rx);

            // Spawn a background thread to perform the network request without blocking the UI.
            std::thread::spawn(move || {
                let client = reqwest::blocking::Client::builder()
                    .user_agent(format!("{}-App", AppConfig::APP_NAME))
                    .build();

                let result = match client {
                    Ok(client) => {
                        match client.get(AppConfig::GITHUB_RELEASES).send() {
                            Ok(response) => {
                                match response.json::<serde_json::Value>() {
                                    Ok(json) => {
                                        let tag = json["tag_name"]
                                            .as_str()
                                            .unwrap_or("")
                                            .trim_start_matches('v');
                                        let current = AppConfig::APP_VERSION;

                                        if !tag.is_empty() && tag != current {
                                            // Find the APK asset if it exists
                                            let mut download_url =
                                                json["html_url"].as_str().unwrap_or("").to_owned();
                                            if let Some(assets) = json["assets"].as_array() {
                                                for asset in assets {
                                                    let name = asset["name"].as_str().unwrap_or("");
                                                    if std::path::Path::new(name)
                                                        .extension()
                                                        .is_some_and(|ext| {
                                                            ext.eq_ignore_ascii_case("apk")
                                                        })
                                                        && let Some(url) =
                                                            asset["browser_download_url"].as_str()
                                                    {
                                                        download_url = url.to_owned();
                                                        break;
                                                    }
                                                }
                                            }

                                            crate::types::UpdateStatus::Available {
                                                version: tag.to_owned(),
                                                url: download_url,
                                            }
                                        } else {
                                            crate::types::UpdateStatus::UpToDate
                                        }
                                    }
                                    Err(e) => crate::types::UpdateStatus::Error(format!(
                                        "Parse error: {e}"
                                    )),
                                }
                            }
                            Err(e) => {
                                crate::types::UpdateStatus::Error(format!("Network error: {e}"))
                            }
                        }
                    }
                    Err(e) => crate::types::UpdateStatus::Error(format!("Client error: {e}")),
                };

                drop(tx.send(result));
            });
        }
    }

    pub(crate) fn install_update(&mut self, url: &str) {
        #[cfg(target_os = "android")]
        {
            crate::platform_feedback::trigger_update_install(url);
        }
        #[cfg(not(target_os = "android"))]
        {
            let _: &str = url;
            self.set_message("Update download only supported on Android.");
        }
    }

    /// Starts the game using the currently selected mode and difficulty.
    pub(crate) fn start_selected_mode(&mut self) {
        if let Err(err) = self.load_difficulty(self.current_difficulty) {
            log::error!("Failed to load difficulty: {err}");
            self.set_message(format!(
                "Could not reload {} puzzles, starting with current set.",
                self.current_difficulty.as_mode_key()
            ));
        }

        match self.selected_mode {
            GameMode::QuickPlay => {
                self.daily_challenge_active = false;
                self.go_to_screen(Screen::Game);
                self.restart_game();
            }
            GameMode::DailyChallenge => {
                self.start_daily_challenge();
                self.go_to_screen(Screen::Game);
            }
        }
    }

    /// Initialises a new Daily Challenge, using a time-based seed for puzzle selection.
    pub(crate) fn start_daily_challenge(&mut self) {
        self.score = 0;
        self.streak = 0;
        self.challenge_progress = 0;
        self.user_input.clear();
        self.game_over = false;
        self.time_left = self.current_time_limit_secs();
        self.auto_focus = true;
        self.daily_challenge_active = true;

        if self.puzzles.is_empty() {
            self.challenge_total = 0;
            return;
        }

        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
        let seed = daily_seed_for_difficulty(self.current_difficulty, since_epoch.as_secs());
        let mut daily_order = self.puzzles.clone();
        let mut rng_state = seed;
        shuffle_slice(&mut daily_order, &mut rng_state);

        let total = if self.challenge_total == 0 || self.challenge_total >= daily_order.len() {
            daily_order.len()
        } else {
            self.challenge_total
        };
        self.challenge_total = total;
        self.puzzles = daily_order.into_iter().take(total).collect();
        self.current_index = 0;
        self.scrambled_source = scramble_word(
            &self.puzzles[self.current_index].source,
            &mut self.rng_state,
        );
    }

    fn record_score(&mut self) {
        self.leaderboard.push(self.score);
        self.leaderboard.sort_unstable_by(|a, b| b.cmp(a));
        self.leaderboard.dedup();
        self.leaderboard.truncate(5);
    }

    pub(crate) fn reset_progress(&mut self) {
        self.best_score = 0;
        self.streak = 0;
        self.best_streak = 0;
        self.leaderboard.clear();
        self.set_message("Progress reset");
    }

    /// Resets the game state for a new round of play.
    pub(crate) fn restart_game(&mut self) {
        self.score = 0;
        self.streak = 0;
        self.message.clear();
        self.user_input.clear();
        self.game_over = false;
        self.time_left = self.current_time_limit_secs();
        self.auto_focus = true;
        self.challenge_progress = 0;
        self.daily_challenge_active = false;
        self.revealed_answer_indices.clear();

        if self.puzzles.is_empty() {
            return;
        }

        shuffle_slice(&mut self.puzzles, &mut self.rng_state);
        self.current_index = 0;
        self.scrambled_source = scramble_word(
            &self.puzzles[self.current_index].source,
            &mut self.rng_state,
        );
    }

    /// Advances the game to the next puzzle in the current set.
    pub(crate) fn next_puzzle(&mut self) {
        if self.puzzles.is_empty() {
            return;
        }

        if self.current_index + 1 >= self.puzzles.len() {
            shuffle_slice(&mut self.puzzles, &mut self.rng_state);
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        self.scrambled_source = scramble_word(
            &self.puzzles[self.current_index].source,
            &mut self.rng_state,
        );
        self.user_input.clear();
        self.message.clear();
        self.auto_focus = true;
        self.time_left = self.current_time_limit_secs();
        self.game_over = false;
        self.revealed_answer_indices.clear();
    }

    pub(crate) fn reveal_hint_letter(&mut self) {
        if self.game_over {
            return;
        }

        if self.puzzles.is_empty() {
            self.set_message("No puzzles loaded");
            return;
        }

        if self.score < AppConfig::LETTER_REVEAL_COST_POINTS {
            self.set_message(format!(
                "Need {} points to reveal a letter",
                AppConfig::LETTER_REVEAL_COST_POINTS
            ));
            return;
        }

        let answer_chars = self.current_answer_chars();
        let unrevealed: Vec<usize> = (0..answer_chars.len())
            .filter(|idx| !self.is_revealed_answer_index(*idx))
            .collect();

        if unrevealed.is_empty() {
            self.set_message("All letters are already revealed for this answer.");
            return;
        }

        // Choose a random unrevealed slot using internal RNG state.
        let random_offset =
            usize::try_from(self.rng_state).unwrap_or(usize::MAX) % unrevealed.len();
        self.rng_state = self
            .rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let idx = unrevealed[random_offset];

        let letter = answer_chars[idx];

        // If this slot was previously filled manually, remove that manual character.
        self.remove_manual_input_for_slot(idx);

        self.revealed_answer_indices.push(idx);
        self.revealed_answer_indices.sort_unstable();
        self.revealed_answer_indices.dedup();
        self.score = self
            .score
            .saturating_sub(AppConfig::LETTER_REVEAL_COST_POINTS);

        if self.score == 0 {
            self.game_over = true;
            self.streak = 0;
            self.record_score();
            self.set_message("Game over! Score reached zero.");
            self.user_input.clear();
            self.auto_focus = true;
            self.daily_challenge_active = false;
            return;
        }

        self.set_message(format!(
            "Letter revealed: letter {} is '{}' (-{} points)",
            idx + 1,
            letter,
            AppConfig::LETTER_REVEAL_COST_POINTS
        ));
    }

    /// Triggers physical answer feedback (both Sound Effects and Haptic Vibrations)
    /// if the audio & feedback option is enabled in settings.
    fn trigger_answer_feedback(&self, correct: bool) {
        if self.sound_enabled {
            crate::platform_feedback::trigger_answer_feedback(correct, self.sound_enabled);
        }
    }

    pub(crate) fn update_timer(&mut self, dt: f32) {
        if self.message_timer > 0.0 {
            self.message_timer -= dt;
            if self.message_timer <= 0.0 {
                self.message.clear();
            }
        }

        if !self.timer_enabled
            || self.game_over
            || self.show_back_confirm
            || self.show_reset_confirm
            || self.show_game_instructions
            || self.screen != Screen::Game
            || self.puzzles.is_empty()
            || self.selected_question_count == 0
        {
            return;
        }

        self.time_left -= dt;
        if self.time_left <= 0.0 {
            self.time_left = 0.0;
            self.game_over = true;
            self.streak = 0;
            self.record_score();
            self.user_input.clear();
            self.auto_focus = true;
        }
    }

    /// Validates the user's current input against the active puzzle's answer.
    pub(crate) fn check_answer(&mut self) {
        if self.game_over {
            return;
        }

        if self.puzzles.is_empty() {
            self.set_message("No puzzles loaded");
            return;
        }

        let puzzle = &self.puzzles[self.current_index];
        let Some(projected_answer) = self.composed_answer_from_slots() else {
            self.set_message("Enter a word first.");
            return;
        };

        if Self::normalise_input(&projected_answer) == Self::normalise_input(&puzzle.answer) {
            // Correct answer logic: increment streak, calculate points and advance to next puzzle.
            self.streak += 1;
            self.best_streak = self.best_streak.max(self.streak);
            #[expect(clippy::cast_possible_truncation)]
            let bonus = (self.time_left.ceil() as u32).max(1);
            let earned = 10 + bonus;
            self.score += earned;
            self.best_score = self.best_score.max(self.score);
            self.record_score();
            self.trigger_answer_feedback(true);

            if self.daily_challenge_active {
                self.challenge_progress += 1;
                if self.challenge_progress >= self.challenge_total {
                    self.game_over = true;
                    self.set_message(format!(
                        "Daily challenge complete! Final score: {}",
                        self.score
                    ));
                    self.daily_challenge_active = false;
                    self.user_input.clear();
                    self.auto_focus = true;
                    return;
                }
            }

            self.set_message(format!("Correct! +{earned} points"));
            self.user_input.clear();
            self.auto_focus = true;
            self.next_puzzle();
        } else {
            self.streak = 0;
            self.trigger_answer_feedback(false);
            self.set_message("Not quite. Try again!");
        }
    }
}

#[cfg(test)]
mod app_tests;

include!("app_ui.rs");
