//! The application state, the window layout and the menus.



use crate::canvas::{self, TextureCache};
use crate::colors;
use crate::doc::{Document, Obj, Shape, Snapshot};
use crate::formats::Format;
use crate::constants;
use crate::panels;
use crate::theme::{self, Palette, ThemeMode};

use image::GenericImageView;

use eframe::egui::{
    self, Align2, Color32, Context, FontId, MenuBar, RichText, Sense, Stroke, TextureHandle,
    Vec2, pos2,
};



#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub enum Tool {

    Select,

    Marquee,

    Lasso,

    Wand,

    Crop,

    Hand,

    Zoom,

    Fit,

    Brush,

    Pencil,

    Eraser,

    Blur,

    Sharpen,

    Dodge,

    Burn,

    Line,

    Rectangle,

    Ellipse,

    Polygon,

    Path,

    Text,

    Picker,

    Fill,

    Gradient,

}



impl Tool {

    pub fn name(self) -> &'static str {

        match self {

            Tool::Select => "Select",

            Tool::Marquee => "Marquee Select",

            Tool::Lasso => "Lasso Select",

            Tool::Wand => "Magic Wand",

            Tool::Crop => "Crop",

            Tool::Hand => "Hand",

            Tool::Zoom => "Zoom",

            Tool::Fit => "Fit in Window",

            Tool::Brush => "Brush",

            Tool::Pencil => "Pencil",

            Tool::Eraser => "Eraser",

            Tool::Blur => "Blur",

            Tool::Sharpen => "Sharpen",

            Tool::Dodge => "Dodge",

            Tool::Burn => "Burn",

            Tool::Line => "Line",

            Tool::Rectangle => "Rectangle",

            Tool::Ellipse => "Ellipse",

            Tool::Polygon => "Polygon",

            Tool::Path => "Pen",

            Tool::Text => "Text",

            Tool::Picker => "Eyedropper",

            Tool::Fill => "Fill",

            Tool::Gradient => "Gradient",

        }

    }

}



#[derive(Clone, Copy, PartialEq, Eq, Debug)]

pub enum DragKind {

    Marquee,

    Create,

    Move,

    Pan,

    Paint,

    Path,

}



pub struct Drag {

    pub kind: DragKind,

    pub start: Vec2,

    pub current: Vec2,

    pub path: Vec<Vec2>,

}



pub struct FireCraft {

    pub ctx: Context,

    pub doc: Document,

    pub theme_mode: ThemeMode,

    pub palette: Palette,

    pub tool: Tool,

    pub fill: Color32,

    pub stroke: Color32,

    pub swatches: Vec<Color32>,

    pub selected: Option<usize>,

    pub selected_layer: usize,

    pub drag: Option<Drag>,

    pub undo: Vec<Snapshot>,

    pub redo: Vec<Snapshot>,

    pub status: String,

    pub show_start: bool,

    pub color_tab: usize,

    pub layers_tab: usize,

    pub library_tab: usize,

    pub path_tab: usize,

    pub style_name: String,

    pub textures: TextureCache,

    pub logo_tex: Option<egui::TextureHandle>,
    pub frames: u32,

    pub screenshot_path: Option<String>,

    pub screenshot_saved: bool,

    pub screenshot_start: std::time::Instant,

    pub capture_requested: bool,

    pub open_path: String,

    pub export_path: String,

    pub export_format: Format,

    pub message: String,

    pub show_rulers: bool,

    pub show_grid: bool,

    pub show_guides: bool,

}



impl FireCraft {

    pub fn new(screenshot_path: Option<String>) -> Self {

        let theme_mode = ThemeMode::Dark;

        let palette = theme::resolve(theme_mode);

        Self {

            ctx: Context::default(),

            doc: Document::new(constants::DEFAULT_WIDTH, constants::DEFAULT_HEIGHT),

            theme_mode,

            palette,

            tool: Tool::Select,

            fill: Color32::from_rgba_premultiplied(0xFF, 0xFF, 0xFF, 0xFF),

            stroke: Color32::from_rgb(0, 0, 0),

            swatches: colors::swatch_grid(),

            selected: None,

            selected_layer: 0,

            drag: None,

            undo: Vec::new(),

            redo: Vec::new(),

            status: "Ready".to_string(),

            show_start: true,

            color_tab: 0,

            layers_tab: 2,

            library_tab: 0,

            path_tab: 0,

            style_name: "Current Document".to_string(),

            textures: TextureCache::new(),

            logo_tex: Self::load_logo(&Context::default()),
            frames: 0,

            screenshot_path,

            screenshot_saved: false,

            screenshot_start: std::time::Instant::now(),

            capture_requested: false,

            open_path: String::new(),

            export_path: "export.png".to_string(),

            export_format: Format::Png,

            message: String::new(),

            show_rulers: false,

            show_grid: false,

            show_guides: false,

        }

    }



    fn load_logo(ctx: &Context) -> Option<egui::TextureHandle> {
        let paths = [
            "logo.png",
            "src/logo.png",
            "../logo.png",
        ];
        for p in &paths {
            if let Ok(bytes) = std::fs::read(p) {
                if let Ok(img) = image::load_from_memory(&bytes) {
                    let w = img.width() as usize;
                    let h = img.height() as usize;
                    let mut pixels = Vec::with_capacity(w * h);
                    for (_x, _y, c) in img.pixels() {
                        pixels.push(egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]));
                    }
                    let image = egui::ColorImage::new([w, h], pixels);
                    return Some(ctx.load_texture("firecraft-logo", image, egui::TextureOptions::default()));
                }
            }
        }
        None
    }

    fn push_undo(&mut self, label: &str) {

        let snap = self.doc.snapshot(label);

        self.undo.push(snap);

        if self.undo.len() > constants::UNDO_LIMIT {

            self.undo.remove(0);

        }

        self.redo.clear();

    }



    fn undo_action(&mut self) {

        if let Some(snap) = self.undo.pop() {

            self.redo.push(self.doc.snapshot("redo"));

            self.doc.restore(&snap);

            self.textures.clear();

            self.selected = None;

            self.status = format!("Undo: {}", snap.label);

        }

    }



    fn redo_action(&mut self) {

        if let Some(snap) = self.redo.pop() {

            self.undo.push(self.doc.snapshot("redo"));

            self.doc.restore(&snap);

            self.textures.clear();

            self.selected = None;

            self.status = "Redo".to_string();

        }

    }



    fn delete_selected(&mut self) {

        if let Some(index) = self.selected {

            self.push_undo("Delete");

            let layer = self.selected_layer;

            let page = self.doc.page_mut();

            if layer < page.layers.len() && index < page.layers[layer].objects.len() {

                page.layers[layer].objects.remove(index);

            }

            self.selected = None;

            self.textures.clear();

        }

    }



    fn duplicate_selected(&mut self) {

        if let Some(index) = self.selected {

            self.push_undo("Duplicate");

            let layer = self.selected_layer;

            let page = self.doc.page_mut();

            if layer < page.layers.len() && index < page.layers[layer].objects.len() {

                let mut copy = page.layers[layer].objects[index].clone();

                copy.translate(Vec2::new(12.0, 12.0));

                page.layers[layer].objects.push(copy);

            }

        }

    }



    pub fn bitmap_texture(

        &mut self,

        key: (usize, usize),

        pixels: &[u8],

        size: Vec2,

    ) -> Option<TextureHandle> {

        if let Some(tex) = self.textures.get(&key) {

            return Some(tex.clone());

        }

        let w = size.x.max(1.0) as usize;

        let h = size.y.max(1.0) as usize;

        if pixels.len() < w * h * 4 {

            return None;

        }

        let mut data = Vec::with_capacity(w * h);

        for i in 0..w * h {

            data.push(Color32::from_rgba_unmultiplied(

                pixels[i * 4],

                pixels[i * 4 + 1],

                pixels[i * 4 + 2],

                pixels[i * 4 + 3],

            ));

        }

        let image = egui::ColorImage::new([w, h], data);

        let tex = self

            .ctx

            .load_texture(format!("bitmap-{key:?}"), image, egui::TextureOptions::LINEAR);

        self.textures.insert(key, tex.clone());

        Some(tex)

    }



    pub fn set_selected_name(&mut self, name: String) {

        if let Some(index) = self.selected {

            if let Some(obj) = self

                .doc

                .page_mut()

                .layers

                .get_mut(self.selected_layer)

                .and_then(|l| l.objects.get_mut(index))

            {

                obj.name = name;

            }

        }

    }



    pub fn set_selected_fill(&mut self, color: Option<Color32>) {

        if let Some(index) = self.selected {

            if let Some(obj) = self

                .doc

                .page_mut()

                .layers

                .get_mut(self.selected_layer)

                .and_then(|l| l.objects.get_mut(index))

            {

                obj.fill = color;

            }

        }

    }



    pub fn set_selected_stroke(&mut self, color: Option<Color32>) {

        if let Some(index) = self.selected {

            if let Some(obj) = self

                .doc

                .page_mut()

                .layers

                .get_mut(self.selected_layer)

                .and_then(|l| l.objects.get_mut(index))

            {

                obj.stroke = color;

            }

        }

    }



    pub fn set_selected_width(&mut self, width: f32) {

        if let Some(index) = self.selected {

            if let Some(obj) = self

                .doc

                .page_mut()

                .layers

                .get_mut(self.selected_layer)

                .and_then(|l| l.objects.get_mut(index))

            {

                obj.width = width;

            }

        }

    }



    pub fn set_selected_text(&mut self, text: String) {

        if let Some(index) = self.selected {

            if let Some(obj) = self

                .doc

                .page_mut()

                .layers

                .get_mut(self.selected_layer)

                .and_then(|l| l.objects.get_mut(index))

            {

                if let Shape::Text { text: t, .. } = &mut obj.kind {

                    *t = text.clone();

                }

                obj.name = text;

            }

        }

    }



    // ---- pointer handling -------------------------------------------------



    pub fn on_pointer_down(&mut self, doc_pos: Vec2, _screen: Vec2, alt: bool, _ctrl: bool) {

        match self.tool {

            Tool::Hand => {

                self.drag = Some(Drag {

                    kind: DragKind::Pan,

                    start: doc_pos,

                    current: doc_pos,

                    path: vec![],

                });

            }

            Tool::Zoom => {

                let factor = if alt { 1.0 / 1.25 } else { 1.25 };

                self.doc.zoom = (self.doc.zoom * factor).clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);

                self.status = format!("Zoom: {:.0}%", self.doc.zoom * 100.0);

            }

            Tool::Fit => {

                self.doc.zoom = 1.0;

                self.doc.pan = Vec2::ZERO;

                self.status = "Fit in window".to_string();

            }

            Tool::Select => {

                let hit = canvas::hit_test(self, doc_pos);

                match hit {

                    Some((layer, index)) => {

                        self.selected_layer = layer;

                        self.selected = Some(index);

                        self.drag = Some(Drag {

                            kind: DragKind::Move,

                            start: doc_pos,

                            current: doc_pos,

                            path: vec![],

                        });

                    }

                    None => {

                        self.selected = None;

                        self.drag = Some(Drag {

                            kind: DragKind::Marquee,

                            start: doc_pos,

                            current: doc_pos,

                            path: vec![],

                        });

                    }

                }

            }

            Tool::Marquee | Tool::Lasso | Tool::Wand | Tool::Crop => {

                self.drag = Some(Drag {

                    kind: DragKind::Marquee,

                    start: doc_pos,

                    current: doc_pos,

                    path: vec![],

                });

            }

            Tool::Rectangle | Tool::Ellipse | Tool::Line => {

                self.drag = Some(Drag {

                    kind: DragKind::Create,

                    start: doc_pos,

                    current: doc_pos,

                    path: vec![],

                });

            }

            Tool::Path | Tool::Polygon => {

                self.drag = Some(Drag {

                    kind: DragKind::Path,

                    start: doc_pos,

                    current: doc_pos,

                    path: vec![doc_pos],

                });

            }

            Tool::Text => {

                self.push_undo("Add text");

                let obj = Obj::new(

                    Shape::Text {

                        at: doc_pos,

                        text: "Text".to_string(),

                        size: 24.0,

                    },

                    Some(self.fill),

                    None,

                );

                let page = self.doc.page_mut();

                page.layers[self.selected_layer].objects.push(obj);

                self.selected = Some(page.layers[self.selected_layer].objects.len() - 1);

                self.textures.clear();

                self.status = "Text added".to_string();

            }

            Tool::Picker => {

                if let Some((layer, index)) = canvas::hit_test(self, doc_pos) {

                    let color = self.doc.page().layers[layer].objects[index].fill;

                    if let Some(c) = color {

                        self.fill = c;

                        self.status = format!("Picked {}", colors::hex_of(c));

                    }

                }

            }

            Tool::Fill => {

                self.push_undo("Fill page");

                let page = self.doc.page().clone();

                let obj = Obj::new(

                    Shape::Rect {

                        min: Vec2::ZERO,

                        max: Vec2::new(page.width, page.height),

                        filled: true,

                    },

                    Some(self.fill),

                    None,

                );

                let p = self.doc.page_mut();

                p.layers[self.selected_layer].objects.push(obj);

                self.selected = Some(p.layers[self.selected_layer].objects.len() - 1);

                self.status = "Page filled".to_string();

            }

            Tool::Gradient => {

                self.push_undo("Gradient");

                let page = self.doc.page().clone();

                let obj = Obj::new(

                    Shape::Rect {

                        min: Vec2::ZERO,

                        max: Vec2::new(page.width, page.height),

                        filled: true,

                    },

                    Some(self.fill),

                    Some(self.stroke),

                );

                let p = self.doc.page_mut();

                p.layers[self.selected_layer].objects.push(obj);

                self.selected = Some(p.layers[self.selected_layer].objects.len() - 1);

                self.status = "Gradient shape added".to_string();

            }

            Tool::Brush | Tool::Pencil | Tool::Eraser => {

                self.push_undo("Paint");

                self.drag = Some(Drag {

                    kind: DragKind::Paint,

                    start: doc_pos,

                    current: doc_pos,

                    path: vec![doc_pos],

                });

                self.paint_at(doc_pos);

            }

            Tool::Blur | Tool::Sharpen | Tool::Dodge | Tool::Burn => {

                self.apply_image_filter();

            }

        }

    }



    fn paint_at(&mut self, doc_pos: Vec2) {

        let color = if self.tool == Tool::Eraser {

            None

        } else {

            Some(self.fill)

        };

        let obj = Obj::new(

            Shape::Ellipse {

                center: doc_pos,

                radius: Vec2::splat(3.0),

                filled: true,

            },

            color,

            None,

        );

        let page = self.doc.page_mut();

        page.layers[self.selected_layer].objects.push(obj);

    }



    fn apply_image_filter(&mut self) {

        let Some(index) = self.selected else {

            self.status = "Select a bitmap first".to_string();

            return;

        };

        let layer = self.selected_layer;

        let Some(obj) = self

            .doc

            .page_mut()

            .layers

            .get_mut(layer)

            .and_then(|l| l.objects.get_mut(index))

        else {

            return;

        };

        let Shape::Bitmap { pixels, .. } = &mut obj.kind else {

            self.status = "That filter needs a bitmap object".to_string();

            return;

        };

        let w = pixels.len() / 4;

        let count = pixels.len() / 4;

        let original = pixels.clone();

        for i in 0..count {

            match self.tool {

                Tool::Blur => {

                    let x = (i % w) as i64;

                    let y = (i / w) as i64;

                    let mut r = 0u32;

                    let mut g = 0u32;

                    let mut b = 0u32;

                    let mut n = 0u32;

                    for dy in -1..=1 {

                        for dx in -1..=1 {

                            let nx = x + dx;

                            let ny = y + dy;

                            if nx < 0 || ny < 0 {

                                continue;

                            }

                            let ni = (ny as usize) * w + nx as usize;

                            if ni >= count {

                                continue;

                            }

                            r += original[ni * 4] as u32;

                            g += original[ni * 4 + 1] as u32;

                            b += original[ni * 4 + 2] as u32;

                            n += 1;

                        }

                    }

                    if n > 0 {

                        pixels[i * 4] = (r / n) as u8;

                        pixels[i * 4 + 1] = (g / n) as u8;

                        pixels[i * 4 + 2] = (b / n) as u8;

                    }

                }

                Tool::Sharpen => {

                    let x = (i % w) as i64;

                    let y = (i / w) as i64;

                    if x == 0 || y == 0 {

                        continue;

                    }

                    let ni = (y as usize) * w + x as usize;

                    if ni >= count {

                        continue;

                    }

                    for c in 0..3 {

                        let value = original[i * 4 + c] as i32 * 2 - original[ni * 4 + c] as i32;

                        pixels[i * 4 + c] = value.clamp(0, 255) as u8;

                    }

                }

                Tool::Dodge => {

                    pixels[i * 4] = (original[i * 4] as f32 * 1.15).min(255.0) as u8;

                    pixels[i * 4 + 1] = (original[i * 4 + 1] as f32 * 1.15).min(255.0) as u8;

                    pixels[i * 4 + 2] = (original[i * 4 + 2] as f32 * 1.15).min(255.0) as u8;

                }

                Tool::Burn => {

                    pixels[i * 4] = (original[i * 4] as f32 * 0.85) as u8;

                    pixels[i * 4 + 1] = (original[i * 4 + 1] as f32 * 0.85) as u8;

                    pixels[i * 4 + 2] = (original[i * 4 + 2] as f32 * 0.85) as u8;

                }

                _ => {}

            }

        }

        self.textures.clear();

        self.status = format!("{} applied", self.tool.name());

    }



    pub fn on_pointer_drag(&mut self, doc_pos: Vec2) {

        let Some(drag) = self.drag.as_mut() else {

            return;

        };

        match drag.kind {

            DragKind::Pan => {

                let delta = doc_pos - drag.current;

                drag.current = doc_pos;

                self.doc.pan += delta * self.doc.zoom;

            }

            DragKind::Move => {

                let delta = doc_pos - drag.current;

                drag.current = doc_pos;

                if let Some(index) = self.selected {

                    let layer = self.selected_layer;

                    let page = self.doc.page_mut();

                    if let Some(obj) = page.layers.get_mut(layer).and_then(|l| l.objects.get_mut(index))

                    {

                        obj.translate(delta);

                    }

                }

            }

            DragKind::Paint => {

                drag.current = doc_pos;

                if (doc_pos - drag.path.last().copied().unwrap_or(doc_pos)).length() > 2.0 {

                    drag.path.push(doc_pos);

                    self.paint_at(doc_pos);

                }

            }

            _ => {

                drag.current = doc_pos;

            }

        }

    }



    pub fn on_pointer_up(&mut self, doc_pos: Vec2) {

        let Some(drag) = self.drag.take() else {

            return;

        };

        match drag.kind {

            DragKind::Marquee => {

                let lo = drag.start.min(doc_pos);

                let hi = drag.start.max(doc_pos);

                let mut picked = None;

                let page = self.doc.page();

                if let Some(layer) = page.layers.get(self.selected_layer) {

                    for (index, obj) in layer.objects.iter().enumerate().rev() {

                        if let Some((a, b)) = obj.bounds() {

                            if a.x >= lo.x && b.x <= hi.x && a.y >= lo.y && b.y <= hi.y {

                                picked = Some(index);

                                break;

                            }

                        }

                    }

                }

                self.selected = picked;

                self.status = if picked.is_some() {

                    "Object selected".to_string()

                } else {

                    "Nothing selected".to_string()

                };

            }

            DragKind::Create => {

                let lo = drag.start.min(doc_pos);

                let hi = drag.start.max(doc_pos);

                if hi.x - lo.x < 2.0 && hi.y - lo.y < 2.0 {

                    return;

                }

                self.push_undo("Draw");

                let kind = match self.tool {

                    Tool::Rectangle => Shape::Rect {

                        min: lo,

                        max: hi,

                        filled: true,

                    },

                    Tool::Ellipse => Shape::Ellipse {

                        center: (lo + hi) / 2.0,

                        radius: (hi - lo) / 2.0,

                        filled: true,

                    },

                    Tool::Line => Shape::Line {

                        a: drag.start,

                        b: doc_pos,

                    },

                    _ => return,

                };

                let (fill, stroke) = match self.tool {

                    Tool::Line => (None, Some(self.stroke)),

                    _ => (Some(self.fill), None),

                };

                let obj = Obj::new(kind, fill, stroke);

                let page = self.doc.page_mut();

                page.layers[self.selected_layer].objects.push(obj);

                self.selected = Some(page.layers[self.selected_layer].objects.len() - 1);

                self.status = format!("{} drawn", self.tool.name());

            }

            DragKind::Path => {

                if drag.path.len() >= 2 {

                    self.push_undo("Path");

                    let obj = Obj::new(

                        Shape::Path {

                            points: drag.path.clone(),

                        },

                        Some(self.fill),

                        Some(self.stroke),

                    );

                    let page = self.doc.page_mut();

                    page.layers[self.selected_layer].objects.push(obj);

                    self.selected = Some(page.layers[self.selected_layer].objects.len() - 1);

                }

            }

            _ => {}

        }

    }

}













impl eframe::App for FireCraft {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Keyboard shortcuts
        if ui.input(|i| i.key_pressed(egui::Key::Z) && i.modifiers.ctrl) {
            self.undo_action();
            ui.ctx().request_repaint();
            return;
        }
        if ui.input(|i| i.key_pressed(egui::Key::Y) && i.modifiers.ctrl) {
            self.redo_action();
            ui.ctx().request_repaint();
            return;
        }
        if ui.input(|i| i.key_pressed(egui::Key::N) && i.modifiers.ctrl) {
            self.doc = Document::new(constants::DEFAULT_WIDTH, constants::DEFAULT_HEIGHT);
            self.selected = None;
            self.undo.clear();
            self.redo.clear();
            self.textures.clear();
            self.status = "New document created".to_string();
            ui.ctx().request_repaint();
            return;
        }
        if ui.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            self.status = format!("Saved to {}", self.export_path);
            ui.ctx().request_repaint();
            return;
        }

        // === FIREWORKS CS6-STYLE UI ===
        let avail = ui.available_size();
        // Fireworks CS6 uses a light gray color scheme with specific proportions
        let menu_bg = Color32::from_rgb(183, 183, 183);
        let menu_h = avail.y * 0.099;
        let status_h = avail.y * 0.099;
        let main_h = avail.y - menu_h - status_h;
        let palette_w = avail.x * 0.05;
        let props_w = avail.x * 0.04;
        let canvas_w = avail.x - palette_w - props_w;
        
        // Menu bar
        {
            let frame = egui::Frame { fill: menu_bg, outer_margin: egui::Margin::same(0), inner_margin: egui::Margin::symmetric(5, 3), ..Default::default() };
            frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.2, Color32::from_rgb(50, 50, 50));
                    ui.style_mut().visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.2, Color32::from_rgb(0, 0, 0));
                    ui.strong("FireCraft");
                    ui.separator();
                    if ui.button("File").clicked() { self.status = "File".to_string(); }
                    if ui.button("Edit").clicked() { self.status = "Edit".to_string(); }
                    if ui.button("View").clicked() { self.status = "View".to_string(); }
                    if ui.button("Select").clicked() { self.status = "Select".to_string(); }
                    if ui.button("Filters").clicked() { self.status = "Filters".to_string(); }
                    if ui.button("Modify").clicked() { self.status = "Modify".to_string(); }
                    if ui.button("Window").clicked() { self.status = "Window".to_string(); }
                    ui.separator();
                    ui.label(format!("Zoom: {:.0}%", self.doc.zoom * 100.0));
                });
            });
        }
        
        ui.horizontal(|ui| {
            // Tool palette (left side)
            ui.allocate_ui(egui::vec2(palette_w, main_h), |ui| {
                let panel_bg = Color32::from_rgb(173, 173, 173);
                let frame = egui::Frame { fill: panel_bg, outer_margin: egui::Margin::same(2), ..Default::default() };
                frame.show(ui, |ui| {
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 1.0);
                    ui.vertical(|ui| {
                        let tools: &[(&str, &str, Tool)] = &[
                            ("s", "Select", Tool::Select), ("m", "Marquee", Tool::Marquee),
                            ("l", "Lasso", Tool::Lasso), ("w", "Wand", Tool::Wand),
                            ("b", "Brush", Tool::Brush), ("p", "Pencil", Tool::Pencil),
                            ("e", "Eraser", Tool::Eraser), ("g", "Fill", Tool::Fill),
                            ("r", "Line", Tool::Line), ("t", "Rect", Tool::Rectangle),
                            ("o", "Ellipse", Tool::Ellipse),
                        ];
                        for (icon, name, tool) in tools {
                            let sel = self.tool == *tool;
                            let btn = egui::Button::new(*icon)
                                .selected(sel)
                                .fill(if sel { Color32::from_rgb(100, 100, 100) } else { panel_bg })
                                .stroke(egui::Stroke::new(1.0, if sel { Color32::from_rgb(255, 255, 255) } else { Color32::from_rgb(50, 50, 50) }))
                                .min_size(egui::Vec2::new(palette_w - 8.0, 32.0));
                            if ui.add(btn).clicked() { self.tool = *tool; self.status = name.to_string(); }
                        }
                    });
                });
            });
            
            // Canvas (center)
            ui.allocate_ui(egui::vec2(canvas_w, main_h), |ui| {
                let workspace_bg = Color32::from_rgb(180, 179, 175);
                let rect = ui.max_rect();
                ui.painter().rect_filled(rect, egui::CornerRadius::same(0), workspace_bg);
                
                // Grid pattern
                let grid_size = 20.0;
                let grid_color = Color32::from_rgb(185, 184, 180);
                for x in (rect.min.x as i32)..(rect.max.x as i32) {
                    if x % (grid_size as i32) == 0 {
                        ui.painter().vline(x as f32, rect.min.y..=rect.max.y, egui::Stroke::new(0.5, grid_color));
                    }
                }
                for y in (rect.min.y as i32)..(rect.max.y as i32) {
                    if y % (grid_size as i32) == 0 {
                        ui.painter().hline(rect.min.x..=rect.max.x, y as f32, egui::Stroke::new(0.5, grid_color));
                    }
                }
                
                // Document area (matches Fireworks CS6 reference: 25.9%-74.9% width, 20%-80% height)
                let doc_w = canvas_w * 0.49;
                let doc_h = main_h * 0.599;
                let doc_x = canvas_w * 0.259;
                let doc_y = main_h * 0.20;
                let doc_rect = egui::Rect::from_min_size(egui::Pos2::new(doc_x, doc_y), egui::Vec2::new(doc_w, doc_h));
                let doc_bg = Color32::from_rgb(191, 190, 184);
                ui.painter().rect_filled(doc_rect, egui::CornerRadius::same(2), doc_bg);
                
                canvas::show(ui, self);
            });
            
            // Right panel (Properties, Layers, etc.)
            ui.allocate_ui(egui::vec2(props_w, main_h), |ui| {
                let panel_bg = Color32::from_rgb(219, 219, 219);
                let frame = egui::Frame { fill: panel_bg, outer_margin: egui::Margin::same(2), ..Default::default() };
                frame.show(ui, |ui| {
                    ui.style_mut().visuals = ui.style().visuals.clone();
                    ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(50, 50, 50));
                    ui.style_mut().spacing.item_spacing = egui::vec2(4.0, 2.0);
                    
                    // Tabs
                    ui.horizontal(|ui| {
                        ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Color32::from_rgb(50, 50, 50));
                        ui.style_mut().visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Color32::from_rgb(0, 0, 0));
                        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, Color32::from_rgb(100, 100, 100));
                        ui.style_mut().visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Color32::from_rgb(255, 255, 255));
                        if ui.button("Properties").clicked() { self.status = "Properties".to_string(); }
                        if ui.button("Stroke").clicked() { self.status = "Stroke".to_string(); }
                        if ui.button("Transform").clicked() { self.status = "Transform".to_string(); }
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.strong("Layer");
                        ui.label(format!("Layer {}", self.selected_layer + 1));
                        if let Some(layer) = self.doc.page().layers.get(self.selected_layer) {
                            ui.horizontal(|ui| { ui.label("Objects:"); ui.label(format!("{}", layer.objects.len())); });
                        }
                        ui.separator();
                        ui.horizontal(|ui| { ui.label("Fill:"); });
                        ui.horizontal(|ui| { ui.label("Stroke:"); });
                    });
                });
            });
        });
        
        // Status bar
        let status_bg = Color32::from_rgb(209, 209, 209);
        let frame = egui::Frame { fill: status_bg, ..Default::default() };
        frame.show(ui, |ui| {
            ui.style_mut().visuals = ui.style().visuals.clone();
            ui.style_mut().visuals.override_text_color = Some(Color32::from_rgb(50, 50, 50));
            ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 0.0);
            ui.label(self.status.as_str());
            ui.separator();
            ui.label(format!("{} x {}", self.doc.page().width as u32, self.doc.page().height as u32));
        });

// === SCREENSHOT: capture rendered UI ===
        if let Some(ref sp) = self.screenshot_path {
            if !self.screenshot_saved {
                self.screenshot_saved = true;
                // Request screenshot callback — closure receives Arc<ColorImage>
                let sp_clone = sp.clone();
                let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
                let done_clone = done.clone();
                ui.ctx().request_screenshot(move |image: std::sync::Arc<egui::ColorImage>| {
                    let w = image.width();
                    let h = image.height();
                    let mut rgba = Vec::with_capacity(w * h * 4);
                    for c in &image.pixels {
                        rgba.push(c.r());
                        rgba.push(c.g());
                        rgba.push(c.b());
                        rgba.push(c.a());
                    }
                    if let Some(img) = image::RgbaImage::from_raw(w as u32, h as u32, rgba) {
                        if img.save(&sp_clone).is_ok() {
                            eprintln!("Screenshot saved to {}", sp_clone);
                        }
                    }
                    done_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                });
                // Force repaint so screenshot has content
                ui.ctx().request_repaint();
                // Exit after screenshot callback completes
                let done_clone = done.clone();
                std::thread::spawn(move || {
                    for _ in 0..50 {
                        if done_clone.load(std::sync::atomic::Ordering::SeqCst) { break; }
                        std::thread::sleep(std::time::Duration::from_millis(20));
                    }
                    std::process::exit(0);
                });
            }
        }
}
}
