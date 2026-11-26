//! Colour maths shared by the swatches, the colour picker and the canvas.

use crate::constants;
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
    } else if (max - r).abs() <= f32::EPSILON {
        60.0 * (((g - b) / d) % 6.0)
    } else if (max - g).abs() <= f32::EPSILON {
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
    let s = hsv.s.clamp(0.0, 1.0);
    let v = hsv.v.clamp(0.0, 1.0);
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;
    let (r, g, b) = match h as u32 / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color32::from_rgb(
        ((r + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b + m) * 255.0).round().clamp(0.0, 255.0) as u8,
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
    let h = if (max - r).abs() <= f32::EPSILON {
        60.0 * (((g - b) / d) % 6.0)
    } else if (max - g).abs() <= f32::EPSILON {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    let h = if h < 0.0 { h + 360.0 } else { h };
    Hls { h, l, s }
}

pub fn hls_to_rgb(hls: Hls) -> Color32 {
    let h = hls.h.rem_euclid(360.0);
    let l = hls.l.clamp(0.0, 1.0);
    let s = hls.s.clamp(0.0, 1.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match h as u32 / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    Color32::from_rgb(
        ((r + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

pub fn rgb_to_cmyk(c: Color32) -> Cmyk {
    let r = c.r() as f32 / 255.0;
    let g = c.g() as f32 / 255.0;
    let b = c.b() as f32 / 255.0;
    let k = 1.0 - r.max(g).max(b);
    if (1.0 - k) <= f32::EPSILON {
        return Cmyk {
            c: 0.0,
            m: 0.0,
            y: 0.0,
            k: 1.0,
        };
    }
    Cmyk {
        c: (1.0 - r - k) / (1.0 - k),
        m: (1.0 - g - k) / (1.0 - k),
        y: (1.0 - b - k) / (1.0 - k),
        k,
    }
}

pub fn cmyk_to_rgb(v: Cmyk) -> Color32 {
    let c = v.c.clamp(0.0, 1.0);
    let m = v.m.clamp(0.0, 1.0);
    let y = v.y.clamp(0.0, 1.0);
    let k = v.k.clamp(0.0, 1.0);
    Color32::from_rgb(
        (255.0 * (1.0 - k) * (1.0 - c)).round().clamp(0.0, 255.0) as u8,
        (255.0 * (1.0 - k) * (1.0 - m)).round().clamp(0.0, 255.0) as u8,
        (255.0 * (1.0 - k) * (1.0 - y)).round().clamp(0.0, 255.0) as u8,
    )
}

pub fn hex_of(c: Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", c.r(), c.g(), c.b())
}

pub fn from_hex(text: &str) -> Option<Color32> {
    let t = text.trim().trim_start_matches('#');
    if t.len() == 3 {
        let r = u8::from_str_radix(&t[0..1], 16).ok()?;
        let g = u8::from_str_radix(&t[1..2], 16).ok()?;
        let b = u8::from_str_radix(&t[2..3], 16).ok()?;
        return Some(Color32::from_rgb(r * 17, g * 17, b * 17));
    }
    if t.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&t[0..2], 16).ok()?;
    let g = u8::from_str_radix(&t[2..4], 16).ok()?;
    let b = u8::from_str_radix(&t[4..6], 16).ok()?;
    Some(Color32::from_rgb(r, g, b))
}

/// The default Fireworks CS6 Color Cubes swatch ramp (21 columns × 12 rows = 252 swatches):
/// Col 0 is the 12-step grayscale + primary ramp, followed by the 6×6×6 web-safe cube
/// laid out in three 6-column blocks across 12 rows.
pub fn swatch_grid() -> Vec<Color32> {
    let mut out = Vec::with_capacity(constants::SWATCH_COUNT);
    let levels = [0u8, 51, 102, 153, 204, 255];
    let prim = [
        Color32::from_rgb(0, 0, 0),
        Color32::from_rgb(51, 51, 51),
        Color32::from_rgb(102, 102, 102),
        Color32::from_rgb(153, 153, 153),
        Color32::from_rgb(204, 204, 204),
        Color32::from_rgb(255, 255, 255),
        Color32::from_rgb(255, 0, 0),
        Color32::from_rgb(0, 255, 0),
        Color32::from_rgb(0, 0, 255),
        Color32::from_rgb(255, 255, 0),
        Color32::from_rgb(0, 255, 255),
        Color32::from_rgb(255, 0, 255),
    ];
    for row in 0..constants::SWATCH_ROWS {
        out.push(prim[row % prim.len()]);
        out.push(Color32::from_rgb(0, 0, 0));
        for block in 0..3 {
            let r_idx = (row / 6) * 3 + block;
            let g_idx = row % 6;
            for &b in &levels {
                out.push(Color32::from_rgb(levels[r_idx], levels[g_idx], b));
            }
        }
        out.push(Color32::from_rgb(255, 255, 255));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_roundtrip() {
        let c = Color32::from_rgb(0x99, 0x00, 0x41);
        assert_eq!(hex_of(c), "#990041");
        assert_eq!(from_hex("#990041"), Some(c));
        assert_eq!(from_hex("990041"), Some(c));
        assert_eq!(from_hex("#f0a"), Some(Color32::from_rgb(0xFF, 0x00, 0xAA)));
        assert_eq!(from_hex("invalid"), None);
    }

    #[test]
    fn hex_parser_rejects_bad_digits_in_every_channel() {
        for text in ["g00", "0g0", "00g", "gg0000", "00gg00", "0000gg"] {
            assert_eq!(from_hex(text), None, "accepted invalid color {text}");
        }
    }

    #[test]
    fn hsv_hls_cmyk_roundtrip() {
        for &c in &[
            Color32::from_rgb(255, 0, 0),
            Color32::from_rgb(0, 180, 90),
            Color32::from_rgb(65, 139, 101),
            Color32::from_rgb(0, 0, 0),
            Color32::from_rgb(255, 255, 255),
        ] {
            let hsv = rgb_to_hsv(c);
            let back = hsv_to_rgb(hsv);
            assert!((c.r() as i16 - back.r() as i16).abs() <= 1);
            assert!((c.g() as i16 - back.g() as i16).abs() <= 1);
            assert!((c.b() as i16 - back.b() as i16).abs() <= 1);

            let hls = rgb_to_hls(c);
            let back_hls = hls_to_rgb(hls);
            assert!((c.r() as i16 - back_hls.r() as i16).abs() <= 1);
            assert!((c.g() as i16 - back_hls.g() as i16).abs() <= 1);
            assert!((c.b() as i16 - back_hls.b() as i16).abs() <= 1);

            let cmyk = rgb_to_cmyk(c);
            let back_cmyk = cmyk_to_rgb(cmyk);
            assert!((c.r() as i16 - back_cmyk.r() as i16).abs() <= 1);
            assert!((c.g() as i16 - back_cmyk.g() as i16).abs() <= 1);
            assert!((c.b() as i16 - back_cmyk.b() as i16).abs() <= 1);
        }
    }

    #[test]
    fn every_channel_combination_roundtrips_through_each_model() {
        // Deterministic sweep: all 8 corner colours plus a stepped lattice.
        let mut colors = vec![
            Color32::from_rgb(0, 0, 0),
            Color32::from_rgb(255, 255, 255),
            Color32::from_rgb(255, 0, 0),
            Color32::from_rgb(0, 255, 0),
            Color32::from_rgb(0, 0, 255),
        ];
        for r in (0..=255).step_by(51) {
            for g in (0..=255).step_by(51) {
                for b in (0..=255).step_by(51) {
                    colors.push(Color32::from_rgb(r, g, b));
                }
            }
        }
        for c in colors {
            for (name, back) in [
                ("hsv", hsv_to_rgb(rgb_to_hsv(c))),
                ("hls", hls_to_rgb(rgb_to_hls(c))),
                ("cmyk", cmyk_to_rgb(rgb_to_cmyk(c))),
            ] {
                let delta = (c.r() as i16 - back.r() as i16)
                    .abs()
                    .max((c.g() as i16 - back.g() as i16).abs())
                    .max((c.b() as i16 - back.b() as i16).abs());
                assert!(
                    delta <= 2,
                    "{name} roundtrip of {c:?} drifted by {delta} ({back:?})"
                );
            }
        }
    }

    #[test]
    fn swatch_grid_dimensions() {
        let grid = swatch_grid();
        assert_eq!(grid.len(), constants::SWATCH_COUNT);
    }
}
