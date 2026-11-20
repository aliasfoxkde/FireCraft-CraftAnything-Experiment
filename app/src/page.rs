use eframe::egui;
use crate::doc::Layer;

#[derive(Debug, Clone)]
pub struct Page {
    pub id: usize,
    pub name: String,
    pub layers: Vec<Layer>,
    pub canvas_width: f32,
    pub canvas_height: f32,
    pub bg_color: egui::Color32,
    pub slices: Vec<SliceInfo>,
}

#[derive(Debug, Clone)]
pub struct SliceInfo {
    pub x: f32, pub y: f32, pub width: f32, pub height: f32,
    pub name: String, pub format: ExportFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat { Png, Jpg, Svg, Html, Gif, Webp }

impl Page {
    pub fn new(name: &str, w: f32, h: f32, bg: egui::Color32) -> Self {
        Page { id: 0, name: name.to_string(), layers: vec![Layer::new("Layer 1")],
            canvas_width: w, canvas_height: h, bg_color: bg, slices: Vec::new() }
    }
}
