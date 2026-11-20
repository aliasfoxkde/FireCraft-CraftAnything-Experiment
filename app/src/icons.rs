#![allow(dead_code, unused_imports, unused_variables)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! Small line-art icons drawn with the painter, so the app needs no icon font.

use eframe::egui::{Color32, Painter, Pos2, Rect, Stroke, Vec2, pos2};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Icon {
    New,
    Open,
    Save,
    Import,
    Export,
    Print,
    Cut,
    Copy,
    Paste,
    Duplicate,
    Undo,
    Redo,
    Select,
    Marquee,
    Lasso,
    Wand,
    Crop,
    Hand,
    Zoom,
    Brush,
    Pencil,
    Eraser,
    Line,
    Rectangle,
    Ellipse,
    Polygon,
    Text,
    Picker,
    Fill,
    Gradient,
    Blur,
    Sharpen,
    Dodge,
    Burn,
    Trim,
    Split,
    Distort,
    Rounded,
    Frame,
    Connect,
    Eyedropper,
    ZoomIn,
    ZoomOut,
    Fit,
    Sync,
    Info,
    Check,
}

fn stroke_of(c: Color32, w: f32) -> Stroke {
    Stroke::new(w, c)
}

/// Points along a circular arc, so arcs can be drawn without an arc primitive.
fn arc_points(center: Pos2, radius: f32, start: f32, end: f32, steps: usize) -> Vec<Pos2> {
    (0..=steps)
        .map(|i| {
            let t = start + (end - start) * i as f32 / steps as f32;
            pos2(
                center.x + radius * t.cos(),
                center.y + radius * t.sin(),
            )
        })
        .collect()
}

/// Draws `icon` inside `rect` (a square button face).
pub fn draw(painter: &Painter, rect: Rect, icon: Icon, color: Color32, width: f32) {
    let r = rect.shrink(3.0);
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height());
    let u = s * 0.5; // half size
    let st = stroke_of(color, width.max(1.0));
    let dot = |p: Pos2| painter.circle_filled(p, 1.4, color);
    match icon {
        Icon::New => {
            painter.rect_stroke(Rect::from_center_size(r.center(), Vec2::new(s * 0.8, s * 0.62)), 1.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
            painter.line_segment(
                [
                    pos2(cx - s * 0.3, cy - s * 0.12),
                    pos2(cx + s * 0.05, cy - s * 0.3),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx + s * 0.05, cy - s * 0.3),
                    pos2(cx + s * 0.05, cy + s * 0.1),
                ],
                st,
            );
        }
        Icon::Open => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.9, cy + u * 0.6),
                    pos2(cx - u * 0.9, cy - u * 0.5),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx - u * 0.9, cy - u * 0.5),
                    pos2(cx - u * 0.2, cy - u * 0.5),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx - u * 0.2, cy - u * 0.5),
                    pos2(cx - u * 0.05, cy - u * 0.2),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx - u * 0.05, cy - u * 0.2),
                    pos2(cx + u * 0.9, cy - u * 0.2),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx + u * 0.9, cy - u * 0.2),
                    pos2(cx + u * 0.9, cy + u * 0.7),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx + u * 0.9, cy + u * 0.7),
                    pos2(cx - u * 0.9, cy + u * 0.7),
                ],
                st,
            );
        }
        Icon::Save => {
            painter.rect_stroke(r, 1.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
            painter.rect_filled(
                Rect::from_min_size(
                    pos2(cx - u * 0.45, cy - u * 0.85),
                    Vec2::new(u * 0.9, u * 0.6),
                ),
                0.0,
                color,
            );
        }
        Icon::Import => {
            painter.line_segment(
                [pos2(cx, cy - u * 0.8), pos2(cx, cy + u * 0.5)],
                st,
            );
            painter.line_segment(
                [pos2(cx - u * 0.4, cy + u * 0.15), pos2(cx, cy + u * 0.6)],
                st,
            );
            painter.line_segment(
                [pos2(cx + u * 0.4, cy + u * 0.15), pos2(cx, cy + u * 0.6)],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx - u * 0.8, cy + u * 0.8),
                    pos2(cx + u * 0.8, cy + u * 0.8),
                ],
                st,
            );
        }
        Icon::Export => {
            painter.line_segment(
                [pos2(cx, cy + u * 0.8), pos2(cx, cy - u * 0.5)],
                st,
            );
            painter.line_segment(
                [pos2(cx - u * 0.4, cy - u * 0.15), pos2(cx, cy - u * 0.55)],
                st,
            );
            painter.line_segment(
                [pos2(cx + u * 0.4, cy - u * 0.15), pos2(cx, cy - u * 0.55)],
                st,
            );
        }
        Icon::Print => {
            painter.rect_stroke(Rect::from_center_size(pos2(cx, cy + u * 0.35), Vec2::new(u * 1.5, u * 0.9)), 1.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
            painter.rect_stroke(Rect::from_center_size(pos2(cx, cy - u * 0.4), Vec2::new(u * 1.2, u * 0.8)), 1.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
        }
        Icon::Cut | Icon::Copy | Icon::Paste => {
            painter.rect_stroke(Rect::from_center_size(pos2(cx, cy + u * 0.3), Vec2::new(u * 1.2, u * 1.2)), 1.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
            if icon == Icon::Paste {
                painter.rect_filled(
                    Rect::from_center_size(pos2(cx, cy - u * 0.55), Vec2::new(u * 1.0, u * 0.4)),
                    0.0,
                    color,
                );
            } else {
                painter.line_segment(
                    [pos2(cx - u * 0.4, cy - u * 0.9), pos2(cx + u * 0.4, cy - u * 0.9)],
                    st,
                );
            }
        }
        Icon::Duplicate => {
            painter.rect_stroke(Rect::from_center_size(pos2(cx - u * 0.3, cy + u * 0.3), Vec2::new(u * 1.2, u * 1.2)), 1.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
            painter.rect_stroke(Rect::from_center_size(pos2(cx + u * 0.3, cy - u * 0.3), Vec2::new(u * 1.2, u * 1.2)), 1.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
        }
        Icon::Undo => {
            painter.add(egui::Shape::closed_line(
                arc_points(
                    pos2(cx, cy + u * 0.2),
                    u * 0.6,
                    std::f32::consts::PI,
                    std::f32::consts::TAU,
                    12,
                ),
                st,
            ));
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.6, cy + u * 0.2),
                    pos2(cx - u * 0.6, cy - u * 0.35),
                    pos2(cx - u * 0.15, cy - u * 0.1),
                ],
                st,
            ));
        }
        Icon::Redo => {
            painter.add(egui::Shape::closed_line(
                arc_points(
                    pos2(cx, cy + u * 0.2),
                    u * 0.6,
                    std::f32::consts::PI,
                    std::f32::consts::TAU,
                    12,
                ),
                st,
            ));
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx + u * 0.6, cy + u * 0.2),
                    pos2(cx + u * 0.6, cy - u * 0.35),
                    pos2(cx + u * 0.15, cy - u * 0.1),
                ],
                st,
            ));
        }
        Icon::Select => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.4, cy - u * 0.8),
                    pos2(cx - u * 0.4, cy + u * 0.7),
                    pos2(cx - u * 0.05, cy + u * 0.25),
                    pos2(cx + u * 0.2, cy + u * 0.85),
                    pos2(cx + u * 0.45, cy + u * 0.75),
                    pos2(cx + u * 0.2, cy + u * 0.15),
                    pos2(cx + u * 0.55, cy + u * 0.1),
                ],
                st,
            ));
        }
        Icon::Marquee => {
            let mut dashes = Vec::new();
            for i in 0..8 {
                let t = i as f32 / 8.0;
                let t2 = (i + 1) as f32 / 8.0;
                let a = pos2(cx - u + 2.0 * u * t, cy - u);
                let b = pos2(cx - u + 2.0 * u * t2, cy - u);
                dashes.push(egui::Shape::line_segment([a, b], st));
                let a = pos2(cx - u + 2.0 * u * t, cy + u);
                let b = pos2(cx - u + 2.0 * u * t2, cy + u);
                dashes.push(egui::Shape::line_segment([a, b], st));
                let a = pos2(cx - u, cy - u + 2.0 * u * t);
                let b = pos2(cx - u, cy - u + 2.0 * u * t2);
                dashes.push(egui::Shape::line_segment([a, b], st));
                let a = pos2(cx + u, cy - u + 2.0 * u * t);
                let b = pos2(cx + u, cy - u + 2.0 * u * t2);
                dashes.push(egui::Shape::line_segment([a, b], st));
            }
            painter.add(egui::Shape::Vec(dashes));
        }
        Icon::Lasso => {
            painter.add(egui::Shape::closed_line(
                arc_points(
                    pos2(cx, cy - u * 0.2),
                    u * 0.75,
                    0.0,
                    std::f32::consts::TAU,
                    16,
                ),
                st,
            ));
            painter.line_segment(
                [pos2(cx, cy + u * 0.5), pos2(cx - u * 0.2, cy + u * 0.9)],
                st,
            );
        }
        Icon::Wand => {
            painter.line_segment(
                [pos2(cx - u * 0.6, cy + u * 0.8), pos2(cx + u * 0.4, cy - u * 0.2)],
                st,
            );
            dot(pos2(cx + u * 0.7, cy - u * 0.5));
            dot(pos2(cx + u * 0.2, cy - u * 0.85));
            dot(pos2(cx + u * 0.9, cy + u * 0.1));
        }
        Icon::Crop => {
            painter.line_segment(
                [pos2(cx - u * 0.4, cy - u), pos2(cx - u * 0.4, cy + u * 0.5)],
                st,
            );
            painter.line_segment(
                [pos2(cx - u * 0.4, cy + u * 0.5), pos2(cx + u * 0.6, cy + u * 0.5)],
                st,
            );
            painter.line_segment(
                [pos2(cx + u * 0.6, cy - u * 0.6), pos2(cx + u * 0.6, cy + u * 0.5)],
                st,
            );
            painter.line_segment(
                [pos2(cx + u * 0.6, cy - u * 0.6), pos2(cx - u * 0.4, cy - u * 0.6)],
                st,
            );
        }
        Icon::Hand => {
            painter.rect_stroke(Rect::from_center_size(pos2(cx, cy + u * 0.25), Vec2::new(u * 1.1, u * 0.9)), 2.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
            painter.line_segment(
                [pos2(cx - u * 0.3, cy - u * 0.2), pos2(cx - u * 0.3, cy + u * 0.2)],
                st,
            );
            painter.line_segment(
                [pos2(cx, cy - u * 0.4), pos2(cx, cy + u * 0.2)],
                st,
            );
            painter.line_segment(
                [pos2(cx + u * 0.3, cy - u * 0.2), pos2(cx + u * 0.3, cy + u * 0.2)],
                st,
            );
        }
        Icon::Zoom | Icon::ZoomIn | Icon::ZoomOut => {
            painter.circle_stroke(pos2(cx - u * 0.2, cy - u * 0.2), u * 0.55, st);
            painter.line_segment(
                [
                    pos2(cx + u * 0.2, cy + u * 0.2),
                    pos2(cx + u * 0.8, cy + u * 0.8),
                ],
                st,
            );
            if icon == Icon::ZoomIn {
                painter.line_segment(
                    [
                        pos2(cx - u * 0.45, cy - u * 0.2),
                        pos2(cx + u * 0.05, cy - u * 0.2),
                    ],
                    st,
                );
                painter.line_segment(
                    [
                        pos2(cx - u * 0.2, cy - u * 0.45),
                        pos2(cx - u * 0.2, cy + u * 0.05),
                    ],
                    st,
                );
            } else if icon == Icon::ZoomOut {
                painter.line_segment(
                    [
                        pos2(cx - u * 0.45, cy - u * 0.2),
                        pos2(cx + u * 0.05, cy - u * 0.2),
                    ],
                    st,
                );
            }
        }
        Icon::Brush => {
            painter.line_segment(
                [pos2(cx - u * 0.6, cy + u * 0.7), pos2(cx + u * 0.4, cy - u * 0.4)],
                st,
            );
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.6, cy + u * 0.7),
                    pos2(cx - u * 0.25, cy + u * 0.85),
                    pos2(cx - u * 0.45, cy + u * 0.35),
                ],
                st,
            ));
        }
        Icon::Pencil => {
            painter.line_segment(
                [pos2(cx - u * 0.7, cy + u * 0.7), pos2(cx + u * 0.7, cy - u * 0.7)],
                st,
            );
            painter.line_segment(
                [pos2(cx - u * 0.9, cy + u * 0.9), pos2(cx - u * 0.7, cy + u * 0.7)],
                st,
            );
        }
        Icon::Eraser => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.8, cy + u * 0.5),
                    pos2(cx - u * 0.2, cy - u * 0.6),
                    pos2(cx + u * 0.8, cy - u * 0.1),
                    pos2(cx + u * 0.2, cy + u * 0.95),
                ],
                st,
            ));
        }
        Icon::Line => {
            painter.line_segment(
                [pos2(cx - u * 0.9, cy + u * 0.8), pos2(cx + u * 0.9, cy - u * 0.8)],
                st,
            );
        }
        Icon::Rectangle => {
            painter.rect_stroke(r, 0.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
        }
        Icon::Ellipse => {
            painter.add(egui::Shape::ellipse_stroke(r.center(), r.size() * 0.5, st));
        }
        Icon::Polygon => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx, cy - u),
                    pos2(cx + u * 0.9, cy + u * 0.7),
                    pos2(cx - u * 0.9, cy + u * 0.7),
                ],
                st,
            ));
        }
        Icon::Text => {
            painter.line_segment(
                [pos2(cx - u * 0.7, cy - u * 0.7), pos2(cx + u * 0.7, cy - u * 0.7)],
                st,
            );
            painter.line_segment(
                [pos2(cx, cy - u * 0.7), pos2(cx, cy + u * 0.8)],
                st,
            );
        }
        Icon::Picker | Icon::Eyedropper => {
            painter.line_segment(
                [pos2(cx - u * 0.8, cy + u * 0.8), pos2(cx + u * 0.5, cy - u * 0.5)],
                st,
            );
            painter.circle_stroke(pos2(cx + u * 0.55, cy - u * 0.55), u * 0.3, st);
        }
        Icon::Fill => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.2, cy - u * 0.8),
                    pos2(cx + u * 0.7, cy + u * 0.1),
                    pos2(cx + u * 0.1, cy + u * 0.8),
                    pos2(cx - u * 0.8, cy - u * 0.1),
                ],
                st,
            ));
        }
        Icon::Gradient => {
            for i in 0..8 {
                let t = i as f32 / 8.0;
                let c = Color32::from_rgb(
                    (255.0 * t) as u8,
                    (255.0 * t) as u8,
                    (255.0 * t) as u8,
                );
                painter.rect_filled(
                    Rect::from_min_size(
                        pos2(cx - u + 2.0 * u * t, cy - u * 0.7),
                        Vec2::new(2.0 * u / 8.0 + 0.5, u * 1.4),
                    ),
                    0.0,
                    c,
                );
            }
            painter.rect_stroke(r.shrink(u * 0.7), 0.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
        }
        Icon::Blur => {
            painter.circle_filled(pos2(cx, cy), u * 0.55, color.gamma_multiply(0.35));
            painter.circle_stroke(pos2(cx, cy), u * 0.8, st);
        }
        Icon::Sharpen => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.7, cy + u * 0.8),
                    pos2(cx - u * 0.7, cy - u * 0.2),
                    pos2(cx + u * 0.7, cy - u * 0.8),
                    pos2(cx + u * 0.7, cy + u * 0.2),
                ],
                st,
            ));
        }
        Icon::Dodge => {
            painter.circle_stroke(pos2(cx, cy), u * 0.7, st);
            dot(pos2(cx, cy));
        }
        Icon::Burn => {
            painter.circle_filled(pos2(cx, cy), u * 0.35, color);
            painter.circle_stroke(pos2(cx, cy), u * 0.8, st);
        }
        Icon::Trim => {
            painter.rect_stroke(r, 0.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
            painter.line_segment(
                [pos2(cx - u, cy + u), pos2(cx + u, cy - u)],
                st,
            );
        }
        Icon::Split => {
            painter.line_segment(
                [pos2(cx - u, cy), pos2(cx + u, cy)],
                st,
            );
            painter.line_segment(
                [pos2(cx, cy - u), pos2(cx, cy + u)],
                st,
            );
        }
        Icon::Distort => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u, cy - u * 0.6),
                    pos2(cx - u * 0.3, cy - u),
                    pos2(cx + u * 0.4, cy - u * 0.5),
                    pos2(cx + u, cy - u),
                    pos2(cx + u * 0.6, cy + u),
                    pos2(cx - u * 0.4, cy + u * 0.7),
                ],
                st,
            ));
        }
        Icon::Rounded => {
            painter.rect_stroke(r, 3.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
        }
        Icon::Frame => {
            painter.rect_stroke(r.shrink(2.0), 0.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
        }
        Icon::Connect => {
            painter.circle_stroke(pos2(cx - u * 0.5, cy), u * 0.4, st);
            painter.circle_stroke(pos2(cx + u * 0.5, cy), u * 0.4, st);
            painter.line_segment(
                [pos2(cx - u * 0.1, cy), pos2(cx + u * 0.1, cy)],
                st,
            );
        }
        Icon::Fit => {
            painter.rect_stroke(r, 0.0, Stroke::new(1.0, color), eframe::egui::StrokeKind::Middle);
            painter.line_segment(
                [pos2(cx - u * 0.4, cy), pos2(cx + u * 0.4, cy)],
                st,
            );
        }
        Icon::Sync => {
            painter.add(egui::Shape::closed_line(
                arc_points(pos2(cx, cy), u * 0.7, 0.6, 5.2, 14),
                st,
            ));
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx + u * 0.55, cy - u * 0.75),
                    pos2(cx + u * 0.95, cy - u * 0.45),
                    pos2(cx + u * 0.45, cy - u * 0.25),
                ],
                st,
            ));
        }
        Icon::Info => {
            painter.circle_stroke(pos2(cx, cy), u * 0.8, st);
            dot(pos2(cx, cy - u * 0.35));
            painter.line_segment(
                [pos2(cx, cy - u * 0.05), pos2(cx, cy + u * 0.45)],
                st,
            );
        }
        Icon::Check => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.7, cy),
                    pos2(cx - u * 0.15, cy + u * 0.55),
                    pos2(cx + u * 0.8, cy - u * 0.6),
                ],
                st,
            ));
        }
    }
}


