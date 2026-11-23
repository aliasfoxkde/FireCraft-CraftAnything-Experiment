//! Opening and saving files.
//!
//! Raster formats go through the `image` crate (PNG, JPEG, GIF, WebP, TIFF, BMP, ICO).
//! SVG and HTML5 (+ CSS/Slices/Hotspots) are generated natively so vectors and web overlays stay editable.

use crate::doc::{Document, LiveEffect, Obj, Shape};
use eframe::egui::{Color32, Vec2};
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Format {
    Png,
    Jpeg,
    Gif,
    WebP,
    Tiff,
    Bmp,
    Ico,
    Svg,
    Html,
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
            "html" | "htm" => Format::Html,
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
            Format::Html => "HTML5",
            Format::Avif => "AVIF",
            Format::Heic => "HEIC",
            Format::Psd => "PSD",
            Format::Ai => "AI",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Format::Png => "png",
            Format::Jpeg => "jpg",
            Format::Gif => "gif",
            Format::WebP => "webp",
            Format::Tiff => "tiff",
            Format::Bmp => "bmp",
            Format::Ico => "ico",
            Format::Svg => "svg",
            Format::Html => "html",
            Format::Avif => "avif",
            Format::Heic => "heic",
            Format::Psd => "psd",
            Format::Ai => "ai",
        }
    }

    pub fn writeable(self) -> bool {
        !matches!(self, Format::Avif | Format::Heic | Format::Psd | Format::Ai)
    }
}

pub fn open(path: &Path) -> Result<Document, String> {
    let format = Format::from_path(path);
    match format {
        Format::Svg => open_svg(path),
        Format::Avif | Format::Heic | Format::Psd | Format::Ai => Err(format!(
            "{} files need a format bridge decoder. Export to PNG or SVG from the source application and open that instead.",
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
    let mut obj = Obj::new(
        Shape::Bitmap {
            at: Vec2::new(0.0, 0.0),
            pixels: rgba.into_raw(),
            size: Vec2::new(w as f32, h as f32),
        },
        None,
        None,
    );
    obj.name = "Bitmap".to_string();
    doc.page_mut().layers[0].objects.push(obj);
    doc.page_mut().sync_state();
    Ok(doc)
}

/// Reads an SVG file into editable vector objects.
pub fn open_svg(path: &Path) -> Result<Document, String> {
    let text =
        std::fs::read_to_string(path).map_err(|e| format!("Could not read the file: {e}"))?;
    let mut doc = parse_svg_str(&text);
    doc.name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Vector.svg")
        .to_string();
    Ok(doc)
}

pub fn parse_svg_str(text: &str) -> Document {
    let mut doc = Document::new(800.0, 600.0);
    let mut max_x = 0.0f32;
    let mut max_y = 0.0f32;
    for (tag, attrs) in tags(text) {
        if tag == "svg" {
            let sw = num(&attrs, "width");
            let sh = num(&attrs, "height");
            if sw > 0.0 && sh > 0.0 {
                doc.width = sw;
                doc.height = sh;
                doc.page_mut().width = sw;
                doc.page_mut().height = sh;
            }
            continue;
        }
        let stroke_col = stroke_attr(&attrs);
        let sw = num_or(&attrs, "stroke-width", 1.0);
        let obj = match tag.as_str() {
            "rect" => {
                let x = num(&attrs, "x");
                let y = num(&attrs, "y");
                let w = num(&attrs, "width");
                let h = num(&attrs, "height");
                let rx = num(&attrs, "rx");
                max_x = max_x.max(x + w);
                max_y = max_y.max(y + h);
                let mut o = Obj::new(
                    Shape::Rect {
                        min: Vec2::new(x, y),
                        max: Vec2::new(x + w, y + h),
                        corner_radius: rx,
                        filled: true,
                    },
                    paint(&attrs),
                    stroke_col,
                );
                o.width = sw;
                o
            }
            "ellipse" => {
                let cx = num(&attrs, "cx");
                let cy = num(&attrs, "cy");
                let rx = num(&attrs, "rx");
                let ry = num(&attrs, "ry");
                max_x = max_x.max(cx + rx);
                max_y = max_y.max(cy + ry);
                let mut o = Obj::new(
                    Shape::Ellipse {
                        center: Vec2::new(cx, cy),
                        radius: Vec2::new(rx, ry),
                        filled: true,
                    },
                    paint(&attrs),
                    stroke_col,
                );
                o.width = sw;
                o
            }
            "circle" => {
                let cx = num(&attrs, "cx");
                let cy = num(&attrs, "cy");
                let r = num(&attrs, "r");
                max_x = max_x.max(cx + r);
                max_y = max_y.max(cy + r);
                let mut o = Obj::new(
                    Shape::Ellipse {
                        center: Vec2::new(cx, cy),
                        radius: Vec2::new(r, r),
                        filled: true,
                    },
                    paint(&attrs),
                    stroke_col,
                );
                o.width = sw;
                o
            }
            "line" => {
                let x1 = num(&attrs, "x1");
                let y1 = num(&attrs, "y1");
                let x2 = num(&attrs, "x2");
                let y2 = num(&attrs, "y2");
                max_x = max_x.max(x1).max(x2);
                max_y = max_y.max(y1).max(y2);
                let mut o = Obj::new(
                    Shape::Line {
                        a: Vec2::new(x1, y1),
                        b: Vec2::new(x2, y2),
                    },
                    None,
                    stroke_col.or_else(|| paint(&attrs)),
                );
                o.width = sw;
                o
            }
            "polygon" | "polyline" => {
                let pts = attrs
                    .get("points")
                    .map(|p| parse_points(p))
                    .unwrap_or_default();
                for p in &pts {
                    max_x = max_x.max(p.x);
                    max_y = max_y.max(p.y);
                }
                let mut o = Obj::new(
                    Shape::Path {
                        points: pts,
                        closed: tag == "polygon",
                    },
                    paint(&attrs),
                    stroke_col,
                );
                o.width = sw;
                o
            }
            _ => continue,
        };
        doc.page_mut().layers[0].objects.push(obj);
    }
    if doc.width <= 0.0 && max_x > 0.0 {
        doc.width = max_x;
        doc.height = max_y;
        doc.page_mut().width = max_x;
        doc.page_mut().height = max_y;
    }
    doc.page_mut().sync_state();
    doc
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
    num_or(attrs, key, 0.0)
}

fn num_or(attrs: &std::collections::HashMap<String, String>, key: &str, default: f32) -> f32 {
    attrs
        .get(key)
        .and_then(|v| v.trim().trim_end_matches("px").parse::<f32>().ok())
        .unwrap_or(default)
}

fn paint(attrs: &std::collections::HashMap<String, String>) -> Option<Color32> {
    let fill = attrs.get("fill")?;
    if fill == "none" {
        return None;
    }
    crate::colors::from_hex(fill.trim())
}

fn stroke_attr(attrs: &std::collections::HashMap<String, String>) -> Option<Color32> {
    let st = attrs.get("stroke")?;
    if st == "none" {
        return None;
    }
    crate::colors::from_hex(st.trim())
}

fn parse_points(text: &str) -> Vec<Vec2> {
    let numbers: Vec<f32> = text
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<f32>().ok())
        .collect();
    numbers
        .as_chunks::<2>()
        .0
        .iter()
        .map(|c| Vec2::new(c[0], c[1]))
        .collect()
}

/// Writes the document's objects as SVG.
pub fn export_svg(doc: &Document) -> String {
    let page = doc.page();
    let mut out = String::new();
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
        page.width, page.height, page.width, page.height
    ));
    if !page.transparent_bg {
        out.push_str(&format!(
            "  <rect width=\"100%\" height=\"100%\" fill=\"{}\"/>\n",
            crate::colors::hex_of(page.bg_color)
        ));
    }
    for layer in &page.layers {
        if !layer.visible || layer.is_web_layer {
            continue;
        }
        out.push_str(&format!(
            "  <g id=\"{}\" opacity=\"{:.2}\">\n",
            layer.name.replace('"', ""),
            layer.opacity.clamp(0.0, 1.0)
        ));
        for (idx, obj) in layer.objects.iter().enumerate() {
            if !obj.visible {
                continue;
            }
            out.push_str("    ");
            out.push_str(&obj_to_svg(obj, idx));
            out.push('\n');
        }
        out.push_str("  </g>\n");
    }
    out.push_str("</svg>\n");
    out
}

fn obj_to_svg(obj: &Obj, idx: usize) -> String {
    let fill = obj
        .fill
        .map(crate::colors::hex_of)
        .unwrap_or_else(|| "none".to_string());
    let stroke = obj
        .stroke
        .map(crate::colors::hex_of)
        .unwrap_or_else(|| "none".to_string());
    let width = obj.width.max(0.1);
    let op = obj.opacity.clamp(0.0, 1.0);
    match &obj.kind {
        Shape::Rect {
            min,
            max,
            corner_radius,
            ..
        } => {
            let lo = min.min(*max);
            let hi = min.max(*max);
            let rx_attr = if *corner_radius > 0.0 {
                format!(" rx=\"{corner_radius}\" ry=\"{corner_radius}\"")
            } else {
                String::new()
            };
            format!(
                "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"{} fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{:.2}\"/>",
                lo.x,
                lo.y,
                (hi.x - lo.x).max(0.0),
                (hi.y - lo.y).max(0.0),
                rx_attr,
                fill,
                stroke,
                width,
                op
            )
        }
        Shape::Ellipse { center, radius, .. } => format!(
            "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{:.2}\"/>",
            center.x,
            center.y,
            radius.x.abs(),
            radius.y.abs(),
            fill,
            stroke,
            width,
            op
        ),
        Shape::Line { a, b } => format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{:.2}\"/>",
            a.x, a.y, b.x, b.y, stroke, width, op
        ),
        Shape::Polygon {
            center,
            radius,
            sides,
            star_inner,
            angle_rad,
        } => {
            let pts: Vec<String> =
                Obj::polygon_vertices(*center, *radius, *sides, *star_inner, *angle_rad)
                    .iter()
                    .map(|p| format!("{:.1},{:.1}", p.x, p.y))
                    .collect();
            format!(
                "<polygon points=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{:.2}\"/>",
                pts.join(" "),
                fill,
                stroke,
                width,
                op
            )
        }
        Shape::Path { points, closed } => {
            let pts: Vec<String> = points.iter().map(|p| format!("{},{}", p.x, p.y)).collect();
            let tag = if *closed { "polygon" } else { "polyline" };
            let fill_attr = if *closed { fill.as_str() } else { "none" };
            format!(
                "<{} points=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\" opacity=\"{:.2}\"/>",
                tag,
                pts.join(" "),
                fill_attr,
                stroke,
                width,
                op
            )
        }
        Shape::Text {
            at,
            text,
            size,
            bold,
            italic,
            font_family,
        } => {
            let weight = if *bold { "bold" } else { "normal" };
            let style = if *italic { "italic" } else { "normal" };
            format!(
                "<text x=\"{}\" y=\"{}\" font-family=\"{}\" font-size=\"{}\" font-weight=\"{}\" font-style=\"{}\" fill=\"{}\" opacity=\"{:.2}\">{}</text>",
                at.x,
                at.y,
                font_family,
                size,
                weight,
                style,
                fill,
                op,
                text.replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
            )
        }
        Shape::Gradient {
            min,
            max,
            from,
            to,
            radial,
        } => {
            let lo = min.min(*max);
            let hi = min.max(*max);
            let gid = format!("grad_{idx}");
            let def = if *radial {
                format!(
                    "<radialGradient id=\"{gid}\"><stop offset=\"0%\" stop-color=\"{}\"/><stop offset=\"100%\" stop-color=\"{}\"/></radialGradient>",
                    crate::colors::hex_of(*from),
                    crate::colors::hex_of(*to)
                )
            } else {
                format!(
                    "<linearGradient id=\"{gid}\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0%\" stop-color=\"{}\"/><stop offset=\"100%\" stop-color=\"{}\"/></linearGradient>",
                    crate::colors::hex_of(*from),
                    crate::colors::hex_of(*to)
                )
            };
            format!(
                "<defs>{def}</defs><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"url(#{gid})\"/>",
                lo.x,
                lo.y,
                (hi.x - lo.x).max(0.0),
                (hi.y - lo.y).max(0.0)
            )
        }
        Shape::Bitmap { at, size, .. } => format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"#cccccc\" stroke=\"#666666\"/>",
            at.x, at.y, size.x, size.y
        ),
        Shape::Slice { .. } | Shape::SymbolInstance { .. } => String::new(),
        Shape::Group { children } => {
            let mut g = String::from("<g>");
            for (ci, ch) in children.iter().enumerate() {
                g.push_str(&obj_to_svg(ch, idx * 100 + ci));
            }
            g.push_str("</g>");
            g
        }
    }
}

/// Export the document as an HTML5 + CSS + inline SVG + Slice/Hotspot interactive web page.
pub fn export_html(doc: &Document) -> String {
    let page = doc.page();
    let svg = export_svg(doc);
    let mut hotspots = String::new();
    for layer in &page.layers {
        for obj in &layer.objects {
            if let Shape::Slice {
                min,
                max,
                url,
                alt,
                target,
                ..
            } = &obj.kind
            {
                let lo = min.min(*max);
                let hi = min.max(*max);
                let href = if url.is_empty() { "#" } else { url.as_str() };
                let tgt = if target.is_empty() {
                    "_self"
                } else {
                    target.as_str()
                };
                hotspots.push_str(&format!(
                    "    <a class=\"fw-slice\" href=\"{}\" target=\"{}\" title=\"{}\" style=\"left:{:.0}px;top:{:.0}px;width:{:.0}px;height:{:.0}px;\"></a>\n",
                    href,
                    tgt,
                    alt.replace('"', "&quot;"),
                    lo.x,
                    lo.y,
                    (hi.x - lo.x).max(1.0),
                    (hi.y - lo.y).max(1.0)
                ));
            }
        }
    }
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"utf-8\">\n  <title>{}</title>\n  <style>\n    body {{ margin: 0; background: #222; display: flex; justify-content: center; padding: 24px; font-family: sans-serif; }}\n    .fw-stage {{ position: relative; width: {:.0}px; height: {:.0}px; box-shadow: 0 8px 24px rgba(0,0,0,0.45); }}\n    .fw-slice {{ position: absolute; display: block; text-decoration: none; }}\n  </style>\n</head>\n<body>\n  <div class=\"fw-stage\">\n{}\n{}  </div>\n</body>\n</html>\n",
        doc.name, page.width, page.height, svg, hotspots
    )
}

/// Generate CSS rules for the currently selected object or all slices on the page.
pub fn generate_css_for_object(obj: &Obj) -> String {
    let mut css = format!("/* {} ({}) */\n", obj.name, obj.type_label());
    let slug = crate::page::slugify(&obj.name);
    css.push_str(&format!(".{slug} {{\n  position: absolute;\n"));
    if let Some((lo, hi)) = obj.bounds() {
        css.push_str(&format!(
            "  left: {:.0}px;\n  top: {:.0}px;\n  width: {:.0}px;\n  height: {:.0}px;\n",
            lo.x,
            lo.y,
            (hi.x - lo.x).max(0.0),
            (hi.y - lo.y).max(0.0)
        ));
    }
    if let Some(fill) = obj.fill {
        css.push_str(&format!(
            "  background-color: {};\n",
            crate::colors::hex_of(fill)
        ));
    }
    if let Some(stroke) = obj.stroke
        && obj.width > 0.0
    {
        css.push_str(&format!(
            "  border: {:.0}px solid {};\n",
            obj.width.max(1.0),
            crate::colors::hex_of(stroke)
        ));
    }
    if let Shape::Rect { corner_radius, .. } = &obj.kind
        && *corner_radius > 0.0
    {
        css.push_str(&format!("  border-radius: {:.0}px;\n", corner_radius));
    }
    if matches!(&obj.kind, Shape::Ellipse { .. }) {
        css.push_str("  border-radius: 50%;\n");
    }
    if let Shape::Gradient {
        from, to, radial, ..
    } = &obj.kind
    {
        if *radial {
            css.push_str(&format!(
                "  background-image: radial-gradient({}, {});\n",
                crate::colors::hex_of(*from),
                crate::colors::hex_of(*to)
            ));
        } else {
            css.push_str(&format!(
                "  background-image: linear-gradient(180deg, {}, {});\n",
                crate::colors::hex_of(*from),
                crate::colors::hex_of(*to)
            ));
        }
    }
    if obj.opacity < 0.999 {
        css.push_str(&format!("  opacity: {:.2};\n", obj.opacity));
    }
    for eff in &obj.effects {
        if !eff.is_enabled() {
            continue;
        }
        match eff {
            LiveEffect::DropShadow {
                distance,
                angle_deg,
                blur,
                color,
                ..
            } => {
                let rad = angle_deg.to_radians();
                let dx = distance * rad.cos();
                let dy = distance * rad.sin();
                css.push_str(&format!(
                    "  box-shadow: {:.0}px {:.0}px {:.0}px rgba({},{},{},{:.2});\n",
                    dx,
                    dy,
                    blur,
                    color.r(),
                    color.g(),
                    color.b(),
                    color.a() as f32 / 255.0
                ));
            }
            LiveEffect::Glow {
                radius,
                color,
                inner,
                ..
            } => {
                let inset = if *inner { "inset " } else { "" };
                css.push_str(&format!(
                    "  box-shadow: {}0 0 {:.0}px {};\n",
                    inset,
                    radius,
                    crate::colors::hex_of(*color)
                ));
            }
            LiveEffect::GaussianBlur { radius, .. } => {
                css.push_str(&format!("  filter: blur({:.1}px);\n", radius));
            }
            _ => {}
        }
    }
    css.push_str("}\n");
    css
}

/// Renders the document's vector and bitmap objects into an RGBA buffer for raster export.
pub fn rasterize(doc: &Document) -> image::RgbaImage {
    let page = doc.page();
    let w = page.width.max(1.0) as u32;
    let h = page.height.max(1.0) as u32;
    let bg = if page.transparent_bg {
        image::Rgba::<u8>([0, 0, 0, 0])
    } else {
        image::Rgba::<u8>([
            page.bg_color.r(),
            page.bg_color.g(),
            page.bg_color.b(),
            page.bg_color.a(),
        ])
    };
    let mut img = image::RgbaImage::from_pixel(w, h, bg);
    for layer in &page.layers {
        if !layer.visible || layer.is_web_layer {
            continue;
        }
        for obj in &layer.objects {
            if !obj.visible {
                continue;
            }
            let combined_alpha =
                ((layer.opacity * obj.opacity).clamp(0.0, 1.0) * 255.0).round() as u8;
            draw_obj(&mut img, obj, combined_alpha);
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
            let lo_x = x0.min(x1);
            let lo_y = y0.min(y1);
            let hi_x = x0.max(x1);
            let hi_y = y0.max(y1);
            for y in lo_y.max(0)..=hi_y.min(img.height() as i32 - 1) {
                for x in lo_x.max(0)..=hi_x.min(img.width() as i32 - 1) {
                    let old = img.get_pixel(x as u32, y as u32);
                    img.put_pixel(x as u32, y as u32, blend(*old, color));
                }
            }
        }
        Shape::Ellipse { center, radius, .. } => {
            let color = obj
                .fill
                .map(|c| image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]))
                .unwrap_or(image::Rgba::<u8>([0, 0, 0, 0]));
            let (cx, cy) = to_px(*center);
            let rx = radius.x.abs().max(1.0);
            let ry = radius.y.abs().max(1.0);
            for y in (cy - ry as i32).max(0)..=(cy + ry as i32).min(img.height() as i32 - 1) {
                for x in (cx - rx as i32).max(0)..=(cx + rx as i32).min(img.width() as i32 - 1) {
                    let dx = (x as f32 - cx as f32) / rx;
                    let dy = (y as f32 - cy as f32) / ry;
                    if dx * dx + dy * dy <= 1.0 {
                        let old = img.get_pixel(x as u32, y as u32);
                        img.put_pixel(x as u32, y as u32, blend(*old, color));
                    }
                }
            }
        }
        Shape::Line { a, b } => {
            if let Some(c) = obj.stroke.or(obj.fill) {
                let color = image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]);
                draw_line_px(img, to_px(*a), to_px(*b), color);
            }
        }
        Shape::Polygon {
            center,
            radius,
            sides,
            star_inner,
            angle_rad,
        } => {
            let pts = Obj::polygon_vertices(*center, *radius, *sides, *star_inner, *angle_rad);
            if let Some(c) = obj.stroke.or(obj.fill) {
                let color = image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]);
                for i in 0..pts.len() {
                    draw_line_px(img, to_px(pts[i]), to_px(pts[(i + 1) % pts.len()]), color);
                }
            }
        }
        Shape::Path { points, closed } => {
            if let Some(c) = obj.stroke.or(obj.fill) {
                let color = image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]);
                for w in points.windows(2) {
                    draw_line_px(img, to_px(w[0]), to_px(w[1]), color);
                }
                if *closed && points.len() > 2 {
                    draw_line_px(
                        img,
                        to_px(points[points.len() - 1]),
                        to_px(points[0]),
                        color,
                    );
                }
            }
        }
        Shape::Text { at, text, size, .. } => {
            if let Some(c) = obj.fill {
                let color = image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]);
                let (x, y) = to_px(*at);
                let cell = (size * 0.6).round() as i32;
                for (i, ch) in text.chars().enumerate() {
                    if ch == ' ' {
                        continue;
                    }
                    let gx = x + i as i32 * cell;
                    for gy in (y - cell)..y {
                        if gx >= 0 && gy >= 0 && gx < img.width() as i32 && gy < img.height() as i32
                        {
                            let old = img.get_pixel(gx as u32, gy as u32);
                            img.put_pixel(gx as u32, gy as u32, blend(*old, color));
                        }
                    }
                }
            }
        }
        Shape::Gradient {
            min, max, from, to, ..
        } => {
            let (x0, y0) = to_px(*min);
            let (x1, y1) = to_px(*max);
            let lo_x = x0.min(x1);
            let hi_x = x0.max(x1);
            let lo_y = y0.min(y1);
            let hi_y = y0.max(y1);
            let span = (hi_y - lo_y).max(1);
            for y in lo_y.max(0)..=hi_y.min(img.height() as i32 - 1) {
                let t = ((y - lo_y) as f32 / span as f32).clamp(0.0, 1.0);
                let c = from.lerp_to_gamma(*to, t);
                let color = image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]);
                for x in lo_x.max(0)..=hi_x.min(img.width() as i32 - 1) {
                    let old = img.get_pixel(x as u32, y as u32);
                    img.put_pixel(x as u32, y as u32, blend(*old, color));
                }
            }
        }
        Shape::Bitmap { at, pixels, size } => {
            let src = image::RgbaImage::from_raw(
                size.x.max(1.0) as u32,
                size.y.max(1.0) as u32,
                pixels.clone(),
            )
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
        Shape::Slice { .. } | Shape::SymbolInstance { .. } => {}
        Shape::Group { children } => {
            for ch in children {
                draw_obj(img, ch, alpha);
            }
        }
    }
}

fn draw_line_px(
    img: &mut image::RgbaImage,
    (x0, y0): (i32, i32),
    (x1, y1): (i32, i32),
    color: image::Rgba<u8>,
) {
    let steps = (x1 - x0).abs().max((y1 - y0).abs()).max(1);
    for i in 0..=steps {
        let x = x0 + (x1 - x0) * i / steps;
        let y = y0 + (y1 - y0) * i / steps;
        if x >= 0 && y >= 0 && x < img.width() as i32 && y < img.height() as i32 {
            let old = img.get_pixel(x as u32, y as u32);
            img.put_pixel(x as u32, y as u32, blend(*old, color));
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
            "FireCraft cannot write {} directly. Choose PNG, JPEG, WebP, TIFF, GIF, BMP, ICO, SVG or HTML.",
            format.label()
        ));
    }
    match format {
        Format::Svg => std::fs::write(path, export_svg(doc))
            .map_err(|e| format!("Could not write the file: {e}")),
        Format::Html => std::fs::write(path, export_html(doc))
            .map_err(|e| format!("Could not write the file: {e}")),
        _ => {
            let img = rasterize(doc);
            let dyn_img = image::DynamicImage::ImageRgba8(img);
            let result = match format {
                Format::Jpeg | Format::Bmp => dyn_img.to_rgb8().save(path),
                _ => dyn_img.to_rgba8().save(path),
            };
            result.map_err(|e| format!("Could not write the file: {e}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn svg_roundtrip_and_css_export() {
        let mut doc = Document::new(400.0, 300.0);
        let mut rect = Obj::new(
            Shape::Rect {
                min: Vec2::new(10.0, 15.0),
                max: Vec2::new(110.0, 95.0),
                corner_radius: 8.0,
                filled: true,
            },
            Some(Color32::from_rgb(232, 163, 61)),
            Some(Color32::from_rgb(20, 20, 20)),
        );
        rect.effects.push(LiveEffect::DropShadow {
            distance: 4.0,
            angle_deg: 45.0,
            blur: 6.0,
            color: Color32::BLACK,
            enabled: true,
        });
        let css = generate_css_for_object(&rect);
        assert!(css.contains("border-radius: 8px;"));
        assert!(css.contains("box-shadow:"));

        doc.page_mut().layers[0].objects.push(rect);
        let svg = export_svg(&doc);
        assert!(svg.contains("<rect"));
        assert!(svg.contains("rx=\"8\""));

        let parsed = parse_svg_str(&svg);
        assert_eq!(parsed.width, 400.0);
        assert!(!parsed.page().layers[0].objects.is_empty());

        let html = export_html(&doc);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("<svg"));
    }

    #[test]
    fn rasterize_produces_expected_dimensions() {
        let mut doc = Document::new(64.0, 48.0);
        doc.page_mut().layers[0].objects.push(Obj::new(
            Shape::Ellipse {
                center: Vec2::new(32.0, 24.0),
                radius: Vec2::new(12.0, 10.0),
                filled: true,
            },
            Some(Color32::from_rgb(255, 0, 0)),
            None,
        ));
        let img = rasterize(&doc);
        assert_eq!(img.dimensions(), (64, 48));
        assert_eq!(*img.get_pixel(32, 24), image::Rgba([255, 0, 0, 255]));
    }
}
