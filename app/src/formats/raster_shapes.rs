//! Shared vector contour rasterization used by flat image exports.

use eframe::egui::Vec2;

/// Fill a valid simple contour by scanline. Triangulation validation deliberately matches the
/// interactive canvas: degenerate and self-intersecting paths keep their editable outline but do
/// not gain an invented fill. Scanlines avoid double-blending shared triangle edges at partial
/// opacity and evaluate gradients once per covered pixel.
pub(super) fn fill_simple_polygon(
    img: &mut image::RgbaImage,
    points: &[Vec2],
    mut color_at: impl FnMut(i32, i32) -> image::Rgba<u8>,
) {
    if crate::geometry::triangulate_simple_polygon(points).is_none() {
        return;
    }
    let min_y = points
        .iter()
        .map(|point| point.y)
        .fold(f32::INFINITY, f32::min);
    let max_y = points
        .iter()
        .map(|point| point.y)
        .fold(f32::NEG_INFINITY, f32::max);
    let first_y = ((min_y - 0.5).ceil() as i32).max(0);
    let end_y = ((max_y - 0.5).ceil() as i32).min(img.height() as i32);
    let mut intersections = Vec::with_capacity(points.len());

    for y in first_y..end_y {
        let scan_y = y as f32 + 0.5;
        intersections.clear();
        for index in 0..points.len() {
            let a = points[index];
            let b = points[(index + 1) % points.len()];
            if (a.y <= scan_y && b.y > scan_y) || (b.y <= scan_y && a.y > scan_y) {
                intersections.push(a.x + (scan_y - a.y) * (b.x - a.x) / (b.y - a.y));
            }
        }
        intersections.sort_by(f32::total_cmp);

        for span in intersections.as_chunks::<2>().0 {
            let first_x = ((span[0] - 0.5).ceil() as i32).max(0);
            let end_x = ((span[1] - 0.5).ceil() as i32).min(img.width() as i32);
            for x in first_x..end_x {
                let source = color_at(x, y);
                let old = img.get_pixel(x as u32, y as u32);
                img.put_pixel(x as u32, y as u32, super::blend(*old, source));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::rasterize;
    use crate::doc::{Document, GradientFill, Obj, Shape};
    use eframe::egui::{Color32, Vec2};

    #[test]
    fn raster_export_fills_closed_paths_and_preserves_concave_notches() {
        let mut doc = Document::new(24.0, 24.0);
        doc.page_mut().transparent_bg = true;
        doc.page_mut().layers[0].objects.push(Obj::new(
            Shape::Path {
                points: vec![
                    Vec2::new(2.0, 2.0),
                    Vec2::new(20.0, 2.0),
                    Vec2::new(20.0, 8.0),
                    Vec2::new(8.0, 8.0),
                    Vec2::new(8.0, 20.0),
                    Vec2::new(2.0, 20.0),
                ],
                closed: true,
            },
            Some(Color32::from_rgb(230, 40, 10)),
            None,
        ));

        let image = rasterize(&doc);

        assert_eq!(image.get_pixel(4, 4).0, [230, 40, 10, 255]);
        assert_eq!(image.get_pixel(4, 16).0, [230, 40, 10, 255]);
        assert_eq!(image.get_pixel(16, 16).0, [0, 0, 0, 0]);
    }

    #[test]
    fn raster_export_fills_polygon_shapes() {
        let mut doc = Document::new(24.0, 24.0);
        doc.page_mut().transparent_bg = true;
        doc.page_mut().layers[0].objects.push(Obj::new(
            Shape::Polygon {
                center: Vec2::new(12.0, 12.0),
                radius: 9.0,
                sides: 3,
                star_inner: 0.0,
                angle_rad: 0.0,
            },
            Some(Color32::from_rgb(30, 180, 240)),
            None,
        ));

        let image = rasterize(&doc);

        assert_eq!(image.get_pixel(12, 12).0, [30, 180, 240, 255]);
        assert_eq!(image.get_pixel(1, 1).0, [0, 0, 0, 0]);
    }

    #[test]
    fn raster_export_uses_gradient_fill_for_closed_paths() {
        let mut doc = Document::new(24.0, 24.0);
        doc.page_mut().transparent_bg = true;
        let mut path = Obj::new(
            Shape::Path {
                points: vec![
                    Vec2::new(1.0, 1.0),
                    Vec2::new(22.0, 1.0),
                    Vec2::new(22.0, 22.0),
                    Vec2::new(1.0, 22.0),
                ],
                closed: true,
            },
            Some(Color32::BLACK),
            None,
        );
        let mut gradient = GradientFill::new(Color32::BLACK, Color32::WHITE);
        gradient.start_pos = Vec2::new(0.0, 0.5);
        gradient.end_pos = Vec2::new(1.0, 0.5);
        path.gradient = Some(gradient);
        doc.page_mut().layers[0].objects.push(path);

        let image = rasterize(&doc);
        let left = image.get_pixel(2, 10).0[0];
        let middle = image.get_pixel(10, 10).0[0];
        let right = image.get_pixel(20, 10).0[0];

        assert!(
            left < middle && middle < right,
            "gradient samples: {left}, {middle}, {right}"
        );
    }
}
