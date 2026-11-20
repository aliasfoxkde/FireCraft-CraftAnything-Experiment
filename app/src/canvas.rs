#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
use crate::app::{DragKind, FireCraft};
use crate::constants;
use crate::doc::{Obj, Shape};
use crate::icons;
use eframe::egui::{
    Color32, FontId, Painter, Pos2, Rect, Sense, Stroke, TextureHandle, Vec2, pos2,
};

pub fn show(ui: &mut eframe::egui::Ui, app: &mut FireCraft) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter().clone();
    painter.rect_filled(rect, 0.0, app.palette.canvas_pad);
    let zoom = app.doc.zoom;
    let page = app.doc.page();
    let page_size = Vec2::new(page.width * zoom, page.height * zoom);
    let origin: Vec2 = (rect.center() - page_size / 2.0 + app.doc.pan).to_vec2();
    if !app.show_start {
        let page_rect = Rect::from_min_size(origin.to_pos2(), page_size);
        painter.rect_filled(
            page_rect.translate(Vec2::new(3.0, 3.0)),
            0.0,
            Color32::from_black_alpha(70),
        );
        painter.rect_filled(page_rect, 0.0, app.palette.paper);
        let clip = painter.with_clip_rect(page_rect);
        draw_page(&clip, app, origin, zoom);
        clip.rect_stroke(page_rect, 0.0, Stroke::new(1.0, app.palette.stroke), eframe::egui::StrokeKind::Middle);
    }
    let pointer = ui.input(|i| (
        i.pointer.primary_pressed(), i.pointer.primary_down(),
        i.pointer.is_decidedly_dragging(), i.pointer.primary_released(),
        i.pointer.interact_pos().or_else(|| i.pointer.latest_pos()).unwrap_or(Pos2::ZERO),
        i.modifiers.alt, i.modifiers.ctrl,
    ));
    let (pressed, down, dragged, released, pointer_pos, alt, ctrl) = pointer;
    let over_canvas = rect.contains(pointer_pos);
    if over_canvas {
        let doc_pos: Vec2 = (pointer_pos - origin.to_pos2()) / zoom;
        if pressed { app.on_pointer_down(doc_pos, pointer_pos.to_vec2(), alt, ctrl); }
        if down && dragged { app.on_pointer_drag(doc_pos); }
        if released { app.on_pointer_up(doc_pos); }
        let scroll = ui.input(|i| i.smooth_scroll_delta().y);
        if scroll.abs() > 0.01 && over_canvas {
            let before = doc_pos;
            let factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
            app.doc.zoom = (app.doc.zoom * factor).clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);
            let new_origin = pointer_pos - before * app.doc.zoom;
            let page_size = Vec2::new(app.doc.page().width * app.doc.zoom, app.doc.page().height * app.doc.zoom);
            app.doc.pan = new_origin.to_vec2() - (rect.center() - page_size / 2.0).to_vec2();
        }
    }
    if let Some(drag) = &app.drag {
        match drag.kind {
            DragKind::Marquee => {
                let a = (origin + drag.start * zoom).to_pos2();
                let b = (origin + drag.current * zoom).to_pos2();
                let sel = Rect::from_min_max(a.min(b), a.max(b));
                painter.rect_stroke(sel, 0.0, Stroke::new(1.0, app.palette.selection), eframe::egui::StrokeKind::Middle);
                painter.rect_filled(sel, 0.0, app.palette.selection.gamma_multiply(0.15));
            }
            DragKind::Create => {
                let a = (origin + drag.start * zoom).to_pos2();
                let b = (origin + drag.current * zoom).to_pos2();
                let sel = Rect::from_min_max(a.min(b), a.max(b));
                match app.tool {
                    crate::app::Tool::Rectangle => {
                        painter.rect_stroke(sel, 0.0, Stroke::new(1.0, app.fill), eframe::egui::StrokeKind::Middle);
                    }
                    crate::app::Tool::Ellipse => {
                        painter.add(egui::Shape::ellipse_stroke(sel.center(), sel.size() * 0.5, Stroke::new(1.0, app.fill)));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
fn draw_page(painter: &Painter, app: &mut FireCraft, origin: Vec2, zoom: f32) {
    let doc = app.doc.clone();
    for (li, layer) in doc.page().layers.iter().enumerate() {
        if !layer.visible { continue; }
        for (oi, obj) in layer.objects.iter().enumerate() {
            if !obj.visible { continue; }
            let alpha = layer.opacity.clamp(0.0, 1.0);
            draw_obj(painter, app, obj, origin, zoom, alpha, li, oi);
        }
    }
}

fn to_screen(origin: Vec2, zoom: f32, p: Vec2) -> Pos2 {
    pos2(origin.x + p.x * zoom, origin.y + p.y * zoom)
}

fn draw_obj(
    painter: &Painter,
    app: &mut FireCraft,
    obj: &Obj,
    origin: Vec2,
    zoom: f32,
    alpha: f32,
    _layer_index: usize,
    _obj_index: usize,
) {
    let fill = obj.fill.map(|c| c.gamma_multiply(alpha));
    let stroke = obj.stroke.map(|c| c.gamma_multiply(alpha)).unwrap_or(Color32::TRANSPARENT);
    let sw = (obj.width * zoom).max(0.6);
    match &obj.kind {
        Shape::Rect { min, max, .. } => {
            let a = to_screen(origin, zoom, *min);
            let b = to_screen(origin, zoom, *max);
            let r = Rect::from_min_max(a.min(b), a.max(b));
            match (fill, obj.stroke) {
                (Some(f), Some(_)) => {
                    painter.rect_filled(r, 0.0, f);
                    painter.rect_stroke(r, 0.0, Stroke::new(sw, stroke), eframe::egui::StrokeKind::Middle);
                }
                (Some(f), None) => { painter.rect_filled(r, 0.0, f); }
                (None, _) => { painter.rect_stroke(r, 0.0, Stroke::new(sw, stroke), eframe::egui::StrokeKind::Middle); }
            }
        }
        Shape::Ellipse { center, radius, .. } => {
            let c = to_screen(origin, zoom, *center);
            let rr = Vec2::new(radius.x * zoom, radius.y * zoom);
            match (fill, obj.stroke) {
                (Some(f), Some(_)) => {
                    painter.add(egui::Shape::ellipse_filled(c, rr, f));
                    painter.add(egui::Shape::ellipse_stroke(c, rr, Stroke::new(sw, stroke)));
                }
                (Some(f), None) => { painter.add(egui::Shape::ellipse_filled(c, rr, f)); }
                (None, _) => { painter.add(egui::Shape::ellipse_stroke(c, rr, Stroke::new(sw, stroke))); }
            }
        }
        Shape::Line { a, b } => {
            painter.line_segment([to_screen(origin, zoom, *a), to_screen(origin, zoom, *b)], Stroke::new(sw, stroke));
        }
        Shape::Path { points } => {
            if points.len() > 1 {
                let pts: Vec<Pos2> = points.iter().map(|p| to_screen(origin, zoom, *p)).collect();
                painter.add(egui::Shape::closed_line(pts, Stroke::new(sw, stroke)));
            }
        }
        Shape::Text { at, text, size } => {
            let p = to_screen(origin, zoom, *at);
            painter.text(p, eframe::egui::Align2::LEFT_BOTTOM, text, FontId::proportional(size * zoom), fill.unwrap_or(Color32::BLACK));
        }
        Shape::Bitmap { at, pixels, size } => {
            let key = (_layer_index, _obj_index);
            let tex = app.bitmap_texture(key, pixels, *size);
            if let Some(tex) = tex {
                let rect = Rect::from_min_size(to_screen(origin, zoom, *at), Vec2::new(size.x * zoom, size.y * zoom));
                painter.image(tex.id(), rect, Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
            }
        }
    }
    if app.selected_layer == _layer_index && app.selected == Some(_obj_index) {
        if let Some((lo, hi)) = obj.bounds() {
            let a = to_screen(origin, zoom, lo);
            let b = to_screen(origin, zoom, hi);
            let sel = Rect::from_min_max(a.min(b), a.max(b));
            painter.rect_stroke(sel, 0.0, Stroke::new(1.0, app.palette.selection), eframe::egui::StrokeKind::Middle);
            for corner in [sel.left_top(), sel.right_top(), sel.left_bottom(), sel.right_bottom()] {
                painter.rect_filled(Rect::from_center_size(corner, Vec2::splat(4.0)), 0.0, app.palette.selection);
            }
        }
    }
}

pub fn hit_test(app: &FireCraft, p: Vec2) -> Option<(usize, usize)> {
    let doc = &app.doc;
    for (li, layer) in doc.page().layers.iter().enumerate().rev() {
        if !layer.visible { continue; }
        for (oi, obj) in layer.objects.iter().enumerate().rev() {
            if !obj.visible || obj.locked { continue; }
            if hit_obj(obj, p) { return Some((li, oi)); }
        }
    }
    None
}

fn hit_obj(obj: &Obj, p: Vec2) -> bool {
    match &obj.kind {
        Shape::Rect { min, max, .. } => {
            let lo = min.min(*max); let hi = min.max(*max);
            p.x >= lo.x && p.x <= hi.x && p.y >= lo.y && p.y <= hi.y
        }
        Shape::Ellipse { center, radius, .. } => {
            let rx = radius.x.max(0.001); let ry = radius.y.max(0.001);
            let dx = (p.x - center.x) / rx; let dy = (p.y - center.y) / ry;
            dx * dx + dy * dy <= 1.0
        }
        Shape::Line { a, b } => {
            let dir = *b - *a; let len2 = dir.length_sq();
            if len2 <= f32::EPSILON { return (*a - p).length() < 6.0; }
            let t = ((p - *a).dot(dir) / len2).clamp(0.0, 1.0);
            let proj = *a + dir * t; (proj - p).length() < 6.0
        }
        Shape::Path { points } => {
            for w in points.windows(2) {
                let a = w[0]; let b = w[1]; let dir = b - a; let len2 = dir.length_sq();
                if len2 <= f32::EPSILON { continue; }
                let t = ((p - a).dot(dir) / len2).clamp(0.0, 1.0);
                let proj = a + dir * t;
                if (proj - p).length() < 6.0 { return true; }
            }
            false
        }
        Shape::Text { at, size, .. } => {
            let w = 0.55 * size * 8.0;
            p.x >= at.x && p.x <= at.x + w && p.y >= at.y - size * 1.2 && p.y <= at.y
        }
        Shape::Bitmap { at, size, .. } => {
            p.x >= at.x && p.x <= at.x + size.x && p.y >= at.y && p.y <= at.y + size.y
        }
    }
}

pub fn tool_button(
    ui: &mut eframe::egui::Ui,
    icon: icons::Icon,
    active: bool,
    tip: &str,
) -> eframe::egui::Response {
    let size = eframe::egui::vec2(20.0, 20.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if active { painter.rect_filled(rect, 2.0, ui.visuals().widgets.active.bg_fill); }
        else if response.hovered() { painter.rect_filled(rect, 2.0, ui.visuals().widgets.hovered.bg_fill); }
        let color = if active { ui.visuals().widgets.active.weak_bg_fill } else { ui.visuals().text_color() };
        icons::draw(painter, rect, icon, color, 1.2);
    }
    response.on_hover_text(tip)
}

pub type TextureCache = std::collections::HashMap<(usize, usize), TextureHandle>;

