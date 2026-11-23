//! Theme: FireCraft ships in a crisp Fireworks CS6 Charcoal Dark theme, can follow the OS,
//! and can be switched to the measured Fireworks CS6 Classic Light theme (`#D6D6D6` / `#EDEDED` / `#999999`)
//! from the top header bar or the Window menu.
//!
//! All widgets, panels, menus, and windows use `CornerRadius::ZERO` and tight CS6 spacing
//! (extracted from `C:\Repos\FireCraft\crates\app\src\shell\glyphs.rs`) so nothing looks "bubbly".

use eframe::egui::{self, Color32, CornerRadius, Visuals};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

impl ThemeMode {
    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::Dark => "Dark (CS6 Charcoal)",
            ThemeMode::Light => "Light (CS6 Classic)",
            ThemeMode::System => "System Theme",
        }
    }

    #[allow(dead_code)]
    pub fn short_label(self) -> &'static str {
        match self {
            ThemeMode::Dark => "Theme: Dark",
            ThemeMode::Light => "Theme: Light",
            ThemeMode::System => "Theme: Auto",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg: Color32,
    pub panel: Color32,
    pub panel_alt: Color32,
    pub well: Color32,
    pub stroke: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub accent: Color32,
    pub accent_text: Color32,
    pub canvas_pad: Color32,
    pub paper: Color32,
    pub selection: Color32,
    pub slice_overlay: Color32,
    pub hotspot_overlay: Color32,
    pub guide: Color32,
    pub banner: Color32,
    pub banner_text: Color32,
    pub welcome: Color32,
}

impl Palette {
    pub fn is_dark(&self) -> bool {
        let _ = (self.accent_text, self.paper);
        let c = self.bg;
        let luma = 0.2126 * c.r() as f32 + 0.7152 * c.g() as f32 + 0.0722 * c.b() as f32;
        luma < 128.0
    }
}

/// Measured dark charcoal palette from `C:\Repos\FireCraft\crates\app\src\shell\glyphs.rs`
/// (`#3C3C3C` panel fill, `#323232` window fill, `#282828` canvas surround, `#444444` surface,
/// `#DDDDDD` WCAG AAA text, `#2F5D8A` / `#E8A33D` accents).
pub const DARK: Palette = Palette {
    bg: Color32::from_rgb(0x28, 0x28, 0x28),
    panel: Color32::from_rgb(0x3c, 0x3c, 0x3c),
    panel_alt: Color32::from_rgb(0x32, 0x32, 0x32),
    well: Color32::from_rgb(0x22, 0x22, 0x22),
    stroke: Color32::from_rgb(0x55, 0x55, 0x55),
    text: Color32::from_rgb(0xdd, 0xdd, 0xdd),
    text_dim: Color32::from_rgb(0xaa, 0xaa, 0xaa),
    accent: Color32::from_rgb(0x3a, 0x72, 0xa8),
    accent_text: Color32::WHITE,
    canvas_pad: Color32::from_rgb(0x28, 0x28, 0x28),
    paper: Color32::WHITE,
    selection: Color32::from_rgb(0x38, 0x98, 0xff),
    slice_overlay: Color32::from_rgba_premultiplied(0, 70, 36, 85),
    hotspot_overlay: Color32::from_rgba_premultiplied(0, 60, 85, 85),
    guide: Color32::from_rgb(0, 235, 255),
    banner: Color32::from_rgb(56, 50, 22),
    banner_text: Color32::from_rgb(248, 202, 24),
    welcome: Color32::from_rgb(0x32, 0x32, 0x32),
};

/// Measured Fireworks CS6 Windows light chrome (`#D6D6D6` panel bars, `#EDEDED` interiors,
/// `#999999` canvas surround, `#3388DA` active selection highlight).
pub const LIGHT: Palette = Palette {
    bg: Color32::from_rgb(0x99, 0x99, 0x99),
    panel: Color32::from_rgb(0xd6, 0xd6, 0xd6),
    panel_alt: Color32::from_rgb(0xc4, 0xc4, 0xc4),
    well: Color32::from_rgb(0xed, 0xed, 0xed),
    stroke: Color32::from_rgb(0x9a, 0x9a, 0x9a),
    text: Color32::from_rgb(0x18, 0x18, 0x1a),
    text_dim: Color32::from_rgb(0x50, 0x50, 0x56),
    accent: Color32::from_rgb(0x33, 0x80, 0xc8),
    accent_text: Color32::WHITE,
    canvas_pad: Color32::from_rgb(0x99, 0x99, 0x99),
    paper: Color32::WHITE,
    selection: Color32::from_rgb(0x18, 0x78, 0xe6),
    slice_overlay: Color32::from_rgba_premultiplied(0, 65, 32, 85),
    hotspot_overlay: Color32::from_rgba_premultiplied(0, 55, 78, 85),
    guide: Color32::from_rgb(0, 210, 230),
    banner: Color32::from_rgb(58, 52, 22),
    banner_text: Color32::from_rgb(245, 196, 0),
    welcome: Color32::from_rgb(0xed, 0xed, 0xed),
};

/// Reads the operating system's light/dark preference on Windows (cached).
pub fn system_is_light() -> bool {
    static CACHED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *CACHED.get_or_init(|| {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            let out = std::process::Command::new("reg")
                .args([
                    "query",
                    r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                    "/v",
                    "AppsUseLightTheme",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .output();
            if let Ok(out) = out {
                let text = String::from_utf8_lossy(&out.stdout);
                for token in text.split_whitespace() {
                    if token.eq_ignore_ascii_case("0x0") {
                        return false;
                    }
                    if token.eq_ignore_ascii_case("0x1") {
                        return true;
                    }
                }
            }
            false
        }
        #[cfg(not(target_os = "windows"))]
        {
            false
        }
    })
}

pub fn resolve(mode: ThemeMode) -> Palette {
    match mode {
        ThemeMode::Dark => DARK,
        ThemeMode::Light => LIGHT,
        ThemeMode::System => {
            if system_is_light() {
                LIGHT
            } else {
                DARK
            }
        }
    }
}

/// Apply crisp, square-cornered Fireworks CS6 desktop chrome to `ctx`.
pub fn apply_cs6_style(ctx: &egui::Context, p: &Palette, dark: bool) {
    let mut v = if dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    v.override_text_color = Some(p.text);
    v.panel_fill = p.panel;
    v.window_fill = p.panel_alt;
    v.extreme_bg_color = p.well;
    v.faint_bg_color = p.panel_alt;
    v.window_corner_radius = CornerRadius::ZERO;
    v.menu_corner_radius = CornerRadius::ZERO;

    v.widgets.noninteractive.bg_fill = p.panel_alt;
    v.widgets.noninteractive.weak_bg_fill = p.panel;
    v.widgets.noninteractive.bg_stroke = egui_stroke(p.stroke);
    v.widgets.noninteractive.corner_radius = CornerRadius::ZERO;

    v.widgets.inactive.bg_fill = p.panel_alt;
    v.widgets.inactive.weak_bg_fill = p.panel_alt;
    v.widgets.inactive.bg_stroke = egui_stroke(p.stroke);
    v.widgets.inactive.corner_radius = CornerRadius::ZERO;

    v.widgets.hovered.bg_fill = p.panel_alt;
    v.widgets.hovered.weak_bg_fill = p.panel_alt;
    v.widgets.hovered.bg_stroke = egui_stroke(p.accent);
    v.widgets.hovered.corner_radius = CornerRadius::ZERO;
    v.widgets.hovered.expansion = 0.0;

    v.widgets.active.bg_fill = p.accent;
    v.widgets.active.weak_bg_fill = p.accent;
    v.widgets.active.bg_stroke = egui_stroke(p.selection);
    v.widgets.active.corner_radius = CornerRadius::ZERO;
    v.widgets.active.expansion = 0.0;

    v.widgets.open.bg_fill = p.panel_alt;
    v.widgets.open.weak_bg_fill = p.panel;
    v.widgets.open.bg_stroke = egui_stroke(p.stroke);
    v.widgets.open.corner_radius = CornerRadius::ZERO;
    v.widgets.open.expansion = 0.0;

    v.selection.bg_fill = p.accent;
    v.selection.stroke = egui_stroke(p.selection);
    v.hyperlink_color = p.selection;
    v.window_stroke = egui_stroke(p.stroke);

    ctx.all_styles_mut(|style| {
        style.visuals = v.clone();
        style.spacing.menu_margin = egui::Margin::same(2);
        style.spacing.button_padding = egui::Vec2::new(5.0, 2.0);
        style.spacing.item_spacing = egui::Vec2::new(4.0, 2.0);
        style.spacing.window_margin = egui::Margin::same(6);
    });
}

fn egui_stroke(c: Color32) -> eframe::egui::Stroke {
    eframe::egui::Stroke::new(1.0, c)
}
