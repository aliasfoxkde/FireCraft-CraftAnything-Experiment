//! Theme: FireCraft ships in a dark theme, can follow the operating system,
//! and can be switched to the classic Fireworks CS6 light theme from the Window menu.

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
            ThemeMode::Dark => "Dark (Modern)",
            ThemeMode::Light => "Light (CS6 Classic)",
            ThemeMode::System => "Follow System",
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

pub const DARK: Palette = Palette {
    bg: Color32::from_rgb(30, 31, 35),
    panel: Color32::from_rgb(43, 45, 50),
    panel_alt: Color32::from_rgb(54, 57, 63),
    well: Color32::from_rgb(24, 25, 28),
    stroke: Color32::from_rgb(72, 75, 84),
    text: Color32::from_rgb(235, 236, 240),
    text_dim: Color32::from_rgb(160, 164, 174),
    accent: Color32::from_rgb(232, 163, 61),
    accent_text: Color32::from_rgb(20, 16, 10),
    canvas_pad: Color32::from_rgb(56, 58, 64),
    paper: Color32::WHITE,
    selection: Color32::from_rgb(56, 152, 255),
    slice_overlay: Color32::from_rgba_premultiplied(0, 70, 36, 85),
    hotspot_overlay: Color32::from_rgba_premultiplied(0, 60, 85, 85),
    guide: Color32::from_rgb(0, 235, 255),
    banner: Color32::from_rgb(62, 54, 24),
    banner_text: Color32::from_rgb(248, 202, 24),
    welcome: Color32::from_rgb(38, 40, 45),
};

pub const LIGHT: Palette = Palette {
    bg: Color32::from_rgb(166, 166, 166),
    panel: Color32::from_rgb(214, 214, 214),
    panel_alt: Color32::from_rgb(194, 194, 194),
    well: Color32::from_rgb(246, 246, 246),
    stroke: Color32::from_rgb(148, 148, 148),
    text: Color32::from_rgb(24, 24, 26),
    text_dim: Color32::from_rgb(84, 84, 92),
    accent: Color32::from_rgb(51, 136, 218),
    accent_text: Color32::WHITE,
    canvas_pad: Color32::from_rgb(166, 166, 166),
    paper: Color32::WHITE,
    selection: Color32::from_rgb(24, 120, 230),
    slice_overlay: Color32::from_rgba_premultiplied(0, 65, 32, 85),
    hotspot_overlay: Color32::from_rgba_premultiplied(0, 55, 78, 85),
    guide: Color32::from_rgb(0, 210, 230),
    banner: Color32::from_rgb(58, 52, 22),
    banner_text: Color32::from_rgb(245, 196, 0),
    welcome: Color32::from_rgb(244, 244, 244),
};

/// Reads the operating system's light/dark preference on Windows.
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
