// Game Screen
// This module renders the active puzzle interface, including scrambled tiles, hint areas and answer slots.

use crate::app::LexivoApp;
use crate::config::AppConfig;
use crate::utils::{draw_tile, primary_action_button};
use eframe::egui;

pub fn render_game_screen(app: &mut LexivoApp, ui: &mut egui::Ui) {
    let theme = crate::theme::theme_for_ctx(ui.ctx());
    let palette = &theme.palette;
    let metrics = theme.metrics;

    if app.puzzles.is_empty() {
        ui.add_space(60.0);
        ui.heading(
            egui::RichText::new(&AppConfig::APP_NAME.to_uppercase())
                .size(metrics.game_heading_size)
                .strong(),
        );
        ui.add_space(20.0);
        ui.label("No puzzles loaded from assets/data/puzzles-xx.json");
        return;
    }

    let (hint, answer_len) = {
        let puzzle = &app.puzzles[app.current_index];
        (puzzle.hint.clone(), puzzle.answer.chars().count())
    };
    let time = ui.ctx().input(|i| i.time);

    // Header section
    ui.heading(
        egui::RichText::new(&AppConfig::APP_NAME.to_uppercase())
            .size(metrics.game_heading_size)
            .strong(),
    );
    ui.add_space(8.0);

    // Score area
    ui.label(format!(
        "Score: {}          Best: {}          Streak: {}",
        app.score, app.best_score, app.streak
    ));

    if app.daily_challenge_active {
        ui.label(format!(
            "Daily challenge: {}/{}",
            app.challenge_progress + 1,
            app.challenge_total
        ));
    }

    ui.add_space(12.0);

    // Time left area
    if app.timer_enabled {
        let total_seconds = app.time_left.max(0.0).ceil() as u32;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        ui.label(format!("Time left: {}:{:02}", minutes, seconds));
    } else {
        ui.label("Timer: Off");
    }

    ui.add_space(8.0);

    // Show game over state if applicable
    if app.game_over {
        ui.add_space(16.0);
        ui.group(|ui| {
            ui.set_min_width(metrics.hint_group_min_width);
            ui.heading("Game over");
            ui.label(format!("Final score: {}", app.score));
            ui.label(palette.muted_text(format!("Best streak: {}", app.best_streak)));
        });
        ui.add_space(20.0);
        if primary_action_button(ui, "Play again", metrics.game_action_button_size).clicked() {
            app.restart_game();
        }
        return;
    }

    ui.add_space(25.0);

    // Scrambled tiles area
    ui.horizontal(|ui| {
        let scrambled = app.scrambled_source.clone();
        let char_count = scrambled.chars().count();
        let total_width = (char_count as f32 * metrics.tile_size)
            + (char_count.saturating_sub(1) as f32 * ui.spacing().item_spacing.x);
        ui.add_space((ui.available_width() - total_width).max(0.0) / 2.0);

        for (idx, c) in scrambled.chars().enumerate() {
            if draw_tile(
                ui,
                c,
                idx,
                time,
                palette.tile_fill,
                palette.tile_text,
                metrics,
                app.animations_enabled,
            ) && app.can_append_char(c) {
                app.user_input.push(c);
            }
        }
    });

    ui.add_space(8.0);

    // Hint text group
    ui.group(|ui| {
        ui.set_min_width(metrics.hint_group_min_width);
        ui.label(
            egui::RichText::new("HINT:")
                .strong()
                .color(palette.hint_label),
        );
        ui.label(
            egui::RichText::new(hint)
                .size(metrics.hint_text_size)
                .italics(),
        );
    });

    ui.add_space(4.0);

    // Answer section - Using horizontal without centred logic to keep it tight
    let help_id = egui::Id::new("game_answer_help_modal");
    let mut show_answer_help = ui
        .ctx()
        .data_mut(|data| data.get_temp::<bool>(help_id).unwrap_or(false));

    ui.horizontal(|ui| {
        // Manually centre the label + button row
        let label_text = "Enter answer:";
        let spacing = ui.spacing().item_spacing.x;
        let total_row_width = 100.0 + spacing + 18.0; // Approx label width + spacing + button width
        ui.add_space((ui.available_width() - total_row_width).max(0.0) / 2.0);

        ui.label(label_text);
        if ui
            .add(
                egui::Button::new(
                    egui::RichText::new(egui_phosphor::regular::INFO)
                        .size(metrics.info_icon_size)
                        .strong(),
                )
                .frame(false)
                .min_size(egui::vec2(18.0, 18.0)),
            )
            .clicked()
        {
            show_answer_help = true;
        }
    });

    if show_answer_help {
        egui::Window::new("How to answer")
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(metrics.dialogue_content_size, egui::FontFamily::Proportional),
                );

                ui.label("Tap a tile to move it into the answer slots.");
                ui.label("Tap a filled slot to remove a letter.");
                ui.label("Revealed letters cost 5 points, are locked, are highlighted and cannot be removed.");
                ui.add_space(8.0);
                ui.vertical_centered(|ui| {
                    if ui
                        .add_sized(egui::vec2(96.0, 36.0), egui::Button::new("Close"))
                        .clicked()
                    {
                        show_answer_help = false;
                    }
                });
            });
    }

    ui.ctx()
        .data_mut(|data| data.insert_temp(help_id, show_answer_help));

    ui.add_space(2.0);

    // Visual ANSWER slots
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let slots = app.answer_slots();
        let mut remove_index = None;

        let slot_size = 46.0;
        let gap = 4.0;
        let row_width = answer_len as f32 * slot_size + (answer_len.saturating_sub(1)) as f32 * gap;
        ui.add_space((ui.available_width() - row_width).max(0.0) / 2.0);

        for idx in 0..answer_len {
            if let Some(ch) = slots.get(idx).copied().flatten() {
                let is_locked = app.is_revealed_answer_index(idx);
                let label = if is_locked {
                    palette.revealed_text(ch)
                } else {
                    egui::RichText::new(ch.to_string())
                };
                let fill = if is_locked {
                    palette.group_fill
                } else {
                    palette.tile_fill
                };
                let stroke = if is_locked {
                    egui::Stroke::new(2.0, palette.revealed_text)
                } else {
                    palette.button_stroke(metrics)
                };
                let button = egui::Button::new(label)
                    .fill(fill)
                    .stroke(stroke)
                    .min_size(egui::vec2(slot_size, slot_size));

                let response = ui.add(button);
                if !is_locked && response.clicked() {
                    remove_index = Some(idx);
                }
            } else {
                if app.focused_slot == Some(idx) {
                    let mut tmp = String::new();
                    let response = ui.add(
                        egui::TextEdit::singleline(&mut tmp)
                            .id_salt(egui::Id::new("slot_input").with(idx))
                            .hint_text("_")
                            .desired_width(slot_size),
                    );

                    if app.auto_focus {
                        response.request_focus();
                        app.auto_focus = false;
                    }

                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        app.check_answer();
                    }

                    if ui.input(|i| i.key_pressed(egui::Key::Backspace)) && tmp.is_empty() {
                        app.user_input.pop();
                        app.focused_slot = None;
                    }

                    if !tmp.is_empty() {
                        if let Some(c) = tmp.chars().next().and_then(|c| c.to_uppercase().next()) {
                            if app.can_append_char(c) {
                                app.user_input.push(c);
                                app.sync_manual_input();
                                // Auto-tab to next empty slot
                                app.focused_slot = app.answer_slots().iter().enumerate()
                                    .find(|(_, s)| s.is_none())
                                    .map(|(i, _)| i);
                                app.auto_focus = true;
                            } else {
                                app.focused_slot = None;
                            }
                        } else {
                            app.focused_slot = None;
                        }
                    }

                    if response.lost_focus() && app.focused_slot == Some(idx) {
                        app.focused_slot = None;
                    }
                } else {
                    if ui
                        .add(egui::Button::new("_").min_size(egui::vec2(slot_size, slot_size)))
                        .clicked()
                    {
                        app.focused_slot = Some(idx);
                        app.auto_focus = true;
                    }
                }
            }
            if idx < answer_len - 1 {
                ui.add_space(gap);
            }
        }

        if let Some(idx) = remove_index {
            if app.remove_manual_input_for_slot(idx) {
                app.auto_focus = true;
            }
        }
    });

    ui.add_space(40.0);

    // Action buttons
    ui.horizontal(|ui| {
        let total_width =
            3.0 * metrics.game_action_button_size.x + 2.0 * metrics.item_spacing.x;
        ui.add_space((ui.available_width() - total_width).max(0.0) / 2.0);

        if primary_action_button(
            ui,
            &format!("{} Check", egui_phosphor::regular::CHECK_CIRCLE),
            metrics.game_action_button_size,
        )
        .clicked()
        {
            app.check_answer();
        }

        if ui
            .add_sized(
                metrics.game_action_button_size,
                egui::Button::new(format!("{} Reveal", egui_phosphor::regular::LIGHTBULB)),
            )
            .clicked()
        {
            app.reveal_hint_letter();
        }

        if ui
            .add_sized(
                metrics.game_action_button_size,
                egui::Button::new(format!("{} Next", egui_phosphor::regular::ARROW_RIGHT)),
            )
            .clicked()
        {
            app.next_puzzle();
        }
    });
}
