// Start Screen
// The initial landing page where users select game modes, difficulty and challenge length.

use crate::app::LexivoApp;
use crate::config::AppConfig;
use crate::types::{Difficulty, GameMode};
use crate::utils::primary_action_button;
use eframe::egui;

pub fn render_start_screen(app: &mut LexivoApp, ui: &mut egui::Ui) {
    let theme = crate::theme::theme_for_ctx(ui.ctx());
    let palette = &theme.palette;
    let metrics = theme.metrics;

    ui.vertical_centered(|ui| {
        ui.add_space(60.0);
        ui.heading(
            egui::RichText::new(AppConfig::APP_NAME.to_uppercase())
                .size(metrics.start_heading_size)
                .strong(),
        );
        ui.add_space(16.0);
        ui.label("Choose game mode");
        ui.add_space(18.0);

        render_mode_selection(app, ui, palette, metrics);

        ui.add_space(30.0);
        ui.label("Difficulty");
        render_difficulty_selection(app, ui, palette, metrics);

        ui.add_space(30.0);
        ui.label("Number of puzzles");
        render_question_count_selection(app, ui, palette, metrics);

        ui.add_space(45.0);
        if primary_action_button(ui, "Start", metrics.primary_button_size).clicked() {
            app.start_selected_mode();
        }
    });
}

fn render_mode_selection(
    app: &mut LexivoApp,
    ui: &mut egui::Ui,
    palette: &crate::theme::Palette,
    metrics: crate::theme::ThemeMetrics,
) {
    ui.vertical_centered(|ui| {
        let quick_selected = app.selected_mode == GameMode::QuickPlay;
        let quick_button = egui::Button::new("Quick play").fill(if quick_selected {
            palette.button_selected
        } else {
            palette.button_idle
        });
        if ui
            .add_sized(metrics.mode_button_size, quick_button)
            .clicked()
        {
            app.set_mode(GameMode::QuickPlay);
        }

        ui.add_space(10.0);
        let daily_selected = app.selected_mode == GameMode::DailyChallenge;
        let daily_button = egui::Button::new("Daily challenge").fill(if daily_selected {
            palette.button_selected
        } else {
            palette.button_idle
        });
        if ui
            .add_sized(metrics.mode_button_size, daily_button)
            .clicked()
        {
            app.set_mode(GameMode::DailyChallenge);
        }
    });
}

fn render_difficulty_selection(
    app: &mut LexivoApp,
    ui: &mut egui::Ui,
    palette: &crate::theme::Palette,
    metrics: crate::theme::ThemeMetrics,
) {
    ui.horizontal(|ui| {
        ui.add_space(
            (ui.available_width() - (3.0 * metrics.difficulty_button_size.x + 2.0 * 8.0))
                .max(0.0)
                / 2.0,
        );

        for (label, level) in [
            ("Easy", Difficulty::Easy),
            ("Medium", Difficulty::Medium),
            ("Hard", Difficulty::Hard),
        ] {
            let selected = app.current_difficulty == level;
            let fill = palette.button_fill(selected);

            if ui
                .add_sized(
                    metrics.difficulty_button_size,
                    egui::Button::new(label).fill(fill),
                )
                .clicked()
            {
                app.current_difficulty = level;
                if let Err(err) = app.load_difficulty(level) {
                    log::error!("Failed to switch difficulty: {err}");
                    app.set_message(format!(
                        "Could not load {} puzzles, using current set.",
                        level.as_mode_key()
                    ));
                }
            }

            ui.add_space(8.0);
        }
    });
}

fn render_question_count_selection(
    app: &mut LexivoApp,
    ui: &mut egui::Ui,
    palette: &crate::theme::Palette,
    metrics: crate::theme::ThemeMetrics,
) {
    ui.horizontal(|ui| {
        let button_width = metrics.question_count_button_width;
        let count_buttons = 4;
        let total_width =
            (count_buttons as f32 * button_width) + ((count_buttons - 1) as f32 * 8.0);
        ui.add_space((ui.available_width() - total_width).max(0.0) / 2.0);

        for count in [10, 20, 30, 0] {
            let label = if count == 0 {
                "All".to_owned()
            } else {
                count.to_string()
            };
            let selected = app.selected_question_count == count;
            let fill = palette.button_fill(selected);

            if ui
                .add_sized(
                    egui::vec2(button_width, metrics.question_count_button_height),
                    egui::Button::new(label).fill(fill),
                )
                .clicked()
            {
                app.set_question_count(count);
            }
        }
    });
}
