//! Small line-art icons drawn with the painter, so the app needs no icon font.

use eframe::egui::{self, Color32, Painter, Pos2, Rect, Stroke, Vec2, pos2};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Icon {
    New,
    Open,
    Save,
    Import,
    Export,
    Cut,
    Copy,
    Paste,
    Duplicate,
    Undo,
    Redo,
    Select,
    Subselect,
    Scale,
    Marquee,
    Lasso,
    Wand,
    Crop,
    Hand,
    Zoom,
    Brush,
    Pencil,
    Eraser,
    Stamp,
    Line,
    Rectangle,
    Rounded,
    Ellipse,
    Polygon,
    Star,
    Pen,
    Knife,
    Text,
    Picker,
    Fill,
    Gradient,
    Blur,
    Sharpen,
    Dodge,
    Burn,
    Slice,
    Hotspot,
    ZoomIn,
    ZoomOut,
    Fit,
}

fn stroke_of(c: Color32, w: f32) -> Stroke {
    Stroke::new(w, c)
}

fn arc_points(center: Pos2, radius: f32, start: f32, end: f32, steps: usize) -> Vec<Pos2> {
    (0..=steps)
        .map(|i| {
            let t = start + (end - start) * i as f32 / steps as f32;
            pos2(center.x + radius * t.cos(), center.y + radius * t.sin())
        })
        .collect()
}

/// Draws `icon` inside `rect` (a square button face).
pub fn draw(painter: &Painter, rect: Rect, icon: Icon, color: Color32, width: f32) {
    let r = rect.shrink(3.0);
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height());
    let u = s * 0.5;
    let st = stroke_of(color, width.max(1.0));
    let dot = |p: Pos2| painter.circle_filled(p, 1.4, color);
    match icon {
        Icon::New => {
            painter.rect_stroke(
                Rect::from_center_size(r.center(), Vec2::new(s * 0.72, s * 0.86)),
                1.0,
                Stroke::new(1.0, color),
                egui::StrokeKind::Middle,
            );
            painter.line_segment([pos2(cx, cy - u * 0.35), pos2(cx, cy + u * 0.35)], st);
            painter.line_segment([pos2(cx - u * 0.35, cy), pos2(cx + u * 0.35, cy)], st);
        }
        Icon::Open => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.85, cy + u * 0.65),
                    pos2(cx - u * 0.85, cy - u * 0.45),
                    pos2(cx - u * 0.2, cy - u * 0.45),
                    pos2(cx - u * 0.05, cy - u * 0.2),
                    pos2(cx + u * 0.85, cy - u * 0.2),
                    pos2(cx + u * 0.85, cy + u * 0.65),
                ],
                st,
            ));
        }
        Icon::Save => {
            painter.rect_stroke(r, 1.0, Stroke::new(1.0, color), egui::StrokeKind::Middle);
            painter.rect_filled(
                Rect::from_min_size(
                    pos2(cx - u * 0.45, cy - u * 0.85),
                    Vec2::new(u * 0.9, u * 0.55),
                ),
                0.0,
                color,
            );
        }
        Icon::Import => {
            painter.line_segment([pos2(cx, cy - u * 0.8), pos2(cx, cy + u * 0.45)], st);
            painter.line_segment(
                [pos2(cx - u * 0.4, cy + u * 0.1), pos2(cx, cy + u * 0.55)],
                st,
            );
            painter.line_segment(
                [pos2(cx + u * 0.4, cy + u * 0.1), pos2(cx, cy + u * 0.55)],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx - u * 0.75, cy + u * 0.8),
                    pos2(cx + u * 0.75, cy + u * 0.8),
                ],
                st,
            );
        }
        Icon::Export => {
            painter.line_segment([pos2(cx, cy + u * 0.8), pos2(cx, cy - u * 0.5)], st);
            painter.line_segment(
                [pos2(cx - u * 0.4, cy - u * 0.15), pos2(cx, cy - u * 0.55)],
                st,
            );
            painter.line_segment(
                [pos2(cx + u * 0.4, cy - u * 0.15), pos2(cx, cy - u * 0.55)],
                st,
            );
        }
        Icon::Cut | Icon::Copy | Icon::Paste => {
            painter.rect_stroke(
                Rect::from_center_size(pos2(cx, cy + u * 0.25), Vec2::new(u * 1.2, u * 1.2)),
                1.0,
                st,
                egui::StrokeKind::Middle,
            );
            if icon == Icon::Paste {
                painter.rect_filled(
                    Rect::from_center_size(pos2(cx, cy - u * 0.55), Vec2::new(u * 0.9, u * 0.35)),
                    0.0,
                    color,
                );
            } else {
                painter.line_segment(
                    [
                        pos2(cx - u * 0.4, cy - u * 0.8),
                        pos2(cx + u * 0.4, cy - u * 0.8),
                    ],
                    st,
                );
            }
        }
        Icon::Duplicate => {
            painter.rect_stroke(
                Rect::from_center_size(
                    pos2(cx - u * 0.25, cy + u * 0.25),
                    Vec2::new(u * 1.1, u * 1.1),
                ),
                1.0,
                st,
                egui::StrokeKind::Middle,
            );
            painter.rect_stroke(
                Rect::from_center_size(
                    pos2(cx + u * 0.25, cy - u * 0.25),
                    Vec2::new(u * 1.1, u * 1.1),
                ),
                1.0,
                st,
                egui::StrokeKind::Middle,
            );
        }
        Icon::Undo => {
            painter.add(egui::Shape::line(
                arc_points(
                    pos2(cx, cy + u * 0.2),
                    u * 0.55,
                    std::f32::consts::PI,
                    std::f32::consts::TAU,
                    10,
                ),
                st,
            ));
            painter.line_segment(
                [
                    pos2(cx - u * 0.55, cy + u * 0.2),
                    pos2(cx - u * 0.2, cy - u * 0.15),
                ],
                st,
            );
        }
        Icon::Redo => {
            painter.add(egui::Shape::line(
                arc_points(
                    pos2(cx, cy + u * 0.2),
                    u * 0.55,
                    std::f32::consts::PI,
                    std::f32::consts::TAU,
                    10,
                ),
                st,
            ));
            painter.line_segment(
                [
                    pos2(cx + u * 0.55, cy + u * 0.2),
                    pos2(cx + u * 0.2, cy - u * 0.15),
                ],
                st,
            );
        }
        Icon::Select => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.4, cy - u * 0.8),
                    pos2(cx - u * 0.4, cy + u * 0.65),
                    pos2(cx - u * 0.05, cy + u * 0.25),
                    pos2(cx + u * 0.2, cy + u * 0.85),
                    pos2(cx + u * 0.4, cy + u * 0.75),
                    pos2(cx + u * 0.15, cy + u * 0.15),
                    pos2(cx + u * 0.55, cy + u * 0.1),
                ],
                st,
            ));
        }
        Icon::Subselect => {
            painter.add(egui::Shape::convex_polygon(
                vec![
                    pos2(cx - u * 0.35, cy - u * 0.75),
                    pos2(cx - u * 0.35, cy + u * 0.55),
                    pos2(cx + u * 0.5, cy + u * 0.05),
                ],
                color.gamma_multiply(0.35),
                st,
            ));
        }
        Icon::Scale => {
            painter.rect_stroke(r.shrink(2.0), 0.0, st, egui::StrokeKind::Middle);
            for corner in [
                r.left_top(),
                r.right_top(),
                r.left_bottom(),
                r.right_bottom(),
            ] {
                painter.rect_filled(Rect::from_center_size(corner, Vec2::splat(3.0)), 0.0, color);
            }
        }
        Icon::Marquee => {
            painter.rect_stroke(r, 0.0, st, egui::StrokeKind::Middle);
            dot(r.center());
        }
        Icon::Lasso => {
            painter.add(egui::Shape::closed_line(
                arc_points(
                    pos2(cx, cy - u * 0.15),
                    u * 0.7,
                    0.0,
                    std::f32::consts::TAU,
                    14,
                ),
                st,
            ));
            painter.line_segment(
                [pos2(cx, cy + u * 0.55), pos2(cx - u * 0.3, cy + u * 0.9)],
                st,
            );
        }
        Icon::Wand => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.65, cy + u * 0.75),
                    pos2(cx + u * 0.35, cy - u * 0.25),
                ],
                st,
            );
            dot(pos2(cx + u * 0.65, cy - u * 0.55));
            dot(pos2(cx + u * 0.2, cy - u * 0.8));
            dot(pos2(cx + u * 0.8, cy + u * 0.05));
        }
        Icon::Crop => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.4, cy - u),
                    pos2(cx - u * 0.4, cy + u * 0.45),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx - u * 0.4, cy + u * 0.45),
                    pos2(cx + u, cy + u * 0.45),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx + u * 0.45, cy - u * 0.45),
                    pos2(cx + u * 0.45, cy + u),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx - u, cy - u * 0.45),
                    pos2(cx + u * 0.45, cy - u * 0.45),
                ],
                st,
            );
        }
        Icon::Hand => {
            painter.rect_stroke(
                Rect::from_center_size(pos2(cx, cy + u * 0.25), Vec2::new(u * 1.1, u * 0.85)),
                2.0,
                st,
                egui::StrokeKind::Middle,
            );
            painter.line_segment(
                [pos2(cx - u * 0.3, cy - u * 0.55), pos2(cx - u * 0.3, cy)],
                st,
            );
            painter.line_segment([pos2(cx, cy - u * 0.75), pos2(cx, cy)], st);
            painter.line_segment(
                [pos2(cx + u * 0.3, cy - u * 0.55), pos2(cx + u * 0.3, cy)],
                st,
            );
        }
        Icon::Zoom | Icon::ZoomIn | Icon::ZoomOut => {
            painter.circle_stroke(pos2(cx - u * 0.15, cy - u * 0.15), u * 0.55, st);
            painter.line_segment(
                [
                    pos2(cx + u * 0.25, cy + u * 0.25),
                    pos2(cx + u * 0.85, cy + u * 0.85),
                ],
                st,
            );
            if icon == Icon::ZoomIn {
                painter.line_segment(
                    [
                        pos2(cx - u * 0.4, cy - u * 0.15),
                        pos2(cx + u * 0.1, cy - u * 0.15),
                    ],
                    st,
                );
                painter.line_segment(
                    [
                        pos2(cx - u * 0.15, cy - u * 0.4),
                        pos2(cx - u * 0.15, cy + u * 0.1),
                    ],
                    st,
                );
            } else if icon == Icon::ZoomOut {
                painter.line_segment(
                    [
                        pos2(cx - u * 0.4, cy - u * 0.15),
                        pos2(cx + u * 0.1, cy - u * 0.15),
                    ],
                    st,
                );
            }
        }
        Icon::Brush => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.5, cy + u * 0.6),
                    pos2(cx + u * 0.65, cy - u * 0.65),
                ],
                st,
            );
            painter.circle_filled(pos2(cx - u * 0.55, cy + u * 0.65), u * 0.28, color);
        }
        Icon::Pencil => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.75, cy + u * 0.75),
                    pos2(cx + u * 0.75, cy - u * 0.75),
                ],
                st,
            );
            dot(pos2(cx - u * 0.8, cy + u * 0.8));
        }
        Icon::Eraser => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.75, cy + u * 0.35),
                    pos2(cx - u * 0.15, cy - u * 0.65),
                    pos2(cx + u * 0.75, cy - u * 0.1),
                    pos2(cx + u * 0.15, cy + u * 0.85),
                ],
                st,
            ));
        }
        Icon::Stamp => {
            painter.rect_filled(
                Rect::from_center_size(pos2(cx, cy + u * 0.55), Vec2::new(u * 1.4, u * 0.35)),
                1.0,
                color,
            );
            painter.circle_stroke(pos2(cx, cy - u * 0.35), u * 0.35, st);
            painter.line_segment([pos2(cx, cy), pos2(cx, cy + u * 0.4)], st);
        }
        Icon::Line => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.85, cy + u * 0.85),
                    pos2(cx + u * 0.85, cy - u * 0.85),
                ],
                st,
            );
        }
        Icon::Rectangle => {
            painter.rect_stroke(r, 0.0, st, egui::StrokeKind::Middle);
        }
        Icon::Rounded => {
            painter.rect_stroke(r, 3.5, st, egui::StrokeKind::Middle);
        }
        Icon::Ellipse => {
            painter.add(egui::Shape::ellipse_stroke(r.center(), r.size() * 0.48, st));
        }
        Icon::Polygon => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx, cy - u * 0.9),
                    pos2(cx + u * 0.85, cy - u * 0.2),
                    pos2(cx + u * 0.55, cy + u * 0.85),
                    pos2(cx - u * 0.55, cy + u * 0.85),
                    pos2(cx - u * 0.85, cy - u * 0.2),
                ],
                st,
            ));
        }
        Icon::Star => {
            let pts = crate::doc::Obj::polygon_vertices(Vec2::new(cx, cy), u * 0.9, 5, 0.45, 0.0)
                .into_iter()
                .map(|v| pos2(v.x, v.y))
                .collect();
            painter.add(egui::Shape::closed_line(pts, st));
        }
        Icon::Pen => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.75, cy + u * 0.75),
                    pos2(cx - u * 0.1, cy - u * 0.55),
                    pos2(cx + u * 0.55, cy + u * 0.1),
                ],
                st,
            ));
            dot(pos2(cx - u * 0.75, cy + u * 0.75));
        }
        Icon::Knife => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.8, cy + u * 0.8),
                    pos2(cx + u * 0.8, cy - u * 0.8),
                ],
                st,
            );
            painter.line_segment(
                [
                    pos2(cx - u * 0.2, cy + u * 0.6),
                    pos2(cx + u * 0.8, cy - u * 0.4),
                ],
                st,
            );
        }
        Icon::Text => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.65, cy - u * 0.7),
                    pos2(cx + u * 0.65, cy - u * 0.7),
                ],
                st,
            );
            painter.line_segment([pos2(cx, cy - u * 0.7), pos2(cx, cy + u * 0.75)], st);
        }
        Icon::Picker => {
            painter.line_segment(
                [
                    pos2(cx - u * 0.75, cy + u * 0.75),
                    pos2(cx + u * 0.45, cy - u * 0.45),
                ],
                st,
            );
            painter.circle_filled(pos2(cx + u * 0.55, cy - u * 0.55), u * 0.28, color);
        }
        Icon::Fill => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx - u * 0.2, cy - u * 0.75),
                    pos2(cx + u * 0.65, cy + u * 0.1),
                    pos2(cx + u * 0.1, cy + u * 0.75),
                    pos2(cx - u * 0.75, cy - u * 0.1),
                ],
                st,
            ));
        }
        Icon::Gradient => {
            for i in 0..6 {
                let t = i as f32 / 6.0;
                let c = color.gamma_multiply(0.2 + 0.8 * t);
                painter.rect_filled(
                    Rect::from_min_size(
                        pos2(cx - u + 2.0 * u * t, cy - u * 0.65),
                        Vec2::new(2.0 * u / 6.0 + 0.5, u * 1.3),
                    ),
                    0.0,
                    c,
                );
            }
            painter.rect_stroke(r.shrink(1.0), 0.0, st, egui::StrokeKind::Middle);
        }
        Icon::Blur => {
            painter.circle_filled(pos2(cx, cy), u * 0.45, color.gamma_multiply(0.4));
            painter.circle_stroke(pos2(cx, cy), u * 0.75, st);
        }
        Icon::Sharpen => {
            painter.add(egui::Shape::closed_line(
                vec![
                    pos2(cx, cy - u * 0.8),
                    pos2(cx + u * 0.7, cy + u * 0.7),
                    pos2(cx - u * 0.7, cy + u * 0.7),
                ],
                st,
            ));
        }
        Icon::Dodge => {
            painter.circle_stroke(pos2(cx - u * 0.2, cy - u * 0.2), u * 0.45, st);
            painter.line_segment(
                [
                    pos2(cx + u * 0.15, cy + u * 0.15),
                    pos2(cx + u * 0.8, cy + u * 0.8),
                ],
                st,
            );
        }
        Icon::Burn => {
            painter.circle_filled(pos2(cx, cy), u * 0.35, color);
            painter.circle_stroke(pos2(cx, cy), u * 0.75, st);
        }
        Icon::Slice => {
            painter.rect_stroke(r, 0.0, st, egui::StrokeKind::Middle);
            painter.line_segment([pos2(r.left(), cy), pos2(r.right(), cy)], st);
            painter.line_segment([pos2(cx, r.top()), pos2(cx, r.bottom())], st);
        }
        Icon::Hotspot => {
            painter.circle_stroke(pos2(cx, cy), u * 0.75, st);
            painter.circle_filled(pos2(cx, cy), u * 0.28, color);
        }
        Icon::Fit => {
            painter.rect_stroke(r, 0.0, st, egui::StrokeKind::Middle);
            painter.line_segment([pos2(cx - u * 0.45, cy), pos2(cx + u * 0.45, cy)], st);
        }
    }
}
