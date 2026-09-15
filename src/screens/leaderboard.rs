// Leaderboard Screen
// Displays the user's top scores and personal best for the current session.

use crate::app::LexivoApp;
use eframe::egui;

pub fn render_leaderboard_screen(app: &mut LexivoApp, ui: &mut egui::Ui) {
    let palette = crate::theme::palette_for_ctx(ui.ctx());
    ui.vertical_centered(|ui| {
        // Main content area for Leaderboard screen
        // Title heading
        ui.heading("Leaderboard");
        ui.add_space(16.0);

        // Display list of top scores
        if app.leaderboard.is_empty() {
            ui.label("No scores yet. Start a round to set a new record!");
        } else {
            for (index, score) in app.leaderboard.iter().enumerate() {
                ui.label(format!("{}. {} pts", index + 1, score));
                ui.add_space(8.0);
            }
        }

        ui.add_space(20.0);
        // Personal best summary
        ui.label(palette.muted_text(format!("Best score: {}", app.best_score)));
    });
}
