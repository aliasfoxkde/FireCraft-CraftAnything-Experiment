//! Legacy painter fallbacks for the compact color/display controls.

use super::Icon;
use eframe::egui::{Color32, Painter, Rect, Stroke, Vec2, pos2};

pub(super) fn draw_legacy(painter: &Painter, rect: Rect, icon: Icon, color: Color32, width: f32) {
    let bounds = rect.shrink(3.0);
    let center = bounds.center();
    let unit = bounds.width().min(bounds.height()) * 0.5;
    let stroke = Stroke::new(width.max(1.0), color);

    match icon {
        Icon::Swap => {
            for (y, direction) in [(-unit * 0.35, 1.0), (unit * 0.35, -1.0)] {
                let left = center.x - unit * 0.7;
                let right = center.x + unit * 0.7;
                let (start, end) = if direction > 0.0 {
                    (pos2(left, center.y + y), pos2(right, center.y + y))
                } else {
                    (pos2(right, center.y + y), pos2(left, center.y + y))
                };
                painter.line_segment([start, end], stroke);
                for dy in [-unit * 0.2, unit * 0.2] {
                    painter.line_segment(
                        [end, pos2(end.x - direction * unit * 0.28, end.y + dy)],
                        stroke,
                    );
                }
            }
        }
        Icon::SquareHalf => {
            let square = Rect::from_center_size(center, Vec2::splat(unit * 1.45));
            painter.rect_filled(
                Rect::from_min_max(square.left_top(), pos2(center.x, square.bottom())),
                0.0,
                color.gamma_multiply(0.42),
            );
            painter.rect_stroke(square, 0.0, stroke, eframe::egui::StrokeKind::Middle);
            painter.line_segment(
                [
                    pos2(center.x, square.top()),
                    pos2(center.x, square.bottom()),
                ],
                stroke,
            );
        }
        Icon::DropSlash => {
            painter.circle_stroke(center, unit * 0.62, stroke);
            painter.line_segment(
                [
                    pos2(center.x - unit * 0.8, center.y + unit * 0.8),
                    pos2(center.x + unit * 0.8, center.y - unit * 0.8),
                ],
                stroke,
            );
        }
        Icon::CornersOut => {
            let edge = unit * 0.8;
            let inner = unit * 0.35;
            for (x, y) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
                let corner = pos2(center.x + x * edge, center.y + y * edge);
                painter.line_segment([corner, pos2(corner.x - x * inner, corner.y)], stroke);
                painter.line_segment([corner, pos2(corner.x, corner.y - y * inner)], stroke);
            }
        }
        _ => {}
    }
}
