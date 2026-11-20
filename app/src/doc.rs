//! Document model: pages, layers and vector/bitmap objects.

use eframe::egui::{Color32, Vec2};

use crate::constants;

#[derive(Clone, PartialEq, Debug)]
pub enum Shape {
    Rect {
        min: Vec2,
        max: Vec2,
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
    Path {
        points: Vec<Vec2>,
    },
    Text {
        at: Vec2,
        text: String,
        size: f32,
    },
    Gradient {
        min: Vec2,
        max: Vec2,
        from: Color32,
        to: Color32,
    },
    Bitmap {
        at: Vec2,
        pixels: Vec<u8>,
        size: Vec2,
    },
}

#[derive(Clone, PartialEq, Debug)]
pub struct Obj {
    pub kind: Shape,
    pub fill: Option<Color32>,
    pub stroke: Option<Color32>,
    pub width: f32,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
}

impl Obj {
    pub fn new(kind: Shape, fill: Option<Color32>, stroke: Option<Color32>) -> Self {
        let name = match &kind {
            Shape::Rect { .. } => "Rectangle".to_string(),
            Shape::Ellipse { .. } => "Ellipse".to_string(),
            Shape::Line { .. } => "Line".to_string(),
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
        };
        Self {
            kind,
            fill,
            stroke,
            width: 1.0,
            name,
            visible: true,
            locked: false,
        }
    }

    /// The axis-aligned bounding box, always normalised so `lo <= hi` on both axes.
    pub fn bounds(&self) -> Option<(Vec2, Vec2)> {
        match &self.kind {
            Shape::Rect { min, max, .. } => Some(((*min).min(*max), (*min).max(*max))),
            Shape::Ellipse { center, radius, .. } => {
                Some((*center - *radius, *center + *radius))
            }
            Shape::Line { a, b } => Some(((*a).min(*b), (*a).max(*b))),
            Shape::Path { points } => {
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
            Shape::Text { at, text, size } => {
                // egui anchors text by its baseline, so the box sits *above* `at`.
                Some((
                    Vec2::new(at.x, at.y - size * 1.2),
                    Vec2::new(at.x + text_width(text, *size), at.y),
                ))
            }
            Shape::Gradient { min, max, .. } => Some(((*min).min(*max), (*min).max(*max))),
            Shape::Bitmap { at, size, .. } => Some((*at, *at + *size)),
        }
    }

    pub fn translate(&mut self, delta: Vec2) {
        match &mut self.kind {
            Shape::Rect { min, max, .. } => {
                *min += delta;
                *max += delta;
            }
            Shape::Ellipse { center, .. } => *center += delta,
            Shape::Line { a, b } => {
                *a += delta;
                *b += delta;
            }
            Shape::Path { points } => {
                for p in points.iter_mut() {
                    *p += delta;
                }
            }
            Shape::Text { at, .. } => *at += delta,
            Shape::Gradient { min, max, .. } => {
                *min += delta;
                *max += delta;
            }
            Shape::Bitmap { at, .. } => *at += delta,
        }
    }
}

/// Approximate rendered width of a string at `size` points.
///
/// egui's proportional font averages a little over half the point size per
/// character; the exact advance widths depend on the glyphs, so this is a
/// deliberately conservative estimate used for hit testing and selection boxes.
pub fn text_width(text: &str, size: f32) -> f32 {
    0.55 * size * text.chars().count() as f32
}

#[derive(Clone, Debug)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub objects: Vec<Obj>,
}

impl Layer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            visible: true,
            locked: false,
            opacity: 1.0,
            objects: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Page {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub layers: Vec<Layer>,
}

impl Page {
    pub fn new(name: &str, width: f32, height: f32) -> Self {
        let mut page = Self {
            name: name.to_string(),
            width,
            height,
            layers: Vec::new(),
        };
        page.layers.push(Layer::new("Layer 1"));
        page
    }
}

#[derive(Clone)]
pub struct Document {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub pages: Vec<Page>,
    pub current_page: usize,
    pub zoom: f32,
    pub pan: Vec2,
    pub dirty: bool,
}

impl Document {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            name: "Untitled".to_string(),
            width,
            height,
            pages: vec![Page::new("Page 1", width, height)],
            current_page: 0,
            zoom: constants::DEFAULT_ZOOM,
            pan: Vec2::new(0.0, 0.0),
            dirty: false,
        }
    }

    pub fn page(&self) -> &Page {
        // A document always has at least one page, but `page()` must not panic
        // if a caller clears the list.
        let i = self.current_page.min(self.pages.len().saturating_sub(1));
        &self.pages[i]
    }

    pub fn page_mut(&mut self) -> &mut Page {
        let i = self.current_page.min(self.pages.len().saturating_sub(1));
        &mut self.pages[i]
    }

    pub fn layer(&self) -> &Layer {
        let page = self.page();
        &page.layers[page.layers.len().saturating_sub(1)]
    }
}

/// Snapshot of a page, used for undo/redo.
///
/// The whole layer list is captured, not just the layers that happen to exist
/// at snapshot time, so that adding or deleting a layer can be undone.
#[derive(Clone)]
pub struct Snapshot {
    pub label: String,
    pub page: usize,
    pub layers: Vec<Layer>,
}

impl Document {
    pub fn snapshot(&self, label: &str) -> Snapshot {
        Snapshot {
            label: label.to_string(),
            page: self.current_page,
            layers: self.page().layers.clone(),
        }
    }

    pub fn restore(&mut self, snap: &Snapshot) {
        if self.pages.is_empty() {
            return;
        }
        let idx = snap.page.min(self.pages.len() - 1);
        self.pages[idx].layers = snap.layers.clone();
        self.current_page = idx;
        self.dirty = true;
    }
}


