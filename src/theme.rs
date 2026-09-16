/// Lexivo Theming Engine
/// Defines the colour palettes, metrics and visual styles for Light and Dark modes.
use eframe::egui;

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub button_selected: egui::Color32,
    pub button_idle: egui::Color32,
    pub button_text: egui::Color32,
    pub tile_fill: egui::Color32,
    pub hint_label: egui::Color32,
    pub input_text: egui::Color32,
    pub revealed_text: egui::Color32,
    pub tile_text: egui::Color32,
    pub panel_fill: egui::Color32,
    pub group_fill: egui::Color32,
    pub text_primary: egui::Color32,
    pub text_muted: egui::Color32,
    pub message_text: egui::Color32,
    pub border: egui::Color32,
    pub hyperlink: egui::Color32,
    pub success: egui::Color32,
    pub error: egui::Color32,
}

#[derive(Clone, Copy, Debug)]
pub struct ThemeMetrics {
    pub top_padding: f32,
    pub item_spacing: egui::Vec2,
    pub button_padding: egui::Vec2,
    pub interact_size: egui::Vec2,
    pub text_edit_width: f32,
    pub start_heading_size: f32,
    pub game_heading_size: f32,
    pub dialogue_title_size: f32,
    pub dialogue_content_size: f32,
    pub settings_section_title_size: f32,
    pub changelog_version_size: f32,
    pub info_icon_size: f32,
    pub message_text_size: f32,
    pub hint_text_size: f32,
    pub nav_button_width: f32,
    pub nav_button_height: f32,
    pub nav_button_text_size: f32,
    pub nav_top_icon_size: f32,
    pub nav_bottom_icon_size: f32,
    pub mode_button_size: egui::Vec2,
    pub difficulty_button_size: egui::Vec2,
    pub question_count_button_width: f32,
    pub question_count_button_height: f32,
    pub settings_question_count_button_size: egui::Vec2,
    pub primary_button_size: egui::Vec2,
    pub wide_primary_button_size: egui::Vec2,
    pub game_action_button_size: egui::Vec2,
    pub hint_group_min_width: f32,
    pub settings_group_min_width: f32,
    pub tile_size: f32,
    pub tile_font_size: f32,
    pub tile_spacing_factor: f32,
    pub window_corner_radius: u8,
    pub widget_corner_radius: u8,
    pub border_width: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct AppTheme {
    pub palette: Palette,
    pub metrics: ThemeMetrics,
}

impl Palette {
    pub fn button_fill(self, selected: bool) -> egui::Color32 {
        if selected {
            self.button_selected
        } else {
            self.button_idle
        }
    }

    /// Generates an `egui::Stroke` based on current theme metrics.
    pub fn button_stroke(self, metrics: ThemeMetrics) -> egui::Stroke {
        egui::Stroke::new(metrics.border_width, self.border)
    }

    pub fn muted_text(self, text: impl Into<String>) -> egui::RichText {
        egui::RichText::new(text.into()).color(self.text_muted)
    }

    pub fn emphasised_text(self, text: impl Into<String>) -> egui::RichText {
        egui::RichText::new(text.into())
            .color(self.message_text)
            .strong()
    }

    pub fn revealed_text(self, text: impl Into<String>) -> egui::RichText {
        egui::RichText::new(text.into())
            .color(self.revealed_text)
            .strong()
    }
}

pub const LIGHT_THEME: AppTheme = AppTheme {
    palette: Palette {
        button_selected: egui::Color32::from_rgb(82, 122, 255),
        button_idle: egui::Color32::from_rgb(220, 220, 225),
        button_text: egui::Color32::from_rgb(24, 28, 36),
        tile_fill: egui::Color32::from_rgb(200, 200, 205),
        hint_label: egui::Color32::from_rgb(45, 95, 200),
        input_text: egui::Color32::from_rgb(156, 102, 0),
        revealed_text: egui::Color32::from_rgb(218, 165, 32), // Goldenrod (Vibrant Yellow/Gold)
        tile_text: egui::Color32::from_rgb(0, 0, 0),
        panel_fill: egui::Color32::from_rgb(255, 255, 255), // 246, 247, 251
        group_fill: egui::Color32::from_rgb(235, 237, 243),
        text_primary: egui::Color32::from_rgb(24, 28, 36),
        text_muted: egui::Color32::from_rgb(93, 99, 115),
        message_text: egui::Color32::from_rgb(37, 78, 190),
        border: egui::Color32::from_rgb(187, 193, 208),
        hyperlink: egui::Color32::from_rgb(45, 95, 200),
        success: egui::Color32::from_rgb(0, 153, 0),
        error: egui::Color32::from_rgb(204, 0, 0),
    },
    metrics: ThemeMetrics {
        top_padding: 30.0,
        item_spacing: egui::vec2(6.0, 8.0),
        button_padding: egui::vec2(8.0, 7.0),
        interact_size: egui::vec2(48.0, 48.0),
        text_edit_width: 320.0,
        start_heading_size: 40.0,
        game_heading_size: 38.0,
        dialogue_title_size: 25.0,
        dialogue_content_size: 16.0,
        settings_section_title_size: 22.0,
        changelog_version_size: 24.0,
        info_icon_size: 20.0,
        message_text_size: 28.0,
        hint_text_size: 24.0,
        nav_button_width: 90.0,
        nav_button_height: 64.0,
        nav_button_text_size: 14.0,
        nav_top_icon_size: 35.0,
        nav_bottom_icon_size: 20.0,
        mode_button_size: egui::vec2(220.0, 52.0),
        difficulty_button_size: egui::vec2(80.0, 34.0),
        question_count_button_width: 55.0,
        question_count_button_height: 34.0,
        settings_question_count_button_size: egui::vec2(50.0, 34.0),
        primary_button_size: egui::vec2(180.0, 44.0),
        wide_primary_button_size: egui::vec2(220.0, 40.0),
        game_action_button_size: egui::vec2(120.0, 50.0),
        hint_group_min_width: 300.0,
        settings_group_min_width: 230.0,
        tile_size: 56.0,
        tile_font_size: 34.0,
        tile_spacing_factor: 35.0,
        window_corner_radius: 12,
        widget_corner_radius: 8,
        border_width: 1.0,
    },
};

pub const DARK_THEME: AppTheme = AppTheme {
    palette: Palette {
        button_selected: egui::Color32::from_rgb(82, 122, 255),
        button_idle: egui::Color32::from_rgb(60, 60, 65),
        button_text: egui::Color32::from_rgb(244, 246, 252),
        tile_fill: egui::Color32::from_rgb(70, 70, 70),
        hint_label: egui::Color32::from_rgb(140, 190, 255),
        input_text: egui::Color32::from_rgb(255, 220, 110),
        revealed_text: egui::Color32::from_rgb(255, 215, 0), // Gold (Bright Yellow)
        tile_text: egui::Color32::from_rgb(255, 255, 255),
        panel_fill: egui::Color32::from_rgb(24, 26, 32),
        group_fill: egui::Color32::from_rgb(36, 39, 47),
        text_primary: egui::Color32::from_rgb(244, 246, 252),
        text_muted: egui::Color32::from_rgb(166, 173, 189),
        message_text: egui::Color32::from_rgb(168, 200, 255),
        border: egui::Color32::from_rgb(84, 89, 102),
        hyperlink: egui::Color32::from_rgb(140, 190, 255),
        success: egui::Color32::from_rgb(0, 255, 127),
        error: egui::Color32::from_rgb(255, 107, 107),
    },
    metrics: ThemeMetrics {
        top_padding: 30.0,
        item_spacing: egui::vec2(6.0, 8.0),
        button_padding: egui::vec2(8.0, 7.0),
        interact_size: egui::vec2(48.0, 48.0),
        text_edit_width: 320.0,
        start_heading_size: 40.0,
        game_heading_size: 38.0,
        dialogue_title_size: 25.0,
        dialogue_content_size: 16.0,
        settings_section_title_size: 22.0,
        changelog_version_size: 24.0,
        info_icon_size: 20.0,
        message_text_size: 28.0,
        hint_text_size: 24.0,
        nav_button_width: 90.0,
        nav_button_height: 64.0,
        nav_button_text_size: 14.0,
        nav_top_icon_size: 35.0,
        nav_bottom_icon_size: 20.0,
        mode_button_size: egui::vec2(220.0, 52.0),
        difficulty_button_size: egui::vec2(80.0, 34.0),
        question_count_button_width: 55.0,
        question_count_button_height: 34.0,
        settings_question_count_button_size: egui::vec2(50.0, 34.0),
        primary_button_size: egui::vec2(180.0, 44.0),
        wide_primary_button_size: egui::vec2(220.0, 40.0),
        game_action_button_size: egui::vec2(120.0, 50.0),
        hint_group_min_width: 300.0,
        settings_group_min_width: 230.0,
        tile_size: 56.0,
        tile_font_size: 34.0,
        tile_spacing_factor: 35.0,
        window_corner_radius: 12,
        widget_corner_radius: 8,
        border_width: 1.0,
    },
};

pub fn theme_for_ctx(ctx: &egui::Context) -> &'static AppTheme {
    if ctx.theme() == egui::Theme::Dark {
        &DARK_THEME
    } else {
        &LIGHT_THEME
    }
}

pub fn palette_for_ctx(ctx: &egui::Context) -> &'static Palette {
    &theme_for_ctx(ctx).palette
}

pub fn apply_to_ctx(ctx: &egui::Context) {
    apply_theme_to_variant(ctx, egui::Theme::Light, LIGHT_THEME);
    apply_theme_to_variant(ctx, egui::Theme::Dark, DARK_THEME);

    // Explicitly apply the correct visuals for the current active theme
    let current_theme = if ctx.theme() == egui::Theme::Dark {
        DARK_THEME
    } else {
        LIGHT_THEME
    };
    let mut visuals = if ctx.theme() == egui::Theme::Dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    // Re-apply the specific palette overrides to the context's current visuals
    let palette = current_theme.palette;

    visuals.window_fill = palette.panel_fill;
    visuals.panel_fill = palette.panel_fill;
    visuals.faint_bg_color = palette.group_fill;
    visuals.extreme_bg_color = palette.group_fill;
    visuals.override_text_color = Some(palette.text_primary);
    visuals.hyperlink_color = palette.hyperlink;
    ctx.set_visuals(visuals);
}

fn apply_theme_to_variant(ctx: &egui::Context, theme: egui::Theme, app_theme: AppTheme) {
    let palette = app_theme.palette;
    let metrics = app_theme.metrics;

    ctx.style_mut_of(theme, |style| {
        style.spacing.item_spacing = metrics.item_spacing;
        style.spacing.button_padding = metrics.button_padding;
        style.spacing.interact_size = metrics.interact_size;
        style.spacing.text_edit_width = metrics.text_edit_width;

        // Increase default text styles by +6 (approx "two sizes up")
        style.text_styles.insert(
            egui::TextStyle::Small,
            egui::FontId::new(15.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(18.5, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(18.5, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(metrics.dialogue_title_size, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(18.0, egui::FontFamily::Monospace),
        );
    });

    let mut visuals = match theme {
        egui::Theme::Light => egui::Visuals::light(),
        egui::Theme::Dark => egui::Visuals::dark(),
    };

    visuals.window_fill = palette.panel_fill;
    visuals.panel_fill = palette.panel_fill;
    visuals.faint_bg_color = palette.group_fill;
    visuals.extreme_bg_color = palette.group_fill;
    visuals.code_bg_color = palette.group_fill;
    visuals.override_text_color = Some(palette.text_primary);
    visuals.hyperlink_color = palette.hyperlink;
    visuals.window_stroke = palette.button_stroke(metrics);
    visuals.window_corner_radius = egui::CornerRadius::same(metrics.window_corner_radius);

    visuals.widgets.noninteractive.bg_fill = palette.group_fill;
    visuals.widgets.noninteractive.bg_stroke = palette.button_stroke(metrics);
    visuals.widgets.noninteractive.fg_stroke =
        egui::Stroke::new(metrics.border_width, palette.text_primary);
    visuals.widgets.noninteractive.corner_radius =
        egui::CornerRadius::same(metrics.widget_corner_radius);

    visuals.widgets.inactive.bg_fill = palette.button_idle;
    visuals.widgets.inactive.bg_stroke = palette.button_stroke(metrics);
    visuals.widgets.inactive.fg_stroke =
        egui::Stroke::new(metrics.border_width, palette.button_text);
    visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(metrics.widget_corner_radius);

    visuals.widgets.hovered.bg_fill = palette.button_selected.gamma_multiply(0.9);
    visuals.widgets.hovered.bg_stroke = palette.button_stroke(metrics);
    visuals.widgets.hovered.fg_stroke = egui::Stroke::new(metrics.border_width, palette.tile_text);
    visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(metrics.widget_corner_radius);

    visuals.widgets.active.bg_fill = palette.button_selected.gamma_multiply(0.8);
    visuals.widgets.active.bg_stroke = palette.button_stroke(metrics);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(metrics.border_width, palette.tile_text);
    visuals.widgets.active.corner_radius = egui::CornerRadius::same(metrics.widget_corner_radius);

    visuals.widgets.open.bg_fill = palette.group_fill;
    visuals.widgets.open.bg_stroke = palette.button_stroke(metrics);
    visuals.widgets.open.fg_stroke = egui::Stroke::new(metrics.border_width, palette.text_primary);
    visuals.widgets.open.corner_radius = egui::CornerRadius::same(metrics.widget_corner_radius);

    ctx.set_visuals_of(theme, visuals);
}
