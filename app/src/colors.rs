#![allow(dead_code, unused_imports, unused_variables)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! Colour maths shared by the swatches, the colour picker and the canvas.

use eframe::egui::Color32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Hsv {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Hls {
    pub h: f32,
    pub l: f32,
    pub s: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Cmyk {
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
}

pub fn rgb_to_hsv(c: Color32) -> Hsv {
    let r = c.r() as f32 / 255.0;
    let g = c.g() as f32 / 255.0;
    let b = c.b() as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let h = if d <= f32::EPSILON {
        0.0
    } else if max == r {
        60.0 * (((g - b) / d) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max <= f32::EPSILON { 0.0 } else { d / max };
    Hsv { h, s, v: max }
}

pub fn hsv_to_rgb(hsv: Hsv) -> Color32 {
    let h = hsv.h.rem_euclid(360.0);
    let c = hsv.v * hsv.s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = hsv.v - c;
    let (r, g, b) = match h as u32 / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color32::from_rgb(
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

pub fn rgb_to_hls(c: Color32) -> Hls {
    let r = c.r() as f32 / 255.0;
    let g = c.g() as f32 / 255.0;
    let b = c.b() as f32 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    let d = max - min;
    if d <= f32::EPSILON {
        return Hls { h: 0.0, l, s: 0.0 };
    }
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let h = if max == r {
        60.0 * (((g - b) / d) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    let h = if h < 0.0 { h + 360.0 } else { h };
    Hls { h, l, s }
}

pub fn hls_to_rgb(hls: Hls) -> Color32 {
    let h = hls.h.rem_euclid(360.0);
    let c = (1.0 - (2.0 * hls.l - 1.0).abs()) * hls.s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = hls.l - c / 2.0;
    let (r, g, b) = match h as u32 / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color32::from_rgb(
        ((r + m) * 255.0).round() as u8,
        ((g + m) * 255.0).round() as u8,
        ((b + m) * 255.0).round() as u8,
    )
}

pub fn rgb_to_cmyk(c: Color32) -> Cmyk {
    let r = c.r() as f32 / 255.0;
    let g = c.g() as f32 / 255.0;
    let b = c.b() as f32 / 255.0;
    let k = 1.0 - r.max(g).max(b);
    if (1.0 - k) <= f32::EPSILON {
        return Cmyk { c: 0.0, m: 0.0, y: 0.0, k: 1.0 };
    }
    Cmyk {
        c: (1.0 - r - k) / (1.0 - k),
        m: (1.0 - g - k) / (1.0 - k),
        y: (1.0 - b - k) / (1.0 - k),
        k,
    }
}

pub fn cmyk_to_rgb(v: Cmyk) -> Color32 {
    Color32::from_rgb(
        (255.0 * (1.0 - v.k) * (1.0 - v.c)).round() as u8,
        (255.0 * (1.0 - v.k) * (1.0 - v.m)).round() as u8,
        (255.0 * (1.0 - v.k) * (1.0 - v.y)).round() as u8,
    )
}

pub fn hex_of(c: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b())
}

pub fn from_hex(text: &str) -> Option<Color32> {
    let t = text.trim().trim_start_matches('#');
    if t.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&t[0..2], 16).ok()?;
    let g = u8::from_str_radix(&t[2..4], 16).ok()?;
    let b = u8::from_str_radix(&t[4..6], 16).ok()?;
    Some(Color32::from_rgb(r, g, b))
}

/// The default Fireworks-like swatch ramp: 16 hues x 9 lightness/saturation steps.
pub fn swatch_grid() -> Vec<Color32> {
    let mut out = Vec::new();
    for row in 0..9 {
        let v = 1.0 - row as f32 * 0.09;
        for col in 0..16 {
            let h = col as f32 * 22.5;
            let s = if row == 8 { 0.0 } else { 0.85 };
            out.push(hsv_to_rgb(Hsv { h, s, v }));
        }
    }
    out
}


