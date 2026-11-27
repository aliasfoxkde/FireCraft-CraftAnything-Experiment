use super::draw_obj_content;
use crate::doc::{LiveEffect, Obj, Shape};
use eframe::egui::{Color32, Vec2};

pub(super) fn render_drop_shadows(img: &mut image::RgbaImage, obj: &Obj, alpha: u8) {
    for effect in &obj.effects {
        if let LiveEffect::DropShadow {
            distance,
            angle_deg,
            blur,
            color,
            enabled: true,
        } = effect
        {
            draw_raster_shadow(img, obj, alpha, *distance, *angle_deg, *blur, *color);
        }
    }
}

fn blend(dst: image::Rgba<u8>, src: image::Rgba<u8>) -> image::Rgba<u8> {
    super::blend(dst, src)
}

fn draw_raster_shadow(
    img: &mut image::RgbaImage,
    obj: &Obj,
    alpha: u8,
    distance: f32,
    angle_deg: f32,
    blur: f32,
    color: Color32,
) {
    let Some((bounds_min, bounds_max)) = obj.bounds() else {
        return;
    };
    if !distance.is_finite() || !angle_deg.is_finite() || !blur.is_finite() {
        return;
    }

    let radians = angle_deg.to_radians();
    let offset = Vec2::new(distance * radians.cos(), distance * radians.sin());
    let radius = blur.clamp(0.0, img.width().max(img.height()) as f32 / 3.0);
    if let Shape::Rect { min, max, .. } = &obj.kind {
        draw_blurred_rect_shadow(img, *min, *max, alpha, offset, radius, color);
        return;
    }
    if matches!(&obj.kind, Shape::Ellipse { .. } | Shape::Polygon { .. }) {
        draw_vector_shadow_layers(img, obj, alpha, offset, radius, color);
        return;
    }
    let padding = (radius * 3.0).ceil();
    let raw_min_x = (bounds_min.x.min(bounds_min.x + offset.x) - padding).floor();
    let raw_max_x = (bounds_max.x.max(bounds_max.x + offset.x) + padding).ceil();
    let raw_min_y = (bounds_min.y.min(bounds_min.y + offset.y) - padding).floor();
    let raw_max_y = (bounds_max.y.max(bounds_max.y + offset.y) + padding).ceil();
    let page_max_x = img.width().saturating_sub(1) as f32;
    let page_max_y = img.height().saturating_sub(1) as f32;
    if raw_max_x < 0.0 || raw_max_y < 0.0 || raw_min_x > page_max_x || raw_min_y > page_max_y {
        return;
    }
    let x0 = raw_min_x.max(0.0) as u32;
    let y0 = raw_min_y.max(0.0) as u32;
    let x1 = raw_max_x.min(page_max_x) as u32;
    let y1 = raw_max_y.min(page_max_y) as u32;
    let width = x1 - x0 + 1;
    let height = y1 - y0 + 1;

    let mut mask = image::RgbaImage::from_pixel(width, height, image::Rgba([0, 0, 0, 0]));
    let mut silhouette = obj.clone();
    silhouette.effects.clear();
    silhouette.translate(Vec2::new(-(x0 as f32) + offset.x, -(y0 as f32) + offset.y));
    draw_obj_content(&mut mask, &silhouette, alpha);

    let alpha_mask = image::GrayImage::from_fn(width, height, |x, y| {
        image::Luma([mask.get_pixel(x, y).0[3]])
    });
    let blurred = if radius > 0.0 {
        // Large document-space shadows can cover most of a page. Blur a reduced alpha mask so
        // work scales with the visible softness instead of multiplying full-page pixels by the
        // entire Gaussian kernel; resizing back preserves a smooth silhouette at canvas scale.
        let scale = (radius / 2.0).ceil().max(1.0) as u32;
        if scale == 1 {
            image::imageops::blur(&alpha_mask, radius)
        } else {
            let small_width = width.div_ceil(scale).max(1);
            let small_height = height.div_ceil(scale).max(1);
            let small = image::imageops::resize(
                &alpha_mask,
                small_width,
                small_height,
                image::imageops::FilterType::Triangle,
            );
            let small_blurred = image::imageops::blur(&small, radius / scale as f32);
            image::imageops::resize(
                &small_blurred,
                width,
                height,
                image::imageops::FilterType::Triangle,
            )
        }
    } else {
        alpha_mask
    };
    for (mask_x, mask_y, pixel) in blurred.enumerate_pixels() {
        let shadow_alpha = ((pixel.0[0] as u16 * color.a() as u16 + 127) / 255) as u8;
        if shadow_alpha == 0 {
            continue;
        }
        let x = x0 + mask_x;
        let y = y0 + mask_y;
        let dst = img.get_pixel(x, y);
        let src = image::Rgba([color.r(), color.g(), color.b(), shadow_alpha]);
        img.put_pixel(x, y, blend(*dst, src));
    }
}

fn draw_blurred_rect_shadow(
    img: &mut image::RgbaImage,
    min: Vec2,
    max: Vec2,
    alpha: u8,
    offset: Vec2,
    blur: f32,
    color: Color32,
) {
    let lo_x = (min.x.min(max.x) + offset.x).round() as i32;
    let hi_x = (min.x.max(max.x) + offset.x).round() as i32;
    let lo_y = (min.y.min(max.y) + offset.y).round() as i32;
    let hi_y = (min.y.max(max.y) + offset.y).round() as i32;
    let support = (blur * 3.0).ceil() as i32;
    let out_x0 = lo_x.saturating_sub(support).max(0);
    let out_y0 = lo_y.saturating_sub(support).max(0);
    let out_x1 = hi_x
        .saturating_add(support)
        .min(img.width().saturating_sub(1) as i32);
    let out_y1 = hi_y
        .saturating_add(support)
        .min(img.height().saturating_sub(1) as i32);
    if out_x1 < out_x0 || out_y1 < out_y0 || color.a() == 0 || alpha == 0 {
        return;
    }

    let kernel = if blur > 0.0 {
        (-support..=support)
            .map(|distance| {
                let normalized = distance as f32 / blur;
                (-0.5 * normalized * normalized).exp()
            })
            .collect::<Vec<_>>()
    } else {
        vec![1.0]
    };
    let kernel_sum: f32 = kernel.iter().sum();
    let coverage_for_axis = |position: i32, low: i32, high: i32| {
        if blur <= 0.0 {
            return f32::from((low..=high).contains(&position));
        }
        let covered: f32 = (-support..=support)
            .filter(|delta| (low..=high).contains(&(position - delta)))
            .map(|delta| kernel[(delta + support) as usize])
            .sum();
        covered / kernel_sum
    };
    let x_coverages: Vec<f32> = (out_x0..=out_x1)
        .map(|x| coverage_for_axis(x, lo_x, hi_x))
        .collect();
    let y_coverages: Vec<f32> = (out_y0..=out_y1)
        .map(|y| coverage_for_axis(y, lo_y, hi_y))
        .collect();
    let base_alpha = alpha as f32 * color.a() as f32 / 255.0;
    for (dy, y_coverage) in y_coverages.into_iter().enumerate() {
        let y = out_y0 as u32 + dy as u32;
        for (dx, x_coverage) in x_coverages.iter().enumerate() {
            let shadow_alpha = (x_coverage * y_coverage * base_alpha).round() as u8;
            if shadow_alpha == 0 {
                continue;
            }
            let x = out_x0 as u32 + dx as u32;
            let dst = img.get_pixel(x, y);
            let src = image::Rgba([color.r(), color.g(), color.b(), shadow_alpha]);
            img.put_pixel(x, y, blend(*dst, src));
        }
    }
}

fn draw_vector_shadow_layers(
    img: &mut image::RgbaImage,
    obj: &Obj,
    alpha: u8,
    offset: Vec2,
    blur: f32,
    color: Color32,
) {
    let steps = (blur.ceil() as usize).clamp(1, 6);
    for step in (1..=steps).rev() {
        let expand = (step as f32 / steps as f32) * blur * 0.5;
        let shadow_alpha = ((alpha as f32 * color.a() as f32) / (255.0 * steps as f32 * 0.9))
            .round()
            .clamp(0.0, 255.0) as u8;
        if shadow_alpha == 0 {
            continue;
        }

        let mut silhouette = obj.clone();
        silhouette.effects.clear();
        silhouette.gradient = None;
        silhouette.fill = Some(color);
        silhouette.stroke = Some(color);
        match &mut silhouette.kind {
            Shape::Rect { min, max, .. } => {
                *min -= Vec2::splat(expand);
                *max += Vec2::splat(expand);
            }
            Shape::Ellipse { radius, .. } => *radius += Vec2::splat(expand),
            Shape::Polygon { radius, .. } => *radius += expand,
            _ => unreachable!("only filled vector silhouettes use this path"),
        }
        silhouette.translate(offset);
        draw_obj_content(img, &silhouette, shadow_alpha);
    }
}
