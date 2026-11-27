//! Document model: pages, animation states, layers, vector/bitmap objects,
//! live effects, web slices/hotspots, symbols and styles.

use eframe::egui::{Color32, Vec2};

use crate::constants;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Live non-destructive effects attached to an object (Fireworks CS6 Live Filters).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum LiveEffect {
    DropShadow {
        distance: f32,
        angle_deg: f32,
        blur: f32,
        color: Color32,
        enabled: bool,
    },
    InnerShadow {
        distance: f32,
        blur: f32,
        color: Color32,
        enabled: bool,
    },
    Glow {
        radius: f32,
        color: Color32,
        inner: bool,
        enabled: bool,
    },
    Bevel {
        width: f32,
        highlight: Color32,
        shadow: Color32,
        enabled: bool,
    },
    GaussianBlur {
        radius: f32,
        enabled: bool,
    },
    ColorAdjust {
        brightness: f32,
        contrast: f32,
        saturation: f32,
        enabled: bool,
    },
}

impl LiveEffect {
    pub fn label(&self) -> &'static str {
        match self {
            LiveEffect::DropShadow { .. } => "Drop Shadow",
            LiveEffect::InnerShadow { .. } => "Inner Shadow",
            LiveEffect::Glow { inner: false, .. } => "Outer Glow",
            LiveEffect::Glow { inner: true, .. } => "Inner Glow",
            LiveEffect::Bevel { .. } => "Inner Bevel",
            LiveEffect::GaussianBlur { .. } => "Gaussian Blur",
            LiveEffect::ColorAdjust { .. } => "Brightness/Contrast",
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            LiveEffect::DropShadow { enabled, .. }
            | LiveEffect::InnerShadow { enabled, .. }
            | LiveEffect::Glow { enabled, .. }
            | LiveEffect::Bevel { enabled, .. }
            | LiveEffect::GaussianBlur { enabled, .. }
            | LiveEffect::ColorAdjust { enabled, .. } => *enabled,
        }
    }

    pub fn set_enabled(&mut self, val: bool) {
        match self {
            LiveEffect::DropShadow { enabled, .. }
            | LiveEffect::InnerShadow { enabled, .. }
            | LiveEffect::Glow { enabled, .. }
            | LiveEffect::Bevel { enabled, .. }
            | LiveEffect::GaussianBlur { enabled, .. }
            | LiveEffect::ColorAdjust { enabled, .. } => *enabled = val,
        }
    }

    /// The values the menu and Properties inspector show for this effect. Keeping the
    /// formatting beside the enum prevents menus from drifting into their own static copy.
    pub fn summary(&self) -> String {
        match self {
            LiveEffect::DropShadow {
                distance,
                angle_deg,
                blur,
                ..
            } => format!("{distance:.0}px · {angle_deg:.0}° · blur {blur:.0}"),
            LiveEffect::InnerShadow { distance, blur, .. } => {
                format!("{distance:.0}px · blur {blur:.0}")
            }
            LiveEffect::Glow { radius, inner, .. } => {
                format!(
                    "{} · radius {radius:.0}",
                    if *inner { "Inner" } else { "Outer" }
                )
            }
            LiveEffect::Bevel { width, .. } => format!("width {width:.0}"),
            LiveEffect::GaussianBlur { radius, .. } => format!("radius {radius:.1}"),
            LiveEffect::ColorAdjust {
                brightness,
                contrast,
                saturation,
                ..
            } => format!("B {brightness:+.0} · C {contrast:+.0} · S {saturation:+.0}"),
        }
    }
}

/// Auto Shape variants from Fireworks CS6.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AutoShapeKind {
    RoundedRect,
    Star,
    Polygon,
    Arrow,
    Callout,
    Pie,
    Donut,
    Spiral,
    Gear,
    ChamferRect,
    LShape,
    Cross,
}

impl AutoShapeKind {
    pub const ALL: [AutoShapeKind; 12] = [
        AutoShapeKind::RoundedRect,
        AutoShapeKind::Star,
        AutoShapeKind::Polygon,
        AutoShapeKind::Arrow,
        AutoShapeKind::Callout,
        AutoShapeKind::Pie,
        AutoShapeKind::Donut,
        AutoShapeKind::Spiral,
        AutoShapeKind::Gear,
        AutoShapeKind::ChamferRect,
        AutoShapeKind::LShape,
        AutoShapeKind::Cross,
    ];

    pub fn label(self) -> &'static str {
        match self {
            AutoShapeKind::RoundedRect => "Rounded Rectangle",
            AutoShapeKind::Star => "Star",
            AutoShapeKind::Polygon => "Smart Polygon",
            AutoShapeKind::Arrow => "Arrow",
            AutoShapeKind::Callout => "Talk Bubble",
            AutoShapeKind::Pie => "Pie",
            AutoShapeKind::Donut => "Doughnut",
            AutoShapeKind::Spiral => "Spiral",
            AutoShapeKind::Gear => "Cog / Gear",
            AutoShapeKind::ChamferRect => "Chamfer Rectangle",
            AutoShapeKind::LShape => "L-Shape",
            AutoShapeKind::Cross => "Cross",
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Shape {
    Rect {
        min: Vec2,
        max: Vec2,
        corner_radius: f32,
        filled: bool,
    },
    Ellipse {
        center: Vec2,
        radius: Vec2,
        filled: bool,
    },
    Line {
        a: Vec2,
        b: Vec2,
    },
    Polygon {
        center: Vec2,
        radius: f32,
        sides: u32,
        star_inner: f32,
        angle_rad: f32,
    },
    Path {
        points: Vec<Vec2>,
        closed: bool,
    },
    Text {
        at: Vec2,
        text: String,
        size: f32,
        bold: bool,
        italic: bool,
        font_family: String,
    },
    Gradient {
        min: Vec2,
        max: Vec2,
        from: Color32,
        to: Color32,
        radial: bool,
    },
    Bitmap {
        at: Vec2,
        pixels: Vec<u8>,
        size: Vec2,
        /// Source raster dimensions. Missing values in older project files fall back to `size`.
        #[serde(default)]
        pixel_size: Vec2,
    },
    Slice {
        min: Vec2,
        max: Vec2,
        url: String,
        alt: String,
        target: String,
        format: String,
        is_hotspot: bool,
    },
    SymbolInstance {
        symbol_id: usize,
        at: Vec2,
        scale: Vec2,
    },
    Group {
        children: Vec<Obj>,
    },
}

/// Blend modes supported by Fireworks CS6 layers and objects.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    Difference,
}

impl BlendMode {
    pub const ALL: [BlendMode; 7] = [
        BlendMode::Normal,
        BlendMode::Multiply,
        BlendMode::Screen,
        BlendMode::Overlay,
        BlendMode::Darken,
        BlendMode::Lighten,
        BlendMode::Difference,
    ];

    pub fn label(self) -> &'static str {
        match self {
            BlendMode::Normal => "Normal",
            BlendMode::Multiply => "Multiply",
            BlendMode::Screen => "Screen",
            BlendMode::Overlay => "Overlay",
            BlendMode::Darken => "Darken",
            BlendMode::Lighten => "Lighten",
            BlendMode::Difference => "Difference",
        }
    }
}

/// Per-object linear gradient fill. `fill` in `Obj` remains the representative/fallback color used
/// by thumbnails and CSS export.
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct GradientFill {
    pub start: Color32,
    pub end: Color32,
    /// Gradient axis endpoints in normalized object-bounds coordinates. Missing values in older
    /// project files retain the historic top-to-bottom gradient direction.
    #[serde(default = "default_gradient_start_pos")]
    pub start_pos: Vec2,
    #[serde(default = "default_gradient_end_pos")]
    pub end_pos: Vec2,
}

fn default_gradient_start_pos() -> Vec2 {
    Vec2::new(0.5, 0.0)
}

fn default_gradient_end_pos() -> Vec2 {
    Vec2::new(0.5, 1.0)
}

impl GradientFill {
    pub fn new(start: Color32, end: Color32) -> Self {
        Self {
            start,
            end,
            start_pos: default_gradient_start_pos(),
            end_pos: default_gradient_end_pos(),
        }
    }

    /// Return the clamped interpolation parameter for a normalized point inside the object's
    /// bounds. The end colors extend beyond the axis endpoints; a zero-length axis is neutral.
    pub fn t_at(self, normalized_pos: Vec2) -> f32 {
        let axis = self.end_pos - self.start_pos;
        let length_sq = axis.length_sq();
        if !length_sq.is_finite() || length_sq <= 1.0e-12 {
            return 0.5;
        }
        let t = (normalized_pos - self.start_pos).dot(axis) / length_sq;
        if t.is_finite() {
            t.clamp(0.0, 1.0)
        } else {
            0.5
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Obj {
    pub kind: Shape,
    pub fill: Option<Color32>,
    /// Optional gradient fill rendered instead of the flat `fill`.
    pub gradient: Option<GradientFill>,
    pub stroke: Option<Color32>,
    pub width: f32,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub effects: Vec<LiveEffect>,
}

impl Obj {
    pub fn new(kind: Shape, fill: Option<Color32>, stroke: Option<Color32>) -> Self {
        let name = match &kind {
            Shape::Rect { corner_radius, .. } => {
                if *corner_radius > 0.0 {
                    "Rounded Rectangle".to_string()
                } else {
                    "Rectangle".to_string()
                }
            }
            Shape::Ellipse { .. } => "Ellipse".to_string(),
            Shape::Line { .. } => "Line".to_string(),
            Shape::Polygon { star_inner, .. } => {
                if *star_inner < 0.99 {
                    "Star".to_string()
                } else {
                    "Polygon".to_string()
                }
            }
            Shape::Path { .. } => "Path".to_string(),
            Shape::Text { text, .. } => {
                if text.is_empty() {
                    "Text".to_string()
                } else {
                    text.clone()
                }
            }
            Shape::Gradient { .. } => "Gradient".to_string(),
            Shape::Bitmap { .. } => "Bitmap".to_string(),
            Shape::Slice { is_hotspot, .. } => {
                if *is_hotspot {
                    "Hotspot".to_string()
                } else {
                    "Slice".to_string()
                }
            }
            Shape::SymbolInstance { symbol_id, .. } => format!("Symbol {}", symbol_id + 1),
            Shape::Group { children } => format!("Group ({} objects)", children.len()),
        };
        Self {
            kind,
            fill,
            gradient: None,
            stroke,
            width: 1.0,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            name,
            visible: true,
            locked: false,
            effects: Vec::new(),
        }
    }

    pub fn type_label(&self) -> &'static str {
        match &self.kind {
            Shape::Rect { corner_radius, .. } => {
                if *corner_radius > 0.0 {
                    "Rounded Rect"
                } else {
                    "Rectangle"
                }
            }
            Shape::Ellipse { .. } => "Ellipse",
            Shape::Line { .. } => "Line",
            Shape::Polygon { star_inner, .. } => {
                if *star_inner < 0.99 {
                    "Star"
                } else {
                    "Polygon"
                }
            }
            Shape::Path { .. } => "Vector Path",
            Shape::Text { .. } => "Text",
            Shape::Gradient { .. } => "Gradient",
            Shape::Bitmap { .. } => "Bitmap",
            Shape::Slice {
                is_hotspot: true, ..
            } => "Web Hotspot",
            Shape::Slice {
                is_hotspot: false, ..
            } => "Web Slice",
            Shape::SymbolInstance { .. } => "Symbol Instance",
            Shape::Group { .. } => "Group",
        }
    }

    /// Generate polygon vertices in document coordinates.
    pub fn polygon_vertices(
        center: Vec2,
        radius: f32,
        sides: u32,
        star_inner: f32,
        angle_rad: f32,
    ) -> Vec<Vec2> {
        let n = sides.max(3) as usize;
        let is_star = (star_inner - 1.0).abs() > 0.01;
        let total = if is_star { n * 2 } else { n };
        let mut pts = Vec::with_capacity(total);
        for i in 0..total {
            let frac = i as f32 / total as f32;
            let theta = angle_rad - std::f32::consts::FRAC_PI_2 + frac * std::f32::consts::TAU;
            let r = if is_star && i % 2 == 1 {
                radius * star_inner.clamp(0.1, 1.0)
            } else {
                radius
            };
            pts.push(center + Vec2::new(r * theta.cos(), r * theta.sin()));
        }
        pts
    }

    /// The axis-aligned bounding box, always normalised so `lo <= hi` on both axes.
    pub fn bounds(&self) -> Option<(Vec2, Vec2)> {
        match &self.kind {
            Shape::Rect { min, max, .. } => Some(((*min).min(*max), (*min).max(*max))),
            Shape::Ellipse { center, radius, .. } => {
                let r = radius.abs();
                Some((*center - r, *center + r))
            }
            Shape::Line { a, b } => Some(((*a).min(*b), (*a).max(*b))),
            Shape::Polygon {
                center,
                radius,
                sides,
                star_inner,
                angle_rad,
            } => {
                let pts = Self::polygon_vertices(*center, *radius, *sides, *star_inner, *angle_rad);
                let mut lo = pts[0];
                let mut hi = pts[0];
                for p in &pts {
                    lo = lo.min(*p);
                    hi = hi.max(*p);
                }
                Some((lo, hi))
            }
            Shape::Path { points, .. } => {
                if points.is_empty() {
                    return None;
                }
                let mut lo = points[0];
                let mut hi = points[0];
                for p in points {
                    lo = lo.min(*p);
                    hi = hi.max(*p);
                }
                Some((lo, hi))
            }
            Shape::Text { at, text, size, .. } => {
                // Multi-line text occupies one line height per line, and its width comes from
                // the longest line — counting the newline itself made selection boxes too wide
                // and made multi-line bodies look like they overflowed their cards.
                let lines = text.split('\n').count().max(1) as f32;
                Some((
                    Vec2::new(at.x, at.y - size * 1.2 * lines),
                    Vec2::new(at.x + text_width(text, *size), at.y),
                ))
            }
            Shape::Gradient { min, max, .. } => Some(((*min).min(*max), (*min).max(*max))),
            Shape::Bitmap { at, size, .. } => Some((*at, *at + *size)),
            Shape::Slice { min, max, .. } => Some(((*min).min(*max), (*min).max(*max))),
            Shape::SymbolInstance { at, scale, .. } => {
                let s = Vec2::new(64.0 * scale.x.abs(), 64.0 * scale.y.abs());
                Some((*at, *at + s))
            }
            Shape::Group { children } => {
                let mut lo = Vec2::splat(f32::MAX);
                let mut hi = Vec2::splat(f32::MIN);
                let mut any = false;
                for ch in children {
                    if let Some((clo, chi)) = ch.bounds() {
                        lo = lo.min(clo);
                        hi = hi.max(chi);
                        any = true;
                    }
                }
                any.then_some((lo, hi))
            }
        }
    }

    /// Dimensions of the stored RGBA buffer, independent of its on-canvas display size.
    pub fn bitmap_pixel_size(&self) -> Option<Vec2> {
        let Shape::Bitmap {
            pixels,
            size,
            pixel_size,
            ..
        } = &self.kind
        else {
            return None;
        };
        let stored = if pixel_size.x >= 1.0 && pixel_size.y >= 1.0 {
            *pixel_size
        } else {
            *size
        };
        let width = stored.x.max(1.0).floor() as usize;
        let height = stored.y.max(1.0).floor() as usize;
        (pixels.len() == width.saturating_mul(height).saturating_mul(4))
            .then_some(Vec2::new(width as f32, height as f32))
    }

    pub fn translate(&mut self, delta: Vec2) {
        match &mut self.kind {
            Shape::Rect { min, max, .. }
            | Shape::Gradient { min, max, .. }
            | Shape::Slice { min, max, .. } => {
                *min += delta;
                *max += delta;
            }
            Shape::Ellipse { center, .. } | Shape::Polygon { center, .. } => *center += delta,
            Shape::Line { a, b } => {
                *a += delta;
                *b += delta;
            }
            Shape::Path { points, .. } => {
                for p in points.iter_mut() {
                    *p += delta;
                }
            }
            Shape::Text { at, .. }
            | Shape::Bitmap { at, .. }
            | Shape::SymbolInstance { at, .. } => *at += delta,
            Shape::Group { children } => {
                for ch in children.iter_mut() {
                    ch.translate(delta);
                }
            }
        }
    }

    /// Resize the object's bounding box to `(new_lo, new_hi)`.
    pub fn set_bounds(&mut self, new_lo: Vec2, new_hi: Vec2) {
        let lo = new_lo.min(new_hi);
        let hi = new_lo.max(new_hi);
        let new_size = (hi - lo).max(Vec2::splat(1.0));
        let prev_bounds = self.bounds();
        match &mut self.kind {
            Shape::Rect { min, max, .. }
            | Shape::Gradient { min, max, .. }
            | Shape::Slice { min, max, .. } => {
                *min = lo;
                *max = lo + new_size;
            }
            Shape::Ellipse { center, radius, .. } => {
                *center = lo + new_size * 0.5;
                *radius = new_size * 0.5;
            }
            Shape::Polygon { center, radius, .. } => {
                *center = lo + new_size * 0.5;
                *radius = new_size.x.min(new_size.y) * 0.5;
            }
            Shape::Line { a, b } => {
                *a = lo;
                *b = hi;
            }
            Shape::Path { points, .. } => {
                if let Some((old_lo, old_hi)) = prev_bounds {
                    let old_size = (old_hi - old_lo).max(Vec2::splat(1.0));
                    for p in points.iter_mut() {
                        let norm = (*p - old_lo) / old_size;
                        *p = lo + norm * new_size;
                    }
                }
            }
            Shape::Text { at, size, .. } => {
                *at = Vec2::new(lo.x, hi.y);
                *size = (new_size.y / 1.2).clamp(6.0, 240.0);
            }
            Shape::Bitmap {
                at,
                pixels,
                size,
                pixel_size,
            } => {
                if pixel_size.x < 1.0 || pixel_size.y < 1.0 {
                    let inferred = size.max(Vec2::splat(1.0));
                    let width = inferred.x.floor() as usize;
                    let height = inferred.y.floor() as usize;
                    if pixels.len() == width.saturating_mul(height).saturating_mul(4) {
                        *pixel_size = Vec2::new(width as f32, height as f32);
                    }
                }
                *at = lo;
                *size = new_size;
            }
            Shape::SymbolInstance { at, scale, .. } => {
                *at = lo;
                *scale = new_size / 64.0;
            }
            Shape::Group { children } => {
                if let Some((old_lo, old_hi)) = prev_bounds {
                    let old_size = (old_hi - old_lo).max(Vec2::splat(1.0));
                    for ch in children.iter_mut() {
                        if let Some((clo, chi)) = ch.bounds() {
                            let n_lo = lo + ((clo - old_lo) / old_size) * new_size;
                            let n_hi = lo + ((chi - old_lo) / old_size) * new_size;
                            ch.set_bounds(n_lo, n_hi);
                        }
                    }
                }
            }
        }
    }
}

/// Approximate rendered width of a string at `size` points.
pub fn text_width(text: &str, size: f32) -> f32 {
    let widest = text
        .split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1)
        .max(1);
    (0.55 * size * widest as f32).max(size * 0.5)
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub blend_mode: BlendMode,
    pub is_web_layer: bool,
    pub objects: Vec<Obj>,
}

impl Layer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            visible: true,
            locked: false,
            opacity: 1.0,
            blend_mode: BlendMode::Normal,
            is_web_layer: false,
            objects: Vec::new(),
        }
    }

    pub fn web_layer() -> Self {
        let mut l = Self::new("Web Layer");
        l.is_web_layer = true;
        l
    }
}

/// An animation / rollover state in Fireworks CS6 (formerly Frames).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct AnimationState {
    pub name: String,
    /// Frame delay in 1/100s of a second (CS6 default is 7 = 70ms).
    pub delay_cs: u32,
    pub layers: Vec<Layer>,
}

impl AnimationState {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            delay_cs: 7,
            layers: vec![Layer::new("Layer 1"), Layer::web_layer()],
        }
    }
}

/// Reusable Symbol definition in the Document Library.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct SymbolDef {
    pub id: usize,
    pub name: String,
    pub kind: String, // "Graphic", "Button", "Animation"
    pub objects: Vec<Obj>,
}

/// Built-in CS6 Style Library categories.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum StyleLibrary {
    #[default]
    CurrentDocument,
    DarkChrome,
    ChromeMetal,
    GlassGel,
    PlasticWeb,
    ShadowGlow,
}

impl StyleLibrary {
    pub const ALL: [StyleLibrary; 6] = [
        StyleLibrary::CurrentDocument,
        StyleLibrary::DarkChrome,
        StyleLibrary::ChromeMetal,
        StyleLibrary::GlassGel,
        StyleLibrary::PlasticWeb,
        StyleLibrary::ShadowGlow,
    ];

    pub fn label(self) -> &'static str {
        match self {
            StyleLibrary::CurrentDocument => "Current Document",
            StyleLibrary::DarkChrome => "Dark Chrome Styles",
            StyleLibrary::ChromeMetal => "Chrome & Metal Styles",
            StyleLibrary::GlassGel => "Glass & Gel Styles",
            StyleLibrary::PlasticWeb => "Plastic & Web UI Styles",
            StyleLibrary::ShadowGlow => "Shadow & Glow Styles",
        }
    }
}

/// Reusable graphic Style preset (fill, optional gradient end, stroke, width, live effects).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StylePreset {
    pub name: String,
    pub fill: Option<Color32>,
    pub gradient_end: Option<Color32>,
    pub stroke: Option<Color32>,
    pub stroke_width: f32,
    pub effects: Vec<LiveEffect>,
}

impl StylePreset {
    fn preset(
        name: &str,
        top: Color32,
        bot: Option<Color32>,
        stroke: Option<Color32>,
        stroke_width: f32,
        effects: Vec<LiveEffect>,
    ) -> Self {
        Self {
            name: name.to_string(),
            fill: Some(top),
            gradient_end: bot,
            stroke,
            stroke_width,
            effects,
        }
    }

    pub fn library_styles(lib: StyleLibrary) -> Vec<Self> {
        let shadow = |d, b, a| LiveEffect::DropShadow {
            distance: d,
            angle_deg: 90.0,
            blur: b,
            color: Color32::from_black_alpha(a),
            enabled: true,
        };
        let bevel = |w, hi, sh| LiveEffect::Bevel {
            width: w,
            highlight: Color32::from_white_alpha(hi),
            shadow: Color32::from_black_alpha(sh),
            enabled: true,
        };
        let glow = |r, c: Color32, inner| LiveEffect::Glow {
            radius: r,
            color: c,
            inner,
            enabled: true,
        };
        match lib {
            StyleLibrary::CurrentDocument => Self::default_styles(),
            StyleLibrary::DarkChrome => vec![
                Self::preset(
                    "Obsidian Gloss",
                    Color32::from_rgb(56, 58, 64),
                    Some(Color32::from_rgb(14, 15, 18)),
                    Some(Color32::from_rgb(20, 20, 24)),
                    1.5,
                    vec![bevel(1.5, 130, 180)],
                ),
                Self::preset(
                    "Graphite Satin",
                    Color32::from_rgb(82, 86, 94),
                    Some(Color32::from_rgb(30, 32, 36)),
                    Some(Color32::from_rgb(24, 24, 28)),
                    1.0,
                    vec![shadow(3.0, 4.0, 120)],
                ),
                Self::preset(
                    "Silver-Top Chrome",
                    Color32::from_rgb(185, 190, 198),
                    Some(Color32::from_rgb(18, 20, 24)),
                    Some(Color32::from_rgb(40, 42, 46)),
                    1.5,
                    vec![bevel(2.0, 190, 170)],
                ),
                Self::preset(
                    "Matte Carbon",
                    Color32::from_rgb(40, 42, 46),
                    Some(Color32::from_rgb(20, 22, 25)),
                    Some(Color32::from_rgb(10, 10, 12)),
                    1.5,
                    vec![shadow(4.0, 6.0, 160)],
                ),
                Self::preset(
                    "Smoked Steel",
                    Color32::from_rgb(68, 72, 78),
                    Some(Color32::from_rgb(12, 13, 16)),
                    Some(Color32::from_rgb(95, 100, 108)),
                    1.0,
                    vec![bevel(1.0, 120, 160)],
                ),
                Self::preset(
                    "Banded Dark Chrome",
                    Color32::from_rgb(130, 135, 144),
                    Some(Color32::from_rgb(12, 14, 18)),
                    Some(Color32::from_rgb(22, 24, 28)),
                    1.5,
                    vec![bevel(2.0, 200, 180)],
                ),
                Self::preset(
                    "Brushed Pewter",
                    Color32::from_rgb(198, 202, 210),
                    Some(Color32::from_rgb(64, 66, 72)),
                    Some(Color32::from_rgb(36, 38, 42)),
                    1.0,
                    vec![shadow(3.0, 5.0, 130)],
                ),
                Self::preset(
                    "Midnight Bevel",
                    Color32::from_rgb(32, 34, 40),
                    Some(Color32::from_rgb(10, 11, 14)),
                    Some(Color32::from_rgb(76, 80, 88)),
                    1.5,
                    vec![bevel(2.5, 150, 200)],
                ),
                Self::preset(
                    "Specular Slate",
                    Color32::from_rgb(210, 214, 220),
                    Some(Color32::from_rgb(26, 28, 32)),
                    Some(Color32::from_rgb(18, 20, 24)),
                    1.0,
                    vec![shadow(4.0, 6.0, 140)],
                ),
                Self::preset(
                    "Jet Black Piano",
                    Color32::from_rgb(28, 28, 32),
                    Some(Color32::from_rgb(6, 6, 8)),
                    Some(Color32::from_rgb(68, 70, 76)),
                    1.5,
                    vec![bevel(1.5, 170, 210)],
                ),
                Self::preset(
                    "Anodized Charcoal",
                    Color32::from_rgb(60, 62, 68),
                    Some(Color32::from_rgb(18, 19, 22)),
                    Some(Color32::from_rgb(12, 12, 14)),
                    2.0,
                    vec![glow(3.0, Color32::from_white_alpha(60), true)],
                ),
                Self::preset(
                    "Gunmetal Ridge",
                    Color32::from_rgb(98, 102, 110),
                    Some(Color32::from_rgb(24, 26, 30)),
                    Some(Color32::from_rgb(150, 156, 166)),
                    1.0,
                    vec![bevel(2.0, 160, 180)],
                ),
                Self::preset(
                    "Deep Chrome Well",
                    Color32::from_rgb(16, 18, 22),
                    Some(Color32::from_rgb(54, 58, 64)),
                    Some(Color32::from_rgb(10, 10, 12)),
                    1.5,
                    vec![glow(4.0, Color32::from_black_alpha(180), true)],
                ),
                Self::preset(
                    "Stealth Frame",
                    Color32::from_rgb(72, 75, 82),
                    Some(Color32::from_rgb(16, 17, 20)),
                    Some(Color32::from_rgb(180, 186, 196)),
                    1.5,
                    vec![shadow(4.0, 5.0, 150)],
                ),
                Self::preset(
                    "Inverted Chrome",
                    Color32::from_rgb(22, 24, 28),
                    Some(Color32::from_rgb(172, 178, 188)),
                    Some(Color32::from_rgb(28, 30, 34)),
                    1.0,
                    vec![bevel(1.5, 150, 170)],
                ),
                Self::preset(
                    "Titanium Plate",
                    Color32::from_rgb(140, 145, 154),
                    Some(Color32::from_rgb(38, 40, 46)),
                    Some(Color32::from_rgb(20, 22, 26)),
                    1.5,
                    vec![shadow(4.0, 6.0, 130)],
                ),
                Self::preset(
                    "Carbon Diagonal",
                    Color32::from_rgb(48, 50, 56),
                    Some(Color32::from_rgb(18, 20, 24)),
                    Some(Color32::from_rgb(84, 88, 96)),
                    1.5,
                    vec![bevel(1.5, 140, 180)],
                ),
                Self::preset(
                    "Dark Alloy Tab",
                    Color32::from_rgb(44, 46, 52),
                    Some(Color32::from_rgb(12, 13, 16)),
                    Some(Color32::from_rgb(96, 100, 110)),
                    1.0,
                    vec![shadow(3.0, 4.0, 150)],
                ),
                Self::preset(
                    "Onyx Bevel",
                    Color32::from_rgb(20, 22, 26),
                    Some(Color32::from_rgb(8, 8, 10)),
                    Some(Color32::from_rgb(56, 58, 64)),
                    2.0,
                    vec![bevel(2.0, 120, 200)],
                ),
                Self::preset(
                    "Chrome Horizon",
                    Color32::from_rgb(165, 170, 178),
                    Some(Color32::from_rgb(14, 15, 18)),
                    Some(Color32::from_rgb(32, 34, 38)),
                    1.5,
                    vec![bevel(2.0, 180, 180)],
                ),
            ],
            StyleLibrary::ChromeMetal => vec![
                Self::preset(
                    "Brushed Silver",
                    Color32::from_rgb(230, 234, 240),
                    Some(Color32::from_rgb(142, 148, 158)),
                    Some(Color32::from_rgb(95, 100, 110)),
                    1.5,
                    vec![bevel(2.0, 210, 130)],
                ),
                Self::preset(
                    "Polished Aluminum",
                    Color32::from_rgb(245, 247, 250),
                    Some(Color32::from_rgb(170, 176, 186)),
                    Some(Color32::from_rgb(110, 116, 126)),
                    1.0,
                    vec![shadow(4.0, 6.0, 90)],
                ),
                Self::preset(
                    "Warm Gold",
                    Color32::from_rgb(246, 208, 102),
                    Some(Color32::from_rgb(176, 124, 28)),
                    Some(Color32::from_rgb(120, 82, 14)),
                    1.5,
                    vec![bevel(2.0, 200, 140)],
                ),
                Self::preset(
                    "Burnished Copper",
                    Color32::from_rgb(226, 140, 88),
                    Some(Color32::from_rgb(138, 68, 34)),
                    Some(Color32::from_rgb(92, 42, 18)),
                    1.5,
                    vec![bevel(2.0, 180, 150)],
                ),
                Self::preset(
                    "Steel Blue",
                    Color32::from_rgb(146, 172, 198),
                    Some(Color32::from_rgb(66, 90, 118)),
                    Some(Color32::from_rgb(42, 60, 82)),
                    1.5,
                    vec![shadow(3.0, 5.0, 110)],
                ),
                Self::preset(
                    "Titanium Gray",
                    Color32::from_rgb(178, 182, 188),
                    Some(Color32::from_rgb(94, 98, 106)),
                    Some(Color32::from_rgb(60, 64, 70)),
                    1.5,
                    vec![bevel(1.5, 170, 140)],
                ),
            ],
            StyleLibrary::GlassGel => vec![
                Self::preset(
                    "FireCraft Amber Gel",
                    Color32::from_rgb(252, 194, 88),
                    Some(Color32::from_rgb(218, 126, 24)),
                    Some(Color32::from_rgb(166, 92, 14)),
                    1.5,
                    vec![bevel(2.5, 220, 120), shadow(4.0, 6.0, 110)],
                ),
                Self::preset(
                    "Aqua Cyan Gel",
                    Color32::from_rgb(110, 224, 252),
                    Some(Color32::from_rgb(24, 148, 212)),
                    Some(Color32::from_rgb(16, 108, 162)),
                    1.5,
                    vec![bevel(2.5, 220, 110)],
                ),
                Self::preset(
                    "Emerald Glass",
                    Color32::from_rgb(98, 226, 150),
                    Some(Color32::from_rgb(26, 146, 82)),
                    Some(Color32::from_rgb(16, 102, 56)),
                    1.5,
                    vec![bevel(2.5, 210, 120)],
                ),
                Self::preset(
                    "Ruby Crystal",
                    Color32::from_rgb(248, 108, 120),
                    Some(Color32::from_rgb(178, 28, 46)),
                    Some(Color32::from_rgb(122, 16, 28)),
                    1.5,
                    vec![bevel(2.5, 210, 130)],
                ),
                Self::preset(
                    "Sapphire Pill",
                    Color32::from_rgb(104, 158, 248),
                    Some(Color32::from_rgb(36, 82, 188)),
                    Some(Color32::from_rgb(24, 56, 138)),
                    1.5,
                    vec![bevel(2.5, 210, 130)],
                ),
                Self::preset(
                    "Frosted Smoke",
                    Color32::from_rgba_unmultiplied(235, 240, 248, 210),
                    Some(Color32::from_rgba_unmultiplied(185, 196, 212, 200)),
                    Some(Color32::WHITE),
                    1.5,
                    vec![shadow(4.0, 8.0, 80)],
                ),
            ],
            StyleLibrary::PlasticWeb => vec![
                Self::preset(
                    "CS6 Primary Blue",
                    Color32::from_rgb(78, 168, 240),
                    Some(Color32::from_rgb(38, 118, 196)),
                    Some(Color32::from_rgb(26, 86, 148)),
                    1.5,
                    vec![bevel(1.5, 180, 110)],
                ),
                Self::preset(
                    "Action Orange",
                    Color32::from_rgb(244, 152, 54),
                    Some(Color32::from_rgb(206, 98, 18)),
                    Some(Color32::from_rgb(154, 70, 12)),
                    1.5,
                    vec![shadow(3.0, 5.0, 100)],
                ),
                Self::preset(
                    "Success Mint",
                    Color32::from_rgb(76, 198, 124),
                    Some(Color32::from_rgb(42, 148, 86)),
                    Some(Color32::from_rgb(28, 110, 62)),
                    1.0,
                    vec![shadow(3.0, 4.0, 90)],
                ),
                Self::preset(
                    "Danger Coral",
                    Color32::from_rgb(238, 86, 86),
                    Some(Color32::from_rgb(186, 44, 48)),
                    Some(Color32::from_rgb(138, 28, 32)),
                    1.0,
                    vec![shadow(3.0, 4.0, 100)],
                ),
                Self::preset(
                    "Clean Pill Light",
                    Color32::from_rgb(252, 252, 254),
                    Some(Color32::from_rgb(226, 230, 236)),
                    Some(Color32::from_rgb(180, 186, 196)),
                    1.0,
                    vec![shadow(2.0, 4.0, 60)],
                ),
                Self::preset(
                    "Dark UI Button",
                    Color32::from_rgb(72, 76, 84),
                    Some(Color32::from_rgb(46, 48, 54)),
                    Some(Color32::from_rgb(26, 28, 32)),
                    1.0,
                    vec![bevel(1.0, 120, 150)],
                ),
            ],
            StyleLibrary::ShadowGlow => vec![
                Self::preset(
                    "Soft Card Shadow",
                    Color32::WHITE,
                    None,
                    Some(Color32::from_rgb(210, 215, 222)),
                    1.0,
                    vec![shadow(6.0, 8.0, 90)],
                ),
                Self::preset(
                    "Deep Elevation",
                    Color32::from_rgb(248, 250, 252),
                    None,
                    Some(Color32::from_rgb(200, 206, 214)),
                    1.0,
                    vec![shadow(10.0, 14.0, 130)],
                ),
                Self::preset(
                    "Neon Cyan Glow",
                    Color32::from_rgb(28, 32, 44),
                    None,
                    Some(Color32::from_rgb(0, 220, 255)),
                    2.0,
                    vec![glow(
                        8.0,
                        Color32::from_rgba_unmultiplied(0, 220, 255, 180),
                        false,
                    )],
                ),
                Self::preset(
                    "Amber Halo",
                    Color32::from_rgb(38, 30, 24),
                    None,
                    Some(Color32::from_rgb(245, 166, 35)),
                    2.0,
                    vec![glow(
                        8.0,
                        Color32::from_rgba_unmultiplied(245, 166, 35, 180),
                        false,
                    )],
                ),
                Self::preset(
                    "Magenta Pulse",
                    Color32::from_rgb(36, 24, 42),
                    None,
                    Some(Color32::from_rgb(236, 72, 153)),
                    2.0,
                    vec![glow(
                        8.0,
                        Color32::from_rgba_unmultiplied(236, 72, 153, 180),
                        false,
                    )],
                ),
                Self::preset(
                    "Inset Recessed",
                    Color32::from_rgb(44, 48, 56),
                    None,
                    Some(Color32::from_rgb(22, 24, 28)),
                    1.5,
                    vec![glow(5.0, Color32::from_black_alpha(170), true)],
                ),
            ],
        }
    }

    pub fn default_styles() -> Vec<Self> {
        vec![
            Self {
                name: "FireCraft Amber Button".to_string(),
                fill: Some(Color32::from_rgb(232, 163, 61)),
                gradient_end: Some(Color32::from_rgb(196, 118, 26)),
                stroke: Some(Color32::from_rgb(140, 85, 15)),
                stroke_width: 2.0,
                effects: vec![LiveEffect::DropShadow {
                    distance: 4.0,
                    angle_deg: 45.0,
                    blur: 4.0,
                    color: Color32::from_black_alpha(140),
                    enabled: true,
                }],
            },
            Self {
                name: "CS6 Web Gloss Blue".to_string(),
                fill: Some(Color32::from_rgb(66, 158, 224)),
                gradient_end: Some(Color32::from_rgb(32, 104, 168)),
                stroke: Some(Color32::from_rgb(25, 80, 120)),
                stroke_width: 1.5,
                effects: vec![LiveEffect::Bevel {
                    width: 2.0,
                    highlight: Color32::from_white_alpha(180),
                    shadow: Color32::from_black_alpha(140),
                    enabled: true,
                }],
            },
            Self {
                name: "Neon Outer Glow".to_string(),
                fill: Some(Color32::from_rgb(35, 35, 48)),
                gradient_end: None,
                stroke: Some(Color32::from_rgb(0, 220, 255)),
                stroke_width: 2.0,
                effects: vec![LiveEffect::Glow {
                    radius: 6.0,
                    color: Color32::from_rgba_unmultiplied(0, 220, 255, 160),
                    inner: false,
                    enabled: true,
                }],
            },
            Self {
                name: "Soft Card Shadow".to_string(),
                fill: Some(Color32::WHITE),
                gradient_end: Some(Color32::from_rgb(238, 242, 248)),
                stroke: Some(Color32::from_rgb(210, 215, 222)),
                stroke_width: 1.0,
                effects: vec![LiveEffect::DropShadow {
                    distance: 6.0,
                    angle_deg: 90.0,
                    blur: 8.0,
                    color: Color32::from_black_alpha(90),
                    enabled: true,
                }],
            },
        ]
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Page {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub bg_color: Color32,
    pub transparent_bg: bool,
    pub layers: Vec<Layer>,
    pub states: Vec<AnimationState>,
    pub current_state: usize,
    pub guides_h: Vec<f32>,
    pub guides_v: Vec<f32>,
}

impl Page {
    pub fn new(name: &str, width: f32, height: f32) -> Self {
        let initial_layers = vec![Layer::new("Layer 1"), Layer::web_layer()];
        let mut state1 = AnimationState::new("State 1");
        state1.layers = initial_layers.clone();
        Self {
            name: name.to_string(),
            width,
            height,
            bg_color: Color32::WHITE,
            transparent_bg: false,
            layers: initial_layers,
            states: vec![state1],
            current_state: 0,
            guides_h: Vec::new(),
            guides_v: Vec::new(),
        }
    }

    /// Synchronize active `layers` into `states[current_state]`.
    pub fn sync_state(&mut self) {
        if self.states.is_empty() {
            let mut st = AnimationState::new("State 1");
            st.layers = self.layers.clone();
            self.states.push(st);
            self.current_state = 0;
        } else {
            let idx = self.current_state.min(self.states.len() - 1);
            self.states[idx].layers = self.layers.clone();
        }
    }

    /// Switch to animation state `index`, saving the current layers first.
    pub fn select_state(&mut self, index: usize) {
        self.sync_state();
        if index < self.states.len() {
            self.current_state = index;
            self.layers = self.states[index].layers.clone();
        }
    }
}

/// Optimize panel settings (CS6 Optimize panel: format, palette, quality, matte, dither).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct OptimizeSettings {
    pub format: String, // "PNG32", "PNG24", "PNG8", "JPEG", "GIF", "WebP", "SVG", "BMP", "TIFF"
    pub palette: String, // "Adaptive", "WebAdaptive", "Web 216", "Exact", "Grayscale"
    pub quality: u8,    // 1..=100
    pub dither: u8,     // 0..=100
    pub transparency: bool,
    pub matte: Color32,
}

impl Default for OptimizeSettings {
    fn default() -> Self {
        Self {
            format: constants::EXPORT_DEFAULT_FORMAT.to_string(),
            palette: "Adaptive".to_string(),
            quality: 90,
            dither: 0,
            transparency: true,
            matte: Color32::WHITE,
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Document {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub dpi: u32,
    pub pages: Vec<Page>,
    pub current_page: usize,
    pub zoom: f32,
    pub pan: Vec2,
    pub dirty: bool,
    pub symbols: Vec<SymbolDef>,
    pub styles: Vec<StylePreset>,
    pub optimize: OptimizeSettings,
    pub urls: Vec<String>,
    /// Backing file, if this document has been saved. Stored per document so tab switches do
    /// not lose the save target.
    pub file_path: Option<PathBuf>,
}

impl Document {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            name: "Untitled-1.fw.png".to_string(),
            width,
            height,
            dpi: constants::EXPORT_DEFAULT_DPI,
            pages: vec![Page::new("Page 1", width, height)],
            current_page: 0,
            zoom: constants::DEFAULT_ZOOM,
            pan: Vec2::new(0.0, 0.0),
            dirty: false,
            symbols: Vec::new(),
            styles: StylePreset::default_styles(),
            optimize: OptimizeSettings::default(),
            urls: vec![
                "https://github.com/aliasfoxkde/FireCraft".to_string(),
                "https://github.com/aliasfoxkde/FireCraft/wiki".to_string(),
                "index.html".to_string(),
            ],
            file_path: None,
        }
    }

    pub fn page(&self) -> &Page {
        let i = self.current_page.min(self.pages.len().saturating_sub(1));
        &self.pages[i]
    }

    pub fn page_mut(&mut self) -> &mut Page {
        let i = self.current_page.min(self.pages.len().saturating_sub(1));
        &mut self.pages[i]
    }

    pub fn total_objects_on_page(&self) -> usize {
        self.page().layers.iter().map(|l| l.objects.len()).sum()
    }
}

/// Snapshot of a page, used for undo/redo and the interactive History panel.
#[derive(Clone, Debug)]
pub struct Snapshot {
    pub label: String,
    pub page: usize,
    pub layers: Vec<Layer>,
    pub width: f32,
    pub height: f32,
}

impl Document {
    pub fn snapshot(&self, label: &str) -> Snapshot {
        Snapshot {
            label: label.to_string(),
            page: self.current_page,
            layers: self.page().layers.clone(),
            width: self.page().width,
            height: self.page().height,
        }
    }

    pub fn restore(&mut self, snap: &Snapshot) {
        if self.pages.is_empty() {
            return;
        }
        let idx = snap.page.min(self.pages.len() - 1);
        self.pages[idx].layers = snap.layers.clone();
        self.pages[idx].width = snap.width;
        self.pages[idx].height = snap.height;
        self.pages[idx].sync_state();
        self.width = snap.width;
        self.height = snap.height;
        self.current_page = idx;
        self.dirty = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient_projection_uses_normalized_direction_and_preserves_legacy_vertical_fill() {
        let legacy: GradientFill =
            serde_json::from_str(r#"{"start":[1,2,3,255],"end":[4,5,6,255]}"#)
                .expect("two-stop gradients from older documents remain readable");
        assert_eq!(legacy.t_at(Vec2::new(0.5, 0.0)), 0.0);
        assert_eq!(legacy.t_at(Vec2::new(0.5, 0.5)), 0.5);
        assert_eq!(legacy.t_at(Vec2::new(0.5, 1.0)), 1.0);
        assert_eq!(legacy.t_at(Vec2::new(0.5, 2.0)), 1.0);

        let horizontal: GradientFill = serde_json::from_str(
            r#"{"start":[1,2,3,255],"end":[4,5,6,255],"start_pos":[0.0,0.5],"end_pos":[1.0,0.5]}"#,
        )
        .expect("gradient endpoints are document data");
        assert_eq!(horizontal.t_at(Vec2::new(0.25, 0.0)), 0.25);
        assert_eq!(horizontal.t_at(Vec2::new(0.75, 1.0)), 0.75);
    }

    #[test]
    fn zero_length_gradient_axis_has_stable_midpoint_parameter() {
        let gradient: GradientFill = serde_json::from_str(
            r#"{"start":[1,2,3,255],"end":[4,5,6,255],"start_pos":[0.4,0.6],"end_pos":[0.4,0.6]}"#,
        )
        .expect("gradient endpoints are document data");
        assert_eq!(gradient.t_at(Vec2::new(0.0, 0.0)), 0.5);
    }

    #[test]
    fn older_bitmap_documents_infer_source_dimensions_from_display_size() {
        let kind: Shape = serde_json::from_str(
            r#"{"Bitmap":{"at":[0.0,0.0],"pixels":[255,0,0,255],"size":[1.0,1.0]}}"#,
        )
        .expect("legacy bitmap without pixel_size should deserialize");
        let mut obj = Obj::new(kind, None, None);
        assert_eq!(obj.bitmap_pixel_size(), Some(Vec2::splat(1.0)));
        obj.set_bounds(Vec2::ZERO, Vec2::new(4.0, 3.0));
        assert_eq!(obj.bitmap_pixel_size(), Some(Vec2::splat(1.0)));
        assert_eq!(obj.bounds(), Some((Vec2::ZERO, Vec2::new(4.0, 3.0))));
    }

    #[test]
    fn multiline_text_bounds_use_the_longest_line() {
        assert_eq!(
            text_width("abc\ndef", 20.0),
            text_width("abc", 20.0),
            "a newline must not add width"
        );
        assert!(text_width("abc\ndef", 20.0) < text_width("abcdef", 20.0));
        let obj = Obj::new(
            Shape::Text {
                at: Vec2::new(10.0, 100.0),
                text: "one\ntwo\nthree".to_string(),
                size: 20.0,
                bold: false,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            None,
            None,
        );
        let (lo, hi) = obj.bounds().expect("text has bounds");
        assert_eq!(hi.x - lo.x, text_width("three", 20.0));
        assert!(
            (hi.y - lo.y - 20.0 * 1.2 * 3.0).abs() < 0.01,
            "three lines should be three line heights tall"
        );
    }

    #[test]
    fn document_creation_and_bounds() {
        let mut doc = Document::new(1920.0, 1080.0);
        assert_eq!(doc.page().width, 1920.0);
        assert_eq!(doc.page().height, 1080.0);
        assert_eq!(doc.dpi, 144);
        assert_eq!(doc.page().layers.len(), 2); // Layer 1 + Web Layer

        let mut obj = Obj::new(
            Shape::Rect {
                min: Vec2::new(10.0, 20.0),
                max: Vec2::new(110.0, 70.0),
                corner_radius: 4.0,
                filled: true,
            },
            Some(Color32::RED),
            Some(Color32::BLACK),
        );
        assert_eq!(
            obj.bounds(),
            Some((Vec2::new(10.0, 20.0), Vec2::new(110.0, 70.0)))
        );
        obj.translate(Vec2::new(5.0, -5.0));
        assert_eq!(
            obj.bounds(),
            Some((Vec2::new(15.0, 15.0), Vec2::new(115.0, 65.0)))
        );
        obj.set_bounds(Vec2::new(0.0, 0.0), Vec2::new(200.0, 100.0));
        assert_eq!(
            obj.bounds(),
            Some((Vec2::new(0.0, 0.0), Vec2::new(200.0, 100.0)))
        );

        let snap = doc.snapshot("Before push");
        doc.page_mut().layers[0].objects.push(obj);
        assert_eq!(doc.total_objects_on_page(), 1);
        doc.restore(&snap);
        assert_eq!(doc.total_objects_on_page(), 0);
    }

    #[test]
    fn polygon_and_star_vertices() {
        let hex = Obj::polygon_vertices(Vec2::new(50.0, 50.0), 40.0, 6, 1.0, 0.0);
        assert_eq!(hex.len(), 6);
        let star = Obj::polygon_vertices(Vec2::new(50.0, 50.0), 40.0, 5, 0.5, 0.0);
        assert_eq!(star.len(), 10);
    }

    #[test]
    fn animation_state_switching_preserves_layers() {
        let mut page = Page::new("Page 1", 800.0, 600.0);
        page.layers[0].objects.push(Obj::new(
            Shape::Ellipse {
                center: Vec2::new(40.0, 40.0),
                radius: Vec2::new(20.0, 20.0),
                filled: true,
            },
            Some(Color32::BLUE),
            None,
        ));
        page.sync_state();
        let mut st2 = AnimationState::new("State 2");
        st2.layers = vec![Layer::new("Layer 1"), Layer::web_layer()];
        page.states.push(st2);

        page.select_state(1);
        assert_eq!(page.layers[0].objects.len(), 0);
        page.select_state(0);
        assert_eq!(page.layers[0].objects.len(), 1);
    }
}
