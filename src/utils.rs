/// Lexivo Utility Functions
/// General-purpose helpers for word scrambling, scoring and UI drawing.

use crate::types::Difficulty;
use eframe::egui;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{RngCore, SeedableRng};
use std::time::{SystemTime, UNIX_EPOCH};

/// Normalises a string by trimming whitespace, converting to uppercase and removing non-alphanumeric characters.
pub fn normalise_answer(value: &str) -> String {
    value
        .trim()
        .to_uppercase()
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect()
}

pub fn primary_action_button(ui: &mut egui::Ui, label: &str, size: egui::Vec2) -> egui::Response {
    let theme = crate::theme::theme_for_ctx(ui.ctx());

    ui.add(
        egui::Button::new(egui::RichText::new(label).strong().color(theme.palette.tile_text))
            .min_size(size)
            .fill(theme.palette.button_selected)
            .stroke(theme.palette.button_stroke(theme.metrics)),
    )
}

/// Draws a bouncy word tile for the scrambling area.
pub fn draw_tile(
    ui: &mut egui::Ui,
    c: char,
    index: usize,
    time: f64,
    fill_colour: egui::Color32,
    text_colour: egui::Color32,
    metrics: crate::theme::ThemeMetrics,
    animations_enabled: bool,
) -> bool {
    let scale = if animations_enabled {
        1.0 + ((time * 2.5 + index as f64 * 0.8).sin() * 0.16)
    } else {
        1.0
    };

    let tile_size = egui::vec2(
        metrics.tile_size * scale as f32,
        metrics.tile_size * scale as f32,
    );
    let (rect, response) = ui.allocate_at_least(tile_size, egui::Sense::click());
    ui.painter().rect_filled(rect, 7.0, fill_colour);

    let mut buf = [0u8; 4];
    let s = c.encode_utf8(&mut buf);

    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        s,
        egui::FontId::proportional(metrics.tile_font_size * scale as f32),
        text_colour,
    );

    response.clicked()
}

/// Generates a stable seed for the daily challenge based on the current date.
pub fn daily_seed_for_day(epoch_seconds: u64) -> u64 {
    let days_since_epoch = epoch_seconds / 86_400;
    days_since_epoch ^ 0xC0FFEE_1234u64
}

/// Generates a stable, difficulty-specific seed for the daily challenge.
pub fn daily_seed_for_difficulty(level: Difficulty, epoch_seconds: u64) -> u64 {
    let base = daily_seed_for_day(epoch_seconds);
    let twist = match level {
        Difficulty::Easy => 0x11_11_11_11_11_11_11u64,
        Difficulty::Medium => 0x22_22_22_22_22_22_22u64,
        Difficulty::Hard => 0x33_33_33_33_33_33_33u64,
    };
    base ^ twist
}

pub fn daily_seed() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();
    daily_seed_for_day(since_epoch.as_secs())
}

pub fn seed_from_time() -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos() as u64);

    if nanos == 0 {
        0x9E37_79B9_7F4A_7C15
    } else {
        nanos
    }
}

/// Shuffles a slice in-place using the provided RNG state.
pub fn shuffle_slice<T>(items: &mut [T], state: &mut u64) {
    if items.len() < 2 {
        return;
    }

    let mut rng = StdRng::seed_from_u64(*state);
    items.shuffle(&mut rng);
    *state = rng.next_u64();
}

/// Scrambles a word by shuffling its characters. Ensures the result is different from the original if possible.
pub fn scramble_word(word: &str, state: &mut u64) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    if chars.len() < 2 {
        return word.to_string();
    }

    let original = chars.clone();
    let can_change = chars.iter().any(|ch| *ch != chars[0]);

    for _ in 0..8 {
        shuffle_slice(&mut chars, state);
        if !can_change || chars != original {
            break;
        }
    }

    // Deterministic fallback: guarantee a different arrangement when one exists.
    if can_change && chars == original {
        chars.rotate_left(1);
    }

    chars.into_iter().collect()
}
