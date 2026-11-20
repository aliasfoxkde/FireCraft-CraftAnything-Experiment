use eframe::egui::{Color32, Pos2, Vec2};

#[derive(Debug, Clone)]
pub enum Shape {
    Rect { min: Vec2, max: Vec2, filled: bool },
    Ellipse { center: Vec2, radius: Vec2, filled: bool },
    Line { a: Vec2, b: Vec2 },
    Path { points: Vec<Vec2>, closed: bool },
    Text { at: Vec2, text: String, size: f32 },
    Bitmap { at: Vec2, pixels: Vec<u8>, size: Vec2 },
}

impl Shape {
    pub fn bounds(&self) -> Option<(Vec2, Vec2)> {
        match self {
            Shape::Rect { min, max, .. } => Some((min.min(*max), min.max(*max))),
            Shape::Ellipse { center, radius, .. } => {
                let lo = Vec2::new(center.x - radius.x, center.y - radius.y);
                let hi = Vec2::new(center.x + radius.x, center.y + radius.y);
                Some((lo, hi))
            }
            Shape::Line { a, b } => Some((a.min(*b), a.max(*b))),
            Shape::Path { points, .. } if !points.is_empty() => {
                let mut lo = points[0];
                let mut hi = points[0];
                for p in points {
                    lo = lo.min(*p);
                    hi = hi.max(*p);
                }
                Some((lo, hi))
            }
            Shape::Text { at, size, .. } => {
                let w = 0.55 * size * 8.0;
                let h = size * 1.2;
                Some((*at, Vec2::new(at.x + w, at.y - h)))
            }
            Shape::Bitmap { at, size, .. } => {
                Some((*at, Vec2::new(at.x + size.x, at.y + size.y)))
            }
        }
    }
}
