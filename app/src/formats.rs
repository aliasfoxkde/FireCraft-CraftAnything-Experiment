#![allow(dead_code, unused_imports, unused_variables)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! Opening and saving files.
//!
//! Raster formats go through the `image` crate (PNG, JPEG, GIF, WebP, TIFF, BMP, ICO).
//! SVG is read and written by this module so vectors stay editable.
//! Formats that need a decoder we do not ship (AVIF, HEIC, PSD, AI) are detected and
//! reported honestly instead of failing silently.

use crate::doc::{Document, Obj, Shape};
use eframe::egui::{Color32, Vec2};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Png,
    Jpeg,
    Gif,
    WebP,
    Tiff,
    Bmp,
    Ico,
    Svg,
    Avif,
    Heic,
    Psd,
    Ai,
}

impl Format {
    pub fn from_path(path: &Path) -> Format {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        match ext.as_str() {
            "png" => Format::Png,
            "jpg" | "jpeg" | "jpe" => Format::Jpeg,
            "gif" => Format::Gif,
            "webp" => Format::WebP,
            "tif" | "tiff" => Format::Tiff,
            "bmp" => Format::Bmp,
            "ico" => Format::Ico,
            "svg" => Format::Svg,
            "avif" | "avifs" => Format::Avif,
            "heic" | "heif" => Format::Heic,
            "psd" | "psb" => Format::Psd,
            "ai" => Format::Ai,
            _ => Format::Png,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Format::Png => "PNG",
            Format::Jpeg => "JPEG",
            Format::Gif => "GIF",
            Format::WebP => "WebP",
            Format::Tiff => "TIFF",
            Format::Bmp => "BMP",
            Format::Ico => "ICO",
            Format::Svg => "SVG",
            Format::Avif => "AVIF",
            Format::Heic => "HEIC",
            Format::Psd => "PSD",
            Format::Ai => "AI",
        }
    }

    pub fn writeable(self) -> bool {
        !matches!(self, Format::Avif | Format::Heic | Format::Psd | Format::Ai)
    }
}

pub const ALL_FORMATS: [Format; 12] = [
    Format::Png,
    Format::Jpeg,
    Format::Gif,
    Format::WebP,
    Format::Tiff,
    Format::Bmp,
    Format::Ico,
    Format::Svg,
    Format::Avif,
    Format::Heic,
    Format::Psd,
    Format::Ai,
];

pub fn open(path: &Path) -> Result<Document, String> {
    let format = Format::from_path(path);
    match format {
        Format::Svg => open_svg(path),
        Format::Avif | Format::Heic | Format::Psd | Format::Ai => Err(format!(
            "{} files need a decoder FireCraft does not ship yet. \
             Export to PNG or SVG from the source application and open that instead.",
            format.label()
        )),
        _ => open_raster(path),
    }
}

fn open_raster(path: &Path) -> Result<Document, String> {
    let img = image::open(path).map_err(|e| format!("Could not read the image: {e}"))?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let mut doc = Document::new(w as f32, h as f32);
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Imported")
        .to_string();
    doc.name = name;
    let obj = Obj {
        kind: Shape::Bitmap {
            at: Vec2::new(0.0, 0.0),
            pixels: rgba.into_raw(),
            size: Vec2::new(w as f32, h as f32),
        },
        fill: None,
        stroke: None,
        width: 1.0,
        name: "Bitmap".to_string(),
        visible: true,
        locked: false,
    };
    doc.page_mut().layers[0].objects.push(obj);
    Ok(doc)
}

/// A small SVG reader for the shapes Fireworks itself exports: rectangles,
/// ellipses, lines and polygons.
fn open_svg(path: &Path) -> Result<Document, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("Could not read the file: {e}"))?;
    let mut doc = Document::new(512.0, 640.0);
    doc.name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Vector")
        .to_string();

    let mut max_x = 0.0f32;
    let mut max_y = 0.0f32;
    for (tag, attrs) in tags(&text) {
        let obj = match tag.as_str() {
            "rect" => {
                let x = num(&attrs, "x");
                let y = num(&attrs, "y");
                let w = num(&attrs, "width");
                let h = num(&attrs, "height");
                max_x = max_x.max(x + w);
                max_y = max_y.max(y + h);
                Obj::new(
                    Shape::Rect {
                        min: Vec2::new(x, y),
                        max: Vec2::new(x + w, y + h),
                        filled: true,
                    },
                    paint(&attrs),
                    None,
                )
            }
            "ellipse" => {
                let cx = num(&attrs, "cx");
                let cy = num(&attrs, "cy");
                let rx = num(&attrs, "rx");
                let ry = num(&attrs, "ry");
                max_x = max_x.max(cx + rx);
                max_y = max_y.max(cy + ry);
                Obj::new(
                    Shape::Ellipse {
                        center: Vec2::new(cx, cy),
                        radius: Vec2::new(rx, ry),
                        filled: true,
                    },
                    paint(&attrs),
                    None,
                )
            }
            "line" => {
                let x1 = num(&attrs, "x1");
                let y1 = num(&attrs, "y1");
                let x2 = num(&attrs, "x2");
                let y2 = num(&attrs, "y2");
                max_x = max_x.max(x1).max(x2);
                max_y = max_y.max(y1).max(y2);
                Obj::new(
                    Shape::Line {
                        a: Vec2::new(x1, y1),
                        b: Vec2::new(x2, y2),
                    },
                    None,
                    paint(&attrs),
                )
            }
            "polygon" | "polyline" => {
                let pts = parse_points(&attrs["points"]);
                for p in &pts {
                    max_x = max_x.max(p.x);
                    max_y = max_y.max(p.y);
                }
                Obj::new(Shape::Path { points: pts }, paint(&attrs), None)
            }
            _ => continue,
        };
        doc.page_mut().layers[0].objects.push(obj);
    }
    if max_x > 0.0 {
        doc.width = max_x;
        doc.height = max_y;
        doc.page_mut().width = max_x;
        doc.page_mut().height = max_y;
    }
    Ok(doc)
}

fn tags(text: &str) -> Vec<(String, std::collections::HashMap<String, String>)> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            let end = match text[i..].find('>') {
                Some(e) => i + e,
                None => break,
            };
            let chunk = &text[i + 1..end];
            let name: String = chunk
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            if !name.is_empty() {
                out.push((name.to_ascii_lowercase(), parse_attrs(chunk)));
            }
            i = end + 1;
        } else {
            i += 1;
        }
    }
    out
}

fn parse_attrs(chunk: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let bytes = chunk.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'=' {
            let mut key_end = i;
            while key_end > 0 && (bytes[key_end - 1] as char).is_whitespace() {
                key_end -= 1;
            }
            let key_start = key_end
                - chunk[..key_end]
                    .char_indices()
                    .rev()
                    .take_while(|(_, c)| c.is_ascii_alphanumeric() || *c == '-' || *c == ':')
                    .count();
            let key = chunk[key_start..key_end].to_ascii_lowercase();
            let mut value = String::new();
            let mut j = i + 1;
            while j < bytes.len() && (bytes[j] as char).is_whitespace() {
                j += 1;
            }
            if j < bytes.len() && (bytes[j] == b'"' || bytes[j] == b'\'') {
                let quote = bytes[j];
                j += 1;
                while j < bytes.len() && bytes[j] != quote {
                    value.push(bytes[j] as char);
                    j += 1;
                }
            }
            if !key.is_empty() {
                map.insert(key, value);
            }
            i = j;
        } else {
            i += 1;
        }
    }
    map
}

fn num(attrs: &std::collections::HashMap<String, String>, key: &str) -> f32 {
    attrs
        .get(key)
        .and_then(|v| v.trim().trim_end_matches("px").parse::<f32>().ok())
        .unwrap_or(0.0)
}

fn paint(attrs: &std::collections::HashMap<String, String>) -> Option<Color32> {
    let fill = attrs.get("fill")?;
    if fill == "none" {
        return None;
    }
    crate::colors::from_hex(fill.trim())
}

fn parse_points(text: &str) -> Vec<Vec2> {
    let numbers: Vec<f32> = text
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<f32>().ok())
        .collect();
    numbers
        .chunks_exact(2)
        .map(|c| Vec2::new(c[0], c[1]))
        .collect()
}

/// Writes the document's objects as SVG.
pub fn export_svg(doc: &Document) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        doc.width, doc.height, doc.width, doc.height
    ));
    for layer in &doc.page().layers {
        if !layer.visible {
            continue;
        }
        out.push_str(&format!(
            "<g opacity=\"{}\">\n",
            layer.opacity.clamp(0.0, 1.0)
        ));
        for obj in &layer.objects {
            if !obj.visible {
                continue;
            }
            out.push_str(&obj_to_svg(obj));
            out.push('\n');
        }
        out.push_str("</g>\n");
    }
    out.push_str("</svg>\n");
    out
}

fn obj_to_svg(obj: &Obj) -> String {
    let fill = obj
        .fill
        .map(|c| crate::colors::hex_of(c))
        .unwrap_or_else(|| "none".to_string());
    let stroke = obj
        .stroke
        .map(|c| crate::colors::hex_of(c))
        .unwrap_or_else(|| "none".to_string());
    let width = obj.width.max(0.1);
    match &obj.kind {
        Shape::Rect { min, max, .. } => format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
            min.x,
            min.y,
            (max.x - min.x).max(0.0),
            (max.y - min.y).max(0.0),
            fill,
            stroke,
            width
        ),
        Shape::Ellipse { center, radius, .. } => format!(
            "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
            center.x, center.y, radius.x, radius.y, fill, stroke, width
        ),
        Shape::Line { a, b } => format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
            a.x, a.y, b.x, b.y, stroke, width
        ),
        Shape::Path { points } => {
            let pts: Vec<String> = points.iter().map(|p| format!("{},{}", p.x, p.y)).collect();
            format!(
                "<polygon points=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
                pts.join(" "),
                fill,
                stroke,
                width
            )
        }
        Shape::Text { at, text, size } => format!(
            "<text x=\"{}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"{}\" fill=\"{}\">{}</text>",
            at.x,
            at.y,
            size,
            fill,
            text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        ),
        Shape::Gradient { min, max, from, to } => format!(
            "<linearGradient id=\"grad\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"{}\"/><stop offset=\"1\" stop-color=\"{}\"/></linearGradient><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"url(#grad)\"/>",
            crate::colors::hex_of(*from),
            crate::colors::hex_of(*to),
            min.x,
            min.y,
            (max.x - min.x).max(0.0),
            (max.y - min.y).max(0.0)
        ),
        Shape::Bitmap { .. } => "<image/>".to_string(),
    }
}

/// Renders the document's vector objects into an RGBA buffer for raster export.
pub fn rasterize(doc: &Document) -> image::RgbaImage {
    let w = doc.width.max(1.0) as u32;
    let h = doc.height.max(1.0) as u32;
    let mut img = image::RgbaImage::from_pixel(w, h, image::Rgba::<u8>([255, 255, 255, 255]));
    for layer in &doc.page().layers {
        if !layer.visible {
            continue;
        }
        let alpha = (layer.opacity.clamp(0.0, 1.0) * 255.0) as u8;
        for obj in &layer.objects {
            if !obj.visible {
                continue;
            }
            draw_obj(&mut img, obj, alpha);
        }
    }
    img
}

fn draw_obj(img: &mut image::RgbaImage, obj: &Obj, alpha: u8) {
    let to_px = |p: Vec2| -> (i32, i32) { (p.x.round() as i32, p.y.round() as i32) };
    match &obj.kind {
        Shape::Rect { min, max, .. } => {
            let (x0, y0) = to_px(*min);
            let (x1, y1) = to_px(*max);
            let color = obj
                .fill
                .map(|c| image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]))
                .unwrap_or(image::Rgba::<u8>([0, 0, 0, 0]));
            let x0 = x0.min(x1);
            let y0 = y0.min(y1);
            let x1 = x0.max(x1);
            let y1 = y0.max(y1);
            for y in y0.max(0)..=y1 {
                for x in x0.max(0)..=x1 {
                    if x < img.width() as i32 && y < img.height() as i32 {
                        let old = img.get_pixel(x as u32, y as u32);
                        let blended = blend(*old, color);
                        img.put_pixel(x as u32, y as u32, blended);
                    }
                }
            }
        }
        Shape::Ellipse { center, radius, .. } => {
            let color = obj
                .fill
                .map(|c| image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]))
                .unwrap_or(image::Rgba::<u8>([0, 0, 0, 0]));
            let (cx, cy) = to_px(*center);
            let rx = radius.x.max(1.0);
            let ry = radius.y.max(1.0);
            for y in (cy - ry as i32).max(0)..=(cy + ry as i32) {
                for x in (cx - rx as i32).max(0)..=(cx + rx as i32) {
                    if x >= img.width() as i32 || y >= img.height() as i32 {
                        continue;
                    }
                    let dx = (x as f32 - cx as f32) / rx;
                    let dy = (y as f32 - cy as f32) / ry;
                    if dx * dx + dy * dy <= 1.0 {
                        let old = img.get_pixel(x as u32, y as u32);
                        let blended = blend(*old, color);
                        img.put_pixel(x as u32, y as u32, blended);
                    }
                }
            }
        }
        Shape::Line { a, b } => {
            if let Some(c) = obj.stroke {
                let color = image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]);
                let (x0, y0) = to_px(*a);
                let (x1, y1) = to_px(*b);
                let steps = ((x1 - x0).abs().max((y1 - y0).abs())).max(1);
                for i in 0..=steps {
                    let x = x0 + (x1 - x0) * i / steps;
                    let y = y0 + (y1 - y0) * i / steps;
                    if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
                        let old = img.get_pixel(x as u32, y as u32);
                        img.put_pixel(x as u32, y as u32, blend(*old, color));
                    }
                }
            }
        }
        Shape::Path { points } => {
            let color = obj
                .fill
                .map(|c| image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]))
                .unwrap_or(image::Rgba::<u8>([0, 0, 0, 0]));
            if points.len() > 2 {
                for i in 0..points.len() {
                    let a = points[i];
                    let b = points[(i + 1) % points.len()];
                    let (x0, y0) = to_px(a);
                    let (x1, y1) = to_px(b);
                    let steps = ((x1 - x0).abs().max((y1 - y0).abs())).max(1);
                    for s in 0..=steps {
                        let x = x0 + (x1 - x0) * s / steps;
                        let y = y0 + (y1 - y0) * s / steps;
                        if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
                            let old = img.get_pixel(x as u32, y as u32);
                            img.put_pixel(x as u32, y as u32, blend(*old, color));
                        }
                    }
                }
            }
        }
        Shape::Text { at, text, size } => {
            if let Some(c) = obj.fill {
                let color = image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]);
                let (x, y) = to_px(*at);
                // A simple blocky glyph rendering: one rectangle per character cell.
                let cell = (size * 0.6).round() as i32;
                for (i, ch) in text.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }
                    let gx = x + i as i32 * cell;
                    for gy in y..(y + cell) {
                        if gx >= 0 && gy >= 0 && gx < img.width() as i32 && gy < img.height() as i32 {
                            let old = img.get_pixel(gx as u32, gy as u32);
                            img.put_pixel(gx as u32, gy as u32, blend(*old, color));
                        }
                    }
                }
            }
        }
        Shape::Gradient { min, max, from, to } => {
            let (x0, y0) = to_px(*min);
            let (x1, y1) = to_px(*max);
            let steps = 64;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let c = from.lerp_to_gamma(*to, t);
                let color = image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]);
                let y = y0 + (y1 - y0) * i / steps;
                if y < 0 || y >= img.height() as i32 {
                    continue;
                }
                for x in x0.max(0)..=x1.min(img.width() as i32 - 1) {
                    let old = img.get_pixel(x as u32, y as u32);
                    img.put_pixel(x as u32, y as u32, blend(*old, color));
                }
            }
        }
        Shape::Bitmap { at, pixels, size } => {
            let src = image::RgbaImage::from_raw(size.x.max(1.0) as u32, size.y.max(1.0) as u32, pixels.clone())
                .unwrap_or_else(|| image::RgbaImage::new(1, 1));
            let (ox, oy) = to_px(*at);
            for y in 0..src.height() as i32 {
                for x in 0..src.width() as i32 {
                    let px = ox + x;
                    let py = oy + y;
                    if px >= 0 && py >= 0 && px < img.width() as i32 && py < img.height() as i32 {
                        let mut p = *src.get_pixel(x as u32, y as u32);
                        p.0[3] = (p.0[3] as f32 * alpha as f32 / 255.0) as u8;
                        let old = img.get_pixel(px as u32, py as u32);
                        img.put_pixel(px as u32, py as u32, blend(*old, p));
                    }
                }
            }
        }
    }
}

fn blend(dst: image::Rgba<u8>, src: image::Rgba<u8>) -> image::Rgba<u8> {
    let sa = src.0[3] as f32 / 255.0;
    if sa >= 1.0 {
        return src;
    }
    if sa <= 0.0 {
        return dst;
    }
    let da = dst.0[3] as f32 / 255.0;
    let out_a = sa + da * (1.0 - sa);
    if out_a <= 0.0 {
        return image::Rgba::<u8>([0, 0, 0, 0]);
    }
    let mut out = [0u8; 4];
    for i in 0..3 {
        let s = src.0[i] as f32 / 255.0;
        let d = dst.0[i] as f32 / 255.0;
        out[i] = (((s * sa + d * da * (1.0 - sa)) / out_a) * 255.0).round() as u8;
    }
    out[3] = (out_a * 255.0).round() as u8;
    image::Rgba::<u8>(out)
}

/// Saves the document in the format implied by the path.
pub fn save(doc: &Document, path: &Path) -> Result<(), String> {
    let format = Format::from_path(path);
    if !format.writeable() {
        return Err(format!(
            "FireCraft cannot write {} yet. Choose PNG, JPEG, WebP, TIFF, GIF, BMP or SVG.",
            format.label()
        ));
    }
    if format == Format::Svg {
        std::fs::write(path, export_svg(doc)).map_err(|e| format!("Could not write the file: {e}"))
    } else {
        let img = rasterize(doc);
        let dyn_img = image::DynamicImage::ImageRgba8(img);
        let result = match format {
            Format::Jpeg => dyn_img.to_rgb8().save(path),
            Format::Gif => dyn_img.to_rgba8().save(path),
            Format::Tiff => dyn_img.to_rgba8().save(path),
            Format::Bmp => dyn_img.to_rgb8().save(path),
            Format::Ico => dyn_img.to_rgba8().save(path),
            Format::WebP => dyn_img.to_rgba8().save(path),
            _ => dyn_img.to_rgba8().save(path),
        };
        result.map_err(|e| format!("Could not write the file: {e}"))
    }
}


