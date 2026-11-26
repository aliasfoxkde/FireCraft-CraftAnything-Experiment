//! Workspace chrome: the document tab strip and the canvas footer bar
//! (Original/Preview/2-Up/4-Up modes, state navigation, zoom controls).

use super::*;

/// Treat a pointer release with only a few pixels of movement as a tab click. A plain
/// `Sense::click_and_drag` response reports `clicked()` only for a perfectly still click,
/// which made document and panel tabs feel like they lost the click.
pub(super) fn tab_click_without_drag(resp: &egui::Response) -> bool {
    if resp.clicked() {
        return true;
    }
    if !resp.drag_stopped() {
        return false;
    }
    let released = resp.interact_pointer_pos();
    resp.ctx.input(|i| {
        i.pointer.press_origin().is_some_and(|origin| {
            released.is_some_and(|release| (release - origin).length() <= 8.0)
        })
    })
}

pub(super) fn compact_tab_label(name: &str, max_chars: usize) -> String {
    let chars: Vec<char> = name.chars().collect();
    if chars.len() <= max_chars.max(4) {
        name.to_string()
    } else {
        let keep = max_chars.max(4) - 1;
        format!("{}…", chars[..keep].iter().collect::<String>())
    }
}

pub(super) fn document_tab_strip(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    egui::Frame::new()
        .fill(p.panel_alt)
        .inner_margin(egui::Margin::symmetric(4, 2))
        .show(ui, |ui| {
            // Row 1: Unified sculpted CS6 Document Tabs (middle-click or embedded '×' closes the tab)
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);
                let doc_count = app.open_docs.len();
                let non_floating: Vec<usize> = (0..doc_count)
                    .filter(|i| !app.floating_doc_tabs.contains(i))
                    .collect();
                let mut switch_to = None;
                let mut close_idx = None;
                let mut close_batch: Option<Vec<usize>> = None;
                let mut float_idx = None;
                let mut swap_pair: Option<(usize, usize)> = None;
                let mut dragging_tab: Option<(usize, egui::Rect, egui::Pos2)> = None;
                let mut tab_rects: Vec<(usize, egui::Rect, String)> = Vec::new();

                // Mouse-wheel over the strip walks the open documents. Reading input here does not
                // consume it; the canvas only reacts when the pointer is actually over the canvas.
                let strip_hover = egui::Rect::from_min_size(
                    ui.cursor().min,
                    Vec2::new(ui.available_width(), 24.0),
                );
                let wheel_delta = if ui.rect_contains_pointer(strip_hover) && non_floating.len() > 1 {
                    ui.input(|i| {
                        // `smooth_scroll_delta` decays over several frames, which would step through
                        // many tabs for one wheel notch. Sum the raw wheel events for this frame so
                        // one notch = one tab, and accept horizontal trackpad gestures too.
                        let mut delta = Vec2::ZERO;
                        for event in &i.raw.events {
                            if let egui::Event::MouseWheel { delta: wheel, .. } = event {
                                delta += *wheel;
                            }
                        }
                        if delta.y.abs() >= delta.x.abs() {
                            delta.y
                        } else {
                            delta.x
                        }
                    })
                } else {
                    0.0
                };
                if wheel_delta.abs() > 0.5 {
                    let pos = non_floating
                        .iter()
                        .position(|&i| i == app.active_doc)
                        .unwrap_or(0);
                    let step = if wheel_delta > 0.0 { -1 } else { 1 };
                    let next = (pos as isize + step).rem_euclid(non_floating.len() as isize) as usize;
                    switch_to = Some(non_floating[next]);
                }

                // Fit the strip to the available width. When space runs out keep the active tab plus
                // the most recent others, shrink padding, and fold the remainder into the … menu.
                let controls_reserve = 26.0
                    + 26.0
                    + if app.floating_doc_tabs.is_empty() { 0.0 } else { 82.0 }
                    + 8.0;
                let tab_area = (ui.available_width() - controls_reserve).max(64.0);
                let full_capacity = ((tab_area + 2.0) / 60.0).floor().max(1.0) as usize;
                let max_tabs = if non_floating.len() > full_capacity {
                    full_capacity.saturating_sub(1).max(1)
                } else {
                    full_capacity
                };
                let mut visible_docs: Vec<usize> = Vec::new();
                if !app.floating_doc_tabs.contains(&app.active_doc)
                    && app.active_doc < doc_count
                {
                    visible_docs.push(app.active_doc);
                }
                for &idx in &app.doc_tab_history {
                    if visible_docs.len() >= max_tabs {
                        break;
                    }
                    if idx < doc_count
                        && !app.floating_doc_tabs.contains(&idx)
                        && !visible_docs.contains(&idx)
                    {
                        visible_docs.push(idx);
                    }
                }
                for &idx in &non_floating {
                    if visible_docs.len() >= max_tabs {
                        break;
                    }
                    if !visible_docs.contains(&idx) {
                        visible_docs.push(idx);
                    }
                }
                visible_docs.sort_unstable();
                let hidden_count = non_floating.len().saturating_sub(visible_docs.len());
                let overflow_w = if hidden_count > 0 { 26.0 } else { 0.0 };
                let visible_count = visible_docs.len().max(1);
                let tab_w = ((tab_area - overflow_w) / visible_count as f32 - 2.0)
                    .clamp(58.0, 210.0);

                for i in visible_docs {
                    let is_active = i == app.active_doc;
                    let name = if is_active {
                        if app.doc.dirty {
                            format!("*{}", app.doc.name)
                        } else {
                            app.doc.name.clone()
                        }
                    } else if app.open_docs[i].dirty {
                        format!("*{}", app.open_docs[i].name)
                    } else {
                        app.open_docs[i].name.clone()
                    };
                    let (tab_r, tab_resp) =
                        ui.allocate_exact_size(Vec2::new(tab_w, 21.0), egui::Sense::click_and_drag());
                    tab_rects.push((i, tab_r, name.clone()));
                    let compact = tab_w < 96.0;
                    let pad = if compact { 5.0 } else { 8.0 };
                    let close_size = if compact { 12.0 } else { 14.0 };
                    let close_r = egui::Rect::from_center_size(
                        egui::pos2(tab_r.right() - close_size * 0.5 - 3.0, tab_r.center().y),
                        Vec2::splat(close_size),
                    );
                    let close_resp = ui.interact(
                        close_r,
                        ui.make_persistent_id(("doc_tab_close", i)),
                        egui::Sense::click(),
                    );
                    let painter = ui.painter();
                    let bg = if is_active {
                        p.panel
                    } else if tab_resp.hovered() {
                        p.well
                    } else {
                        p.panel_alt
                    };
                    painter.rect_filled(tab_r, 0.0, bg);
                    painter.rect_stroke(
                        tab_r,
                        0.0,
                        egui::Stroke::new(1.0, p.stroke),
                        egui::StrokeKind::Middle,
                    );
                    if is_active {
                        painter.line_segment(
                            [tab_r.left_top(), tab_r.right_top()],
                            egui::Stroke::new(2.0, p.accent),
                        );
                        painter.line_segment(
                            [
                                egui::pos2(tab_r.left() + 1.0, tab_r.bottom()),
                                egui::pos2(tab_r.right() - 1.0, tab_r.bottom()),
                            ],
                            egui::Stroke::new(1.5, bg),
                        );
                    } else {
                        painter.line_segment(
                            [tab_r.left_bottom(), tab_r.right_bottom()],
                            egui::Stroke::new(1.0, p.stroke),
                        );
                    }
                    let label_room = (close_r.left() - tab_r.left() - pad - 4.0).max(24.0);
                    let label = compact_tab_label(&name, (label_room / 6.0) as usize);
                    painter
                        .with_clip_rect(egui::Rect::from_min_max(
                            egui::pos2(tab_r.left() + pad, tab_r.top()),
                            egui::pos2(close_r.left() - 3.0, tab_r.bottom()),
                        ))
                        .text(
                            egui::pos2(tab_r.left() + pad, tab_r.center().y),
                            egui::Align2::LEFT_CENTER,
                            &label,
                            egui::FontId::proportional(if compact { 10.5 } else { 11.0 }),
                            if is_active { p.text } else { p.text_dim },
                        );
                    if close_resp.hovered() {
                        painter.rect_filled(close_r, 0.0, p.accent.gamma_multiply(0.35));
                    }
                    painter.text(
                        close_r.center(),
                        egui::Align2::CENTER_CENTER,
                        "×",
                        egui::FontId::proportional(11.5),
                        if close_resp.hovered() {
                            p.text
                        } else {
                            p.text_dim
                        },
                    );
                    let tab_resp = tab_resp.on_hover_text(
                        "Click to switch · Drag left/right to reorder · Drag out to float · Middle-click to close",
                    );
                    if close_resp
                        .on_hover_text("Close Document Tab (Middle-Click or Ctrl+W)")
                        .clicked()
                        || tab_resp.middle_clicked()
                    {
                        close_idx = Some(i);
                    } else if tab_click_without_drag(&tab_resp) && !is_active {
                        switch_to = Some(i);
                    }
                    if tab_resp.dragged()
                        && let Some(ptr) = ui.ctx().pointer_interact_pos()
                        && ui
                            .input(|i| i.pointer.press_origin())
                            .is_none_or(|origin| (ptr - origin).length() > 7.0)
                    {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
                        let pulling_out = ptr.y > tab_r.bottom() + 20.0 || ptr.y < tab_r.top() - 20.0;
                        // Floating ghost tab preview at pointer
                        let ghost_p = ui.ctx().layer_painter(egui::LayerId::new(
                            egui::Order::Tooltip,
                            ui.make_persistent_id("tab_drag_ghost"),
                        ));
                        let ghost_r = egui::Rect::from_center_size(ptr + Vec2::new(0.0, 14.0), Vec2::new(tab_w, 20.0));
                        ghost_p.rect_filled(ghost_r, 2.0, p.panel.gamma_multiply(0.94));
                        ghost_p.rect_stroke(
                            ghost_r,
                            2.0,
                            egui::Stroke::new(1.5, p.accent),
                            egui::StrokeKind::Middle,
                        );
                        ghost_p.text(
                            ghost_r.center(),
                            egui::Align2::CENTER_CENTER,
                            if pulling_out {
                                format!("⧉ Float: {name}")
                            } else {
                                name.clone()
                            },
                            egui::FontId::proportional(10.5),
                            p.text,
                        );
                        if pulling_out {
                            float_idx = Some(i);
                        } else {
                            dragging_tab = Some((i, tab_r, ptr));
                        }
                    }
                    tab_resp.context_menu(|ui| {
                        if ui.button("Close Tab").clicked() {
                            close_idx = Some(i);
                            ui.close();
                        }
                        let left: Vec<usize> = non_floating
                            .iter()
                            .copied()
                            .filter(|&j| j < i)
                            .collect();
                        let right: Vec<usize> = non_floating
                            .iter()
                            .copied()
                            .filter(|&j| j > i)
                            .collect();
                        let others: Vec<usize> = non_floating
                            .iter()
                            .copied()
                            .filter(|&j| j != i)
                            .collect();
                        if ui
                            .add_enabled(!others.is_empty(), egui::Button::new("Close Other Tabs"))
                            .clicked()
                        {
                            close_batch = Some(others);
                            ui.close();
                        }
                        if ui
                            .add_enabled(!left.is_empty(), egui::Button::new("Close Tabs to the Left"))
                            .clicked()
                        {
                            close_batch = Some(left);
                            ui.close();
                        }
                        if ui
                            .add_enabled(!right.is_empty(), egui::Button::new("Close Tabs to the Right"))
                            .clicked()
                        {
                            close_batch = Some(right);
                            ui.close();
                        }
                        if ui
                            .add_enabled(
                                non_floating.len() > 1,
                                egui::Button::new("Close All Tabs"),
                            )
                            .clicked()
                        {
                            close_batch = Some(non_floating.clone());
                            ui.close();
                        }
                        ui.separator();
                        if ui
                            .add_enabled(i > 0, egui::Button::new("Move Tab Left"))
                            .clicked()
                        {
                            swap_pair = Some((i, i - 1));
                            ui.close();
                        }
                        if ui
                            .add_enabled(i + 1 < doc_count, egui::Button::new("Move Tab Right"))
                            .clicked()
                        {
                            swap_pair = Some((i, i + 1));
                            ui.close();
                        }
                        if ui.button("Float in New Window (Multi-Monitor)").clicked() {
                            float_idx = Some(i);
                            ui.close();
                        }
                    });
                }
                if hidden_count > 0 {
                    let menu = ui.menu_button("…", |ui| {
                        ui.set_min_width(220.0);
                        ui.label(
                            RichText::new(format!("Open Documents ({doc_count})"))
                                .small()
                                .color(p.text_dim),
                        );
                        ui.separator();
                        egui::ScrollArea::vertical()
                            .max_height(320.0)
                            .show(ui, |ui| {
                                for &idx in &non_floating {
                                    let doc = &app.open_docs[idx];
                                    let label = format!(
                                        "{}{}",
                                        if doc.dirty { "*" } else { "" },
                                        doc.name
                                    );
                                    if ui
                                        .selectable_label(
                                            idx == app.active_doc,
                                            format!("{label}  [{}]", idx + 1),
                                        )
                                        .clicked()
                                    {
                                        switch_to = Some(idx);
                                        ui.close();
                                    }
                                }
                            });
                        ui.separator();
                        if ui.button("Close All Tabs").clicked() {
                            close_batch = Some(non_floating.clone());
                            ui.close();
                        }
                    });
                    menu.response
                        .on_hover_text(format!("{hidden_count} hidden tab(s) — click to switch"));
                }
                if ui
                    .small_button("+")
                    .on_hover_text("New Document Tab (1920×1080 @ 144 ppi)")
                    .clicked()
                {
                    app.new_document_tab(constants::DEFAULT_WIDTH, constants::DEFAULT_HEIGHT);
                }
                if !app.open_docs.is_empty()
                    && !app.floating_doc_tabs.contains(&app.active_doc)
                    && ui
                        .small_button("⧉")
                        .on_hover_text("Float active canvas in a movable / multi-monitor OS window")
                        .clicked()
                {
                    float_idx = Some(app.active_doc);
                }
                if !app.floating_doc_tabs.is_empty()
                    && ui
                        .small_button(format!("Dock All ({})", app.floating_doc_tabs.len()))
                        .on_hover_text("Dock all floating canvas windows back into main window")
                        .clicked()
                {
                    app.floating_doc_tabs.clear();
                }
                if let Some((drag_i, _drag_r, ptr)) = dragging_tab {
                    for &(other_i, other_r, _) in &tab_rects {
                        if other_i != drag_i
                            && ((other_i < drag_i && ptr.x < other_r.center().x)
                                || (other_i > drag_i && ptr.x > other_r.center().x))
                        {
                            swap_pair = Some((drag_i, other_i));
                            break;
                        }
                    }
                }
                if let Some((a, b)) = swap_pair {
                    app.swap_document_tabs(a, b);
                }
                if let Some(fi) = float_idx {
                    app.sync_active_doc_slot();
                    if !app.floating_doc_tabs.contains(&fi) {
                        app.floating_doc_tabs.push(fi);
                        app.status = format!(
                            "Floated {} to movable OS window (drag to any monitor)",
                            app.open_docs[fi].name
                        );
                    }
                }
                if let Some(target) = switch_to {
                    app.switch_document_tab(target);
                }
                if let Some(batch) = close_batch {
                    app.request_close_document_tabs(batch);
                } else if let Some(ci) = close_idx {
                    app.request_close_document_tab(ci);
                }
            });

            // Row 2: Original | Preview | 2-Up | 4-Up with CS6 view icons + Page Selector
            egui::Frame::new()
                .fill(p.panel)
                .inner_margin(egui::Margin::symmetric(4, 2))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
                        for (mode, icon, label) in [
                            (CanvasViewMode::Original, icons::Icon::Pen, "Original"),
                            (CanvasViewMode::Preview, icons::Icon::Eye, "Preview"),
                            (CanvasViewMode::TwoUp, icons::Icon::Slice, "2-Up"),
                            (CanvasViewMode::FourUp, icons::Icon::Scale, "4-Up"),
                        ] {
                            let active = app.view_mode == mode;
                            let w = 24.0 + label.len() as f32 * 5.8;
                            let (r, resp) =
                                ui.allocate_exact_size(Vec2::new(w, 19.0), egui::Sense::click());
                            let painter = ui.painter();
                            if active {
                                painter.rect_filled(r, 0.0, p.selection.gamma_multiply(0.28));
                                painter.rect_stroke(
                                    r,
                                    0.0,
                                    egui::Stroke::new(1.0, p.selection),
                                    egui::StrokeKind::Middle,
                                );
                            } else if resp.hovered() {
                                painter.rect_filled(r, 0.0, p.well);
                            }
                            let ic_r = egui::Rect::from_min_size(
                                egui::pos2(r.left() + 3.0, r.top() + 2.0),
                                Vec2::splat(15.0),
                            );
                            icons::draw(
                                painter,
                                ic_r,
                                icon,
                                if active { p.accent } else { p.text_dim },
                                1.0,
                            );
                            painter.text(
                                egui::pos2(ic_r.right() + 3.0, r.center().y),
                                egui::Align2::LEFT_CENTER,
                                label,
                                egui::FontId::proportional(10.5),
                                if active { p.text } else { p.text_dim },
                            );
                            if resp.clicked() {
                                app.view_mode = mode;
                                app.focus_canvas = true;
                                app.status = format!("{} view active", label);
                            }
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let page_count = app.doc.pages.len();
                            let cur_page_name = app.doc.page().name.clone();
                            egui::ComboBox::from_id_salt("canvas_page_selector")
                                .width(95.0)
                                .selected_text(cur_page_name)
                                .show_ui(ui, |ui| {
                                    for pi in 0..page_count {
                                        let pname = app.doc.pages[pi].name.clone();
                                        if ui
                                            .selectable_label(app.doc.current_page == pi, pname)
                                            .clicked()
                                        {
                                            app.doc.current_page = pi;
                                            app.clear_selection();
                                        }
                                    }
                                });
                        });
                    });
                });
        });
}

pub(super) fn bottom_properties_dock(ui: &mut Ui, app: &mut FireCraft) {
    const TABS: &[&str] = &["◊ Properties", "Symbol Properties", "Intelligence"];
    let current = app.bottom_tab.min(TABS.len() - 1);
    let mut tab = current;
    let palette = app.palette;
    let collapsed = app.bottom_collapsed;

    if !collapsed {
        // Slim resize grip owned by the app so the height survives content re-measurement.
        let (grip, grip_resp) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 4.0), Sense::drag());
        if grip_resp.hovered() || grip_resp.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
            ui.painter().line_segment(
                [grip.left_center(), grip.right_center()],
                egui::Stroke::new(1.0, palette.accent),
            );
        }
        if grip_resp.dragged() {
            let dy = ui.ctx().input(|i| i.pointer.delta().y);
            app.bottom_height = (app.bottom_height - dy).clamp(118.0, 340.0);
            ui.ctx().request_repaint();
        }
        grip_resp.on_hover_text("Drag to resize the Properties inspector");
    }

    let toggle = panels::panel_collapsible(
        ui,
        "Properties",
        TABS,
        &mut tab,
        palette,
        if collapsed {
            None
        } else {
            Some((app.bottom_height - 42.0).clamp(58.0, 300.0))
        },
        collapsed,
        |ui| {
            if current == 0 {
                panels::properties_panel(ui, app);
            } else if current == 1 {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Symbol Library Instances:")
                            .small()
                            .color(palette.text_dim),
                    );
                    if ui
                        .small_button("Convert Selection to Symbol (F8)")
                        .clicked()
                    {
                        app.convert_selection_to_symbol();
                    }
                    ui.label(
                        RichText::new(format!(
                            "{} symbol definition(s) in current document",
                            app.doc.symbols.len()
                        ))
                        .small(),
                    );
                });
            } else {
                panels::intelligence_panel(ui, app);
            }
        },
    );
    if toggle {
        app.bottom_collapsed = !app.bottom_collapsed;
    }
    app.bottom_tab = tab;
}

/// The stacked right-hand dock matching Fireworks CS6's Expanded Mode, with fixed body heights
/// so switching tabs never causes adjacent dock groups to jump around:
/// 1. Optimize | History | Align | Path (122 px)
/// 2. Pages | States | Layers (160 px)
/// 3. Styles | Color Palette | Swatches | Info (145 px)
/// 4. Chars | Image Edit | Auto Shapes | Library | CSS (115 px)
pub(super) fn canvas_footer_bar(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
        ui.label(
            RichText::new(format!("{} (Document)", app.doc.optimize.format))
                .small()
                .color(p.text_dim),
        );
        ui.separator();
        let snap_on = app.snap_to_guides || app.snap_to_objects || app.snap_to_grid;
        if ui
            .selectable_label(
                snap_on,
                RichText::new(if snap_on {
                    "🧲 Snap: On"
                } else {
                    "Snap: Off"
                })
                .small(),
            )
            .on_hover_text("Toggle Smart Guides & Object Snapping (View > Snap)")
            .clicked()
        {
            let next = !snap_on;
            app.snap_to_guides = next;
            app.snap_to_objects = next;
            app.status = if next {
                "Smart snapping enabled (Guides + Objects)".to_string()
            } else {
                "Snapping disabled".to_string()
            };
        }
        ui.separator();
        if ui
            .selectable_label(
                app.infinite_canvas,
                RichText::new(if app.infinite_canvas {
                    "∞ Canvas: On"
                } else {
                    "∞ Canvas"
                })
                .small(),
            )
            .on_hover_text(
                "Infinite canvas: objects may extend beyond the page. The dotted box marks the page bounds. Flat raster, SVG, and HTML exports crop to the page; out-of-page objects stay editable in the project.",
            )
            .clicked()
        {
            app.infinite_canvas = !app.infinite_canvas;
            app.status = if app.infinite_canvas {
                "Infinite canvas ON — dotted box marks page bounds; flat exports crop to it"
                    .to_string()
            } else {
                "Infinite canvas OFF".to_string()
            };
        }
        if app.infinite_canvas {
            ui.label(
                RichText::new("only page bounds are included in flat exports")
                    .small()
                    .color(p.text_dim),
            );
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // DPI / PPI Dropdown (moved down from the Main Toolbar)
            egui::ComboBox::from_id_salt("canvas_footer_dpi")
                .width(68.0)
                .selected_text(format!("{} ppi", app.doc.dpi))
                .show_ui(ui, |ui| {
                    for &dpi in constants::DPI_PRESETS {
                        if ui
                            .selectable_label(app.doc.dpi == dpi, format!("{dpi} ppi"))
                            .clicked()
                        {
                            app.doc.dpi = dpi;
                            app.status = format!("Document resolution set to {dpi} ppi");
                        }
                    }
                });

            // Resolution Dropdown (e.g. 1920 × 1080 ▾)
            let (pw, ph) = (
                app.doc.page().width.round() as i32,
                app.doc.page().height.round() as i32,
            );
            egui::ComboBox::from_id_salt("canvas_footer_res")
                .width(122.0)
                .selected_text(format!("{pw} × {ph}"))
                .show_ui(ui, |ui| {
                    for &(label, w, h) in constants::RESOLUTION_PRESETS {
                        let active = (app.doc.page().width - w).abs() < 0.5
                            && (app.doc.page().height - h).abs() < 0.5;
                        if ui.selectable_label(active, label).clicked() {
                            app.resize_canvas(w, h);
                        }
                    }
                    ui.separator();
                    if ui.button("Custom Canvas Size…").clicked() {
                        app.open_canvas_size_dialog();
                    }
                });

            // Zoom Scale Dropdown (e.g. 50% ▾)
            let zoom_pct = (app.doc.zoom * 100.0).round() as i32;
            egui::ComboBox::from_id_salt("canvas_footer_zoom")
                .width(62.0)
                .selected_text(format!("{zoom_pct}%"))
                .show_ui(ui, |ui| {
                    for pct in [12, 25, 33, 50, 66, 100, 150, 200, 300, 400] {
                        if ui
                            .selectable_label(zoom_pct == pct, format!("{pct}%"))
                            .clicked()
                        {
                            app.set_zoom(pct as f32 / 100.0);
                        }
                    }
                    ui.separator();
                    if ui.button("Fit in Window (Insert)").clicked() {
                        app.fit_in_window();
                    }
                });

            ui.separator();

            // Single canonical state transport. Widgets are added right-to-left so the
            // visual order is: First · Previous · Play/Pause · 1 / N · Next · Last.
            let state_count = app.doc.page().states.len().max(1);
            let cur_state = app.doc.page().current_state.min(state_count - 1);
            let can_prev = cur_state > 0;
            let can_next = cur_state + 1 < state_count;
            let can_play = state_count > 1;

            if footer_icon_button(ui, icons::Icon::StateLast, can_next, "Last State")
                && can_next
            {
                app.doc.page_mut().select_state(state_count - 1);
                app.selected = None;
                app.selected_set.clear();
            }
            if footer_icon_button(ui, icons::Icon::StateNext, can_next, "Next State")
                && can_next
            {
                app.select_state_step(1);
            }
            ui.label(
                RichText::new(format!("{} / {}", cur_state + 1, state_count))
                    .small()
                    .strong(),
            );
            let play_icon = if app.anim_playing {
                icons::Icon::StatePause
            } else {
                icons::Icon::StatePlay
            };
            if footer_icon_button(
                ui,
                play_icon,
                can_play,
                if app.anim_playing {
                    "Pause Animation States"
                } else {
                    "Play Animation States"
                },
            ) && can_play
            {
                app.anim_playing = !app.anim_playing;
                app.anim_last_tick = ui.input(|i| i.time);
            }
            if footer_icon_button(ui, icons::Icon::StatePrev, can_prev, "Previous State")
                && can_prev
            {
                app.select_state_step(-1);
            }
            if footer_icon_button(ui, icons::Icon::StateFirst, can_prev, "First State")
                && can_prev
            {
                app.doc.page_mut().select_state(0);
                app.selected = None;
                app.selected_set.clear();
            }
        });
    });
}

/// Compact monochrome transport button used by the canvas footer. Disabled controls stay
/// visible (dimmed) so the available actions never reflow while stepping through states.
pub(super) fn footer_icon_button(ui: &mut Ui, icon: icons::Icon, enabled: bool, tip: &str) -> bool {
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(20.0, 18.0), Sense::click());
    let hovered = enabled && resp.hovered();
    if hovered {
        ui.painter()
            .rect_filled(rect.shrink(1.0), 0.0, egui::Color32::from_white_alpha(18));
    }
    let color = if enabled {
        if hovered {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(196)
        }
    } else {
        egui::Color32::from_gray(96)
    };
    icons::draw(ui.painter(), rect.shrink(2.0), icon, color, 1.15);
    let clicked = resp.clicked() && enabled;
    if enabled {
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        resp.on_hover_text(tip);
    } else {
        resp.on_hover_text(format!("{tip} (unavailable)"));
    }
    clicked
}
