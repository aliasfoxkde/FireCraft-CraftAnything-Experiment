use eframe::egui;

/// A quadratic bezier path segment
#[derive(Debug, Clone)]
pub struct PathSegment {
    pub points: Vec<egui::Pos2>,
    pub closed: bool,
    pub fill_color: Option<egui::Color32>,
    pub stroke_color: Option<egui::Color32>,
    pub stroke_width: f32,
}

/// A drawable object
#[derive(Debug, Clone)]
pub enum Drawable {
    Vector { path: PathSegment, layer_name: String },
    Bitmap { image: egui::ColorImage, layer_name: String, x: f32, y: f32 },
    Text { text: String, font_size: f32, color: egui::Color32, pos: egui::Pos2, layer_name: String },
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f32,
    pub drawables: Vec<Drawable>,
}

impl Layer {
    pub fn new(name: &str) -> Self {
        Layer { name: name.to_string(), visible: true, locked: false, opacity: 1.0, drawables: Vec::new() }
    }
}
