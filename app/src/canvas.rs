//! The canvas viewport: page rendering, checkerboard transparency, live effects,
//! web slices & hotspots, 8-handle transform boxes, subselection nodes, rulers, grid and guides.

use crate::app::{CanvasViewMode, DragKind, FireCraft, Tool};
use crate::constants;
use crate::doc::{GradientFill, LiveEffect, Obj, Shape};
use crate::geometry::{point_in_polygon, triangulate_simple_polygon};
use crate::icons;
use eframe::egui::{
    self, Align2, Color32, FontId, Painter, Pos2, Rect, Sense, Stroke, TextureHandle, Vec2, pos2,
};

/// Shorten `text` with an ellipsis so it fits `max_w` at `font`. Used for slice URL tabs,
/// which otherwise paint a 400 px bar across the artwork.
fn elide_to_width(painter: &egui::Painter, text: &str, font: &FontId, max_w: f32) -> String {
    let width = |s: &str| {
        painter
            .layout_no_wrap(s.to_string(), font.clone(), Color32::WHITE)
            .size()
            .x
    };
    if width(text) <= max_w {
        return text.to_string();
    }
    let mut chars: Vec<char> = text.chars().collect();
    while !chars.is_empty() {
        chars.pop();
        let candidate = format!("{}…", chars.iter().collect::<String>());
        if width(&candidate) <= max_w {
            return candidate;
        }
    }
    "…".to_string()
}

pub fn show(ui: &mut egui::Ui, app: &mut FireCraft) {
    let rect = ui.available_rect_before_wrap();
    let canvas_resp = ui.allocate_rect(rect, Sense::click_and_drag());
    if std::mem::take(&mut app.focus_canvas) {
        canvas_resp.request_focus();
    }
    let painter = ui.painter().clone();
    painter.rect_filled(rect, 0.0, app.palette.canvas_pad);

    app.canvas_size = rect.size();
    if std::mem::take(&mut app.pending_fit) {
        app.apply_fit_from_canvas();
    }

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
        draw_rulers(
            &painter,
            rect,
            origin,
            zoom,
            app.palette,
            app.ruler_unit,
            app.doc.dpi as f32,
        );
        let corner = Rect::from_min_size(rect.min, Vec2::splat(16.0));
        if ui
            .interact(
                corner,
                ui.make_persistent_id("ruler_unit_corner"),
                Sense::click(),
            )
            .on_hover_text("Click to cycle Ruler Units (px / in / cm / mm)")
            .clicked()
        {
            let all = crate::app::RulerUnit::ALL;
            let cur = all.iter().position(|&u| u == app.ruler_unit).unwrap_or(0);
            app.ruler_unit = all[(cur + 1) % all.len()];
            app.status = format!("Ruler units: {}", app.ruler_unit.label());
        }
    }

    let pointer = ui.input(|i| {
        (
            i.pointer.primary_pressed(),
            i.pointer.primary_down(),
            i.pointer.is_decidedly_dragging(),
            i.pointer.primary_released(),
            i.pointer.secondary_clicked(),
            i.pointer.button_down(egui::PointerButton::Middle),
            i.pointer.delta(),
            i.pointer
                .interact_pos()
                .or_else(|| i.pointer.latest_pos())
                .unwrap_or(Pos2::ZERO),
            i.modifiers.alt,
            i.modifiers.ctrl,
            i.modifiers.shift,
            i.key_down(egui::Key::Space),
            i.smooth_scroll_delta(),
            i.zoom_delta(),
        )
    });
    let (
        pressed,
        down,
        dragged,
        released,
        secondary_clicked,
        middle_down,
        pointer_delta,
        pointer_pos,
        alt,
        ctrl,
        shift,
        space_down,
        scroll_vec,
        egui_zoom_delta,
    ) = pointer;

    if !alt {
        app.alt_hand_pan = false;
    }

    // A native child viewport may deliver a press inside this canvas before egui marks the
    // canvas response hovered. The event is already scoped to this viewport, so its local bounds
    // are sufficient to begin the gesture; app.drag then keeps routing the active drag.
    let over_canvas = (canvas_resp.hovered() || app.drag.is_some() || pressed)
        && rect.contains(pointer_pos)
        && !app.show_welcome
        && app.dragging_panel.is_none()
        && app.dragging_group.is_none()
        && app.active_menu.is_none();
    if over_canvas {
        let doc_pos: Vec2 = (pointer_pos - origin.to_pos2()) / zoom;
        app.cursor_doc_pos = Some(doc_pos);

        let hand_mode_active =
            app.tool == Tool::Hand || middle_down || space_down || app.alt_hand_pan;
        if hand_mode_active {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        } else if app.tool == Tool::Path {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
        }

        // Direct 2D Hand panning via Middle-Mouse drag, Space+drag, Hand Tool drag, or Alt+Scroll Hand mode
        if (middle_down
            || (space_down && down)
            || (app.tool == Tool::Hand && down)
            || app.alt_hand_pan)
            && pointer_delta.length_sq() > 0.01
        {
            app.doc.pan += pointer_delta;
            ui.ctx().request_repaint();
        } else {
            if pressed {
                app.context_menu_pos = None;
                if app.tool == Tool::Path {
                    // Pen paths are built from individual clicks. A double-click commits the
                    // open contour; clicking the first node closes it (handled in pen_add_node).
                    if canvas_resp.double_clicked() {
                        app.finish_pen_path(false);
                    } else {
                        app.pen_add_node(doc_pos);
                    }
                } else {
                    app.on_pointer_down(doc_pos, pointer_pos.to_vec2(), alt, ctrl, shift);
                }
            }
            if canvas_resp.double_clicked()
                && let Some((li, oi)) = hit_test(app, doc_pos)
                && matches!(
                    app.doc.page().layers[li].objects[oi].kind,
                    Shape::Text { .. }
                )
            {
                app.selected_layer = li;
                app.select_single(Some(oi));
                app.inline_text_edit_obj = Some(oi);
            }
            if secondary_clicked || canvas_resp.secondary_clicked() {
                if app.tool == Tool::Path && !app.pen_nodes.is_empty() {
                    app.finish_pen_path(false);
                } else {
                    // CS6 context-menu pick: if right-click hits an unselected object, select it first
                    if let Some((li, oi)) = hit_test(app, doc_pos)
                        && (app.selected_layer != li || !app.is_selected(oi))
                    {
                        app.selected_layer = li;
                        app.select_single(Some(oi));
                    }
                    app.context_menu_pos = Some(pointer_pos);
                }
            }
            if down && dragged {
                app.on_pointer_drag(doc_pos, shift);
            }
            if released {
                app.on_pointer_up(doc_pos, shift);
            }
        }

        // Mouse wheel handling:
        // - Ctrl + Scroll: Zoom in and out anchored at mouse cursor
        // - Alt + Scroll: Activate Hand pan mode (move around freely while Alt is held)
        // - Shift + Scroll: Scroll horizontally (left/right) when zoomed in enough to work
        // - Normal Scroll: Scroll vertically (up/down) when zoomed in enough to move around
        let wheel_delta = if scroll_vec.y.abs() >= scroll_vec.x.abs() {
            scroll_vec.y
        } else {
            scroll_vec.x
        };
        let has_egui_zoom = (egui_zoom_delta - 1.0).abs() > 0.001;
        if has_egui_zoom || scroll_vec.length_sq() > 0.05 {
            if ctrl || has_egui_zoom {
                let before = doc_pos;
                let factor = if has_egui_zoom {
                    egui_zoom_delta
                } else if wheel_delta > 0.0 {
                    1.12
                } else {
                    1.0 / 1.12
                };
                app.doc.zoom =
                    (app.doc.zoom * factor).clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);
                let new_page_size = Vec2::new(
                    app.doc.page().width * app.doc.zoom,
                    app.doc.page().height * app.doc.zoom,
                );
                if !app.infinite_canvas
                    && new_page_size.x <= rect.width() - 48.0
                    && new_page_size.y <= rect.height() - 48.0
                {
                    // Gently ease pan toward center when zoomed out smaller than viewport.
                    // God view keeps the user's chosen position instead of snapping back.
                    app.doc.pan *= 0.45;
                    if app.doc.pan.length() < 2.0 {
                        app.doc.pan = Vec2::ZERO;
                    }
                } else {
                    let new_origin = pointer_pos - before * app.doc.zoom;
                    app.doc.pan =
                        new_origin.to_vec2() - (rect.center() - new_page_size / 2.0).to_vec2();
                }
                app.status = format!("Zoom: {:.0}% (Ctrl+Scroll)", app.doc.zoom * 100.0);
            } else if alt {
                app.alt_hand_pan = true;
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                app.doc.pan += Vec2::new(scroll_vec.x, scroll_vec.y) * 1.25;
                app.status =
                    "Hand Pan active (move mouse while holding Alt, or middle-drag to pan)"
                        .to_string();
            } else if shift {
                let cur_page_w = app.doc.page().width * app.doc.zoom;
                let can_scroll_h = cur_page_w > rect.width() - 64.0
                    || app.doc.pan.x.abs() > 1.0
                    || app.infinite_canvas;
                if can_scroll_h {
                    let max_pan_x = if app.infinite_canvas {
                        100_000.0
                    } else {
                        ((cur_page_w - rect.width() * 0.55) * 0.5 + 140.0).max(60.0)
                    };
                    app.doc.pan.x =
                        (app.doc.pan.x + wheel_delta * 1.4).clamp(-max_pan_x, max_pan_x);
                    app.status = format!(
                        "Horizontal scroll: X {:+.0} px (Shift+Scroll)",
                        -app.doc.pan.x
                    );
                } else {
                    app.doc.pan.x = 0.0;
                    app.status =
                        "Canvas fits horizontally — Ctrl+Scroll to zoom in before horizontal scrolling"
                            .to_string();
                }
            } else {
                let cur_page_w = app.doc.page().width * app.doc.zoom;
                let cur_page_h = app.doc.page().height * app.doc.zoom;
                let can_scroll_v = cur_page_h > rect.height() - 64.0
                    || app.doc.pan.y.abs() > 1.0
                    || app.infinite_canvas;
                let can_scroll_h = cur_page_w > rect.width() - 64.0
                    || app.doc.pan.x.abs() > 1.0
                    || app.infinite_canvas;
                if can_scroll_v {
                    let max_pan_y = if app.infinite_canvas {
                        100_000.0
                    } else {
                        ((cur_page_h - rect.height() * 0.55) * 0.5 + 140.0).max(60.0)
                    };
                    app.doc.pan.y =
                        (app.doc.pan.y + scroll_vec.y * 1.4).clamp(-max_pan_y, max_pan_y);
                } else {
                    app.doc.pan.y = 0.0;
                }
                if can_scroll_h && scroll_vec.x.abs() > 0.1 {
                    let max_pan_x = if app.infinite_canvas {
                        100_000.0
                    } else {
                        ((cur_page_w - rect.width() * 0.55) * 0.5 + 140.0).max(60.0)
                    };
                    app.doc.pan.x =
                        (app.doc.pan.x + scroll_vec.x * 1.4).clamp(-max_pan_x, max_pan_x);
                }
                if !can_scroll_v && !can_scroll_h {
                    app.status =
                        "Canvas fits in window — use Ctrl+Scroll to zoom in/out, or Alt+Scroll for Hand pan"
                            .to_string();
                }
            }
            ui.ctx().request_repaint();
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
                if app.tool == Tool::OvalMarquee {
                    painter.add(egui::Shape::ellipse_filled(
                        sel.center(),
                        sel.size() * 0.5,
                        app.palette.selection.gamma_multiply(0.15),
                    ));
                    painter.add(egui::Shape::ellipse_stroke(
                        sel.center(),
                        sel.size() * 0.5,
                        Stroke::new(1.0, app.palette.selection),
                    ));
                } else {
                    painter.rect_filled(sel, 0.0, app.palette.selection.gamma_multiply(0.15));
                    painter.rect_stroke(
                        sel,
                        0.0,
                        Stroke::new(1.0, app.palette.selection),
                        egui::StrokeKind::Middle,
                    );
                }
            }
            DragKind::Lasso if drag.path.len() > 1 => {
                let pts: Vec<Pos2> = drag
                    .path
                    .iter()
                    .map(|p| to_screen(origin, zoom, *p))
                    .collect();
                if pts.len() >= 3 {
                    painter.add(egui::Shape::convex_polygon(
                        pts.clone(),
                        app.palette.selection.gamma_multiply(0.14),
                        Stroke::NONE,
                    ));
                }
                painter.add(egui::Shape::closed_line(
                    pts,
                    Stroke::new(1.2, app.palette.selection),
                ));
            }
            DragKind::Create => {
                let a = (origin + drag.start * zoom).to_pos2();
                let b = (origin + drag.current * zoom).to_pos2();
                let sel = Rect::from_min_max(a.min(b), a.max(b));
                match app.tool {
                    Tool::Rectangle | Tool::RoundedRect | Tool::Gradient => {
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
            DragKind::Paint if drag.path.len() > 1 => {
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

    // In-progress Pen contour: placed anchors, rubber-band segment to the pointer, and a
    // highlighted first anchor that closes the path when clicked.
    if app.tool == Tool::Path && !app.pen_nodes.is_empty() {
        let mut pts: Vec<Pos2> = app
            .pen_nodes
            .iter()
            .map(|p| to_screen(origin, zoom, *p))
            .collect();
        if let Some(cursor) = app.cursor_doc_pos {
            pts.push(to_screen(origin, zoom, cursor));
        }
        if pts.len() > 1 {
            painter.add(egui::Shape::line(
                pts.clone(),
                Stroke::new(1.2, app.palette.accent),
            ));
        }
        for (i, sp) in pts.iter().enumerate().take(app.pen_nodes.len()) {
            let r = Rect::from_center_size(*sp, Vec2::splat(if i == 0 { 7.0 } else { 5.0 }));
            painter.rect_filled(r, 1.0, app.palette.accent);
            painter.rect_stroke(
                r,
                1.0,
                Stroke::new(1.0, Color32::WHITE),
                egui::StrokeKind::Middle,
            );
        }
    }

    inline_text_editor(ui, app, origin, zoom);
    canvas_context_menu(ui, app);
}

fn inline_text_editor(ui: &mut egui::Ui, app: &mut FireCraft, origin: Vec2, zoom: f32) {
    let Some(obj_idx) = app.inline_text_edit_obj else {
        return;
    };
    let layer = app.selected_layer;
    let Some((at, mut text, size, fill)) = app
        .doc
        .page()
        .layers
        .get(layer)
        .and_then(|l| l.objects.get(obj_idx))
        .and_then(|o| match &o.kind {
            Shape::Text { at, text, size, .. } => {
                Some((*at, text.clone(), *size, o.fill.unwrap_or(app.fill)))
            }
            _ => None,
        })
    else {
        app.inline_text_edit_obj = None;
        return;
    };

    let text_h = (size * zoom).clamp(11.0, 72.0);
    let screen_pos = to_screen(origin, zoom, at);
    let area_pos = pos2(screen_pos.x, screen_pos.y - text_h);
    let mut finish = ui.input(|i| {
        i.key_pressed(egui::Key::Escape) || (i.modifiers.ctrl && i.key_pressed(egui::Key::Enter))
    });
    let area_resp = egui::Area::new(ui.make_persistent_id(("inline_text_edit", obj_idx)))
        .fixed_pos(area_pos)
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(Color32::TRANSPARENT)
                .stroke(Stroke::new(1.0, app.palette.accent))
                .inner_margin(egui::Margin::symmetric(3, 1))
                .show(ui, |ui| {
                    let te = egui::TextEdit::multiline(&mut text)
                        .font(FontId::proportional(text_h))
                        .text_color(fill)
                        .frame(egui::Frame::NONE)
                        .desired_width(260.0)
                        .desired_rows(1);
                    let resp = ui.add(te);
                    if !resp.has_focus() {
                        resp.request_focus();
                    }
                    resp
                })
                .inner
        });

    if let Some(obj) = app
        .doc
        .page_mut()
        .layers
        .get_mut(layer)
        .and_then(|l| l.objects.get_mut(obj_idx))
    {
        if let Shape::Text { text: t, .. } = &mut obj.kind {
            *t = text.clone();
        }
        if !text.trim().is_empty() {
            obj.name = text.clone();
        }
    }

    let clicked_outside = ui.input(|i| {
        i.pointer.primary_pressed()
            && i.pointer
                .interact_pos()
                .is_some_and(|p| !area_resp.response.rect.expand(6.0).contains(p))
    });
    if clicked_outside {
        finish = true;
    }

    if finish {
        if text.trim().is_empty()
            && let Some(l) = app.doc.page_mut().layers.get_mut(layer)
            && obj_idx < l.objects.len()
        {
            l.objects.remove(obj_idx);
            app.clear_selection();
        }
        app.doc.page_mut().sync_state();
        app.inline_text_edit_obj = None;
        app.set_tool(Tool::Select);
    }
}

fn ctx_menu_row(
    ui: &mut egui::Ui,
    p: crate::theme::Palette,
    enabled: bool,
    label: &str,
    shortcut: &str,
) -> bool {
    let width = ui.available_width().max(252.0);
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 22.0), Sense::click());
    if enabled && resp.hovered() {
        ui.painter()
            .rect_filled(rect, 0.0, p.selection.gamma_multiply(0.32));
        ui.painter().rect_stroke(
            rect,
            0.0,
            Stroke::new(1.0, p.selection),
            egui::StrokeKind::Inside,
        );
    }
    let fg = if enabled {
        p.text
    } else {
        p.text_dim.gamma_multiply(0.55)
    };
    ui.painter().text(
        pos2(rect.left() + 12.0, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        FontId::proportional(11.5),
        fg,
    );
    if !shortcut.is_empty() {
        ui.painter().text(
            pos2(rect.right() - 10.0, rect.center().y),
            Align2::RIGHT_CENTER,
            shortcut,
            FontId::proportional(10.5),
            if enabled { p.text_dim } else { fg },
        );
    }
    enabled && resp.clicked()
}

fn canvas_context_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    let Some(menu_pos) = app.context_menu_pos else {
        return;
    };
    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        app.context_menu_pos = None;
        return;
    }
    let sel_obj = app.selected_object();
    let has_sel = sel_obj.is_some();
    let multi_sel = app.selected_set.len() >= 2;
    let is_group = sel_obj
        .as_ref()
        .is_some_and(|o| matches!(o.kind, Shape::Group { .. }));
    let is_text = sel_obj
        .as_ref()
        .is_some_and(|o| matches!(o.kind, Shape::Text { .. }));
    let is_bitmap = sel_obj
        .as_ref()
        .is_some_and(|o| matches!(o.kind, Shape::Bitmap { .. }));
    let is_slice = sel_obj
        .as_ref()
        .is_some_and(|o| matches!(o.kind, Shape::Slice { .. }));
    let is_path = sel_obj
        .as_ref()
        .is_some_and(|o| matches!(o.kind, Shape::Path { .. }));
    let is_locked = sel_obj.as_ref().is_some_and(|o| o.locked);

    let p = app.palette;
    let screen = ui.ctx().content_rect();
    let est_h = if has_sel { 440.0 } else { 290.0 };
    let clamped_pos = pos2(
        menu_pos
            .x
            .clamp(screen.left() + 4.0, (screen.right() - 250.0).max(4.0)),
        menu_pos
            .y
            .clamp(screen.top() + 4.0, (screen.bottom() - est_h).max(4.0)),
    );
    let mut close_menu = false;
    let area_resp = egui::Area::new(ui.make_persistent_id("cs6_canvas_context_menu"))
        .fixed_pos(clamped_pos)
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::menu(ui.style())
                .fill(p.panel)
                .stroke(Stroke::new(1.0, p.stroke))
                .inner_margin(egui::Margin::same(3))
                .show(ui, |ui| {
                    ui.set_min_width(256.0);
                    ui.spacing_mut().item_spacing = Vec2::new(0.0, 1.0);
                    if ctx_menu_row(ui, p, !app.undo.is_empty(), "Undo", "Ctrl+Z") {
                        app.undo_action();
                        close_menu = true;
                    }
                    if ctx_menu_row(ui, p, !app.redo.is_empty(), "Redo", "Ctrl+Y") {
                        app.redo_action();
                        close_menu = true;
                    }
                    ui.separator();
                    if ctx_menu_row(ui, p, has_sel, "Cut", "Ctrl+X") {
                        app.cut_selected();
                        close_menu = true;
                    }
                    if ctx_menu_row(ui, p, has_sel, "Copy", "Ctrl+C") {
                        app.copy_selected();
                        close_menu = true;
                    }
                    if ctx_menu_row(ui, p, has_sel, "Copy CSS Properties", "Ctrl+Shift+C") {
                        app.copy_selected_css();
                        close_menu = true;
                    }
                    if ctx_menu_row(ui, p, true, "Paste", "Ctrl+V") {
                        app.paste();
                        close_menu = true;
                    }
                    if has_sel
                        && app.clipboard.is_some()
                        && ctx_menu_row(ui, p, true, "Paste Attributes", "Ctrl+Alt+Shift+V")
                    {
                        app.paste_attributes();
                        close_menu = true;
                    }
                    if has_sel {
                        if ctx_menu_row(ui, p, true, "Duplicate", "Ctrl+D") {
                            app.duplicate_selected();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Clone in Place", "Ctrl+Shift+D") {
                            app.clone_selected();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Delete", "Del") {
                            app.delete_selected();
                            close_menu = true;
                        }
                        ui.separator();
                        if multi_sel && ctx_menu_row(ui, p, true, "Group Objects", "Ctrl+G") {
                            app.group_selected();
                            close_menu = true;
                        }
                        if is_group && ctx_menu_row(ui, p, true, "Ungroup", "Ctrl+Shift+G") {
                            app.ungroup_selected();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Bring to Front", "Ctrl+Shift+↑") {
                            app.reorder_selected(true);
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Bring Forward", "Ctrl+↑") {
                            app.reorder_selected_step(true);
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Send Backward", "Ctrl+↓") {
                            app.reorder_selected_step(false);
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Send to Back", "Ctrl+Shift+↓") {
                            app.reorder_selected(false);
                            close_menu = true;
                        }
                        ui.separator();
                        if is_text {
                            if ctx_menu_row(ui, p, true, "Toggle Bold", "Ctrl+B") {
                                app.toggle_selected_text_bold();
                                close_menu = true;
                            }
                            if ctx_menu_row(ui, p, true, "Toggle Italic", "Ctrl+I") {
                                app.toggle_selected_text_italic();
                                close_menu = true;
                            }
                        } else if is_bitmap {
                            if ctx_menu_row(ui, p, true, "Sharpen Bitmap", "") {
                                app.run_bitmap_filter(Tool::Sharpen);
                                close_menu = true;
                            }
                            if ctx_menu_row(ui, p, true, "Blur Bitmap", "") {
                                app.run_bitmap_filter(Tool::Blur);
                                close_menu = true;
                            }
                        } else if is_slice {
                            if ctx_menu_row(ui, p, true, "Assign URL…", "") {
                                app.focus_or_open_panel(crate::app::PanelId::Url);
                                close_menu = true;
                            }
                            if ctx_menu_row(ui, p, true, "Attach Behavior…", "Shift+F3") {
                                app.focus_or_open_panel(crate::app::PanelId::Behaviors);
                                close_menu = true;
                            }
                        }
                        if ctx_menu_row(ui, p, true, "Flip Horizontal", "") {
                            app.flip_horizontal();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Flip Vertical", "") {
                            app.flip_vertical();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Rotate 90° CW", "") {
                            app.rotate_90();
                            close_menu = true;
                        }
                        ui.separator();
                        if is_path && ctx_menu_row(ui, p, true, "Edit Path Points (Subselect)", "A")
                        {
                            app.set_tool(Tool::Subselect);
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, !is_path, "Convert to Path", "") {
                            app.convert_selected_to_path();
                            close_menu = true;
                        }
                        if multi_sel || is_path {
                            ui.menu_button("  Combine Paths  ▸", |ui| {
                                for (op, lbl, needs_two) in [
                                    ("union", "Union", true),
                                    ("intersect", "Intersect", true),
                                    ("punch", "Punch", true),
                                    ("crop", "Crop", true),
                                    ("join", "Join Endpoints", false),
                                    ("split", "Split Path", false),
                                ] {
                                    if ui
                                        .add_enabled(
                                            !needs_two || multi_sel,
                                            egui::Button::new(lbl),
                                        )
                                        .clicked()
                                    {
                                        app.combine_paths_op(op);
                                        close_menu = true;
                                        ui.close();
                                    }
                                }
                            });
                        }
                        if multi_sel {
                            ui.menu_button("  Align Selection  ▸", |ui| {
                                for (op, lbl) in [
                                    ("left", "Align Left"),
                                    ("center_h", "Align Horizontal Center"),
                                    ("right", "Align Right"),
                                    ("top", "Align Top"),
                                    ("center_v", "Align Vertical Center"),
                                    ("bottom", "Align Bottom"),
                                ] {
                                    if ui.button(lbl).clicked() {
                                        app.align_selected_to_canvas(op);
                                        close_menu = true;
                                        ui.close();
                                    }
                                }
                            });
                        }
                        if ctx_menu_row(ui, p, true, "Snap To Pixel", "") {
                            app.snap_selected_to_pixel();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Flatten Selection", "Ctrl+Alt+Shift+Z") {
                            app.flatten_selected_to_bitmap();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Convert to Symbol…", "F8") {
                            app.convert_selection_to_symbol();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Add Live Drop Shadow", "") {
                            app.add_live_effect(LiveEffect::DropShadow {
                                distance: 5.0,
                                angle_deg: 45.0,
                                blur: 5.0,
                                color: Color32::from_black_alpha(150),
                                enabled: true,
                            });
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Insert Web Slice", "") {
                            app.insert_slice(false);
                            close_menu = true;
                        }
                        if ctx_menu_row(
                            ui,
                            p,
                            true,
                            if is_locked {
                                "Unlock Selection"
                            } else {
                                "Lock Selection"
                            },
                            "Ctrl+L",
                        ) {
                            app.toggle_lock_selection();
                            close_menu = true;
                        }
                    } else {
                        ui.separator();
                        if ctx_menu_row(ui, p, true, "Select All", "Ctrl+A") {
                            app.select_all();
                            close_menu = true;
                        }
                        ui.menu_button("  Insert Shape / Element  ▸", |ui| {
                            if ui.button("Rounded Rectangle").clicked() {
                                app.insert_auto_shape(crate::doc::AutoShapeKind::RoundedRect);
                                close_menu = true;
                                ui.close();
                            }
                            if ui.button("Star").clicked() {
                                app.insert_auto_shape(crate::doc::AutoShapeKind::Star);
                                close_menu = true;
                                ui.close();
                            }
                            if ui.button("Primary Button").clicked() {
                                app.insert_common_library_item("Primary Button");
                                close_menu = true;
                                ui.close();
                            }
                            if ui.button("Web Slice").clicked() {
                                app.insert_slice(false);
                                close_menu = true;
                                ui.close();
                            }
                            if ui.button("Hotspot").clicked() {
                                app.insert_slice(true);
                                close_menu = true;
                                ui.close();
                            }
                        });
                        if ctx_menu_row(ui, p, true, "New Layer", "") {
                            app.add_layer();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Import Image / SVG…", "Ctrl+R") {
                            app.begin_file_dialog(crate::app::FileMode::Import);
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Export Document…", "Ctrl+Shift+R") {
                            app.begin_file_dialog(crate::app::FileMode::Export);
                            close_menu = true;
                        }
                        ui.separator();
                        if ctx_menu_row(ui, p, true, "Canvas Size…", "") {
                            app.open_canvas_size_dialog();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Fit Canvas to Contents", "Ctrl+Alt+F") {
                            app.fit_canvas_to_contents();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Fit in Window", "Insert / Ctrl+0") {
                            app.fit_in_window();
                            close_menu = true;
                        }
                        if ctx_menu_row(ui, p, true, "Zoom 100%", "Ctrl+1") {
                            app.set_zoom(1.0);
                            close_menu = true;
                        }
                        ui.separator();
                        if ctx_menu_row(
                            ui,
                            p,
                            true,
                            if app.show_rulers {
                                "✓ Rulers"
                            } else {
                                "  Rulers"
                            },
                            "Ctrl+Alt+R",
                        ) {
                            app.show_rulers = !app.show_rulers;
                            close_menu = true;
                        }
                        if ctx_menu_row(
                            ui,
                            p,
                            true,
                            if app.show_grid { "✓ Grid" } else { "  Grid" },
                            "Ctrl+Alt+G",
                        ) {
                            app.show_grid = !app.show_grid;
                            close_menu = true;
                        }
                        if ctx_menu_row(
                            ui,
                            p,
                            true,
                            if app.show_guides {
                                "✓ Guides"
                            } else {
                                "  Guides"
                            },
                            "Ctrl+;",
                        ) {
                            app.show_guides = !app.show_guides;
                            close_menu = true;
                        }
                        ui.separator();
                        if ctx_menu_row(ui, p, true, "Load Studio Showcase Demo", "") {
                            app.load_demo_showcase();
                            close_menu = true;
                        }
                    }
                });
        });

    if close_menu {
        app.context_menu_pos = None;
        return;
    }

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
    // God view is an editing view: it deliberately exposes the whole object graph. Preview
    // remains an export-faithful rendering, so it never leaks out-of-page or hidden content.
    let god_view = app.infinite_canvas && !preview_mode;
    let page = app.doc.page();
    let page_size = Vec2::new(page.width * zoom, page.height * zoom);
    let origin: Vec2 = (viewport_rect.center() - page_size / 2.0 + app.doc.pan).to_vec2();
    let page_rect = Rect::from_min_size(origin.to_pos2(), page_size);

    if god_view {
        // Extend the page surface to the entire viewport. Opaque pages fill with their own color;
        // transparent pages show the checkerboard everywhere. No page clip is applied, so every
        // object (including off-page and hidden elements) is visible while panning.
        if page.transparent_bg {
            draw_checkerboard(&clip_vp, viewport_rect);
        } else {
            clip_vp.rect_filled(viewport_rect, 0.0, page.bg_color);
        }
    } else {
        clip_vp.rect_filled(
            page_rect.translate(Vec2::new(4.0, 4.0)),
            0.0,
            Color32::from_black_alpha(95),
        );
        if page.transparent_bg {
            draw_checkerboard(&clip_vp, page_rect);
        } else {
            let bg = if preview_mode && !app.doc.optimize.transparency {
                app.doc.optimize.matte
            } else {
                page.bg_color
            };
            clip_vp.rect_filled(page_rect, 0.0, bg);
        }
    }

    let clip = if god_view {
        clip_vp.clone()
    } else {
        clip_vp.with_clip_rect(page_rect.intersect(viewport_rect))
    };
    draw_page(&clip, app, origin, zoom, preview_mode, god_view);
    if god_view {
        // Keep the page usable as an export artifact: dotted bounds, subtle page wash, and a
        // compact legend. The wash is a border only; the interior remains the page color.
        draw_dotted_rect(
            &clip_vp,
            page_rect,
            Stroke::new(1.4, app.palette.accent),
            7.0,
            4.0,
        );
        let label_pos = pos2(page_rect.left() + 2.0, page_rect.top() - 6.0);
        if label_pos.y > viewport_rect.top() + 10.0 {
            clip_vp.text(
                label_pos,
                Align2::LEFT_BOTTOM,
                "PAGE BOUNDS · flat exports crop here",
                FontId::proportional(10.0),
                app.palette.accent,
            );
        }
        let hidden = app.hidden_object_count();
        let legend = if hidden > 0 {
            format!("GOD VIEW · all layers · {hidden} hidden element(s) ghosted")
        } else {
            "GOD VIEW · all layers and off-page elements visible".to_string()
        };
        let legend_rect = Rect::from_min_size(
            pos2(viewport_rect.left() + 8.0, viewport_rect.bottom() - 26.0),
            Vec2::new(legend.chars().count() as f32 * 6.1 + 18.0, 19.0),
        );
        clip_vp.rect_filled(legend_rect, 3.0, Color32::from_black_alpha(165));
        clip_vp.rect_stroke(
            legend_rect,
            3.0,
            Stroke::new(1.0, app.palette.accent.gamma_multiply(0.8)),
            egui::StrokeKind::Inside,
        );
        clip_vp.text(
            legend_rect.center(),
            Align2::CENTER_CENTER,
            legend,
            FontId::proportional(10.0),
            app.palette.text,
        );
    } else {
        clip_vp.rect_stroke(
            page_rect,
            0.0,
            Stroke::new(1.0, app.palette.stroke),
            egui::StrokeKind::Middle,
        );
    }

    if !badge.is_empty() {
        let badge_font = FontId::proportional(11.0);
        let badge_text = elide_to_width(
            &clip_vp,
            badge,
            &badge_font,
            (viewport_rect.width() - 32.0).max(24.0),
        );
        let text_width = clip_vp
            .layout_no_wrap(badge_text.clone(), badge_font.clone(), Color32::WHITE)
            .size()
            .x;
        let badge_rect = Rect::from_min_size(
            pos2(viewport_rect.left() + 8.0, viewport_rect.top() + 8.0),
            Vec2::new(
                (text_width + 16.0).min((viewport_rect.width() - 16.0).max(0.0)),
                20.0,
            ),
        );
        clip_vp.rect_filled(badge_rect, 3.0, Color32::from_black_alpha(175));
        clip_vp.text(
            badge_rect.center(),
            Align2::CENTER_CENTER,
            badge_text,
            badge_font,
            Color32::WHITE,
        );
    }
}

/// Paints a solid, optionally expanded silhouette of `obj` for shadows and glows. Expansion is
/// applied in document space so the falloff stays proportional at every zoom level.
fn paint_silhouette(
    painter: &Painter,
    obj: &Obj,
    origin: Vec2,
    zoom: f32,
    color: Color32,
    expand_screen: f32,
    offset: Vec2,
) {
    let expand_doc = expand_screen / zoom.max(0.01);
    match &obj.kind {
        Shape::Rect {
            min,
            max,
            corner_radius,
            ..
        } => {
            let a = to_screen(origin, zoom, *min) + offset;
            let b = to_screen(origin, zoom, *max) + offset;
            let r = Rect::from_min_max(a.min(b), a.max(b)).expand(expand_screen);
            painter.rect_filled(r, (*corner_radius * zoom + expand_screen).max(0.0), color);
        }
        Shape::Ellipse { center, radius, .. } => {
            let c = to_screen(origin, zoom, *center) + offset;
            let rr = Vec2::new(
                (radius.x.abs() + expand_doc) * zoom,
                (radius.y.abs() + expand_doc) * zoom,
            );
            painter.add(egui::Shape::ellipse_filled(c, rr, color));
        }
        Shape::Polygon {
            center,
            radius,
            sides,
            star_inner,
            angle_rad,
        } => {
            let pts: Vec<Pos2> = Obj::polygon_vertices(
                *center,
                *radius + expand_doc,
                *sides,
                *star_inner,
                *angle_rad,
            )
            .into_iter()
            .map(|p| to_screen(origin, zoom, p) + offset)
            .collect();
            let c = to_screen(origin, zoom, *center) + offset;
            fill_polygon(painter, &pts, *star_inner, c, color);
        }
        Shape::Path { points, closed } => {
            if points.len() > 1 {
                let bounds = obj.bounds();
                let center = bounds.map(|(lo, hi)| (lo + hi) * 0.5).unwrap_or_default();
                let scaled_points: Vec<Vec2> = points
                    .iter()
                    .map(|p| {
                        let dir = *p - center;
                        *p + dir.normalized() * expand_doc
                    })
                    .collect();
                let pts: Vec<Pos2> = scaled_points
                    .iter()
                    .map(|point| to_screen(origin, zoom, *point) + offset)
                    .collect();
                if *closed {
                    if let Some(triangles) = triangulate_simple_polygon(&scaled_points) {
                        paint_solid_mesh(painter, &pts, &triangles, color);
                    }
                } else {
                    painter.add(egui::Shape::line(
                        pts,
                        Stroke::new((obj.width * zoom + expand_screen * 2.0).max(1.0), color),
                    ));
                }
            }
        }
        _ => {
            if let Some((lo, hi)) = obj.bounds() {
                let a = to_screen(origin, zoom, lo) + offset;
                let b = to_screen(origin, zoom, hi) + offset;
                let r = Rect::from_min_max(a.min(b), a.max(b)).expand(expand_screen);
                painter.rect_filled(r, 0.0, color);
            }
        }
    }
}

fn fill_polygon(painter: &Painter, pts: &[Pos2], star_inner: f32, center: Pos2, color: Color32) {
    if (star_inner - 1.0).abs() <= 0.02 {
        painter.add(egui::Shape::convex_polygon(
            pts.to_vec(),
            color,
            Stroke::NONE,
        ));
    } else {
        // A star is concave. Painting each fan triangle as a separate antialiased shape leaves
        // thin background-colored seams from its center to the vertices. Keep the fan in one mesh
        // so adjacent triangles share exactly the same edge coverage.
        let mut mesh = egui::Mesh::default();
        mesh.colored_vertex(center, color);
        for point in pts {
            mesh.colored_vertex(*point, color);
        }
        for i in 0..pts.len() {
            mesh.add_triangle(0, i as u32 + 1, ((i + 1) % pts.len()) as u32 + 1);
        }
        painter.add(egui::Shape::mesh(mesh));
    }
}

/// Draws a vertical linear gradient in bands, clipping each band to `bounds`. The caller draws
/// the actual shape inside the clip, so rectangles, ellipses, polygons, and paths all get a
/// gradient that respects their silhouette.
#[cfg(test)]
fn paint_gradient(
    painter: &Painter,
    bounds: Rect,
    gradient: GradientFill,
    alpha: f32,
    mut draw: impl FnMut(&Painter, Color32),
) {
    if bounds.height() <= 0.5 {
        draw(painter, gradient.start.gamma_multiply(alpha));
        return;
    }
    let steps = 32;
    let clip = painter.clip_rect();
    for i in 0..steps {
        let t = (i as f32 + 0.5) / steps as f32;
        let color = gradient
            .start
            .lerp_to_gamma(gradient.end, gradient.t_at(Vec2::new(0.5, t)))
            .gamma_multiply(alpha);
        let y0 = bounds.top() + bounds.height() * (i as f32 / steps as f32);
        let y1 = bounds.top() + bounds.height() * ((i + 1) as f32 / steps as f32) + 0.6;
        let band = Rect::from_min_max(
            pos2(bounds.left() - 1.0, y0),
            pos2(bounds.right() + 1.0, y1),
        );
        let clipped = painter.with_clip_rect(band.intersect(clip));
        draw(&clipped, color);
    }
}

fn paint_gradient_mesh(
    painter: &Painter,
    bounds: Rect,
    points: &[Pos2],
    triangles: &[[usize; 3]],
    gradient: GradientFill,
    alpha: f32,
) {
    if points.is_empty() || triangles.is_empty() {
        return;
    }
    let width = bounds.width().max(f32::EPSILON);
    let height = bounds.height().max(f32::EPSILON);
    let mut mesh = egui::Mesh::default();
    for point in points {
        let normalized = Vec2::new(
            (point.x - bounds.left()) / width,
            (point.y - bounds.top()) / height,
        );
        let color = gradient
            .start
            .lerp_to_gamma(gradient.end, gradient.t_at(normalized))
            .gamma_multiply(alpha);
        mesh.colored_vertex(*point, color);
    }
    for [a, b, c] in triangles {
        mesh.add_triangle(*a as u32, *b as u32, *c as u32);
    }
    painter.add(egui::Shape::mesh(mesh));
}

fn paint_solid_mesh(painter: &Painter, points: &[Pos2], triangles: &[[usize; 3]], color: Color32) {
    if points.is_empty() || triangles.is_empty() {
        return;
    }
    let mut mesh = egui::Mesh::default();
    for point in points {
        mesh.colored_vertex(*point, color);
    }
    for [a, b, c] in triangles {
        mesh.add_triangle(*a as u32, *b as u32, *c as u32);
    }
    painter.add(egui::Shape::mesh(mesh));
}

fn gradient_fan_triangles(point_count: usize) -> Vec<[usize; 3]> {
    (0..point_count)
        .map(|index| [0, index + 1, ((index + 1) % point_count) + 1])
        .collect()
}

fn rounded_rect_gradient_points(rect: Rect, radius: f32) -> Vec<Pos2> {
    let radius = radius.clamp(0.0, rect.width().min(rect.height()) * 0.5);
    if radius <= 0.5 {
        return vec![
            rect.center(),
            rect.left_top(),
            rect.right_top(),
            rect.right_bottom(),
            rect.left_bottom(),
        ];
    }
    let corners = [
        (pos2(rect.right() - radius, rect.top() + radius), -90.0_f32),
        (pos2(rect.right() - radius, rect.bottom() - radius), 0.0),
        (pos2(rect.left() + radius, rect.bottom() - radius), 90.0),
        (pos2(rect.left() + radius, rect.top() + radius), 180.0),
    ];
    let mut points = Vec::with_capacity(1 + corners.len() * 8);
    points.push(rect.center());
    let segments = 8;
    for (center, start_deg) in corners {
        for i in 0..segments {
            let angle = (start_deg + 90.0 * i as f32 / (segments - 1) as f32).to_radians();
            points.push(pos2(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            ));
        }
    }
    points
}

fn gradient_handle_points(obj: &Obj, gradient: GradientFill) -> Option<(Vec2, Vec2)> {
    let (lo, hi) = obj.bounds()?;
    let size = hi - lo;
    Some((
        lo + Vec2::new(size.x * gradient.start_pos.x, size.y * gradient.start_pos.y),
        lo + Vec2::new(size.x * gradient.end_pos.x, size.y * gradient.end_pos.y),
    ))
}

fn draw_dotted_rect(painter: &Painter, rect: Rect, stroke: Stroke, dash: f32, gap: f32) {
    let step = (dash + gap).max(2.0);
    let mut x = rect.left();
    while x < rect.right() {
        let x2 = (x + dash).min(rect.right());
        painter.line_segment([pos2(x, rect.top()), pos2(x2, rect.top())], stroke);
        painter.line_segment([pos2(x, rect.bottom()), pos2(x2, rect.bottom())], stroke);
        x += step;
    }
    let mut y = rect.top();
    while y < rect.bottom() {
        let y2 = (y + dash).min(rect.bottom());
        painter.line_segment([pos2(rect.left(), y), pos2(rect.left(), y2)], stroke);
        painter.line_segment([pos2(rect.right(), y), pos2(rect.right(), y2)], stroke);
        y += step;
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
    unit: crate::app::RulerUnit,
    dpi: f32,
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

    let px_per_unit = unit.px_per_unit(dpi);
    let candidates: &[f32] = match unit {
        crate::app::RulerUnit::Pixels => &[10.0, 25.0, 50.0, 100.0, 250.0, 500.0, 1000.0],
        crate::app::RulerUnit::Inches => &[0.25, 0.5, 1.0, 2.0, 5.0, 10.0, 25.0],
        crate::app::RulerUnit::Centimeters => &[0.5, 1.0, 2.0, 5.0, 10.0, 25.0, 50.0],
        crate::app::RulerUnit::Millimeters => &[5.0, 10.0, 25.0, 50.0, 100.0, 250.0, 500.0],
    };
    let unit_step = candidates
        .iter()
        .copied()
        .find(|&c| c * px_per_unit * zoom >= 42.0)
        .unwrap_or(*candidates.last().unwrap_or(&100.0));
    let step_px = unit_step * px_per_unit;
    let subs = unit.subdivisions();
    let major_line = Stroke::new(0.8, palette.text_dim);
    let minor_line = Stroke::new(0.5, palette.text_dim.gamma_multiply(0.55));

    let fmt_val = |v: f32| -> String {
        if (v - v.round()).abs() < 0.01 {
            format!("{:.0}", v.round())
        } else {
            format!("{v:.2}")
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        }
    };

    let start_x = ((rect.left() - origin.x) / (step_px * zoom)).floor() as i64;
    let end_x = ((rect.right() - origin.x) / (step_px * zoom)).ceil() as i64;
    for i in start_x..=end_x {
        let x = origin.x + i as f32 * step_px * zoom;
        if x >= strip_h.left() + RULER && x <= strip_h.right() {
            painter.vline(x, strip_h.top()..=strip_h.bottom(), major_line);
            painter.text(
                pos2(x + 2.0, strip_h.top() + 1.0),
                Align2::LEFT_TOP,
                fmt_val(i as f32 * unit_step),
                FontId::proportional(8.5),
                palette.text_dim,
            );
        }
        // Fractional sub-ticks
        if step_px * zoom >= 24.0 {
            for s in 1..subs {
                let sx = origin.x + (i as f32 + s as f32 / subs as f32) * step_px * zoom;
                if sx < strip_h.left() + RULER || sx > strip_h.right() {
                    continue;
                }
                let tick_h = if s * 2 == subs {
                    9.0
                } else if subs == 8 && s % 2 == 0 {
                    6.5
                } else {
                    4.0
                };
                painter.vline(
                    sx,
                    (strip_h.bottom() - tick_h)..=strip_h.bottom(),
                    minor_line,
                );
            }
        }
    }

    let start_y = ((rect.top() - origin.y) / (step_px * zoom)).floor() as i64;
    let end_y = ((rect.bottom() - origin.y) / (step_px * zoom)).ceil() as i64;
    for i in start_y..=end_y {
        let y = origin.y + i as f32 * step_px * zoom;
        if y >= strip_v.top() + RULER && y <= strip_v.bottom() {
            painter.hline(strip_v.left()..=strip_v.right(), y, major_line);
            painter.text(
                pos2(strip_v.left() + 2.0, y + 1.0),
                Align2::LEFT_TOP,
                fmt_val(i as f32 * unit_step),
                FontId::proportional(8.0),
                palette.text_dim,
            );
        }
        if step_px * zoom >= 24.0 {
            for s in 1..subs {
                let sy = origin.y + (i as f32 + s as f32 / subs as f32) * step_px * zoom;
                if sy < strip_v.top() + RULER || sy > strip_v.bottom() {
                    continue;
                }
                let tick_w = if s * 2 == subs {
                    9.0
                } else if subs == 8 && s % 2 == 0 {
                    6.5
                } else {
                    4.0
                };
                painter.hline((strip_v.right() - tick_w)..=strip_v.right(), sy, minor_line);
            }
        }
    }

    // Top-left corner unit badge
    let corner = Rect::from_min_size(rect.min, Vec2::splat(RULER));
    painter.rect_filled(corner, 0.0, palette.panel_alt);
    painter.rect_stroke(
        corner,
        0.0,
        Stroke::new(1.0, palette.stroke),
        egui::StrokeKind::Inside,
    );
    painter.text(
        corner.center(),
        Align2::CENTER_CENTER,
        unit.short_code(),
        FontId::proportional(8.5),
        palette.accent,
    );
}

fn draw_page(
    painter: &Painter,
    app: &mut FireCraft,
    origin: Vec2,
    zoom: f32,
    preview_mode: bool,
    god_view: bool,
) {
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
        if !layer.visible && !god_view {
            continue;
        }
        if layer.is_web_layer && !god_view && (preview_mode || !app.show_slices) {
            continue;
        }
        // Hidden layers remain inspectable in god view, but at low opacity so visible content
        // still dominates. This is a view-only treatment and never mutates the document.
        let layer_alpha = layer.opacity * if layer.visible { 1.0 } else { 0.28 };
        for (oi, obj) in layer.objects.iter().enumerate() {
            if !obj.visible && !god_view {
                continue;
            }
            if app.inline_text_edit_obj == Some(oi) && li == app.selected_layer {
                continue;
            }
            let obj_alpha = if obj.visible { 1.0 } else { 0.30 };
            let alpha = (layer_alpha * obj.opacity * obj_alpha).clamp(0.0, 1.0);
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

    // Draw Live Effects (Drop Shadow, Outer Glow) underneath the object, following the actual
    // silhouette instead of always painting a rectangle.
    if obj.bounds().is_some() {
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
                        paint_silhouette(painter, obj, origin, zoom, c, expand, offset);
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
                        paint_silhouette(painter, obj, origin, zoom, c, expand, Vec2::ZERO);
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
            if let Some(g) = obj.gradient {
                let points = rounded_rect_gradient_points(r, cr);
                let triangles = gradient_fan_triangles(points.len() - 1);
                paint_gradient_mesh(painter, r, &points, &triangles, g, alpha);
            } else if let Some(f) = fill {
                painter.rect_filled(r, cr, f);
            }
            if obj.stroke.is_some() && obj.width > 0.0 {
                painter.rect_stroke(r, cr, Stroke::new(sw, stroke), egui::StrokeKind::Middle);
            }
        }
        Shape::Ellipse { center, radius, .. } => {
            let c = to_screen(origin, zoom, *center);
            let rr = Vec2::new(radius.x.abs() * zoom, radius.y.abs() * zoom);
            let bounds = Rect::from_center_size(c, rr * 2.0);
            if let Some(g) = obj.gradient {
                let segments = 64;
                let mut points = Vec::with_capacity(segments + 1);
                points.push(c);
                points.extend((0..segments).map(|i| {
                    let angle = std::f32::consts::TAU * i as f32 / segments as f32;
                    pos2(c.x + rr.x * angle.cos(), c.y + rr.y * angle.sin())
                }));
                let triangles = gradient_fan_triangles(segments);
                paint_gradient_mesh(painter, bounds, &points, &triangles, g, alpha);
            } else if let Some(f) = fill {
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
            let center_screen = to_screen(origin, zoom, *center);
            let bounds = Rect::from_points(&pts);
            if let Some(g) = obj.gradient {
                let mut points = Vec::with_capacity(pts.len() + 1);
                points.push(center_screen);
                points.extend(pts.iter().copied());
                let triangles = gradient_fan_triangles(pts.len());
                paint_gradient_mesh(painter, bounds, &points, &triangles, g, alpha);
            } else if let Some(f) = fill {
                fill_polygon(painter, &pts, *star_inner, center_screen, f);
            }
            if obj.stroke.is_some() && obj.width > 0.0 {
                painter.add(egui::Shape::closed_line(pts, Stroke::new(sw, stroke)));
            }
        }
        Shape::Path { points, closed } => {
            if points.len() > 1 {
                let pts: Vec<Pos2> = points.iter().map(|p| to_screen(origin, zoom, *p)).collect();
                if *closed {
                    let bounds = Rect::from_points(&pts);
                    if let Some(triangles) = obj.path_triangulation_cache.triangulate(points) {
                        if let Some(g) = obj.gradient {
                            paint_gradient_mesh(painter, bounds, &pts, &triangles, g, alpha);
                        } else if let Some(f) = fill {
                            paint_solid_mesh(painter, &pts, &triangles, f);
                        }
                    }
                    if obj.stroke.is_some() && obj.width > 0.0 {
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
                // Concentric circles must stay inside the object: without this clip a radial
                // gradient paints a disc over neighbouring objects and outside page bounds.
                let radial_painter = painter.with_clip_rect(r.intersect(painter.clip_rect()));
                // Fill the whole rect with the outer stop first; circles alone leave the
                // rectangle's corners unpainted and the page shows through them.
                radial_painter.rect_filled(r, 0.0, to.gamma_multiply(alpha));
                let steps = 24;
                let max_rad = r.width().min(r.height()) * 0.5;
                for i in (0..steps).rev() {
                    // t runs 1.0 → 0.0 so the outermost ring is exactly the outer stop and
                    // blends seamlessly into the rect fill (no visible disc edge).
                    let t = i as f32 / (steps - 1) as f32;
                    let radius = max_rad * (i + 1) as f32 / steps as f32;
                    let c = from.lerp_to_gamma(*to, t).gamma_multiply(alpha);
                    radial_painter.circle_filled(r.center(), radius, c);
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
        Shape::Bitmap {
            at,
            pixels,
            size,
            pixel_size,
        } => {
            let dimensions = if pixel_size.x >= 1.0 && pixel_size.y >= 1.0 {
                *pixel_size
            } else {
                *size
            };
            let tex = FireCraft::bitmap_texture(&app.ctx, &mut app.textures, pixels, dimensions);
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
                // CS6 shows the slice URL in a tab *above* the slice. Painting it inside the
                // rect covered the artwork — a button's own label disappeared under the URL.
                let label = if url.is_empty() {
                    obj.name.as_str()
                } else {
                    url.as_str()
                };
                let tab_font = FontId::proportional(10.0);
                let label = elide_to_width(painter, label, &tab_font, (r.width() - 8.0).max(48.0));
                let galley = painter.layout_no_wrap(label, tab_font, border);
                let tab_h = 13.0;
                // Below the slice, not above it: a slice sitting on a button put the URL tab
                // on top of the button's own label.
                let tab = Rect::from_min_size(
                    pos2(r.left(), r.bottom() + 1.0),
                    Vec2::new(galley.size().x + 8.0, tab_h),
                );
                if painter.clip_rect().intersects(tab) {
                    painter.rect_filled(tab, 0.0, border);
                    painter.galley(
                        pos2(tab.left() + 4.0, tab.center().y - galley.size().y * 0.5),
                        galley,
                        Color32::from_rgb(12, 16, 12),
                    );
                }
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
    if !preview_mode
        && matches!(app.tool, Tool::Select | Tool::Subselect | Tool::Scale)
        && ((app.selected_layer == layer_index && app.is_selected(obj_index))
            || app
                .selected_extra_layers
                .contains(&(layer_index, obj_index)))
    {
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
        if let Some((_, doc_h)) = auto_handle_for(obj) {
            let sh = to_screen(origin, zoom, doc_h);
            let r = 5.0;
            painter.add(egui::Shape::convex_polygon(
                vec![
                    pos2(sh.x, sh.y - r),
                    pos2(sh.x + r, sh.y),
                    pos2(sh.x, sh.y + r),
                    pos2(sh.x - r, sh.y),
                ],
                Color32::from_rgb(244, 178, 54),
                Stroke::new(1.2, Color32::from_rgb(32, 24, 12)),
            ));
        }
        if app.tool == Tool::Subselect
            && let Shape::Path { points, .. } = &obj.kind
        {
            for (node_idx, p) in points.iter().enumerate() {
                let sp = to_screen(origin, zoom, *p);
                let nr = Rect::from_center_size(sp, Vec2::splat(6.0));
                let color = if app.selected_path_nodes.contains(&node_idx) {
                    app.palette.selection
                } else {
                    app.palette.accent
                };
                painter.rect_filled(nr, 1.0, color);
                painter.rect_stroke(
                    nr,
                    1.0,
                    Stroke::new(1.0, Color32::WHITE),
                    egui::StrokeKind::Middle,
                );
            }
        }
    }

    if !preview_mode
        && app.tool == Tool::Gradient
        && app.selected_layer == layer_index
        && app.is_selected(obj_index)
        && let Some(gradient) = obj.gradient
        && let Some((start, end)) = gradient_handle_points(obj, gradient)
    {
        let start_screen = to_screen(origin, zoom, start);
        let end_screen = to_screen(origin, zoom, end);
        painter.line_segment(
            [start_screen, end_screen],
            Stroke::new(1.5, app.palette.accent),
        );
        for (point, color) in [(start_screen, gradient.start), (end_screen, gradient.end)] {
            painter.circle_filled(point, 6.0, Color32::WHITE);
            painter.circle_filled(point, 4.0, color);
            painter.circle_stroke(point, 6.0, Stroke::new(1.0, app.palette.text));
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

pub fn auto_handle_for(obj: &Obj) -> Option<(usize, Vec2)> {
    match &obj.kind {
        Shape::Rect {
            min,
            max,
            corner_radius,
            ..
        } => {
            let lo = min.min(*max);
            let hi = min.max(*max);
            let x = lo.x + (*corner_radius).max(10.0).min((hi.x - lo.x) * 0.48);
            Some((0, Vec2::new(x, lo.y + 10.0)))
        }
        Shape::Polygon {
            center,
            radius,
            sides,
            star_inner,
            angle_rad,
        } => {
            let ang = *angle_rad + std::f32::consts::PI / (*sides).max(3) as f32;
            let r = *radius * *star_inner;
            Some((1, *center + Vec2::new(ang.cos(), ang.sin()) * r))
        }
        _ => None,
    }
}

/// The editable endpoint selected by the Gradient tool.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GradientHandle {
    Start,
    End,
}

pub fn gradient_handle_positions(app: &FireCraft) -> Option<(Vec2, Vec2)> {
    let obj = app.selected_object()?;
    gradient_handle_points(&obj, obj.gradient?)
}

pub fn hit_test_gradient_handle(app: &FireCraft, doc_pos: Vec2) -> Option<GradientHandle> {
    let (start, end) = gradient_handle_positions(app)?;
    let tolerance = 8.0 / app.doc.zoom.max(0.1);
    let start_distance = (start - doc_pos).length();
    let end_distance = (end - doc_pos).length();
    let start_hit = start_distance <= tolerance;
    let end_hit = end_distance <= tolerance;
    match (start_hit, end_hit) {
        (true, true) if start_distance <= end_distance => Some(GradientHandle::Start),
        (true, true) => Some(GradientHandle::End),
        (true, false) => Some(GradientHandle::Start),
        (false, true) => Some(GradientHandle::End),
        (false, false) => None,
    }
}

/// Check if `doc_pos` hits the CS6 amber diamond Auto Shape control handle.
pub fn hit_test_auto_handle(app: &FireCraft, doc_pos: Vec2) -> Option<usize> {
    let idx = app.selected?;
    let obj = app
        .doc
        .page()
        .layers
        .get(app.selected_layer)
        .and_then(|l| l.objects.get(idx))?;
    let (kind_id, h_pos) = auto_handle_for(obj)?;
    let tol = 9.0 / app.doc.zoom.max(0.1);
    if (h_pos - doc_pos).length() <= tol {
        Some(kind_id)
    } else {
        None
    }
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
                && (obj.fill.is_some_and(|color| color.a() > 0) || obj.gradient.is_some())
                && obj.path_triangulation_cache.triangulate(points).is_some()
                && point_in_polygon(points, p)
            {
                return true;
            }
            let segment_count = if *closed {
                points.len()
            } else {
                points.len().saturating_sub(1)
            };
            for index in 0..segment_count {
                let a = points[index];
                let b = points[(index + 1) % points.len()];
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

/// Bitmap texture identity follows pixel content, not a per-frame clone or mutable object slot.
pub type TextureCache = std::collections::HashMap<(u64, usize, usize), TextureHandle>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doc::{Document, Layer, Page};

    fn app_with_objects(objects: Vec<Obj>) -> FireCraft {
        let mut app = FireCraft::new(None);
        let mut page = Page::new("Hit testing", 200.0, 200.0);
        page.layers = vec![Layer::new("Artwork"), Layer::web_layer()];
        page.layers[0].objects = objects;
        app.doc = Document::new(200.0, 200.0);
        app.doc.pages = vec![page];
        app.selected_layer = 0;
        app
    }

    fn rect(x0: f32, y0: f32, x1: f32, y1: f32) -> Obj {
        Obj::new(
            Shape::Rect {
                min: Vec2::new(x0, y0),
                max: Vec2::new(x1, y1),
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::WHITE),
            None,
        )
    }

    fn render_bitmap_ids(app: &mut FireCraft) -> Vec<(Rect, egui::TextureId)> {
        let ctx = app.ctx.clone();
        let objects = app.doc.page().layers[0].objects.clone();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            for (index, object) in objects.iter().enumerate() {
                draw_obj(
                    ui.painter(),
                    app,
                    object,
                    Vec2::ZERO,
                    1.0,
                    1.0,
                    (0, index),
                    false,
                );
            }
        });
        let rendered = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Mesh(mesh) => Some((mesh.calc_bounds(), mesh.texture_id)),
                _ => None,
            })
            .collect();
        output.textures_delta.clear();
        rendered
    }

    fn mesh_covers_point(mesh: &egui::Mesh, point: Pos2) -> bool {
        let cross = |a: Vec2, b: Vec2| a.x * b.y - a.y * b.x;
        let (triangles, _) = mesh.indices.as_chunks::<3>();
        triangles.iter().any(|&[a_index, b_index, c_index]| {
            let a = mesh.vertices[a_index as usize].pos;
            let b = mesh.vertices[b_index as usize].pos;
            let c = mesh.vertices[c_index as usize].pos;
            let ab = cross(b - a, point - a);
            let bc = cross(c - b, point - b);
            let ca = cross(a - c, point - c);
            (ab >= -0.001 && bc >= -0.001 && ca >= -0.001)
                || (ab <= 0.001 && bc <= 0.001 && ca <= 0.001)
        })
    }

    fn rendered_path_fill_mesh(object: Obj) -> egui::Mesh {
        let mut app = app_with_objects(vec![object.clone()]);
        let ctx = app.ctx.clone();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            draw_obj(
                ui.painter(),
                &mut app,
                &object,
                Vec2::ZERO,
                1.0,
                1.0,
                (0, 0),
                false,
            );
        });
        let mesh = output
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Mesh(mesh) => Some(mesh.as_ref().clone()),
                _ => None,
            });
        output.textures_delta.clear();
        mesh.expect("closed paths with fills should render as a triangulated mesh")
    }

    fn concave_l_path(gradient: Option<GradientFill>) -> Obj {
        let mut object = Obj::new(
            Shape::Path {
                points: vec![
                    Vec2::new(10.0, 10.0),
                    Vec2::new(50.0, 10.0),
                    Vec2::new(50.0, 20.0),
                    Vec2::new(20.0, 20.0),
                    Vec2::new(20.0, 50.0),
                    Vec2::new(10.0, 50.0),
                ],
                closed: true,
            },
            Some(Color32::from_rgb(230, 110, 30)),
            Some(Color32::BLACK),
        );
        object.gradient = gradient;
        object
    }

    #[test]
    fn concave_closed_path_solid_fill_leaves_the_notch_unpainted() {
        let mesh = rendered_path_fill_mesh(concave_l_path(None));
        assert!(mesh_covers_point(&mesh, pos2(15.0, 15.0)));
        assert!(
            !mesh_covers_point(&mesh, pos2(40.0, 40.0)),
            "the concave notch must remain transparent instead of being filled by a convex hull"
        );
    }

    #[test]
    fn concave_closed_path_gradient_leaves_the_notch_unpainted() {
        let gradient = GradientFill::new(Color32::BLACK, Color32::WHITE);
        let mesh = rendered_path_fill_mesh(concave_l_path(Some(gradient)));
        assert!(mesh_covers_point(&mesh, pos2(15.0, 15.0)));
        assert!(
            !mesh_covers_point(&mesh, pos2(40.0, 40.0)),
            "gradient triangles must follow the path contour and leave its notch transparent"
        );
    }

    #[test]
    fn concave_closed_path_hit_testing_ignores_the_empty_notch() {
        let app = app_with_objects(vec![concave_l_path(None)]);
        assert_eq!(hit_test(&app, Vec2::new(15.0, 15.0)), Some((0, 0)));
        assert_eq!(
            hit_test(&app, Vec2::new(40.0, 40.0)),
            None,
            "path hit testing must respect the contour instead of its rectangular bounds"
        );
    }

    #[test]
    fn coordinate_conversion_and_resize_handle_order_are_stable() {
        assert_eq!(
            to_screen(Vec2::new(10.0, 20.0), 2.0, Vec2::new(3.0, 4.0)),
            pos2(16.0, 28.0)
        );
        let handles = selection_handles(Rect::from_min_max(pos2(0.0, 0.0), pos2(20.0, 10.0)));
        assert_eq!(
            handles,
            [
                pos2(0.0, 0.0),
                pos2(10.0, 0.0),
                pos2(20.0, 0.0),
                pos2(20.0, 5.0),
                pos2(20.0, 10.0),
                pos2(10.0, 10.0),
                pos2(0.0, 10.0),
                pos2(0.0, 5.0),
            ]
        );
    }

    #[test]
    fn auto_shape_handles_are_specific_to_rectangles_and_polygons() {
        let rectangle = rect(10.0, 20.0, 110.0, 80.0);
        let (kind, point) = auto_handle_for(&rectangle).unwrap();
        assert_eq!(kind, 0);
        assert_eq!(point, Vec2::new(20.0, 30.0));

        let star = Obj::new(
            Shape::Polygon {
                center: Vec2::new(50.0, 50.0),
                radius: 20.0,
                sides: 5,
                star_inner: 0.5,
                angle_rad: 0.0,
            },
            None,
            None,
        );
        let (kind, point) = auto_handle_for(&star).unwrap();
        assert_eq!(kind, 1);
        assert!((point - Vec2::new(50.0, 50.0) - Vec2::new(0.5, 0.0) * 20.0).length() > 0.0);
        assert_eq!(
            auto_handle_for(&Obj::new(
                Shape::Line {
                    a: Vec2::ZERO,
                    b: Vec2::splat(1.0),
                },
                None,
                None
            )),
            None
        );
    }

    #[test]
    fn selected_star_uses_a_seamless_fill_mesh_without_transform_box_in_shape_tool() {
        let mut app = app_with_objects(vec![Obj::new(
            Shape::Polygon {
                center: Vec2::new(50.0, 50.0),
                radius: 30.0,
                sides: 5,
                star_inner: 0.45,
                angle_rad: 0.0,
            },
            Some(Color32::from_rgb(240, 120, 30)),
            None,
        )]);
        app.selected = Some(0);
        app.tool = Tool::Star;
        let star = app.doc.page().layers[0].objects[0].clone();
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            draw_obj(
                ui.painter(),
                &mut app,
                &star,
                Vec2::ZERO,
                1.0,
                1.0,
                (0, 0),
                false,
            );
        });
        assert!(
            output
                .shapes
                .iter()
                .any(|clipped| matches!(clipped.shape, egui::Shape::Mesh(_))),
            "concave star fill must use one shared-edge triangle mesh"
        );
        assert!(
            output.shapes.iter().all(|clipped| !matches!(
                clipped.shape,
                egui::Shape::Rect(_) | egui::Shape::Circle(_)
            )),
            "drawing with the Star tool should not leave a selection box or handles"
        );
        output.textures_delta.clear();
    }

    #[test]
    fn gradient_fill_mesh_uses_its_editable_axis_endpoints() {
        let mut object = rect(0.0, 0.0, 100.0, 80.0);
        let mut gradient = GradientFill::new(Color32::BLACK, Color32::WHITE);
        gradient.start_pos = Vec2::new(0.0, 0.5);
        gradient.end_pos = Vec2::new(1.0, 0.5);
        object.gradient = Some(gradient);
        let mut app = app_with_objects(vec![object.clone()]);
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(egui::RawInput::default(), |ui| {
            draw_obj(
                ui.painter(),
                &mut app,
                &object,
                Vec2::ZERO,
                1.0,
                1.0,
                (0, 0),
                false,
            );
        });
        let mesh = output
            .shapes
            .iter()
            .find_map(|clipped| match &clipped.shape {
                egui::Shape::Mesh(mesh) => Some(mesh),
                _ => None,
            })
            .expect("gradient fills render through a colored mesh");
        let left = mesh
            .vertices
            .iter()
            .min_by(|a, b| a.pos.x.total_cmp(&b.pos.x))
            .unwrap();
        let right = mesh
            .vertices
            .iter()
            .max_by(|a, b| a.pos.x.total_cmp(&b.pos.x))
            .unwrap();
        assert_eq!(left.color, Color32::BLACK);
        assert_eq!(right.color, Color32::WHITE);
        assert!(
            mesh.vertices.len() >= 5,
            "rounded rectangle adds corner samples"
        );
        output.textures_delta.clear();
    }

    #[test]
    fn bitmap_rendering_keeps_each_image_texture_after_hotkey_reorder() {
        let bitmap = |at: Vec2, rgba: [u8; 4]| {
            Obj::new(
                Shape::Bitmap {
                    at,
                    pixels: rgba.to_vec(),
                    size: Vec2::splat(10.0),
                    pixel_size: Vec2::splat(1.0),
                },
                None,
                None,
            )
        };
        let mut app = app_with_objects(vec![
            bitmap(Vec2::new(5.0, 10.0), [255, 0, 0, 255]),
            bitmap(Vec2::new(40.0, 10.0), [0, 0, 255, 255]),
        ]);

        let before = render_bitmap_ids(&mut app);
        assert_eq!(before.len(), 2, "both bitmap objects should render");
        let red_texture = before
            .iter()
            .find(|(bounds, _)| bounds.min.x == 5.0)
            .expect("red bitmap should render at its original position")
            .1;
        let blue_texture = before
            .iter()
            .find(|(bounds, _)| bounds.min.x == 40.0)
            .expect("blue bitmap should render at its original position")
            .1;
        assert_ne!(red_texture, blue_texture);

        app.select_single(Some(0));
        app.reorder_selected_step(true);
        let after = render_bitmap_ids(&mut app);
        assert_eq!(
            after.len(),
            2,
            "both reordered bitmap objects should render"
        );
        assert_eq!(
            after
                .iter()
                .find(|(bounds, _)| bounds.min.x == 5.0)
                .expect("red bitmap should remain at its original position")
                .1,
            red_texture,
            "the red image must retain its texture after its object index changes"
        );
        assert_eq!(
            after
                .iter()
                .find(|(bounds, _)| bounds.min.x == 40.0)
                .expect("blue bitmap should remain at its original position")
                .1,
            blue_texture,
            "the blue image must retain its texture after its object index changes"
        );
    }

    #[test]
    fn bitmap_texture_identity_survives_duplicate_undo_redo_and_delete_add() {
        let bitmap = |at: Vec2, rgba: [u8; 4]| {
            Obj::new(
                Shape::Bitmap {
                    at,
                    pixels: rgba.to_vec(),
                    size: Vec2::splat(10.0),
                    pixel_size: Vec2::splat(1.0),
                },
                None,
                None,
            )
        };
        let mut app = app_with_objects(vec![
            bitmap(Vec2::new(5.0, 10.0), [255, 0, 0, 255]),
            bitmap(Vec2::new(40.0, 10.0), [0, 0, 255, 255]),
        ]);

        let initial = render_bitmap_ids(&mut app);
        let red = initial
            .iter()
            .find(|(bounds, _)| bounds.min.x == 5.0)
            .expect("red bitmap should render")
            .1;
        let blue = initial
            .iter()
            .find(|(bounds, _)| bounds.min.x == 40.0)
            .expect("blue bitmap should render")
            .1;
        assert_ne!(red, blue);

        app.select_single(Some(0));
        app.duplicate_selected();
        let duplicated = render_bitmap_ids(&mut app);
        assert_eq!(duplicated.len(), 3);
        assert_eq!(
            duplicated
                .iter()
                .find(|(bounds, _)| bounds.min.x == 19.0)
                .expect("duplicate should render at its translated position")
                .1,
            red,
            "a cloned bitmap should share the texture for identical pixel content"
        );
        assert_eq!(
            duplicated
                .iter()
                .find(|(bounds, _)| bounds.min.x == 40.0)
                .expect("blue bitmap should remain rendered")
                .1,
            blue
        );

        app.undo_action();
        let undone = render_bitmap_ids(&mut app);
        assert_eq!(undone.len(), 2, "undo should remove the duplicate");
        assert_ne!(
            undone
                .iter()
                .find(|(bounds, _)| bounds.min.x == 5.0)
                .expect("red bitmap should be restored")
                .1,
            undone
                .iter()
                .find(|(bounds, _)| bounds.min.x == 40.0)
                .expect("blue bitmap should be restored")
                .1
        );

        app.redo_action();
        let redone = render_bitmap_ids(&mut app);
        assert_eq!(redone.len(), 3, "redo should restore the duplicate");
        assert_eq!(
            redone
                .iter()
                .find(|(bounds, _)| bounds.min.x == 5.0)
                .expect("original red bitmap should return")
                .1,
            redone
                .iter()
                .find(|(bounds, _)| bounds.min.x == 19.0)
                .expect("duplicated red bitmap should return")
                .1,
            "redo must restore the same source pixels for both red objects"
        );

        app.select_single(Some(0));
        app.delete_selected();
        app.push_object(bitmap(Vec2::new(80.0, 10.0), [0, 255, 0, 255]));
        let replaced = render_bitmap_ids(&mut app);
        assert_eq!(replaced.len(), 3);
        assert_eq!(
            replaced
                .iter()
                .find(|(bounds, _)| bounds.min.x == 80.0)
                .expect("new green bitmap should render")
                .1,
            FireCraft::bitmap_texture(
                &app.ctx,
                &mut app.textures,
                &[0, 255, 0, 255],
                Vec2::splat(1.0),
            )
            .expect("green texture should be cacheable")
            .id(),
            "adding a new bitmap after deletion must render its own pixels"
        );
    }

    #[test]
    fn flattening_bitmaps_creates_a_new_rendered_texture_for_the_composite_pixels() {
        let bitmap = |at: Vec2, rgba: [u8; 4]| {
            Obj::new(
                Shape::Bitmap {
                    at,
                    pixels: rgba.to_vec(),
                    size: Vec2::splat(10.0),
                    pixel_size: Vec2::splat(1.0),
                },
                None,
                None,
            )
        };
        let mut app = app_with_objects(vec![
            bitmap(Vec2::new(5.0, 10.0), [255, 0, 0, 255]),
            bitmap(Vec2::new(40.0, 10.0), [0, 0, 255, 255]),
        ]);
        app.selected = Some(1);
        app.selected_set = vec![0, 1];

        let source_textures = render_bitmap_ids(&mut app);
        assert_eq!(source_textures.len(), 2);
        app.flatten_selected_to_bitmap();

        let objects = &app.doc.page().layers[0].objects;
        assert_eq!(objects.len(), 1, "flatten should replace selected bitmaps");
        let (pixels, pixel_size, at) = match &objects[0].kind {
            Shape::Bitmap {
                pixels,
                pixel_size,
                at,
                ..
            } => (pixels.clone(), *pixel_size, *at),
            _ => panic!("flattened selection should be a bitmap"),
        };
        let flattened_texture = render_bitmap_ids(&mut app);
        assert_eq!(flattened_texture.len(), 1, "composite bitmap should render");
        assert_eq!(flattened_texture[0].0.min.x, at.x);
        let expected_texture =
            FireCraft::bitmap_texture(&app.ctx, &mut app.textures, &pixels, pixel_size)
                .expect("flattened pixel data should produce a texture")
                .id();
        assert_eq!(
            flattened_texture[0].1, expected_texture,
            "the renderer must upload the flattened document pixels"
        );
        assert!(
            source_textures
                .iter()
                .all(|(_, source)| *source != flattened_texture[0].1),
            "the composite must not reuse either source image texture"
        );
    }

    #[test]
    fn hit_testing_obeys_topmost_order_visibility_locks_and_web_layer_toggle() {
        let mut back = rect(0.0, 0.0, 50.0, 50.0);
        back.name = "back".into();
        let mut top = rect(10.0, 10.0, 40.0, 40.0);
        top.name = "top".into();
        let mut app = app_with_objects(vec![back, top]);
        assert_eq!(hit_test(&app, Vec2::new(20.0, 20.0)), Some((0, 1)));
        app.doc.page_mut().layers[0].objects[1].visible = false;
        assert_eq!(hit_test(&app, Vec2::new(20.0, 20.0)), Some((0, 0)));
        app.doc.page_mut().layers[0].objects[0].locked = true;
        assert_eq!(hit_test(&app, Vec2::new(20.0, 20.0)), None);

        let mut web = rect(0.0, 0.0, 50.0, 50.0);
        web.visible = true;
        app.doc.page_mut().layers[1].objects.push(web);
        app.show_slices = false;
        assert_eq!(hit_test(&app, Vec2::new(20.0, 20.0)), None);
        app.show_slices = true;
        assert_eq!(hit_test(&app, Vec2::new(20.0, 20.0)), Some((1, 0)));
        app.doc.page_mut().layers[1].locked = true;
        assert_eq!(hit_test(&app, Vec2::new(20.0, 20.0)), None);
    }

    #[test]
    fn hit_testing_covers_shape_geometry_and_nested_groups() {
        let cases = [
            (
                Shape::Ellipse {
                    center: Vec2::new(20.0, 20.0),
                    radius: Vec2::splat(10.0),
                    filled: true,
                },
                Vec2::new(20.0, 20.0),
            ),
            (
                Shape::Polygon {
                    center: Vec2::new(20.0, 20.0),
                    radius: 10.0,
                    sides: 5,
                    star_inner: 1.0,
                    angle_rad: 0.0,
                },
                Vec2::new(20.0, 20.0),
            ),
            (
                Shape::Line {
                    a: Vec2::new(0.0, 20.0),
                    b: Vec2::new(40.0, 20.0),
                },
                Vec2::new(20.0, 24.0),
            ),
            (
                Shape::Path {
                    points: vec![Vec2::new(0.0, 20.0), Vec2::new(40.0, 20.0)],
                    closed: false,
                },
                Vec2::new(20.0, 24.0),
            ),
            (
                Shape::Text {
                    at: Vec2::new(10.0, 30.0),
                    text: "abc".into(),
                    size: 12.0,
                    bold: false,
                    italic: false,
                    font_family: "sans-serif".into(),
                },
                Vec2::new(11.0, 25.0),
            ),
            (
                Shape::Bitmap {
                    at: Vec2::new(10.0, 10.0),
                    pixels: vec![],
                    size: Vec2::splat(20.0),
                    pixel_size: Vec2::splat(20.0),
                },
                Vec2::new(15.0, 15.0),
            ),
            (
                Shape::SymbolInstance {
                    symbol_id: 0,
                    at: Vec2::new(10.0, 10.0),
                    scale: Vec2::splat(0.5),
                },
                Vec2::new(20.0, 20.0),
            ),
            (
                Shape::Gradient {
                    min: Vec2::new(10.0, 10.0),
                    max: Vec2::new(30.0, 30.0),
                    from: Color32::BLACK,
                    to: Color32::WHITE,
                    radial: false,
                },
                Vec2::new(20.0, 20.0),
            ),
            (
                Shape::Slice {
                    min: Vec2::new(10.0, 10.0),
                    max: Vec2::new(30.0, 30.0),
                    url: String::new(),
                    alt: String::new(),
                    target: String::new(),
                    format: String::new(),
                    is_hotspot: false,
                },
                Vec2::new(20.0, 20.0),
            ),
            (
                Shape::Group {
                    children: vec![rect(10.0, 10.0, 30.0, 30.0)],
                },
                Vec2::new(20.0, 20.0),
            ),
        ];
        for (shape, point) in cases {
            let app = app_with_objects(vec![Obj::new(shape, None, None)]);
            assert_eq!(hit_test(&app, point), Some((0, 0)));
            assert_eq!(hit_test(&app, Vec2::new(190.0, 190.0)), None);
        }

        let closed_path = Obj::new(
            Shape::Path {
                points: vec![
                    Vec2::new(10.0, 10.0),
                    Vec2::new(30.0, 30.0),
                    Vec2::new(10.0, 30.0),
                ],
                closed: true,
            },
            None,
            None,
        );
        let app = app_with_objects(vec![closed_path]);
        assert_eq!(hit_test(&app, Vec2::new(15.0, 25.0)), Some((0, 0)));
    }

    #[test]
    fn selected_object_resize_and_auto_shape_handles_use_zoom_adjusted_tolerance() {
        let mut app = app_with_objects(vec![rect(10.0, 10.0, 50.0, 50.0)]);
        app.selected = Some(0);
        app.doc.zoom = 1.0;
        assert_eq!(hit_test_handle(&app, Vec2::new(10.0, 10.0)), Some(0));
        assert_eq!(hit_test_handle(&app, Vec2::new(100.0, 100.0)), None);
        assert_eq!(hit_test_auto_handle(&app, Vec2::new(20.0, 20.0)), Some(0));
        assert_eq!(hit_test_auto_handle(&app, Vec2::new(100.0, 100.0)), None);
        app.selected = None;
        assert_eq!(hit_test_handle(&app, Vec2::new(10.0, 10.0)), None);
        assert_eq!(hit_test_auto_handle(&app, Vec2::new(20.0, 20.0)), None);
    }

    #[test]
    fn hit_test_handles_degenerate_and_reversed_geometry_boundaries() {
        let shapes = [
            (
                Shape::Rect {
                    min: Vec2::new(30.0, 30.0),
                    max: Vec2::new(10.0, 10.0),
                    corner_radius: 0.0,
                    filled: true,
                },
                Vec2::new(20.0, 20.0),
            ),
            (
                Shape::Ellipse {
                    center: Vec2::new(20.0, 20.0),
                    radius: Vec2::ZERO,
                    filled: true,
                },
                Vec2::new(20.0, 20.0),
            ),
            (
                Shape::Line {
                    a: Vec2::new(20.0, 20.0),
                    b: Vec2::new(20.0, 20.0),
                },
                Vec2::new(24.0, 20.0),
            ),
            (
                Shape::Path {
                    points: vec![
                        Vec2::new(20.0, 20.0),
                        Vec2::new(20.0, 20.0),
                        Vec2::new(40.0, 20.0),
                    ],
                    closed: false,
                },
                Vec2::new(30.0, 20.0),
            ),
        ];
        for (shape, point) in shapes {
            let app = app_with_objects(vec![Obj::new(shape, None, None)]);
            assert_eq!(hit_test(&app, point), Some((0, 0)));
        }

        let app = app_with_objects(vec![Obj::new(
            Shape::Path {
                points: vec![Vec2::ZERO],
                closed: false,
            },
            None,
            None,
        )]);
        assert_eq!(hit_test(&app, Vec2::ZERO), None);
    }

    #[test]
    fn canvas_paint_primitives_cover_grid_guides_rulers_and_gradient_edges() {
        let ctx = egui::Context::default();
        let mut app = FireCraft::new(None);
        app.doc.page_mut().guides_h = vec![10.0, 1_000.0];
        app.doc.page_mut().guides_v = vec![20.0, -1_000.0];
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 240.0))),
            ..Default::default()
        };
        let mut output = ctx.run_ui(input, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter().clone();
            draw_checkerboard(&painter, rect);
            draw_grid(&painter, rect, Vec2::ZERO, 1.0, 20.0, Color32::GRAY);
            // A sub-8px screen grid is intentionally suppressed.
            draw_grid(&painter, rect, Vec2::ZERO, 0.1, 20.0, Color32::GRAY);
            draw_guides(&painter, rect, Vec2::ZERO, 1.0, &app);
            draw_rulers(
                &painter,
                rect,
                Vec2::ZERO,
                1.0,
                app.palette,
                crate::app::RulerUnit::Pixels,
                96.0,
            );
            draw_dotted_rect(
                &painter,
                Rect::from_min_max(pos2(20.0, 20.0), pos2(80.0, 80.0)),
                Stroke::new(1.0, Color32::WHITE),
                5.0,
                3.0,
            );
            let gradient = GradientFill::new(Color32::BLACK, Color32::WHITE);
            paint_gradient(
                &painter,
                Rect::from_min_max(pos2(100.0, 20.0), pos2(150.0, 80.0)),
                gradient,
                0.8,
                |p, color| {
                    p.rect_filled(
                        Rect::from_min_max(pos2(100.0, 20.0), pos2(150.0, 80.0)),
                        0.0,
                        color,
                    );
                },
            );
            // Very short gradients take the single-stop path rather than generating bands.
            paint_gradient(
                &painter,
                Rect::from_min_max(pos2(160.0, 20.0), pos2(180.0, 20.2)),
                gradient,
                1.0,
                |p, color| {
                    p.rect_filled(
                        Rect::from_min_size(pos2(160.0, 20.0), Vec2::new(20.0, 1.0)),
                        0.0,
                        color,
                    );
                },
            );
            fill_polygon(
                &painter,
                &[pos2(200.0, 20.0), pos2(240.0, 20.0), pos2(220.0, 60.0)],
                1.0,
                pos2(220.0, 40.0),
                Color32::RED,
            );
            fill_polygon(
                &painter,
                &[pos2(250.0, 20.0), pos2(290.0, 20.0), pos2(270.0, 60.0)],
                0.5,
                pos2(270.0, 40.0),
                Color32::BLUE,
            );
        });
        assert!(
            output.shapes.len() > 100,
            "expected overlays and gradient bands to paint"
        );
        output.textures_delta.clear();
    }

    #[test]
    fn preview_badge_elides_to_the_available_canvas_width() {
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(220.0, 80.0))),
                ..Default::default()
            },
            |ui| {
                let font = FontId::proportional(11.0);
                let badge = elide_to_width(
                    ui.painter(),
                    "Preview: PNG · ~12345678901234567890 KB",
                    &font,
                    60.0,
                );
                let width = ui
                    .painter()
                    .layout_no_wrap(badge.clone(), font, Color32::WHITE)
                    .size()
                    .x;
                assert!(width <= 60.0);
                assert!(badge.ends_with('…'));
            },
        );
        output.textures_delta.clear();
    }
}
