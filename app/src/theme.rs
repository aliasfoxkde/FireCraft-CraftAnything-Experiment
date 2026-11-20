#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
//! Theme: FireCraft ships in a dark theme, can follow the operating system,
//! and can be switched to light from the FireCraft menu.

#![allow(dead_code, unused_imports, unused_variables, unused_must_use, unused_parens, unused_assignments)]
use eframe::egui::{Color32, Visuals};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

impl ThemeMode {
    pub fn label(self) -> &'static str {
        match self {
            ThemeMode::Dark => "Dark",
            ThemeMode::Light => "Light",
            ThemeMode::System => "System",
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
    /// The welcome screen's banner and its body.
    pub banner: Color32,
    pub banner_text: Color32,
    pub welcome: Color32,
}

impl Palette {
    /// Whether this palette is a dark one, decided from the background's
    /// luminance so a new palette only has to supply colours.
    pub fn is_dark(&self) -> bool {
        let c = self.bg;
        let luma = 0.2126 * c.r() as f32 + 0.7152 * c.g() as f32 + 0.0722 * c.b() as f32;
        luma < 128.0
    }
}

pub const DARK: Palette = Palette {
    bg: Color32::from_rgb(26, 26, 30),
    panel: Color32::from_rgb(40, 40, 46),
    panel_alt: Color32::from_rgb(48, 48, 55),
    well: Color32::from_rgb(30, 30, 35),
    stroke: Color32::from_rgb(66, 66, 76),
    text: Color32::from_rgb(226, 226, 232),
    text_dim: Color32::from_rgb(150, 150, 162),
    accent: Color32::from_rgb(232, 163, 61),
    accent_text: Color32::from_rgb(24, 20, 12),
    canvas_pad: Color32::from_rgb(58, 58, 64),
    paper: Color32::WHITE,
    selection: Color32::from_rgb(232, 163, 61),
    banner: Color32::from_rgb(69, 65, 30),
    banner_text: Color32::from_rgb(245, 196, 0),
    welcome: Color32::from_rgb(249, 249, 249),
};

pub const LIGHT: Palette = Palette {
    bg: Color32::from_rgb(245, 245, 250),
    panel: Color32::from_rgb(255, 255, 255),
    panel_alt: Color32::from_rgb(235, 235, 245),
    well: Color32::from_rgb(230, 230, 240),
    stroke: Color32::from_rgb(200, 200, 215),
    text: Color32::from_rgb(25, 25, 30),
    text_dim: Color32::from_rgb(100, 100, 115),
    accent: Color32::from_rgb(232, 163, 61),
    accent_text: Color32::from_rgb(255, 255, 255),
    canvas_pad: Color32::from_rgb(210, 210, 225),
    paper: Color32::WHITE,
    selection: Color32::from_rgb(232, 163, 61),
    banner: Color32::from_rgb(35, 35, 45),
    banner_text: Color32::from_rgb(255, 255, 255),
    welcome: Color32::from_rgb(255, 255, 255),
};

/// Reads the operating system's light/dark preference.
///
/// Windows stores it in `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize`
/// as `AppsUseLightTheme` (0 = dark, 1 = light). Other platforms fall back to dark,
/// which is FireCraft's own default.
pub fn system_is_light() -> bool {
    #[cfg(target_os = "windows")]
    {
        let out = std::process::Command::new("reg")
            .args([
                "query",
                r"HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize",
                "/v",
                "AppsUseLightTheme",
            ])
            .output();
        if let Ok(out) = out {
            let text = String::from_utf8_lossy(&out.stdout).to_string();
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

pub fn visuals(p: &Palette, dark: bool) -> Visuals {
    let mut v = if dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    v.override_text_color = Some(p.text);
    v.panel_fill = p.panel;
    v.window_fill = p.panel;
    v.extreme_bg_color = p.well;
    v.faint_bg_color = p.panel_alt;
    v.widgets.noninteractive.bg_fill = p.panel_alt;
    v.widgets.noninteractive.weak_bg_fill = p.panel;
    v.widgets.noninteractive.bg_stroke = egui_stroke(p.stroke);
    v.widgets.inactive.bg_fill = p.panel_alt;
    v.widgets.inactive.weak_bg_fill = p.panel_alt;
    v.widgets.inactive.bg_stroke = egui_stroke(p.stroke);
    v.widgets.hovered.bg_fill = p.panel_alt;
    v.widgets.hovered.weak_bg_fill = p.panel_alt;
    v.widgets.hovered.bg_stroke = egui_stroke(p.accent);
    v.widgets.active.bg_fill = p.accent;
    v.widgets.active.weak_bg_fill = p.accent;
    v.widgets.active.bg_stroke = egui_stroke(p.accent);
    v.widgets.open.bg_fill = p.panel_alt;
    v.widgets.open.weak_bg_fill = p.panel;
    v.widgets.open.bg_stroke = egui_stroke(p.stroke);
    v.selection.bg_fill = p.accent.gamma_multiply(0.45);
    v.selection.stroke = egui_stroke(p.accent);
    v.hyperlink_color = p.accent;
    v.window_stroke = egui_stroke(p.stroke);
    v
}

fn egui_stroke(c: Color32) -> eframe::egui::Stroke {
    eframe::egui::Stroke::new(1.0, c)
}


