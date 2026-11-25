//! The FireCraft showcase document.
//!
//! Page-relative geometry for the object set that ships as `--demo`, the Welcome window and
//! document templates. Building it here keeps the app module free of layout arithmetic and
//! lets tests assert the invariant that matters: everything lands inside the page, and text
//! sits inside the shape that carries it.

use crate::doc::{LiveEffect, Obj, Shape};
use eframe::egui::{Color32, Vec2};

/// Build the showcase objects for a page of `w` x `h` document units, back to front.
///
/// The slice is pushed last because it belongs on the web layer; the caller is responsible
/// for inserting objects with its own layer routing.
/// Shrink `size` until `text` fits `max_w`, using the same estimate as the document model
/// (`doc::text_width`). Keeps the showcase inside its cards and page at any template size
/// instead of only at 1920x1080.
pub fn fit_text_size(text: &str, size: f32, max_w: f32) -> f32 {
    let widest = text
        .split('\n')
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1)
        .max(1) as f32;
    let ideal = 0.55 * size * widest;
    if ideal <= max_w {
        size
    } else {
        (max_w / (0.55 * widest)).clamp(8.0, size)
    }
}

pub fn showcase(w: f32, h: f32) -> Vec<Obj> {
    let mut objects = Vec::new();
    let pad = (w * 0.035).clamp(24.0, 64.0);

    // 1. Deep Studio Gradient Backdrop
    let mut backdrop = Obj::new(
        Shape::Gradient {
            min: Vec2::ZERO,
            max: Vec2::new(w, h),
            from: Color32::from_rgb(22, 24, 34),
            to: Color32::from_rgb(14, 15, 22),
            radial: true,
        },
        None,
        None,
    );
    backdrop.name = "Studio Backdrop".to_string();
    objects.push(backdrop);

    // 2. Hero Brand Card
    let hero_h = (h * 0.40).clamp(200.0, 420.0);
    let mut hero = Obj::new(
        Shape::Rect {
            min: Vec2::new(pad, pad),
            max: Vec2::new(w - pad, pad + hero_h),
            corner_radius: 16.0,
            filled: true,
        },
        Some(Color32::from_rgb(30, 34, 48)),
        Some(Color32::from_rgb(232, 163, 61)),
    );
    hero.name = "Hero Brand Card".to_string();
    hero.width = 2.0;
    hero.effects.push(LiveEffect::DropShadow {
        distance: 8.0,
        angle_deg: 90.0,
        blur: 14.0,
        color: Color32::from_black_alpha(150),
        enabled: true,
    });
    objects.push(hero);

    // 3. Stylized Multi-Layer Vector FireCraft Flame Emblem (left of hero)
    let fcx = pad + (w * 0.12).clamp(90.0, 210.0);
    let fcy = pad + hero_h * 0.52;
    let fs = (hero_h * 0.36).clamp(54.0, 140.0);
    let mut flame_outer = Obj::new(
        Shape::Path {
            points: vec![
                Vec2::new(fcx, fcy - fs),
                Vec2::new(fcx + fs * 0.42, fcy - fs * 0.35),
                Vec2::new(fcx + fs * 0.68, fcy + fs * 0.22),
                Vec2::new(fcx + fs * 0.46, fcy + fs * 0.78),
                Vec2::new(fcx, fcy + fs * 0.92),
                Vec2::new(fcx - fs * 0.48, fcy + fs * 0.78),
                Vec2::new(fcx - fs * 0.66, fcy + fs * 0.18),
                Vec2::new(fcx - fs * 0.32, fcy - fs * 0.42),
            ],
            closed: true,
        },
        Some(Color32::from_rgb(230, 74, 38)),
        Some(Color32::from_rgb(255, 190, 90)),
    );
    flame_outer.name = "FireCraft Flame Outer".to_string();
    flame_outer.width = 2.0;
    flame_outer.effects.push(LiveEffect::Glow {
        radius: 14.0,
        color: Color32::from_rgb(255, 130, 40),
        inner: false,
        enabled: true,
    });
    objects.push(flame_outer);

    let mut flame_mid = Obj::new(
        Shape::Path {
            points: vec![
                Vec2::new(fcx + fs * 0.04, fcy - fs * 0.56),
                Vec2::new(fcx + fs * 0.36, fcy + fs * 0.10),
                Vec2::new(fcx + fs * 0.28, fcy + fs * 0.64),
                Vec2::new(fcx, fcy + fs * 0.76),
                Vec2::new(fcx - fs * 0.32, fcy + fs * 0.58),
                Vec2::new(fcx - fs * 0.24, fcy + fs * 0.02),
            ],
            closed: true,
        },
        Some(Color32::from_rgb(245, 158, 42)),
        None,
    );
    flame_mid.name = "FireCraft Flame Mid".to_string();
    objects.push(flame_mid);

    let mut flame_core = Obj::new(
        Shape::Path {
            points: vec![
                Vec2::new(fcx, fcy - fs * 0.12),
                Vec2::new(fcx + fs * 0.18, fcy + fs * 0.34),
                Vec2::new(fcx, fcy + fs * 0.62),
                Vec2::new(fcx - fs * 0.18, fcy + fs * 0.34),
            ],
            closed: true,
        },
        Some(Color32::from_rgb(255, 232, 110)),
        None,
    );
    flame_core.name = "FireCraft Flame Core".to_string();
    objects.push(flame_core);

    // 4. Hero Branding Typography & Tagline
    let text_x = fcx + fs * 1.05;
    let mut title = Obj::new(
        Shape::Text {
            at: Vec2::new(text_x, pad + hero_h * 0.26),
            text: "FireCraft Studio".to_string(),
            size: (hero_h * 0.18).clamp(32.0, 64.0),
            bold: true,
            italic: false,
            font_family: "Sans-Serif".to_string(),
        },
        Some(Color32::from_rgb(250, 208, 68)),
        None,
    );
    title.name = "Hero Brand Title".to_string();
    objects.push(title);

    let mut subtitle = Obj::new(
        Shape::Text {
            at: Vec2::new(text_x, pad + hero_h * 0.50),
            text: "Open Source Bitmap & Vector Graphics Editor · CS6 Parity + MCP".to_string(),
            size: fit_text_size(
                "Open Source Bitmap & Vector Graphics Editor · CS6 Parity + MCP",
                (hero_h * 0.068).clamp(16.0, 26.0),
                (w - pad - text_x - 24.0).max(120.0),
            ),
            bold: false,
            italic: false,
            font_family: "Sans-Serif".to_string(),
        },
        Some(Color32::from_rgb(198, 206, 224)),
        None,
    );
    subtitle.name = "Hero Subtitle".to_string();
    objects.push(subtitle);

    // 5. Rounded CTA Button with Bevel & Web Slice
    let cta_w = (w * 0.16).clamp(180.0, 300.0);
    let cta_h = (hero_h * 0.16).clamp(44.0, 64.0);
    let cta_min = Vec2::new(text_x, pad + hero_h * 0.68);
    let cta_max = cta_min + Vec2::new(cta_w, cta_h);
    let mut cta = Obj::new(
        Shape::Rect {
            min: cta_min,
            max: cta_max,
            corner_radius: 10.0,
            filled: true,
        },
        Some(Color32::from_rgb(232, 163, 61)),
        Some(Color32::from_rgb(255, 224, 150)),
    );
    cta.name = "Primary CTA Button".to_string();
    cta.effects.push(LiveEffect::Bevel {
        width: 2.0,
        highlight: Color32::from_white_alpha(180),
        shadow: Color32::from_black_alpha(140),
        enabled: true,
    });
    objects.push(cta);

    let mut cta_lbl = Obj::new(
        Shape::Text {
            // Text is painted from its baseline, so centre it: half the button plus
            // roughly a third of the cap height.
            at: Vec2::new(
                cta_min.x + 28.0,
                cta_min.y + cta_h * 0.5 + (cta_h * 0.38).clamp(15.0, 22.0) * 0.36,
            ),
            text: "Explore Showcase →".to_string(),
            size: (cta_h * 0.38).clamp(15.0, 22.0),
            bold: true,
            italic: false,
            font_family: "Sans-Serif".to_string(),
        },
        Some(Color32::from_rgb(24, 20, 14)),
        None,
    );
    cta_lbl.name = "CTA Button Label".to_string();
    objects.push(cta_lbl);

    // 6. Four Modern Feature Cards across bottom half
    let cards_top = pad + hero_h + (h * 0.04).clamp(24.0, 48.0);
    let cards_bottom = (h - pad).max(cards_top + 220.0);
    let gap = (w * 0.02).clamp(16.0, 36.0);
    let card_w = ((w - pad * 2.0 - gap * 3.0) / 4.0).max(140.0);
    let features = [
        (
            "Vector + Bitmap",
            "Boolean paths, dynamic\nAuto Shapes & raster tools",
            Color32::from_rgb(66, 153, 225),
        ),
        (
            "Live CS6 Filters",
            "Drop Shadow, Glow, Bevel\n& 21 dockable panels",
            Color32::from_rgb(237, 137, 54),
        ),
        (
            "Web & Slices",
            "PNG, WebP, SVG, HTML5\n2-Up & 4-Up optimization",
            Color32::from_rgb(72, 187, 120),
        ),
        (
            "MCP & Headless CLI",
            "JSON-RPC 2.0 server,\n60+ commands & scripts",
            Color32::from_rgb(159, 122, 234),
        ),
    ];
    for (i, (heading, body, accent_col)) in features.iter().enumerate() {
        let x0 = pad + i as f32 * (card_w + gap);
        let mut card = Obj::new(
            Shape::Rect {
                min: Vec2::new(x0, cards_top),
                max: Vec2::new(x0 + card_w, cards_bottom),
                corner_radius: 12.0,
                filled: true,
            },
            Some(Color32::from_rgb(28, 32, 44)),
            Some(*accent_col),
        );
        card.name = format!("Feature Card: {heading}");
        card.width = 1.5;
        card.effects.push(LiveEffect::DropShadow {
            distance: 6.0,
            angle_deg: 45.0,
            blur: 10.0,
            color: Color32::from_black_alpha(140),
            enabled: true,
        });
        objects.push(card);

        let mut h_txt = Obj::new(
            Shape::Text {
                at: Vec2::new(x0 + 24.0, cards_top + 36.0),
                text: (*heading).to_string(),
                size: fit_text_size(heading, (card_w * 0.068).clamp(18.0, 28.0), card_w - 48.0),
                bold: true,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(*accent_col),
            None,
        );
        h_txt.name = format!("{heading} Title");
        objects.push(h_txt);

        let mut b_txt = Obj::new(
            Shape::Text {
                at: Vec2::new(x0 + 24.0, cards_top + 96.0),
                text: (*body).to_string(),
                size: fit_text_size(body, (card_w * 0.048).clamp(14.0, 20.0), card_w - 48.0),
                bold: false,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(Color32::from_rgb(190, 198, 214)),
            None,
        );
        b_txt.name = format!("{heading} Body");
        objects.push(b_txt);
    }

    // 7. Interactive Web Slice on CTA
    let slice = Obj::new(
        Shape::Slice {
            min: cta_min,
            max: cta_max,
            url: "https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment".to_string(),
            alt: "Launch FireCraft Studio".to_string(),
            target: "_blank".to_string(),
            format: "PNG32".to_string(),
            is_hotspot: false,
        },
        None,
        None,
    );
    objects.push(slice);
    objects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::Shape;

    fn bounds_of(objects: &[Obj], name: &str) -> (Obj, (Vec2, Vec2)) {
        let obj = objects
            .iter()
            .find(|o| o.name == name)
            .unwrap_or_else(|| panic!("missing object {name}"));
        let (lo, hi) = obj.bounds().expect("object has bounds");
        (obj.clone(), (lo, hi))
    }

    #[test]
    fn showcase_builds_four_feature_cards_a_hero_and_one_slice() {
        let objects = showcase(1920.0, 1080.0);
        let cards = objects
            .iter()
            .filter(|o| o.name.starts_with("Feature Card:"))
            .count();
        assert_eq!(cards, 4, "one card per feature");
        assert!(objects.iter().any(|o| o.name == "Hero Brand Card"));
        assert!(
            objects
                .iter()
                .any(|o| matches!(o.kind, Shape::Slice { .. })),
            "the showcase demonstrates a web slice"
        );
    }

    #[test]
    fn every_object_stays_inside_the_page() {
        for (w, h) in [(1920.0, 1080.0), (800.0, 600.0), (1080.0, 1080.0)] {
            let objects = showcase(w, h);
            for obj in &objects {
                let Some((lo, hi)) = obj.bounds() else {
                    continue;
                };
                assert!(
                    lo.x >= -0.5 && lo.y >= -0.5 && hi.x <= w + 0.5 && hi.y <= h + 0.5,
                    "{} escapes the {w}x{h} page: {lo:?}..{hi:?}",
                    obj.name
                );
            }
        }
    }

    #[test]
    fn feature_text_sits_inside_its_card() {
        for (w, h) in [
            (1920.0, 1080.0),
            (1280.0, 720.0),
            (800.0, 600.0),
            (1080.0, 1080.0),
        ] {
            let objects = showcase(w, h);
            for heading in [
                "Vector + Bitmap",
                "Live CS6 Filters",
                "Web & Slices",
                "MCP & Headless CLI",
            ] {
                let (_, (card_lo, card_hi)) =
                    bounds_of(&objects, &format!("Feature Card: {heading}"));
                for suffix in ["Title", "Body"] {
                    let (_, (lo, hi)) = bounds_of(&objects, &format!("{heading} {suffix}"));
                    assert!(
                        lo.x >= card_lo.x && hi.x <= card_hi.x,
                        "{w}x{h}: {heading} {suffix} overflows its card horizontally: {lo:?}..{hi:?}"
                    );
                    assert!(
                        lo.y >= card_lo.y && hi.y <= card_hi.y,
                        "{w}x{h}: {heading} {suffix} overflows its card vertically: {lo:?}..{hi:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn cta_label_is_vertically_centred_in_its_button() {
        let objects = showcase(1920.0, 1080.0);
        let (_, (btn_lo, btn_hi)) = bounds_of(&objects, "Primary CTA Button");
        let (_, (lo, hi)) = bounds_of(&objects, "CTA Button Label");
        let button_centre = (btn_lo.y + btn_hi.y) * 0.5;
        let label_centre = (lo.y + hi.y) * 0.5;
        assert!(
            (label_centre - button_centre).abs() < (btn_hi.y - btn_lo.y) * 0.25,
            "label centre {label_centre} is far from button centre {button_centre}"
        );
    }
}
