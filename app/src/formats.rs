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
    let bytes = std::fs::read(path).map_err(|e| format!("Could not read the file: {e}"))?;
    if let Some(payload) = extract_firecraft_payload(&bytes)
        && let Ok(mut doc) = serde_json::from_slice::<Document>(&payload)
    {
        doc.file_path = Some(path.to_path_buf());
        doc.dirty = false;
        doc.name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&doc.name)
            .to_string();
        doc.current_page = doc.current_page.min(doc.pages.len().saturating_sub(1));
        return Ok(doc);
    }
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
            pixel_size: Vec2::new(w as f32, h as f32),
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HtmlStyle {
    Html5,
    Xhtml1,
    CssGrid,
}

#[derive(Clone, Debug)]
pub struct HtmlExportOptions {
    pub style: HtmlStyle,
    pub extension: &'static str,
    pub include_slices: bool,
    pub center_in_browser: bool,
    pub include_viewport_meta: bool,
    pub export_doc_bg: bool,
}

impl Default for HtmlExportOptions {
    fn default() -> Self {
        Self {
            style: HtmlStyle::Html5,
            extension: ".html",
            include_slices: true,
            center_in_browser: true,
            include_viewport_meta: true,
            export_doc_bg: true,
        }
    }
}

pub fn export_html(doc: &Document) -> String {
    export_html_with_options(doc, &HtmlExportOptions::default())
}

pub fn export_html_with_options(doc: &Document, opts: &HtmlExportOptions) -> String {
    let page = doc.page();
    let svg = export_svg(doc);
    let mut hotspots = String::new();
    if opts.include_slices {
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
    }
    let doctype = match opts.style {
        HtmlStyle::Xhtml1 => {
            "<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">"
        }
        HtmlStyle::Html5 | HtmlStyle::CssGrid => "<!DOCTYPE html>",
    };
    let viewport = if opts.include_viewport_meta {
        "\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
    } else {
        ""
    };
    let body_bg = if opts.export_doc_bg && !page.transparent_bg {
        format!(
            "rgb({},{},{})",
            page.bg_color.r(),
            page.bg_color.g(),
            page.bg_color.b()
        )
    } else {
        "#222".to_string()
    };
    let body_layout = if opts.center_in_browser {
        "display: flex; justify-content: center; padding: 24px;"
    } else {
        "padding: 12px;"
    };
    let stage_display = match opts.style {
        HtmlStyle::CssGrid => "display: grid; place-items: center;",
        HtmlStyle::Html5 | HtmlStyle::Xhtml1 => "",
    };
    format!(
        "{doctype}\n<html lang=\"en\">\n<head>\n  <meta charset=\"utf-8\">{viewport}\n  <title>{}</title>\n  <style>\n    body {{ margin: 0; background: {body_bg}; {body_layout} font-family: sans-serif; }}\n    .fw-stage {{ position: relative; {stage_display} width: {:.0}px; height: {:.0}px; box-shadow: 0 8px 24px rgba(0,0,0,0.45); }}\n    .fw-slice {{ position: absolute; display: block; text-decoration: none; }}\n  </style>\n</head>\n<body>\n  <div class=\"fw-stage\">\n{}\n{}  </div>\n</body>\n</html>\n",
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
            let lo_x = x0.min(x1);
            let lo_y = y0.min(y1);
            let hi_x = x0.max(x1);
            let hi_y = y0.max(y1);
            let span = (hi_y - lo_y).max(1) as f32;
            for y in lo_y.max(0)..=hi_y.min(img.height() as i32 - 1) {
                let t = ((y - lo_y) as f32 / span).clamp(0.0, 1.0);
                let color = match obj.gradient {
                    Some(g) => {
                        let c = g.start.lerp_to_gamma(g.end, t);
                        image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha])
                    }
                    None => obj
                        .fill
                        .map(|c| image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]))
                        .unwrap_or(image::Rgba::<u8>([0, 0, 0, 0])),
                };
                for x in lo_x.max(0)..=hi_x.min(img.width() as i32 - 1) {
                    let old = img.get_pixel(x as u32, y as u32);
                    img.put_pixel(x as u32, y as u32, blend(*old, color));
                }
            }
        }
        Shape::Ellipse { center, radius, .. } => {
            let (cx, cy) = to_px(*center);
            let rx = radius.x.abs().max(1.0);
            let ry = radius.y.abs().max(1.0);
            let top = cy - ry as i32;
            let span = (ry * 2.0).max(1.0);
            for y in (cy - ry as i32).max(0)..=(cy + ry as i32).min(img.height() as i32 - 1) {
                let t = ((y - top) as f32 / span).clamp(0.0, 1.0);
                let color = match obj.gradient {
                    Some(g) => {
                        let c = g.start.lerp_to_gamma(g.end, t);
                        image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha])
                    }
                    None => obj
                        .fill
                        .map(|c| image::Rgba::<u8>([c.r(), c.g(), c.b(), alpha]))
                        .unwrap_or(image::Rgba::<u8>([0, 0, 0, 0])),
                };
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
                let (bx, by) = to_px(*at);
                let px_scale = (size / 7.0).max(1.0);
                let adv = (6.0 * px_scale).round() as i32;
                let top_y = by - (7.0 * px_scale).round() as i32;
                for (ci, ch) in text.chars().enumerate() {
                    let rows = glyph_5x7(ch);
                    let gx0 = bx + ci as i32 * adv;
                    for (ry, row_bits) in rows.iter().enumerate() {
                        for rx in 0..5 {
                            if (row_bits >> (4 - rx)) & 1 == 1 {
                                let x0 = gx0 + (rx as f32 * px_scale).floor() as i32;
                                let y0 = top_y + (ry as f32 * px_scale).floor() as i32;
                                let x1 = gx0 + ((rx + 1) as f32 * px_scale).ceil() as i32;
                                let y1 = top_y + ((ry + 1) as f32 * px_scale).ceil() as i32;
                                for py in y0..y1 {
                                    for px in x0..x1 {
                                        if px >= 0
                                            && py >= 0
                                            && px < img.width() as i32
                                            && py < img.height() as i32
                                        {
                                            let old = img.get_pixel(px as u32, py as u32);
                                            img.put_pixel(px as u32, py as u32, blend(*old, color));
                                        }
                                    }
                                }
                            }
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
        Shape::Bitmap {
            at,
            pixels,
            size,
            pixel_size,
        } => {
            let source_size = if pixel_size.x >= 1.0 && pixel_size.y >= 1.0 {
                *pixel_size
            } else {
                *size
            };
            let src = image::RgbaImage::from_raw(
                source_size.x.max(1.0) as u32,
                source_size.y.max(1.0) as u32,
                pixels.clone(),
            )
            .unwrap_or_else(|| image::RgbaImage::new(1, 1));
            let display_size = (
                size.x.max(1.0).round() as u32,
                size.y.max(1.0).round() as u32,
            );
            let rendered = if src.dimensions() == display_size {
                src
            } else {
                image::imageops::resize(
                    &src,
                    display_size.0,
                    display_size.1,
                    image::imageops::FilterType::Triangle,
                )
            };
            let (ox, oy) = to_px(*at);
            for y in 0..rendered.height() as i32 {
                for x in 0..rendered.width() as i32 {
                    let px = ox + x;
                    let py = oy + y;
                    if px >= 0 && py >= 0 && px < img.width() as i32 && py < img.height() as i32 {
                        let mut p = *rendered.get_pixel(x as u32, y as u32);
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
            // ICO entries are limited to 256x256; scale down instead of failing on a normal page.
            let img = if format == Format::Ico && (img.width() > 256 || img.height() > 256) {
                let scale = (256.0 / img.width() as f32).min(256.0 / img.height() as f32);
                let w = ((img.width() as f32 * scale).round() as u32).max(1);
                let h = ((img.height() as f32 * scale).round() as u32).max(1);
                image::imageops::resize(&img, w, h, image::imageops::FilterType::Lanczos3)
            } else {
                img
            };
            let dyn_img = image::DynamicImage::ImageRgba8(img);
            let result = match format {
                Format::Jpeg | Format::Bmp => dyn_img.to_rgb8().save(path),
                _ => dyn_img.to_rgba8().save(path),
            };
            result.map_err(|e| format!("Could not write the file: {e}"))?;
            // A `.fw.png` is a normal page-bounded PNG plus an embedded, versioned project
            // payload. Reopening it restores layers, states, symbols, styles, and objects that
            // sit outside the visible page bounds.
            let is_layered_png = path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.to_ascii_lowercase().ends_with(".fw.png"));
            if is_layered_png && format == Format::Png {
                let json = serde_json::to_vec(doc)
                    .map_err(|e| format!("Could not encode layered project: {e}"))?;
                embed_firecraft_payload(path, &json)?;
            }
            Ok(())
        }
    }
}

const FIRECRAFT_PNG_KEYWORD: &str = "FireCraft";

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in bytes {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// Appends an iTXt chunk before IEND without disturbing the image data.
fn embed_firecraft_payload(path: &Path, payload: &[u8]) -> Result<(), String> {
    let png = std::fs::read(path).map_err(|e| format!("Could not re-read PNG: {e}"))?;
    if png.len() < 8 || &png[..8] != b"\x89PNG\r\n\x1a\n" {
        return Err("Layered save expected a PNG file".to_string());
    }
    let mut data = Vec::new();
    data.extend_from_slice(FIRECRAFT_PNG_KEYWORD.as_bytes());
    data.push(0); // keyword terminator
    data.push(0); // compression flag: uncompressed
    data.push(0); // compression method
    data.push(0); // language tag terminator
    data.push(0); // translated keyword terminator
    data.extend_from_slice(payload);

    let mut chunk = Vec::with_capacity(data.len() + 12);
    chunk.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let mut crc_input = Vec::with_capacity(data.len() + 4);
    crc_input.extend_from_slice(b"iTXt");
    crc_input.extend_from_slice(&data);
    chunk.extend_from_slice(&crc_input);
    chunk.extend_from_slice(&crc32(&crc_input).to_be_bytes());

    let mut iend_offset = None;
    let mut off = 8usize;
    while off + 12 <= png.len() {
        let len = u32::from_be_bytes([png[off], png[off + 1], png[off + 2], png[off + 3]]) as usize;
        let ty = &png[off + 4..off + 8];
        if ty == b"IEND" {
            iend_offset = Some(off);
            break;
        }
        let next = off.saturating_add(12).saturating_add(len);
        if next <= off || next > png.len() {
            break;
        }
        off = next;
    }
    let insert_at = iend_offset.ok_or("PNG is missing its IEND chunk")?;
    let mut out = Vec::with_capacity(png.len() + chunk.len());
    out.extend_from_slice(&png[..insert_at]);
    out.extend_from_slice(&chunk);
    out.extend_from_slice(&png[insert_at..]);
    std::fs::write(path, out).map_err(|e| format!("Could not write layered PNG: {e}"))
}

fn extract_firecraft_payload(png: &[u8]) -> Option<Vec<u8>> {
    if png.len() < 8 || &png[..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let mut off = 8usize;
    while off + 12 <= png.len() {
        let len = u32::from_be_bytes([png[off], png[off + 1], png[off + 2], png[off + 3]]) as usize;
        if off + 12 + len > png.len() {
            break;
        }
        let ty = &png[off + 4..off + 8];
        let data = &png[off + 8..off + 8 + len];
        if (ty == b"iTXt" || ty == b"tEXt")
            && let Some(key_end) = data.iter().position(|&b| b == 0)
            && &data[..key_end] == FIRECRAFT_PNG_KEYWORD.as_bytes()
        {
            if ty == b"tEXt" {
                return Some(data[key_end + 1..].to_vec());
            }
            let mut p = key_end + 1;
            if p + 2 > data.len() {
                return None;
            }
            let compression_flag = data[p];
            p += 2;
            let mut ok = true;
            for _ in 0..2 {
                if let Some(end) = data[p..].iter().position(|&b| b == 0) {
                    p += end + 1;
                } else {
                    ok = false;
                    break;
                }
            }
            if ok && compression_flag == 0 {
                return Some(data[p..].to_vec());
            }
        }
        off = off.saturating_add(12).saturating_add(len);
    }
    None
}

pub fn rasterize_selection(objs: &[Obj], lo: Vec2, hi: Vec2) -> image::RgbaImage {
    let w = (hi.x - lo.x).ceil().max(4.0) as u32;
    let h = (hi.y - lo.y).ceil().max(4.0) as u32;
    let mut img = image::RgbaImage::from_pixel(w, h, image::Rgba([0, 0, 0, 0]));
    for obj in objs {
        let mut local = obj.clone();
        local.translate(-lo);
        let alpha = (local.opacity * 255.0).round().clamp(0.0, 255.0) as u8;
        draw_obj(&mut img, &local, alpha);
    }
    img
}

fn glyph_5x7(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        ' ' => [0, 0, 0, 0, 0, 0, 0],
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10011, 0b10101, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111,
        ],
        '3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
        ],
        '6' => [
            0b01110, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        '!' => [
            0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100,
        ],
        '.' => [
            0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100,
        ],
        '-' => [
            0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000,
        ],
        _ => [
            0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Golden-file comparison. Normalization is limited to line endings and trailing whitespace,
    /// so real output changes still fail. Regenerate intentionally with `UPDATE_GOLDEN=1 cargo test`.
    fn normalized(text: &str) -> String {
        let mut out: String = text
            .lines()
            .map(str::trim_end)
            .collect::<Vec<_>>()
            .join("\n");
        out.push('\n');
        out
    }

    fn golden_doc() -> Document {
        let mut doc = Document::new(64.0, 48.0);
        let rect = Obj::new(
            Shape::Rect {
                min: Vec2::new(4.0, 4.0),
                max: Vec2::new(60.0, 44.0),
                corner_radius: 3.0,
                filled: true,
            },
            Some(Color32::from_rgb(232, 163, 61)),
            Some(Color32::from_rgb(20, 20, 20)),
        );
        doc.page_mut().layers[0].objects.push(rect);
        doc
    }

    fn assert_golden(name: &str, actual: &str) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/golden")
            .join(name);
        if std::env::var_os("UPDATE_GOLDEN").is_some() {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, normalized(actual)).unwrap();
        }
        let expected = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "golden {} unreadable ({e}); set UPDATE_GOLDEN=1",
                path.display()
            )
        });
        assert_eq!(
            normalized(actual),
            normalized(&expected),
            "{} drifted",
            path.display()
        );
    }

    #[test]
    fn svg_and_html_output_match_their_golden_files() {
        let doc = golden_doc();
        assert_golden("simple.svg", &export_svg(&doc));
        assert_golden("simple.html", &export_html(&doc));
    }

    #[test]
    fn png_output_matches_the_golden_file() {
        let dir = std::env::temp_dir().join(format!("firecraft-golden-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let out = dir.join("simple.png");
        save(&golden_doc(), &out).expect("save png");
        let actual = std::fs::read(&out).unwrap();
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/simple.png");
        if std::env::var_os("UPDATE_GOLDEN").is_some() {
            std::fs::write(&path, &actual).unwrap();
        }
        let expected = std::fs::read(&path).unwrap_or_else(|e| {
            panic!(
                "golden {} unreadable ({e}); set UPDATE_GOLDEN=1",
                path.display()
            )
        });
        assert_eq!(
            actual, expected,
            "PNG encoding drifted; regenerate intentionally"
        );
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn ico_export_scales_a_full_page_down_to_256() {
        let doc = Document::new(1920.0, 1080.0);
        let path = std::env::temp_dir().join(format!("firecraft-ico-{}.ico", std::process::id()));
        save(&doc, &path).expect("ico export of a full page");
        let img = image::open(&path).expect("read back the ico");
        assert_eq!(img.width(), 256, "ico entries cap at 256 px");
        assert!(img.height() <= 256);
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn exporting_an_unwriteable_format_explains_the_choice() {
        let doc = Document::new(100.0, 100.0);
        let err = save(&doc, Path::new("/tmp/firecraft-x.avif")).unwrap_err();
        assert!(err.contains("cannot write AVIF"), "{err}");
        assert!(
            err.contains("PNG"),
            "the error should name a working format: {err}"
        );
    }

    #[test]
    fn saving_into_a_missing_directory_reports_the_io_error() {
        let doc = Document::new(10.0, 10.0);
        let err = save(&doc, Path::new("/nonexistent-firecraft-dir/out.png")).unwrap_err();
        assert!(err.starts_with("Could not write the file:"), "{err}");
    }

    #[test]
    fn opening_a_corrupt_png_is_an_error_not_a_panic() {
        let path =
            std::env::temp_dir().join(format!("firecraft-corrupt-{}.png", std::process::id()));
        std::fs::write(&path, b"not a png").unwrap();
        let err = open(&path).unwrap_err();
        assert!(!err.is_empty());
        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn svg_and_html_escape_user_text() {
        let mut doc = Document::new(120.0, 80.0);
        let mut obj = Obj::new(
            Shape::Text {
                at: Vec2::new(8.0, 8.0),
                text: "<b>& \"quoted\"</b>".to_string(),
                size: 12.0,
                bold: false,
                italic: false,
                font_family: "Inter".to_string(),
            },
            Some(Color32::WHITE),
            None,
        );
        obj.name = "A & B <strip>".to_string();
        doc.page_mut().layers[0].objects.push(obj);

        let svg = export_svg(&doc);
        assert!(
            svg.contains("&lt;b&gt;&amp;"),
            "svg text is not escaped: {svg:.400}"
        );
        let html = export_html(&doc);
        assert!(
            html.contains("&lt;b&gt;&amp;"),
            "html text is not escaped: {html:.400}"
        );
    }

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
    fn layered_fw_png_roundtrip_preserves_out_of_page_objects() {
        let mut doc = Document::new(64.0, 48.0);
        doc.page_mut().layers[0].objects.push(Obj::new(
            Shape::Rect {
                min: Vec2::new(200.0, 200.0),
                max: Vec2::new(260.0, 240.0),
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::from_rgb(12, 34, 56)),
            None,
        ));
        doc.file_path = None;
        let path = std::env::temp_dir().join(format!(
            "firecraft_layered_test_{}.fw.png",
            std::process::id()
        ));
        save(&doc, &path).expect("layered save");
        let loaded = open(&path).expect("layered open");
        assert_eq!(loaded.page().layers[0].objects.len(), 1);
        assert_eq!(loaded.page().layers.len(), doc.page().layers.len());
        let obj = &loaded.page().layers[0].objects[0];
        let (lo, _) = obj.bounds().expect("bounds");
        assert!(lo.x >= 200.0);
        let _ = std::fs::remove_file(path);
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

    #[test]
    fn raster_export_respects_scaled_bitmap_display_bounds() {
        let mut doc = Document::new(4.0, 2.0);
        doc.page_mut().transparent_bg = true;
        let mut bitmap = Obj::new(
            Shape::Bitmap {
                at: Vec2::ZERO,
                pixels: vec![255, 0, 0, 255, 0, 0, 255, 255],
                size: Vec2::new(2.0, 1.0),
                pixel_size: Vec2::new(2.0, 1.0),
            },
            None,
            None,
        );
        bitmap.set_bounds(Vec2::ZERO, Vec2::new(4.0, 2.0));
        doc.page_mut().layers[0].objects.push(bitmap);

        let image = rasterize(&doc);

        assert_eq!(image.dimensions(), (4, 2));
        assert_ne!(*image.get_pixel(3, 1), image::Rgba([0, 0, 0, 0]));
    }
}
