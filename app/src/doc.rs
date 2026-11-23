//! Document model: pages, animation states, layers, vector/bitmap objects,
//! live effects, web slices/hotspots, symbols and styles.

use eframe::egui::{Color32, Vec2};

use crate::constants;

/// Live non-destructive effects attached to an object (Fireworks CS6 Live Filters).
#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
pub struct Obj {
    pub kind: Shape,
    pub fill: Option<Color32>,
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
            Shape::Text { at, text, size, .. } => Some((
                Vec2::new(at.x, at.y - size * 1.2),
                Vec2::new(at.x + text_width(text, *size), at.y),
            )),
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
            Shape::Bitmap { at, .. } => {
                *at = lo;
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
    (0.55 * size * text.chars().count().max(1) as f32).max(size * 0.5)
}

#[derive(Clone, PartialEq, Debug)]
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
#[derive(Clone, PartialEq, Debug)]
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
#[derive(Clone, PartialEq, Debug)]
pub struct SymbolDef {
    pub id: usize,
    pub name: String,
    pub kind: &'static str, // "Graphic", "Button", "Animation"
    pub objects: Vec<Obj>,
}

/// Reusable graphic Style preset (fill, stroke, width, live effects).
#[derive(Clone, PartialEq, Debug)]
pub struct StylePreset {
    pub name: String,
    pub fill: Option<Color32>,
    pub stroke: Option<Color32>,
    pub stroke_width: f32,
    pub effects: Vec<LiveEffect>,
}

impl StylePreset {
    pub fn default_styles() -> Vec<Self> {
        vec![
            Self {
                name: "FireCraft Amber Button".to_string(),
                fill: Some(Color32::from_rgb(232, 163, 61)),
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
                fill: Some(Color32::from_rgb(41, 128, 185)),
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

#[derive(Clone, PartialEq, Debug)]
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
#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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
