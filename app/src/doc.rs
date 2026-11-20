#![allow(dead_code, unused_imports, unused_variables)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

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

    pub fn bounds(&self) -> Option<(Vec2, Vec2)> {
        match &self.kind {
            Shape::Rect { min, max, .. } => Some((*min, *max)),
            Shape::Ellipse { center, radius, .. } => {
                Some((*center - *radius, *center + *radius))
            }
            Shape::Line { a, b } => Some((*a, *b)),
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
            Shape::Text { at, size, .. } => {
                let n = 0.55 * size * 8.0;
                Some((*at, *at + Vec2::new(n, size * 1.2)))
            }
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
            Shape::Bitmap { at, .. } => *at += delta,
        }
    }
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
        &self.pages[self.current_page.min(self.pages.len() - 1)]
    }

    pub fn page_mut(&mut self) -> &mut Page {
        let i = self.current_page.min(self.pages.len() - 1);
        &mut self.pages[i]
    }

    pub fn layer(&self) -> &Layer {
        let page = self.page();
        &page.layers[page.layers.len() - 1]
    }
}

/// Snapshot of a page's objects, used for undo/redo.
#[derive(Clone)]
pub struct Snapshot {
    pub label: String,
    pub page: usize,
    pub layers: Vec<(String, Vec<Obj>)>,
}

impl Document {
    pub fn snapshot(&self, label: &str) -> Snapshot {
        Snapshot {
            label: label.to_string(),
            page: self.current_page,
            layers: self
                .page()
                .layers
                .iter()
                .map(|l| (l.name.clone(), l.objects.clone()))
                .collect(),
        }
    }

    pub fn restore(&mut self, snap: &Snapshot) {
        let idx = snap.page.min(self.pages.len() - 1);
        let page = &mut self.pages[idx];
        for (i, (name, objects)) in snap.layers.iter().enumerate() {
            if i < page.layers.len() {
                page.layers[i].name = name.clone();
                page.layers[i].objects = objects.clone();
            }
        }
        self.current_page = idx;
        self.dirty = true;
    }
}


