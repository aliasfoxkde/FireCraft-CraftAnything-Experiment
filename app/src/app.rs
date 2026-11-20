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

use std::path::{Path, PathBuf};

/// What the file dialog is being used for.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FileMode {
    Open,
    Save,
    Export,
}



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

    /// Size of the canvas viewport, published by `canvas::show` each frame so
    /// `Fit in Window` can work out a zoom factor.
    pub canvas_size: Vec2,

    /// Width of the brush stroke, in document pixels.
    pub brush_size: f32,

    /// Grid spacing in document pixels.
    pub grid_size: f32,

    /// The document's path on disk, when it has one.
    pub current_path: Option<PathBuf>,

    /// The New Document dialog.
    pub show_new_dialog: bool,

    pub new_doc_width: f32,

    pub new_doc_height: f32,

    /// Background colour for a new document, as linear RGB in 0..=1.
    pub new_doc_bg: [f32; 3],

    /// The Open / Save / Export file dialog.
    pub show_file_dialog: bool,

    pub file_mode: FileMode,

    pub file_dialog_dir: PathBuf,

    pub file_dialog_entries: Vec<PathBuf>,

    pub file_dialog_name: String,

    /// The right-hand dock's tab index.
    pub right_tab: usize,

    /// The clipboard, holding one copied object.
    pub clipboard: Option<Obj>,

    /// The welcome splash, drawn over the editor.
    pub show_welcome: bool,

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

            color_tab: 0,

            layers_tab: 2,

            library_tab: 0,

            path_tab: 0,

            style_name: "Current Document".to_string(),

            textures: TextureCache::new(),

            logo_tex: None,
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

            canvas_size: Vec2::new(800.0, 600.0),

            brush_size: constants::BRUSH_SIZE_DEFAULT,

            grid_size: 20.0,

            current_path: None,

            show_new_dialog: false,

            new_doc_width: constants::DEFAULT_WIDTH,

            new_doc_height: constants::DEFAULT_HEIGHT,

            new_doc_bg: [1.0, 1.0, 1.0],

            show_file_dialog: false,

            file_mode: FileMode::Open,

            file_dialog_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),

            file_dialog_entries: Vec::new(),

            file_dialog_name: String::new(),

            right_tab: 0,

            clipboard: None,

            // The editor is usable from the first frame; the splash is one
            // click away under File > Welcome Screen.
            show_welcome: false,

        }

    }



    /// Append an object to the active layer and return its index.
    ///
    /// The layer index is clamped, so a stale `selected_layer` can never
    /// panic the way a direct `layers[self.selected_layer]` did.
    pub fn push_object(&mut self, obj: Obj) -> usize {
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        let Some(layer) = page.layers.get_mut(layer) else {
            return 0;
        };
        layer.objects.push(obj);
        layer.objects.len() - 1
    }

    pub fn load_logo(ctx: &Context) -> Option<egui::TextureHandle> {
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



    pub fn undo_action(&mut self) {

        if let Some(snap) = self.undo.pop() {

            self.redo.push(self.doc.snapshot("redo"));

            self.doc.restore(&snap);

            self.textures.clear();

            self.selected = None;

            self.status = format!("Undo: {}", snap.label);

        }

    }



    pub fn redo_action(&mut self) {

        if let Some(snap) = self.redo.pop() {

            self.undo.push(self.doc.snapshot("redo"));

            self.doc.restore(&snap);

            self.textures.clear();

            self.selected = None;

            self.status = "Redo".to_string();

        }

    }



    pub fn delete_selected(&mut self) {

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



    pub fn duplicate_selected(&mut self) {

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

                let factor = if alt { 1.0 / constants::ZOOM_STEP } else { constants::ZOOM_STEP };

                self.set_zoom(self.doc.zoom * factor);

            }

            Tool::Fit => {

                self.fit_in_window();

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

                let index = self.push_object(obj);

                self.selected = Some(index);

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

                let index = self.push_object(obj);

                self.selected = Some(index);

                self.status = "Page filled".to_string();

            }

            Tool::Gradient => {

                self.push_undo("Gradient");

                let page = self.doc.page().clone();

                // A real gradient shape: `from` is the fill colour and `to` the
                // stroke colour, blended top to bottom over the page.
                let obj = Obj::new(

                    Shape::Gradient {

                        min: Vec2::ZERO,

                        max: Vec2::new(page.width, page.height),

                        from: self.fill,

                        to: self.stroke,

                    },

                    None,

                    None,

                );

                let index = self.push_object(obj);

                self.selected = Some(index);

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

                if self.tool == Tool::Eraser {

                    self.erase_at(doc_pos);

                }

            }

            Tool::Blur | Tool::Sharpen | Tool::Dodge | Tool::Burn => {

                self.apply_image_filter();

            }

        }

    }



    /// Erase every object whose bounds contain `doc_pos`.
    ///
    /// The eraser used to append an invisible ellipse instead of removing
    /// anything, which made it look like it did nothing at all.
    fn erase_at(&mut self, doc_pos: Vec2) {

        let radius = (self.brush_size / 2.0).max(2.0);

        let layer = self.selected_layer;

        let page = self.doc.page_mut();

        let Some(layer) = page.layers.get_mut(layer) else {

            return;

        };

        layer.objects.retain(|obj| {

            match obj.bounds() {

                Some((lo, hi)) => {

                    !(doc_pos.x >= lo.x - radius
                        && doc_pos.x <= hi.x + radius
                        && doc_pos.y >= lo.y - radius
                        && doc_pos.y <= hi.y + radius)

                }

                None => true,

            }

        });

        self.textures.clear();

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

        let Shape::Bitmap { pixels, size, .. } = &mut obj.kind else {

            self.status = "That filter needs a bitmap object".to_string();

            return;

        };

        let count = pixels.len() / 4;

        let w = size.x.max(1.0) as usize;

        let rows = size.y.max(1.0) as usize;

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

                            if nx < 0 || ny < 0 || nx as usize >= w || ny as usize >= rows {

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

                    // A 3×3 unsharp kernel: the centre pixel is weighted 5×
                    // and the four orthogonal neighbours are subtracted.
                    // The old code compared each pixel with itself, so
                    // sharpening did nothing at all.
                    let x = (i % w) as i64;

                    let y = (i / w) as i64;

                    if rows == 0 {

                        continue;

                    }

                    let cx = x.clamp(0, w as i64 - 1).max(0);

                    let cy = y.clamp(0, rows as i64 - 1).max(0);

                    let at = |px: i64, py: i64, c: usize| -> i32 {

                        let px = px.clamp(0, w as i64 - 1) as usize;
let py = py.clamp(0, rows as i64 - 1) as usize;
let idx = (py * w + px) * 4 + c;

                        if idx >= original.len() {

                            return 0;

                        }

                        original.get(idx).copied().unwrap_or(0) as i32

                    };





                    for c in 0..3 {

                        let centre = at(cx, cy, c);

                        let value = centre * 5
                            - at(cx - 1, cy, c)
                            - at(cx + 1, cy, c)
                            - at(cx, cy - 1, c)
                            - at(cx, cy + 1, c);

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

                    if self.tool == Tool::Eraser {

                        self.erase_at(doc_pos);

                    }

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

                            if b.x >= lo.x && a.x <= hi.x && b.y >= lo.y && a.y <= hi.y {

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

                let index = self.push_object(obj);

                self.selected = Some(index);

                self.status = format!("{} drawn", self.tool.name());

            }

            DragKind::Paint => {

                // The brush and pencil draw one stroked path per drag, using
                // the brush size as the stroke width.
                if drag.path.len() >= 2 && self.tool != Tool::Eraser {

                    self.push_undo("Stroke");

                    let mut obj = Obj::new(

                        Shape::Path {

                            points: drag.path.clone(),

                        },

                        None,

                        Some(self.fill),

                    );

                    obj.width = self.brush_size.max(1.0);

                    let index = self.push_object(obj);

                    self.selected = Some(index);

                }

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














    // ---- document lifecycle ------------------------------------------------

    /// Replace the document with a fresh, empty one.
    pub fn new_document(&mut self, width: f32, height: f32) {
        self.doc = Document::new(width, height);
        self.selected = None;
        self.selected_layer = 0;
        self.undo.clear();
        self.redo.clear();
        self.textures.clear();
        self.current_path = None;
        self.doc.zoom = constants::DEFAULT_ZOOM;
        self.doc.pan = Vec2::ZERO;
        self.status = format!("New document: {width:.0} × {height:.0} px");
    }

    /// Close the document and go back to an empty one.
    pub fn close_document(&mut self) {
        self.new_document(constants::DEFAULT_WIDTH, constants::DEFAULT_HEIGHT);
        self.doc.name = "Untitled".to_string();
        self.status = "Document closed".to_string();
    }

    // ---- files -------------------------------------------------------------

    /// Open the file browser for the given purpose.
    pub fn begin_file_dialog(&mut self, mode: FileMode) {
        self.file_mode = mode;
        self.file_dialog_name = match mode {
            FileMode::Open => String::new(),
            FileMode::Save | FileMode::Export => self
                .current_path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| match mode {
                    FileMode::Export => "export.png".to_string(),
                    _ => "untitled.png".to_string(),
                }),
        };
        let dir = self
            .current_path
            .as_ref()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| self.file_dialog_dir.clone());
        self.set_file_dialog_dir(dir);
        self.show_file_dialog = true;
    }

    /// Point the file browser at a directory and list its contents.
    pub fn set_file_dialog_dir(&mut self, dir: PathBuf) {
        self.file_dialog_dir = dir.clone();
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.is_dir() || !p.to_string_lossy().ends_with(".exe"))
                    .collect()
            })
            .unwrap_or_default();
        entries.sort_by_key(|p| (!p.is_dir(), p.to_string_lossy().to_lowercase()));
        self.file_dialog_entries = entries;
    }

    /// Act on the file the user picked in the browser.
    pub fn commit_file_dialog(&mut self) {
        if self.file_dialog_name.trim().is_empty() {
            self.status = "No file name given".to_string();
            return;
        }
        let path = self.file_dialog_dir.join(self.file_dialog_name.trim());
        let result = match self.file_mode {
            FileMode::Open => self.open_path(&path),
            FileMode::Save => self.save_to(&path),
            FileMode::Export => self.export_to(&path),
        };
        if result.is_ok() {
            self.show_file_dialog = false;
        }
    }

    /// Load a document from disk.
    pub fn open_path(&mut self, path: &Path) -> Result<(), String> {
        match crate::formats::open(path) {
            Ok(doc) => {
                self.doc = doc;
                self.selected = None;
                self.selected_layer = 0;
                self.undo.clear();
                self.redo.clear();
                self.textures.clear();
                self.current_path = Some(path.to_path_buf());
                self.status = format!("Opened {}", path.display());
                Ok(())
            }
            Err(e) => {
                self.status = format!("Could not open: {e}");
                Err(e)
            }
        }
    }

    /// Save to the document's current path, falling back to the file browser.
    pub fn save_current(&mut self) {
        match self.current_path.clone() {
            Some(path) => {
                if let Err(e) = self.save_to(&path) {
                    self.status = format!("Save failed: {e}");
                }
            }
            None => self.begin_file_dialog(FileMode::Save),
        }
    }

    /// Write the document to `path`, choosing the format from its extension.
    pub fn save_to(&mut self, path: &Path) -> Result<(), String> {
        crate::formats::save(&self.doc, path)?;
        self.current_path = Some(path.to_path_buf());
        self.status = format!("Saved {}", path.display());
        Ok(())
    }

    /// Write the document to `path` as an export.
    pub fn export_to(&mut self, path: &Path) -> Result<(), String> {
        let format = crate::formats::Format::from_path(path);
        if !format.writeable() {
            let msg = format!(
                "{} export is not supported yet; choose PNG, JPEG, GIF, WebP, TIFF, BMP, ICO or SVG.",
                format.label()
            );
            self.status = msg.clone();
            return Err(msg);
        }
        crate::formats::save(&self.doc, path)?;
        self.status = format!("Exported {}", path.display());
        Ok(())
    }

    // ---- clipboard ---------------------------------------------------------

    pub fn copy_selected(&mut self) {
        if let Some(index) = self.selected {
            let layer = self.selected_layer;
            if let Some(obj) = self
                .doc
                .page()
                .layers
                .get(layer)
                .and_then(|l| l.objects.get(index))
            {
                self.clipboard = Some(obj.clone());
                self.status = "Copied".to_string();
            }
        }
    }

    pub fn cut_selected(&mut self) {
        self.copy_selected();
        self.delete_selected();
    }

    pub fn paste(&mut self) {
        let Some(mut obj) = self.clipboard.clone() else {
            return;
        };
        obj.translate(Vec2::new(10.0, 10.0));
        self.push_undo("Paste");
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        if layer < page.layers.len() {
            page.layers[layer].objects.push(obj);
            self.selected = Some(page.layers[layer].objects.len() - 1);
            self.status = "Pasted".to_string();
        }
    }

    pub fn select_all(&mut self) {
        let layer = self.selected_layer;
        let count = self
            .doc
            .page()
            .layers
            .get(layer)
            .map(|l| l.objects.len())
            .unwrap_or(0);
        self.selected = if count > 0 { Some(count - 1) } else { None };
        self.status = format!("Selected {count} object(s)");
    }

    // ---- geometry ----------------------------------------------------------

    /// Mirror the selection about the vertical axis of its own bounds.
    pub fn flip_horizontal(&mut self) {
        self.mirror_selected(true);
    }

    /// Mirror the selection about the horizontal axis of its own bounds.
    pub fn flip_vertical(&mut self) {
        self.mirror_selected(false);
    }

    fn mirror_selected(&mut self, horizontal: bool) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        let Some((lo, hi)) = self
            .doc
            .page()
            .layers
            .get(layer)
            .and_then(|l| l.objects.get(index))
            .and_then(|o| o.bounds())
        else {
            return;
        };
        self.push_undo("Flip");
        let axis = if horizontal { lo.x + hi.x } else { lo.y + hi.y };
        let Some(obj) = self
            .doc
            .page_mut()
            .layers
            .get_mut(layer)
            .and_then(|l| l.objects.get_mut(index))
        else {
            return;
        };
        let apply = |p: &mut Vec2| {
            if horizontal {
                p.x = axis - p.x;
            } else {
                p.y = axis - p.y;
            }
        };
        match &mut obj.kind {
            crate::doc::Shape::Rect { min, max, .. } => {
                apply(min);
                apply(max);
            }
            crate::doc::Shape::Ellipse { center, .. } => {
                if horizontal {
                    center.x = axis - center.x;
                } else {
                    center.y = axis - center.y;
                }
            }
            crate::doc::Shape::Line { a, b } => {
                apply(a);
                apply(b);
            }
            crate::doc::Shape::Path { points } => {
                for p in points.iter_mut() {
                    apply(p);
                }
            }
            crate::doc::Shape::Text { at, .. } => {
                if horizontal {
                    at.x = axis - at.x;
                } else {
                    at.y = axis - at.y;
                }
            }
            crate::doc::Shape::Gradient { min, max, .. } => {
                apply(min);
                apply(max);
            }
            crate::doc::Shape::Bitmap { at, .. } => {
                if horizontal {
                    at.x = axis - at.x;
                } else {
                    at.y = axis - at.y;
                }
            }
        }
        self.textures.clear();
        self.status = "Flipped".to_string();
    }

    /// Rotate the selection a quarter turn about the centre of its bounds.
    pub fn rotate_90(&mut self) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        let Some((lo, hi)) = self
            .doc
            .page()
            .layers
            .get(layer)
            .and_then(|l| l.objects.get(index))
            .and_then(|o| o.bounds())
        else {
            return;
        };
        let centre = (lo + hi) / 2.0;
        let size = hi - lo;
        self.push_undo("Rotate");
        let Some(obj) = self
            .doc
            .page_mut()
            .layers
            .get_mut(layer)
            .and_then(|l| l.objects.get_mut(index))
        else {
            return;
        };
        // A quarter turn maps (x, y) to (y, -x) about the centre; the box then
        // has its axes swapped.
        let rotate = |p: &mut Vec2| {
            let d = *p - centre;
            *p = centre + Vec2::new(d.y, -d.x);
        };
        match &mut obj.kind {
            crate::doc::Shape::Rect { min, max, .. } => {
                rotate(min);
                rotate(max);
            }
            crate::doc::Shape::Ellipse { center, radius, .. } => {
                rotate(center);
                std::mem::swap(&mut radius.x, &mut radius.y);
            }
            crate::doc::Shape::Line { a, b } => {
                rotate(a);
                rotate(b);
            }
            crate::doc::Shape::Path { points } => {
                for p in points.iter_mut() {
                    rotate(p);
                }
            }
            crate::doc::Shape::Text { at, .. } => rotate(at),
            crate::doc::Shape::Gradient { min, max, .. } => {
                rotate(min);
                rotate(max);
            }
            crate::doc::Shape::Bitmap { at, .. } => rotate(at),
        }
        let _ = size;
        self.textures.clear();
        self.status = "Rotated 90°".to_string();
    }

    /// Move the selection to the front or the back of its layer.
    pub fn reorder_selected(&mut self, to_front: bool) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        self.push_undo("Reorder");
        let page = self.doc.page_mut();
        let Some(objects) = page.layers.get_mut(layer).map(|l| &mut l.objects) else {
            return;
        };
        if index >= objects.len() {
            return;
        }
        let obj = objects.remove(index);
        let new_index = if to_front {
            objects.push(obj);
            objects.len() - 1
        } else {
            objects.insert(0, obj);
            0
        };
        self.selected = Some(new_index);
        self.status = if to_front {
            "Brought to front".to_string()
        } else {
            "Sent to back".to_string()
        };
    }

    // ---- view --------------------------------------------------------------

    pub fn set_zoom(&mut self, zoom: f32) {
        self.doc.zoom = zoom.clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);
        self.status = format!("Zoom: {:.0}%", self.doc.zoom * 100.0);
    }

    pub fn zoom_by(&mut self, factor: f32) {
        let zoom = self.doc.zoom * factor;
        self.set_zoom(zoom);
    }

    /// Zoom so the whole page fits the canvas viewport.
    pub fn fit_in_window(&mut self) {
        let page = self.doc.page();
        let (pw, ph) = (page.width.max(1.0), page.height.max(1.0));
        let (cw, ch) = (self.canvas_size.x.max(1.0), self.canvas_size.y.max(1.0));
        // Leave a margin so the page is visibly inside the workspace.
        let zoom = ((cw * 0.92) / pw).min((ch * 0.92) / ph);
        self.set_zoom(zoom);
        self.doc.pan = Vec2::ZERO;
        self.status = "Fit in window".to_string();
    }

    pub fn set_theme(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
        self.palette = theme::resolve(mode);
        self.status = format!("Theme: {}", mode.label());
    }

}

impl eframe::App for FireCraft {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        crate::layout::show(ui, self);
    }
}
