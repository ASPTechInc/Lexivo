/// Settings Screen
/// Provides user configuration for theme, audio, gameplay rules and data management.
use crate::app::LexivoApp;
use crate::theme::{Palette, ThemeMetrics};
use crate::types::{Screen, TextSize, ThemeMode};
use crate::utils::primary_action_button;
use eframe::egui;

fn settings_section_header(
    ui: &mut egui::Ui,
    palette: &Palette,
    metrics: &ThemeMetrics,
    title: &str,
) {
    let header_bg = palette.button_selected.gamma_multiply(0.15);
    egui::Frame::default()
        .fill(header_bg)
        .inner_margin(egui::Margin::symmetric(10, 4))
        .corner_radius(4.0)
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(title)
                        .color(palette.text_primary)
                        .size(metrics.settings_section_title_size)
                        .strong(),
                );
            });
        });
}

pub fn render_settings_screen(app: &mut LexivoApp, ui: &mut egui::Ui) {
    let theme = crate::theme::theme_for_ctx(ui.ctx());
    let palette = &theme.palette;
    let metrics = theme.metrics;
    ui.vertical_centered(|ui| {
        ui.heading("Settings");
        ui.add_space(18.0);

        render_challenge_setup(app, ui, palette, &metrics);
        ui.add_space(16.0);
        render_appearance(app, ui, palette, &metrics);
        ui.add_space(16.0);
        render_audio_feedback(app, ui, palette, &metrics);
        ui.add_space(16.0);
        render_gameplay(app, ui, palette, &metrics);
        ui.add_space(16.0);
        render_data_about(app, ui, palette, &metrics);

        if app.show_reset_confirm {
            app.show_reset_progress_window(ui.ctx());
        }
    });
}

fn render_challenge_setup(
    app: &mut LexivoApp,
    ui: &mut egui::Ui,
    palette: &Palette,
    metrics: &ThemeMetrics,
) {
    ui.group(|ui| {
        ui.set_min_width(metrics.settings_group_min_width);
        ui.set_max_width(ui.available_width().min(340.0));
        ui.vertical_centered(|ui| {
            settings_section_header(ui, palette, metrics, "Challenge setup");
            ui.add_space(4.0);
            ui.strong("Number of daily challenges:");
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = metrics.item_spacing.x;
                let total_width = 4.0 * metrics.settings_question_count_button_size.x
                    + 3.0 * ui.spacing().item_spacing.x;
                ui.add_space((ui.available_width() - total_width).max(0.0) / 2.0);

                for count in [10, 20, 30, 0] {
                    let label = match count {
                        10 => "10".to_owned(),
                        20 => "20".to_owned(),
                        30 => "30".to_owned(),
                        0 => "All".to_owned(),
                        _ => count.to_string(),
                    };
                    let selected = app.selected_question_count == count;
                    let fill = palette.button_fill(selected);

                    if ui
                        .add_sized(
                            metrics.settings_question_count_button_size,
                            egui::Button::new(label).fill(fill),
                        )
                        .clicked()
                    {
                        app.set_question_count(count);
                    }
                }
            });
            ui.add_space(10.0);
            if primary_action_button(
                ui,
                "Start daily challenge",
                metrics.wide_primary_button_size,
            )
            .clicked()
            {
                app.start_daily_challenge();
                app.go_to_screen(Screen::Game);
            }
            ui.add_space(2.0);
        });
    });
}

fn render_appearance(
    app: &mut LexivoApp,
    ui: &mut egui::Ui,
    palette: &Palette,
    metrics: &ThemeMetrics,
) {
    ui.group(|ui| {
        ui.set_min_width(metrics.settings_group_min_width);
        ui.set_max_width(ui.available_width().min(340.0));
        ui.vertical_centered(|ui| {
            settings_section_header(ui, palette, metrics, "Appearance");
            ui.add_space(4.0);

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 15.0;
                ui.add_space(20.0);
                ui.strong("Theme:");
                ui.selectable_value(&mut app.theme_mode, ThemeMode::System, "System");
                ui.selectable_value(&mut app.theme_mode, ThemeMode::Light, "Light");
                ui.selectable_value(&mut app.theme_mode, ThemeMode::Dark, "Dark");
            });

            ui.add_space(4.0);

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 15.0;
                ui.add_space(15.0);
                ui.strong("Text size:");
                ui.selectable_value(&mut app.text_size, TextSize::Small, "Small");
                ui.selectable_value(&mut app.text_size, TextSize::Normal, "Normal");
                ui.selectable_value(&mut app.text_size, TextSize::Large, "Large");
            });
            ui.add_space(4.0);
            ui.checkbox(&mut app.animations_enabled, "Enable tile animations");
        });
    });
}

fn render_audio_feedback(
    app: &mut LexivoApp,
    ui: &mut egui::Ui,
    palette: &Palette,
    metrics: &ThemeMetrics,
) {
    ui.group(|ui| {
        ui.set_min_width(metrics.settings_group_min_width);
        ui.set_max_width(ui.available_width().min(340.0));
        ui.vertical_centered(|ui| {
            settings_section_header(ui, palette, metrics, "Audio & Feedback");
            ui.add_space(4.0);
            ui.checkbox(&mut app.sound_enabled, "Enable sounds");
        });
    });
}

fn render_gameplay(
    app: &mut LexivoApp,
    ui: &mut egui::Ui,
    palette: &Palette,
    metrics: &ThemeMetrics,
) {
    ui.group(|ui| {
        ui.set_min_width(metrics.settings_group_min_width);
        ui.set_max_width(ui.available_width().min(340.0));
        ui.vertical_centered(|ui| {
            settings_section_header(ui, palette, metrics, "Gameplay");
            ui.add_space(4.0);
            ui.checkbox(&mut app.timer_enabled, "Enable game timer");
            ui.checkbox(&mut app.show_game_instructions, "Show 'How to Play' guide");

            if app.show_game_instructions {
                app.show_game_instructions_guide(ui.ctx());
            }
        });
    });
}

fn render_data_about(
    app: &mut LexivoApp,
    ui: &mut egui::Ui,
    palette: &Palette,
    metrics: &ThemeMetrics,
) {
    ui.group(|ui| {
        ui.set_min_width(metrics.settings_group_min_width);
        ui.set_max_width(ui.available_width().min(340.0));
        ui.vertical_centered(|ui| {
            settings_section_header(ui, palette, metrics, "Data & About");
            ui.add_space(4.0);
            ui.checkbox(&mut app.show_changelog, "What's new?");
            if app.show_changelog {
                app.show_changelog_window(ui.ctx());
            }
            ui.checkbox(&mut app.check_app_for_updates, "Check for updates");
            if app.check_app_for_updates {
                app.show_update_window(ui.ctx());
            }
            ui.checkbox(&mut app.support_development, "Support development");
            if app.support_development {
                app.show_support_development_window(ui.ctx());
            }
            ui.checkbox(&mut app.show_build_info, "Show technical build information");
            if app.show_build_info {
                app.show_build_info_window(ui.ctx());
            }
            ui.add_space(8.0);
            if primary_action_button(ui, "Reset progress", metrics.primary_button_size)
                .clicked()
            {
                app.request_reset_progress();
            }
            ui.add_space(2.0);
        });
    });
}
