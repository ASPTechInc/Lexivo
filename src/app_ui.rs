// Lexivo UI Integration
// Implements the main `eframe::App` trait and handles top-level layout orchestration.

use crate::screens;

impl eframe::App for LexivoApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "lexivo_state", self);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Handle background updates
        if let Some(rx) = &self.update_rx {
            if let Ok(status) = rx.try_recv() {
                if let crate::types::UpdateStatus::Available { version, .. } = &status {
                    if self.screen == Screen::Start {
                        self.set_message(format!("Update available: v{}", version));
                    }
                }
                self.update_status = status;
                self.update_rx = None;
            }
        }

        // Theme
        match self.theme_mode {
            ThemeMode::System => ui.ctx().set_theme(egui::ThemePreference::System),
            ThemeMode::Light => ui.ctx().set_theme(egui::ThemePreference::Light),
            ThemeMode::Dark => ui.ctx().set_theme(egui::ThemePreference::Dark),
        }

        let current_theme = (self.theme_mode, ui.ctx().theme());
        if self.last_applied_theme != Some(current_theme) {
            crate::theme::apply_to_ctx(ui.ctx());
            self.last_applied_theme = Some(current_theme);
        }

        // Text size (zoom factor)
        if self.last_applied_text_size != Some(self.text_size) {
            let zoom = match self.text_size {
                TextSize::Small => 0.8,
                TextSize::Normal => 1.0,
                TextSize::Large => 1.2,
            };
            ui.ctx().set_zoom_factor(zoom);
            self.last_applied_text_size = Some(self.text_size);
        }

        // Animations
        if self.animations_enabled
            || (self.screen == Screen::Game && self.timer_enabled && !self.game_over)
            || self.message_timer > 0.0
        {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(16));
        }

        let dt = ui.ctx().input(|i| i.stable_dt);
        self.update_timer(dt);
        let theme = crate::theme::theme_for_ctx(ui.ctx());
        let palette = &theme.palette;
        let metrics = theme.metrics;

        // Ensure the background is cleared with the correct color for the current theme.
        ui.painter().rect_filled(ui.max_rect(), 0.0, palette.panel_fill);

        if !self.message.is_empty() {
            // Set toast message
            egui::Area::new(egui::Id::new("toast"))
                .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -140.0))
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    egui::Frame::group(ui.style())
                        .fill(palette.group_fill.gamma_multiply(0.9))
                        .stroke(palette.button_stroke(metrics))
                        .corner_radius(egui::CornerRadius::same(metrics.widget_corner_radius))
                        .show(ui, |ui| {
                            ui.label(
                                palette
                                    .emphasised_text(&self.message)
                                    .size(metrics.message_text_size),
                            );
                        });
                });
            }

            ui.add_space(metrics.top_padding);

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let top_bar_height = metrics.nav_button_height + 8.0;

                // Fixed-height top bar region (Back button where applicable).
                ui.allocate_ui(egui::vec2(ui.available_width(), top_bar_height), |ui| {
                    if self.screen != Screen::Start {
                        ui.horizontal(|ui| {
                            ui.add_space(18.0);
                            if ui
                                .add(
                                    egui::Button::new(
                                        egui::RichText::new(egui_phosphor::regular::ARROW_LEFT)
                                            .size(metrics.nav_top_icon_size),
                                    )
                                    .frame(false),
                                )
                                .clicked()
                            {
                                self.request_back_navigation();
                            }
                        });
                    }
                });

                let bottom_nav_height = if self.screen != Screen::Start {
                    metrics.nav_button_height + 20.0
                } else {
                    0.0
                };
                let bottom_nav_inset = if self.screen != Screen::Start {
                    70.0
                } else {
                    0.0
                };

                // Keep fixed header/footer regions so only the centre content scrolls.
                let content_height =
                    (ui.available_height() - bottom_nav_height - bottom_nav_inset).max(0.0);

                // Main content area (Screen switching)
                ui.allocate_ui(egui::vec2(ui.available_width(), content_height), |ui| {
                    // Ensure only this region scrolls if content exceeds height.
                    egui::ScrollArea::vertical()
                        .id_salt("screen_content")
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| match self.screen {
                                Screen::Start => screens::start::render_start_screen(self, ui),
                                Screen::Game => screens::game::render_game_screen(self, ui),
                                Screen::Leaderboard => {
                                    screens::leaderboard::render_leaderboard_screen(self, ui)
                                }
                                Screen::Settings => screens::settings::render_settings_screen(self, ui),
                            });
                        });
                });

                // Fixed-height bottom navigation bar
                if self.screen != Screen::Start {
                    ui.allocate_ui(egui::vec2(ui.available_width(), bottom_nav_height), |ui| {
                        ui.separator();
                        ui.add_space(4.0);
                        ui.columns(3, |columns| {
                            let buttons = [
                                (
                                    egui_phosphor::regular::GAME_CONTROLLER,
                                    "Game",
                                    Screen::Game,
                                ),
                                (
                                    egui_phosphor::regular::TROPHY,
                                    "Leaderboard",
                                    Screen::Leaderboard,
                                ),
                                (egui_phosphor::regular::GEAR, "Settings", Screen::Settings),
                            ];

                            for (i, (icon, label, screen)) in buttons.into_iter().enumerate() {
                                let ui = &mut columns[i];
                                let selected = self.screen == screen;

                                let button_size = egui::vec2(
                                    ui.available_width().min(metrics.nav_button_width),
                                    metrics.nav_button_height,
                                );

                                let button = egui::Button::new("").fill(if selected {
                                    palette.button_selected
                                } else {
                                    palette.button_idle
                                }).stroke(if selected {
                                    egui::Stroke::new(1.0, palette.tile_text)
                                } else {
                                    egui::Stroke::NONE
                                });

                                // Wrap the button in vertical_centered to center it within its 1/3rd of the screen
                                let response = ui.vertical_centered(|ui| {
                                        ui.add_sized(button_size, button)
                                }).inner;

                                if response.clicked() {
                                    self.go_to_screen(screen);
                                }

                                // Draw the icon and text manually over the button's rectangle
                                let rect = response.rect;
                                let mut child_ui = ui.new_child(egui::UiBuilder::new().max_rect(rect));
                                child_ui.vertical_centered(|ui| {
                                    ui.add_space(4.0); // Small top padding
                                    ui.label(egui::RichText::new(icon).size(metrics.nav_bottom_icon_size));
                                    ui.label(
                                        egui::RichText::new(label).size(metrics.nav_button_text_size),
                                    );
                                });
                            }
                        });
                        ui.add_space(8.0);
                    });
                    ui.add_space(bottom_nav_inset);
                }
            });

        if self.show_back_confirm {
            egui::Window::new("Leave game?")
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(metrics.dialogue_content_size, egui::FontFamily::Proportional),
                    );

                    ui.label("Your current game is still in progress.");
                    ui.label("Are you sure you want to go back to the start screen?");
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                egui::vec2(96.0, metrics.nav_button_height),
                                egui::Button::new("Stay"),
                            )
                            .clicked()
                        {
                            self.cancel_back_navigation();
                        }

                        if ui
                            .add_sized(
                                egui::vec2(120.0, metrics.nav_button_height),
                                egui::Button::new("Leave game"),
                            )
                            .clicked()
                        {
                            self.confirm_back_navigation();
                        }
                    });
                });
        }

        if self.screen == Screen::Game && self.show_game_instructions {
            self.show_game_instructions_guide(ui.ctx());
        }
    }
}
