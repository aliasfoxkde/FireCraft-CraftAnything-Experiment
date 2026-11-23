//! The canvas viewport: page rendering, checkerboard transparency, live effects,
//! web slices & hotspots, 8-handle transform boxes, subselection nodes, rulers, grid and guides.

use crate::app::{CanvasViewMode, DragKind, FireCraft, Tool};
use crate::constants;
use crate::doc::{LiveEffect, Obj, Shape};
use crate::icons;
use eframe::egui::{
    self, Align2, Color32, FontId, Painter, Pos2, Rect, Sense, Stroke, TextureHandle, Vec2, pos2,
};

pub fn show(ui: &mut egui::Ui, app: &mut FireCraft) {
    let rect = ui.available_rect_before_wrap();
    let painter = ui.painter().clone();
    painter.rect_filled(rect, 0.0, app.palette.canvas_pad);

    app.canvas_size = rect.size();

    match app.view_mode {
        CanvasViewMode::TwoUp => {
            let half_w = rect.width() * 0.5;
            let left_rect = Rect::from_min_size(rect.min, Vec2::new(half_w - 1.0, rect.height()));
            let right_rect = Rect::from_min_size(
                pos2(rect.min.x + half_w + 1.0, rect.min.y),
                Vec2::new(half_w - 1.0, rect.height()),
            );
            render_viewport(&painter, left_rect, app, false, "Original (PNG32)");
            render_viewport(
                &painter,
                right_rect,
                app,
                true,
                &format!(
                    "Preview ({} · ~{} KB)",
                    app.doc.optimize.format,
                    app.estimated_export_kb()
                ),
            );
            painter.vline(
                rect.min.x + half_w,
                rect.top()..=rect.bottom(),
                Stroke::new(2.0, app.palette.stroke),
            );
        }
        CanvasViewMode::FourUp => {
            let half_w = rect.width() * 0.5;
            let half_h = rect.height() * 0.5;
            let q1 = Rect::from_min_size(rect.min, Vec2::new(half_w - 1.0, half_h - 1.0));
            let q2 = Rect::from_min_size(
                pos2(rect.min.x + half_w + 1.0, rect.min.y),
                Vec2::new(half_w - 1.0, half_h - 1.0),
            );
            let q3 = Rect::from_min_size(
                pos2(rect.min.x, rect.min.y + half_h + 1.0),
                Vec2::new(half_w - 1.0, half_h - 1.0),
            );
            let q4 = Rect::from_min_size(
                pos2(rect.min.x + half_w + 1.0, rect.min.y + half_h + 1.0),
                Vec2::new(half_w - 1.0, half_h - 1.0),
            );
            let kb = app.estimated_export_kb();
            render_viewport(&painter, q1, app, false, "Original");
            render_viewport(
                &painter,
                q2,
                app,
                true,
                &format!("{} ({kb} KB)", app.doc.optimize.format),
            );
            render_viewport(
                &painter,
                q3,
                app,
                true,
                &format!("GIF Web216 ({} KB)", (kb / 2).max(2)),
            );
            render_viewport(
                &painter,
                q4,
                app,
                true,
                &format!("JPEG 60% ({} KB)", (kb / 3).max(1)),
            );
            painter.vline(
                rect.min.x + half_w,
                rect.top()..=rect.bottom(),
                Stroke::new(1.5, app.palette.stroke),
            );
            painter.hline(
                rect.left()..=rect.right(),
                rect.min.y + half_h,
                Stroke::new(1.5, app.palette.stroke),
            );
        }
        CanvasViewMode::Original | CanvasViewMode::Preview => {
            let is_preview = app.view_mode == CanvasViewMode::Preview;
            let badge = if is_preview {
                format!(
                    "Preview: {} · ~{} KB",
                    app.doc.optimize.format,
                    app.estimated_export_kb()
                )
            } else {
                String::new()
            };
            render_viewport(&painter, rect, app, is_preview, &badge);
        }
    }

    let zoom = app.doc.zoom;
    let page = app.doc.page();
    let page_size = Vec2::new(page.width * zoom, page.height * zoom);
    let origin: Vec2 = (rect.center() - page_size / 2.0 + app.doc.pan).to_vec2();

    if app.show_grid {
        draw_grid(
            &painter,
            rect,
            origin,
            zoom,
            app.grid_size,
            app.palette.stroke,
        );
    }
    if app.show_guides {
        draw_guides(&painter, rect, origin, zoom, app);
    }
    if app.show_rulers {
        draw_rulers(&painter, rect, origin, zoom, app.palette);
    }

    let pointer = ui.input(|i| {
        (
            i.pointer.primary_pressed(),
            i.pointer.primary_down(),
            i.pointer.is_decidedly_dragging(),
            i.pointer.primary_released(),
            i.pointer.secondary_clicked(),
            i.pointer
                .interact_pos()
                .or_else(|| i.pointer.latest_pos())
                .unwrap_or(Pos2::ZERO),
            i.modifiers.alt,
            i.modifiers.ctrl,
            i.modifiers.shift,
        )
    });
    let (pressed, down, dragged, released, secondary_clicked, pointer_pos, alt, ctrl, shift) =
        pointer;
    let over_canvas = rect.contains(pointer_pos);
    if over_canvas {
        let doc_pos: Vec2 = (pointer_pos - origin.to_pos2()) / zoom;
        app.cursor_doc_pos = Some(doc_pos);
        if pressed {
            app.context_menu_pos = None;
            app.on_pointer_down(doc_pos, pointer_pos.to_vec2(), alt, ctrl, shift);
        }
        if secondary_clicked {
            // CS6 context-menu pick: if right-click hits an unselected object, select it first
            if let Some((li, oi)) = hit_test(app, doc_pos)
                && (app.selected_layer != li || !app.is_selected(oi))
            {
                app.selected_layer = li;
                app.select_single(Some(oi));
            }
            app.context_menu_pos = Some(pointer_pos);
        }
        if down && dragged {
            app.on_pointer_drag(doc_pos, shift);
        }
        if released {
            app.on_pointer_up(doc_pos, shift);
        }
        let scroll = ui.input(|i| i.smooth_scroll_delta().y);
        if scroll.abs() > 0.01 {
            let before = doc_pos;
            let factor = if scroll > 0.0 { 1.1 } else { 1.0 / 1.1 };
            app.doc.zoom = (app.doc.zoom * factor).clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);
            let new_origin = pointer_pos - before * app.doc.zoom;
            let page_size = Vec2::new(
                app.doc.page().width * app.doc.zoom,
                app.doc.page().height * app.doc.zoom,
            );
            app.doc.pan = new_origin.to_vec2() - (rect.center() - page_size / 2.0).to_vec2();
        }
    } else {
        app.cursor_doc_pos = None;
    }

    if let Some(drag) = &app.drag {
        match drag.kind {
            DragKind::Marquee => {
                let a = (origin + drag.start * zoom).to_pos2();
                let b = (origin + drag.current * zoom).to_pos2();
                let sel = Rect::from_min_max(a.min(b), a.max(b));
                painter.rect_filled(sel, 0.0, app.palette.selection.gamma_multiply(0.15));
                painter.rect_stroke(
                    sel,
                    0.0,
                    Stroke::new(1.0, app.palette.selection),
                    egui::StrokeKind::Middle,
                );
            }
            DragKind::Create => {
                let a = (origin + drag.start * zoom).to_pos2();
                let b = (origin + drag.current * zoom).to_pos2();
                let sel = Rect::from_min_max(a.min(b), a.max(b));
                match app.tool {
                    Tool::Rectangle | Tool::RoundedRect => {
                        let cr = if app.tool == Tool::RoundedRect {
                            12.0 * zoom
                        } else {
                            0.0
                        };
                        painter.rect_stroke(
                            sel,
                            cr,
                            Stroke::new(1.2, app.palette.selection),
                            egui::StrokeKind::Middle,
                        );
                    }
                    Tool::Ellipse => {
                        painter.add(egui::Shape::ellipse_stroke(
                            sel.center(),
                            sel.size() * 0.5,
                            Stroke::new(1.2, app.palette.selection),
                        ));
                    }
                    Tool::Line => {
                        painter.line_segment([a, b], Stroke::new(1.5, app.palette.selection));
                    }
                    Tool::Polygon | Tool::Star => {
                        let center = (drag.start + drag.current) * 0.5;
                        let radius = ((drag.current - drag.start).length() * 0.5).max(2.0);
                        let inner = if app.tool == Tool::Star {
                            app.star_ratio
                        } else {
                            1.0
                        };
                        let pts: Vec<Pos2> =
                            Obj::polygon_vertices(center, radius, app.polygon_sides, inner, 0.0)
                                .into_iter()
                                .map(|p| to_screen(origin, zoom, p))
                                .collect();
                        painter.add(egui::Shape::closed_line(
                            pts,
                            Stroke::new(1.2, app.palette.selection),
                        ));
                    }
                    Tool::Slice | Tool::Hotspot => {
                        let col = if app.tool == Tool::Slice {
                            app.palette.slice_overlay
                        } else {
                            app.palette.hotspot_overlay
                        };
                        painter.rect_filled(sel, 0.0, col);
                        painter.rect_stroke(
                            sel,
                            0.0,
                            Stroke::new(1.2, Color32::from_rgb(0, 230, 120)),
                            egui::StrokeKind::Middle,
                        );
                    }
                    _ => {}
                }
            }
            DragKind::Paint | DragKind::Path if drag.path.len() > 1 => {
                let pts: Vec<Pos2> = drag
                    .path
                    .iter()
                    .map(|p| to_screen(origin, zoom, *p))
                    .collect();
                painter.add(egui::Shape::line(
                    pts,
                    Stroke::new((app.brush_size * zoom).max(1.0), app.fill),
                ));
            }
            _ => {}
        }
    }

    canvas_context_menu(ui, app);
}

fn canvas_context_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    let Some(menu_pos) = app.context_menu_pos else {
        return;
    };
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.context_menu_pos = None;
        return;
    }
    let has_sel = app.selected.is_some();
    let multi_sel = app.selected_set.len() >= 2;
    let is_group = app
        .selected_object()
        .is_some_and(|o| matches!(o.kind, Shape::Group { .. }));
    let area_resp = egui::Area::new(ui.make_persistent_id("cs6_canvas_context_menu"))
        .fixed_pos(menu_pos)
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::menu(ui.style())
                .fill(app.palette.panel)
                .stroke(Stroke::new(1.0, app.palette.stroke))
                .inner_margin(egui::Margin::same(4))
                .show(ui, |ui| {
                    ui.set_min_width(185.0);
                    if ui
                        .add_enabled(
                            has_sel,
                            egui::Button::new("Cut                      Ctrl+X"),
                        )
                        .clicked()
                    {
                        app.cut_selected();
                        app.context_menu_pos = None;
                    }
                    if ui
                        .add_enabled(
                            has_sel,
                            egui::Button::new("Copy                     Ctrl+C"),
                        )
                        .clicked()
                    {
                        app.copy_selected();
                        app.context_menu_pos = None;
                    }
                    if ui
                        .add_enabled(
                            app.clipboard.is_some(),
                            egui::Button::new("Paste                    Ctrl+V"),
                        )
                        .clicked()
                    {
                        app.paste();
                        app.context_menu_pos = None;
                    }
                    if has_sel {
                        if ui.button("Duplicate                Ctrl+D").clicked() {
                            app.duplicate_selected();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Delete                   Del").clicked() {
                            app.delete_selected();
                            app.context_menu_pos = None;
                        }
                        ui.separator();
                        if multi_sel && ui.button("Group Objects            Ctrl+G").clicked() {
                            app.group_selected();
                            app.context_menu_pos = None;
                        }
                        if is_group && ui.button("Ungroup            Ctrl+Shift+G").clicked() {
                            app.ungroup_selected();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Bring to Front").clicked() {
                            app.reorder_selected(true);
                            app.context_menu_pos = None;
                        }
                        if ui.button("Send to Back").clicked() {
                            app.reorder_selected(false);
                            app.context_menu_pos = None;
                        }
                        ui.separator();
                        if ui.button("Flip Horizontal").clicked() {
                            app.flip_horizontal();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Flip Vertical").clicked() {
                            app.flip_vertical();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Rotate 90° CW").clicked() {
                            app.rotate_90();
                            app.context_menu_pos = None;
                        }
                        ui.separator();
                        if ui.button("Convert to Path").clicked() {
                            app.convert_selected_to_path();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Convert to Symbol        F8").clicked() {
                            app.convert_selection_to_symbol();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Add Drop Shadow").clicked() {
                            app.add_live_effect(LiveEffect::DropShadow {
                                distance: 5.0,
                                angle_deg: 45.0,
                                blur: 5.0,
                                color: Color32::from_black_alpha(150),
                                enabled: true,
                            });
                            app.context_menu_pos = None;
                        }
                        if ui.button("Insert Web Slice").clicked() {
                            app.insert_slice(false);
                            app.context_menu_pos = None;
                        }
                        if ui.button("Copy CSS Properties").clicked() {
                            app.copy_selected_css();
                            app.context_menu_pos = None;
                        }
                    } else {
                        ui.separator();
                        if ui.button("Select All               Ctrl+A").clicked() {
                            app.select_all();
                            app.context_menu_pos = None;
                        }
                        if ui.button("New Layer").clicked() {
                            app.add_layer();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Import Image / SVG…      Ctrl+R").clicked() {
                            app.begin_file_dialog(crate::app::FileMode::Import);
                            app.context_menu_pos = None;
                        }
                        if ui.button("Export Document…   Ctrl+Shift+R").clicked() {
                            app.begin_file_dialog(crate::app::FileMode::Export);
                            app.context_menu_pos = None;
                        }
                        if ui.button("Canvas Size…").clicked() {
                            app.open_canvas_size_dialog();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Fit Canvas to Objects").clicked() {
                            app.fit_canvas_to_contents();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Fit in Window            Ctrl+0").clicked() {
                            app.fit_in_window();
                            app.context_menu_pos = None;
                        }
                        if ui.button("Load CS6 Demo Showcase").clicked() {
                            app.load_demo_showcase();
                            app.context_menu_pos = None;
                        }
                    }
                });
        });

    let clicked_outside = ui.input(|i| {
        i.pointer.any_pressed()
            && i.pointer
                .interact_pos()
                .is_some_and(|p| !area_resp.response.rect.contains(p))
    });
    if clicked_outside && !ui.input(|i| i.pointer.secondary_pressed()) {
        app.context_menu_pos = None;
    }
}

fn render_viewport(
    painter: &Painter,
    viewport_rect: Rect,
    app: &mut FireCraft,
    preview_mode: bool,
    badge: &str,
) {
    let clip_vp = painter.with_clip_rect(viewport_rect);
    let zoom = app.doc.zoom;
    let page = app.doc.page();
    let page_size = Vec2::new(page.width * zoom, page.height * zoom);
    let origin: Vec2 = (viewport_rect.center() - page_size / 2.0 + app.doc.pan).to_vec2();
    let page_rect = Rect::from_min_size(origin.to_pos2(), page_size);

    clip_vp.rect_filled(
        page_rect.translate(Vec2::new(4.0, 4.0)),
        0.0,
        Color32::from_black_alpha(95),
    );

    let clip = clip_vp.with_clip_rect(page_rect.intersect(viewport_rect));
    if page.transparent_bg {
        draw_checkerboard(&clip, page_rect);
    } else {
        let bg = if preview_mode && !app.doc.optimize.transparency {
            app.doc.optimize.matte
        } else {
            page.bg_color
        };
        clip.rect_filled(page_rect, 0.0, bg);
    }

    draw_page(&clip, app, origin, zoom, preview_mode);
    clip_vp.rect_stroke(
        page_rect,
        0.0,
        Stroke::new(1.0, app.palette.stroke),
        egui::StrokeKind::Middle,
    );

    if !badge.is_empty() {
        let badge_rect = Rect::from_min_size(
            pos2(viewport_rect.left() + 8.0, viewport_rect.top() + 8.0),
            Vec2::new(badge.len() as f32 * 6.5 + 16.0, 20.0),
        );
        clip_vp.rect_filled(badge_rect, 3.0, Color32::from_black_alpha(175));
        clip_vp.text(
            badge_rect.center(),
            Align2::CENTER_CENTER,
            badge,
            FontId::proportional(11.0),
            Color32::WHITE,
        );
    }
}

fn draw_checkerboard(painter: &Painter, rect: Rect) {
    painter.rect_filled(rect, 0.0, Color32::from_rgb(245, 245, 245));
    let tile = 12.0_f32;
    let cols = (rect.width() / tile).ceil() as i32;
    let rows = (rect.height() / tile).ceil() as i32;
    let dark = Color32::from_rgb(212, 212, 212);
    for r in 0..rows.min(180) {
        for c in 0..cols.min(240) {
            if (r + c) % 2 == 1 {
                let cell = Rect::from_min_size(
                    pos2(rect.left() + c as f32 * tile, rect.top() + r as f32 * tile),
                    Vec2::splat(tile),
                )
                .intersect(rect);
                painter.rect_filled(cell, 0.0, dark);
            }
        }
    }
}

fn draw_grid(painter: &Painter, rect: Rect, origin: Vec2, zoom: f32, step: f32, stroke: Color32) {
    let screen_step = step.max(4.0) * zoom;
    if screen_step < 8.0 {
        return;
    }
    let first_x = ((rect.left() - origin.x) / screen_step).floor() as i64;
    let last_x = ((rect.right() - origin.x) / screen_step).ceil() as i64;
    let first_y = ((rect.top() - origin.y) / screen_step).floor() as i64;
    let last_y = ((rect.bottom() - origin.y) / screen_step).ceil() as i64;
    let line = Stroke::new(0.5, stroke.gamma_multiply(0.45));
    for i in first_x..=last_x {
        let x = origin.x + i as f32 * screen_step;
        painter.vline(x, rect.top()..=rect.bottom(), line);
    }
    for i in first_y..=last_y {
        let y = origin.y + i as f32 * screen_step;
        painter.hline(rect.left()..=rect.right(), y, line);
    }
}

fn draw_guides(painter: &Painter, rect: Rect, origin: Vec2, zoom: f32, app: &FireCraft) {
    let stroke = Stroke::new(1.0, app.palette.guide);
    for &gy in &app.doc.page().guides_h {
        let y = origin.y + gy * zoom;
        if y >= rect.top() && y <= rect.bottom() {
            painter.hline(rect.left()..=rect.right(), y, stroke);
        }
    }
    for &gx in &app.doc.page().guides_v {
        let x = origin.x + gx * zoom;
        if x >= rect.left() && x <= rect.right() {
            painter.vline(x, rect.top()..=rect.bottom(), stroke);
        }
    }
}

fn draw_rulers(
    painter: &Painter,
    rect: Rect,
    origin: Vec2,
    zoom: f32,
    palette: crate::theme::Palette,
) {
    const RULER: f32 = 16.0;
    let strip_h = Rect::from_min_max(
        pos2(rect.left(), rect.top()),
        pos2(rect.right(), rect.top() + RULER),
    );
    let strip_v = Rect::from_min_max(
        pos2(rect.left(), rect.top()),
        pos2(rect.left() + RULER, rect.bottom()),
    );
    painter.rect_filled(strip_h, 0.0, palette.panel);
    painter.rect_filled(strip_v, 0.0, palette.panel);

    let mut step = 10.0_f32;
    while step * zoom < 35.0 {
        step *= 5.0;
    }
    let line = Stroke::new(0.5, palette.text_dim);

    let start_x = ((rect.left() - origin.x) / (step * zoom)).floor() as i64;
    let end_x = ((rect.right() - origin.x) / (step * zoom)).ceil() as i64;
    for i in start_x..=end_x {
        let x = origin.x + i as f32 * step * zoom;
        if x < strip_h.left() + RULER || x > strip_h.right() {
            continue;
        }
        painter.vline(x, strip_h.top()..=strip_h.bottom(), line);
        painter.text(
            pos2(x + 2.0, strip_h.bottom() - 2.0),
            Align2::LEFT_BOTTOM,
            format!("{:.0}", i as f32 * step),
            FontId::proportional(9.0),
            palette.text_dim,
        );
    }

    let start_y = ((rect.top() - origin.y) / (step * zoom)).floor() as i64;
    let end_y = ((rect.bottom() - origin.y) / (step * zoom)).ceil() as i64;
    for i in start_y..=end_y {
        let y = origin.y + i as f32 * step * zoom;
        if y < strip_v.top() + RULER || y > strip_v.bottom() {
            continue;
        }
        painter.hline(strip_v.left()..=strip_v.right(), y, line);
        painter.text(
            pos2(strip_v.right() - 2.0, y - 2.0),
            Align2::RIGHT_BOTTOM,
            format!("{:.0}", i as f32 * step),
            FontId::proportional(9.0),
            palette.text_dim,
        );
    }
}

fn draw_page(painter: &Painter, app: &mut FireCraft, origin: Vec2, zoom: f32, preview_mode: bool) {
    let doc = app.doc.clone();
    if app.onion_skin && !preview_mode && doc.page().states.len() > 1 {
        let cur = doc.page().current_state;
        if cur > 0 {
            for layer in &doc.page().states[cur - 1].layers {
                if !layer.visible || layer.is_web_layer {
                    continue;
                }
                for (oi, obj) in layer.objects.iter().enumerate() {
                    if !obj.visible {
                        continue;
                    }
                    draw_obj(painter, app, obj, origin, zoom, 0.22, (0, oi), true);
                }
            }
        }
    }

    for (li, layer) in doc.page().layers.iter().enumerate() {
        if !layer.visible {
            continue;
        }
        if layer.is_web_layer && (preview_mode || !app.show_slices) {
            continue;
        }
        for (oi, obj) in layer.objects.iter().enumerate() {
            if !obj.visible {
                continue;
            }
            let alpha = (layer.opacity * obj.opacity).clamp(0.0, 1.0);
            draw_obj(
                painter,
                app,
                obj,
                origin,
                zoom,
                alpha,
                (li, oi),
                preview_mode,
            );
        }
    }
}

pub fn to_screen(origin: Vec2, zoom: f32, p: Vec2) -> Pos2 {
    pos2(origin.x + p.x * zoom, origin.y + p.y * zoom)
}

#[allow(clippy::too_many_arguments)]
fn draw_obj(
    painter: &Painter,
    app: &mut FireCraft,
    obj: &Obj,
    origin: Vec2,
    zoom: f32,
    alpha: f32,
    key: (usize, usize),
    preview_mode: bool,
) {
    let (layer_index, obj_index) = key;
    let fill = obj.fill.map(|c| c.gamma_multiply(alpha));
    let stroke = obj
        .stroke
        .map(|c| c.gamma_multiply(alpha))
        .unwrap_or(Color32::TRANSPARENT);
    let sw = (obj.width * zoom).max(0.6);

    // Draw Live Effects (Drop Shadow, Outer Glow) underneath the object
    if let Some((lo, hi)) = obj.bounds() {
        let a = to_screen(origin, zoom, lo);
        let b = to_screen(origin, zoom, hi);
        let base_rect = Rect::from_min_max(a.min(b), a.max(b));
        let cr = match &obj.kind {
            Shape::Rect { corner_radius, .. } => *corner_radius * zoom,
            Shape::Ellipse { .. } => base_rect.width().min(base_rect.height()) * 0.5,
            _ => 0.0,
        };
        for eff in &obj.effects {
            if !eff.is_enabled() {
                continue;
            }
            match eff {
                LiveEffect::DropShadow {
                    distance,
                    angle_deg,
                    blur,
                    color,
                    ..
                } => {
                    let rad = angle_deg.to_radians();
                    let offset = Vec2::new(distance * rad.cos(), distance * rad.sin()) * zoom;
                    let steps = (blur.ceil() as i32).clamp(1, 6);
                    for s in (1..=steps).rev() {
                        let expand = (s as f32 / steps as f32) * blur * zoom * 0.5;
                        let c = color.gamma_multiply(alpha / (steps as f32 * 0.9));
                        painter.rect_filled(
                            base_rect.translate(offset).expand(expand),
                            cr + expand,
                            c,
                        );
                    }
                }
                LiveEffect::Glow {
                    radius,
                    color,
                    inner: false,
                    ..
                } => {
                    let steps = 4;
                    for s in (1..=steps).rev() {
                        let expand = (s as f32 / steps as f32) * radius * zoom * 0.6;
                        let c = color.gamma_multiply(alpha * 0.35);
                        painter.rect_filled(base_rect.expand(expand), cr + expand, c);
                    }
                }
                _ => {}
            }
        }
    }

    match &obj.kind {
        Shape::Rect {
            min,
            max,
            corner_radius,
            ..
        } => {
            let a = to_screen(origin, zoom, *min);
            let b = to_screen(origin, zoom, *max);
            let r = Rect::from_min_max(a.min(b), a.max(b));
            let cr = *corner_radius * zoom;
            if let Some(f) = fill {
                painter.rect_filled(r, cr, f);
            }
            if obj.stroke.is_some() && obj.width > 0.0 {
                painter.rect_stroke(r, cr, Stroke::new(sw, stroke), egui::StrokeKind::Middle);
            }
        }
        Shape::Ellipse { center, radius, .. } => {
            let c = to_screen(origin, zoom, *center);
            let rr = Vec2::new(radius.x.abs() * zoom, radius.y.abs() * zoom);
            if let Some(f) = fill {
                painter.add(egui::Shape::ellipse_filled(c, rr, f));
            }
            if obj.stroke.is_some() && obj.width > 0.0 {
                painter.add(egui::Shape::ellipse_stroke(c, rr, Stroke::new(sw, stroke)));
            }
        }
        Shape::Line { a, b } => {
            painter.line_segment(
                [to_screen(origin, zoom, *a), to_screen(origin, zoom, *b)],
                Stroke::new(sw, stroke),
            );
        }
        Shape::Polygon {
            center,
            radius,
            sides,
            star_inner,
            angle_rad,
        } => {
            let pts: Vec<Pos2> =
                Obj::polygon_vertices(*center, *radius, *sides, *star_inner, *angle_rad)
                    .into_iter()
                    .map(|p| to_screen(origin, zoom, p))
                    .collect();
            if let Some(f) = fill {
                if (*star_inner - 1.0).abs() <= 0.02 {
                    painter.add(egui::Shape::convex_polygon(
                        pts.clone(),
                        f,
                        Stroke::new(sw, stroke),
                    ));
                } else {
                    let c = to_screen(origin, zoom, *center);
                    for i in 0..pts.len() {
                        painter.add(egui::Shape::convex_polygon(
                            vec![c, pts[i], pts[(i + 1) % pts.len()]],
                            f,
                            Stroke::NONE,
                        ));
                    }
                    if obj.stroke.is_some() {
                        painter.add(egui::Shape::closed_line(pts, Stroke::new(sw, stroke)));
                    }
                }
            } else {
                painter.add(egui::Shape::closed_line(pts, Stroke::new(sw, stroke)));
            }
        }
        Shape::Path { points, closed } => {
            if points.len() > 1 {
                let pts: Vec<Pos2> = points.iter().map(|p| to_screen(origin, zoom, *p)).collect();
                if *closed {
                    if let Some(f) = fill {
                        painter.add(egui::Shape::convex_polygon(
                            pts.clone(),
                            f,
                            Stroke::new(sw, stroke),
                        ));
                    } else {
                        painter.add(egui::Shape::closed_line(pts, Stroke::new(sw, stroke)));
                    }
                } else {
                    painter.add(egui::Shape::line(pts, Stroke::new(sw, stroke)));
                }
            }
        }
        Shape::Text { at, text, size, .. } => {
            let p = to_screen(origin, zoom, *at);
            painter.text(
                p,
                Align2::LEFT_BOTTOM,
                text,
                FontId::proportional((size * zoom).max(6.0)),
                fill.unwrap_or(Color32::BLACK),
            );
        }
        Shape::Gradient {
            min,
            max,
            from,
            to,
            radial,
        } => {
            let a = to_screen(origin, zoom, *min);
            let b = to_screen(origin, zoom, *max);
            let r = Rect::from_min_max(a.min(b), a.max(b));
            if *radial {
                let steps = 24;
                let max_rad = r.width().min(r.height()) * 0.5;
                for i in (0..steps).rev() {
                    let t = i as f32 / steps as f32;
                    let c = from.lerp_to_gamma(*to, t).gamma_multiply(alpha);
                    painter.circle_filled(r.center(), max_rad * (t + 1.0 / steps as f32), c);
                }
            } else {
                let steps = 48.max((r.height() as i32).min(128));
                for i in 0..steps {
                    let t = i as f32 / steps as f32;
                    let c = from.lerp_to_gamma(*to, t).gamma_multiply(alpha);
                    let y0 = r.top() + r.height() * t;
                    let y1 = r.top() + r.height() * ((i + 1) as f32 / steps as f32);
                    painter.rect_filled(
                        Rect::from_min_max(pos2(r.left(), y0), pos2(r.right(), y1 + 1.0)),
                        0.0,
                        c,
                    );
                }
            }
            if obj.stroke.is_some() && obj.width > 0.0 {
                painter.rect_stroke(r, 0.0, Stroke::new(sw, stroke), egui::StrokeKind::Middle);
            }
        }
        Shape::Bitmap { at, pixels, size } => {
            let tex = app.bitmap_texture(key, pixels, *size);
            if let Some(tex) = tex {
                let rect = Rect::from_min_size(
                    to_screen(origin, zoom, *at),
                    Vec2::new(size.x * zoom, size.y * zoom),
                );
                painter.image(
                    tex.id(),
                    rect,
                    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                    Color32::from_white_alpha((alpha * 255.0) as u8),
                );
            }
        }
        Shape::Slice {
            min,
            max,
            url,
            is_hotspot,
            ..
        } => {
            if !preview_mode && app.show_slices {
                let a = to_screen(origin, zoom, *min);
                let b = to_screen(origin, zoom, *max);
                let r = Rect::from_min_max(a.min(b), a.max(b));
                let overlay = if *is_hotspot {
                    app.palette.hotspot_overlay
                } else {
                    app.palette.slice_overlay
                };
                let border = if *is_hotspot {
                    Color32::from_rgb(0, 195, 255)
                } else {
                    Color32::from_rgb(0, 225, 110)
                };
                painter.rect_filled(r, 0.0, overlay);
                painter.rect_stroke(r, 0.0, Stroke::new(1.0, border), egui::StrokeKind::Middle);
                let label = if url.is_empty() {
                    obj.name.as_str()
                } else {
                    url.as_str()
                };
                painter.text(
                    pos2(r.left() + 4.0, r.top() + 3.0),
                    Align2::LEFT_TOP,
                    label,
                    FontId::proportional(10.0),
                    Color32::WHITE,
                );
            }
        }
        Shape::SymbolInstance {
            symbol_id,
            at,
            scale,
        } => {
            let a = to_screen(origin, zoom, *at);
            let s = Vec2::new(64.0 * scale.x * zoom, 64.0 * scale.y * zoom);
            let r = Rect::from_min_size(a, s);
            painter.rect_filled(r, 2.0, fill.unwrap_or(Color32::from_rgb(70, 130, 210)));
            painter.rect_stroke(
                r,
                2.0,
                Stroke::new(1.5, app.palette.accent),
                egui::StrokeKind::Middle,
            );
            painter.text(
                r.center(),
                Align2::CENTER_CENTER,
                format!("SYM #{}", symbol_id + 1),
                FontId::proportional(10.0),
                Color32::WHITE,
            );
        }
        Shape::Group { children } => {
            for (ci, ch) in children.iter().enumerate() {
                draw_obj(
                    painter,
                    app,
                    ch,
                    origin,
                    zoom,
                    alpha,
                    (layer_index, obj_index * 1000 + ci + 1),
                    preview_mode,
                );
            }
        }
    }

    // Draw Inner Bevel / Inner Shadow effects on top
    if let Some((lo, hi)) = obj.bounds() {
        let a = to_screen(origin, zoom, lo);
        let b = to_screen(origin, zoom, hi);
        let base_rect = Rect::from_min_max(a.min(b), a.max(b));
        for eff in &obj.effects {
            if !eff.is_enabled() {
                continue;
            }
            if let LiveEffect::Bevel {
                width,
                highlight,
                shadow,
                ..
            } = eff
            {
                let bw = (*width * zoom).max(1.0);
                painter.line_segment(
                    [base_rect.left_top(), base_rect.right_top()],
                    Stroke::new(bw, *highlight),
                );
                painter.line_segment(
                    [base_rect.left_top(), base_rect.left_bottom()],
                    Stroke::new(bw, *highlight),
                );
                painter.line_segment(
                    [base_rect.left_bottom(), base_rect.right_bottom()],
                    Stroke::new(bw, *shadow),
                );
                painter.line_segment(
                    [base_rect.right_top(), base_rect.right_bottom()],
                    Stroke::new(bw, *shadow),
                );
            }
        }
    }

    // Selection handles (8-point resize box for every selected object in `selected_set`)
    if !preview_mode && app.selected_layer == layer_index && app.is_selected(obj_index) {
        if let Some((lo, hi)) = obj.bounds() {
            let a = to_screen(origin, zoom, lo);
            let b = to_screen(origin, zoom, hi);
            let sel = Rect::from_min_max(a.min(b), a.max(b));
            painter.rect_stroke(
                sel,
                0.0,
                Stroke::new(1.0, app.palette.selection),
                egui::StrokeKind::Middle,
            );
            for handle in selection_handles(sel) {
                let h_rect = Rect::from_center_size(handle, Vec2::splat(6.0));
                painter.rect_filled(h_rect, 0.0, Color32::WHITE);
                painter.rect_stroke(
                    h_rect,
                    0.0,
                    Stroke::new(1.2, app.palette.selection),
                    egui::StrokeKind::Middle,
                );
            }
        }
        if app.tool == Tool::Subselect
            && let Shape::Path { points, .. } = &obj.kind
        {
            for p in points {
                let sp = to_screen(origin, zoom, *p);
                let nr = Rect::from_center_size(sp, Vec2::splat(6.0));
                painter.rect_filled(nr, 1.0, app.palette.accent);
                painter.rect_stroke(
                    nr,
                    1.0,
                    Stroke::new(1.0, Color32::WHITE),
                    egui::StrokeKind::Middle,
                );
            }
        }
    }
}

pub fn selection_handles(sel: Rect) -> [Pos2; 8] {
    [
        sel.left_top(),
        pos2(sel.center().x, sel.top()),
        sel.right_top(),
        pos2(sel.right(), sel.center().y),
        sel.right_bottom(),
        pos2(sel.center().x, sel.bottom()),
        sel.left_bottom(),
        pos2(sel.left(), sel.center().y),
    ]
}

/// Check if `doc_pos` hits one of the 8 resize handles of the currently selected object.
pub fn hit_test_handle(app: &FireCraft, doc_pos: Vec2) -> Option<usize> {
    let idx = app.selected?;
    let obj = app
        .doc
        .page()
        .layers
        .get(app.selected_layer)
        .and_then(|l| l.objects.get(idx))?;
    let (lo, hi) = obj.bounds()?;
    let sel = Rect::from_min_max(lo.to_pos2(), hi.to_pos2());
    let tol = 7.0 / app.doc.zoom.max(0.1);
    for (i, h) in selection_handles(sel).iter().enumerate() {
        if (h.to_vec2() - doc_pos).length() <= tol {
            return Some(i);
        }
    }
    None
}

pub fn hit_test(app: &FireCraft, p: Vec2) -> Option<(usize, usize)> {
    let doc = &app.doc;
    for (li, layer) in doc.page().layers.iter().enumerate().rev() {
        if !layer.visible || layer.locked {
            continue;
        }
        if layer.is_web_layer && !app.show_slices {
            continue;
        }
        for (oi, obj) in layer.objects.iter().enumerate().rev() {
            if !obj.visible || obj.locked {
                continue;
            }
            if hit_obj(obj, p) {
                return Some((li, oi));
            }
        }
    }
    None
}

fn hit_obj(obj: &Obj, p: Vec2) -> bool {
    match &obj.kind {
        Shape::Rect { min, max, .. }
        | Shape::Gradient { min, max, .. }
        | Shape::Slice { min, max, .. } => {
            let lo = min.min(*max);
            let hi = min.max(*max);
            p.x >= lo.x && p.x <= hi.x && p.y >= lo.y && p.y <= hi.y
        }
        Shape::Ellipse { center, radius, .. } => {
            let rx = radius.x.abs().max(0.001);
            let ry = radius.y.abs().max(0.001);
            let dx = (p.x - center.x) / rx;
            let dy = (p.y - center.y) / ry;
            dx * dx + dy * dy <= 1.0
        }
        Shape::Polygon { center, radius, .. } => (p - *center).length() <= radius.max(4.0),
        Shape::Line { a, b } => {
            let dir = *b - *a;
            let len2 = dir.length_sq();
            if len2 <= f32::EPSILON {
                return (*a - p).length() < 6.0;
            }
            let t = ((p - *a).dot(dir) / len2).clamp(0.0, 1.0);
            let proj = *a + dir * t;
            (proj - p).length() < 6.0
        }
        Shape::Path { points, closed } => {
            if *closed
                && let Some((lo, hi)) = obj.bounds()
                && p.x >= lo.x
                && p.x <= hi.x
                && p.y >= lo.y
                && p.y <= hi.y
            {
                return true;
            }
            for w in points.windows(2) {
                let a = w[0];
                let b = w[1];
                let dir = b - a;
                let len2 = dir.length_sq();
                if len2 <= f32::EPSILON {
                    continue;
                }
                let t = ((p - a).dot(dir) / len2).clamp(0.0, 1.0);
                let proj = a + dir * t;
                if (proj - p).length() < 6.0 {
                    return true;
                }
            }
            false
        }
        Shape::Text { at, text, size, .. } => {
            let w = crate::doc::text_width(text, *size);
            p.x >= at.x && p.x <= at.x + w && p.y >= at.y - size * 1.2 && p.y <= at.y
        }
        Shape::Bitmap { at, size, .. } => {
            p.x >= at.x && p.x <= at.x + size.x && p.y >= at.y && p.y <= at.y + size.y
        }
        Shape::SymbolInstance { at, scale, .. } => {
            let s = Vec2::new(64.0 * scale.x.abs(), 64.0 * scale.y.abs());
            p.x >= at.x && p.x <= at.x + s.x && p.y >= at.y && p.y <= at.y + s.y
        }
        Shape::Group { children } => children.iter().any(|ch| hit_obj(ch, p)),
    }
}

pub fn tool_button(
    ui: &mut egui::Ui,
    icon: icons::Icon,
    active: bool,
    tip: &str,
) -> egui::Response {
    let size = egui::vec2(22.0, 22.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if active {
            painter.rect_filled(rect, 0.0, ui.visuals().widgets.active.bg_fill);
            painter.rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, ui.visuals().selection.stroke.color),
                egui::StrokeKind::Middle,
            );
        } else if response.hovered() {
            painter.rect_filled(rect, 0.0, ui.visuals().widgets.hovered.bg_fill);
            painter.rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, ui.visuals().widgets.hovered.bg_stroke.color),
                egui::StrokeKind::Middle,
            );
        }
        let color = if active {
            Color32::WHITE
        } else {
            ui.visuals().text_color()
        };
        icons::draw(painter, rect, icon, color, 1.25);
    }
    response.on_hover_text(tip)
}

pub type TextureCache = std::collections::HashMap<(usize, usize), TextureHandle>;
