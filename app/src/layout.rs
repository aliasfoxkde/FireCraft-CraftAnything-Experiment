//! The window layout: menu bar, CS6 Main Toolbar, document tabs + Original/Preview/2-Up/4-Up bar,
//! left 2-column tool palette, right stacked tab-group dock, bottom Properties Inspector,
//! status bar, and modal dialogs.

use crate::app::{CanvasViewMode, FileMode, FireCraft, Tool};
use crate::canvas;
use crate::constants;
use crate::dialogs;
use crate::icons;
use crate::menus;
use crate::panels;
use crate::theme;

use eframe::egui::{self, Key, RichText, Ui, Vec2};

/// Draw the whole editor for one frame.
pub fn show(ui: &mut Ui, app: &mut FireCraft) {
    let ctx = ui.ctx().clone();
    app.ctx = ctx.clone();
    app.frames = app.frames.saturating_add(1);
    if app.logo_tex.is_none() {
        app.logo_tex = FireCraft::load_logo(&ctx);
    }
    let theme_id = ui.make_persistent_id("applied_theme_mode");
    let last_theme = ctx.data(|d| d.get_temp::<crate::theme::ThemeMode>(theme_id));
    if last_theme != Some(app.theme_mode) {
        theme::apply_cs6_style(&ctx, &app.palette, app.palette.is_dark());
        ctx.data_mut(|d| d.insert_temp(theme_id, app.theme_mode));
    }

    if dialogs::startup_splash(ui, app) {
        return;
    }

    shortcuts(ui, app);

    // 1. Top Menu Bar (10 CS6 menus + Bridge/Hand/Zoom/Resolution/Theme/Workspace mode selector)
    egui::Panel::top("menu")
        .exact_size(26.0)
        .frame(
            egui::Frame::new()
                .fill(app.palette.panel)
                .inner_margin(egui::Margin::symmetric(6, 2)),
        )
        .show(ui, |ui| menus::menu_bar(ui, app));

    // 2. CS6 Main Toolbar (New, Open, Save, Import, Export, Cut, Copy, Paste, Undo, Redo, Align, Arrange, Resolution)
    if app.show_main_toolbar {
        egui::Panel::top("main_toolbar")
            .exact_size(28.0)
            .frame(
                egui::Frame::new()
                    .fill(app.palette.panel_alt)
                    .inner_margin(egui::Margin::symmetric(6, 2)),
            )
            .show(ui, |ui| main_toolbar(ui, app));
    }

    // 3. Bottom Status Bar
    egui::Panel::bottom("status")
        .exact_size(22.0)
        .frame(
            egui::Frame::new()
                .fill(app.palette.panel_alt)
                .inner_margin(egui::Margin::symmetric(6, 2)),
        )
        .show(ui, |ui| status_bar(ui, app));

    // 4. Bottom Dock: Properties & Symbol Properties Inspector (fixed default height so switching selection never jumps)
    if app.show_bottom_inspector {
        egui::Panel::bottom("properties_dock")
            .exact_size(104.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(app.palette.panel)
                    .inner_margin(egui::Margin::symmetric(8, 5)),
            )
            .show(ui, |ui| bottom_properties_dock(ui, app));
    }

    // 5. Left Dock: 2-Column CS6 Tool Strip
    if app.show_tools_dock {
        egui::Panel::left("tools")
            .exact_size(76.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(app.palette.panel)
                    .inner_margin(egui::Margin::symmetric(5, 4)),
            )
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| panels::tools_panel(ui, app));
            });
    }

    app.panel_title_rects.clear();
    let now = ui.input(|i| i.time);
    app.tick_animation(ui.ctx(), now);

    let has_left_inner = app
        .panel_groups
        .iter()
        .any(|g| g.zone == crate::app::DockZone::LeftInner && !g.tabs.is_empty());
    if app.show_right_dock && has_left_inner {
        egui::Panel::left("dock_left_inner")
            .default_size(240.0)
            .min_size(200.0)
            .max_size(320.0)
            .resizable(true)
            .frame(
                egui::Frame::new()
                    .fill(app.palette.panel)
                    .inner_margin(egui::Margin::same(3)),
            )
            .show(ui, |ui| {
                dock_column(ui, app, crate::app::DockZone::LeftInner);
            });
    }

    // 6. Right Dock: CS6 Stacked Tab-Groups (Outer + optional Inner 2-Column)
    let mut right_outer_rect = None;
    let mut right_inner_rect = None;
    if app.show_right_dock {
        let resp = egui::Panel::right("dock")
            .default_size(275.0)
            .min_size(240.0)
            .max_size(360.0)
            .resizable(true)
            .frame(
                egui::Frame::new()
                    .fill(app.palette.panel)
                    .inner_margin(egui::Margin::same(3)),
            )
            .show(ui, |ui| right_dock(ui, app));
        right_outer_rect = Some(resp.response.rect);

        let has_right_inner = app
            .panel_groups
            .iter()
            .any(|g| g.zone == crate::app::DockZone::RightInner && !g.tabs.is_empty());
        if has_right_inner {
            let resp2 = egui::Panel::right("dock_inner")
                .default_size(245.0)
                .min_size(210.0)
                .max_size(330.0)
                .resizable(true)
                .frame(
                    egui::Frame::new()
                        .fill(app.palette.panel)
                        .inner_margin(egui::Margin::same(3)),
                )
                .show(ui, |ui| {
                    dock_column(ui, app, crate::app::DockZone::RightInner);
                });
            right_inner_rect = Some(resp2.response.rect);
        }
    }

    // 7. Central Panel: Document Tab Bar + Canvas + CS6 Canvas Footer Bar (Transport, Resolution, Scale)
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(app.palette.bg))
        .show(ui, |ui| {
            document_tab_strip(ui, app);
            egui::Panel::bottom("canvas_footer_bar")
                .exact_size(24.0)
                .frame(
                    egui::Frame::new()
                        .fill(app.palette.panel)
                        .stroke(egui::Stroke::new(1.0, app.palette.stroke))
                        .inner_margin(egui::Margin::symmetric(8, 2)),
                )
                .show(ui, |ui| canvas_footer_bar(ui, app));
            canvas::show(ui, app);
        });

    // 8. Floating Panel Windows with Magnetic Snapping
    let floating_indices: Vec<usize> = app
        .panel_groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.zone == crate::app::DockZone::Floating && !g.tabs.is_empty())
        .map(|(i, _)| i)
        .collect();
    for idx in floating_indices {
        let (gid, fpos, fwidth, fheight, collapsed) = {
            let g = &app.panel_groups[idx];
            (
                g.id,
                g.floating_pos,
                g.floating_width,
                g.body_height,
                g.collapsed,
            )
        };
        let area_resp = egui::Area::new(ui.make_persistent_id(("floating_pg", gid)))
            .order(egui::Order::Middle)
            .current_pos(fpos)
            .movable(true)
            .show(ui.ctx(), |ui| {
                ui.set_width(fwidth);
                panels::panel_group_widget(ui, app, idx, true);
            });
        let new_pos = area_resp.response.rect.min;
        if (new_pos - fpos).length() > 0.5 {
            let est_h = if collapsed { 24.0 } else { fheight + 28.0 };
            let snapped = app.snap_floating_pos(
                gid,
                new_pos,
                Vec2::new(fwidth, est_h),
                ui.ctx().content_rect(),
            );
            if let Some(g) = app.panel_groups.get_mut(idx) {
                g.floating_pos = snapped;
            }
        }
    }

    // 9. Resolve Dragging Panel Tab or Whole Group Drop (Group / Dock / Tear-off Floating)
    if let Some(dragged_gid) = app.dragging_group {
        let pointer_pos = ui.ctx().pointer_latest_pos();
        let released = ui.input(|i| !i.pointer.primary_down());
        if !released && let Some(pos) = pointer_pos {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            let ghost_r =
                egui::Rect::from_min_size(pos + Vec2::new(8.0, 8.0), Vec2::new(200.0, 26.0));
            let fg = ui.ctx().layer_painter(egui::LayerId::new(
                egui::Order::Tooltip,
                ui.make_persistent_id("group_drag_ghost"),
            ));
            fg.rect_filled(ghost_r, 0.0, app.palette.panel_alt.gamma_multiply(0.92));
            fg.rect_stroke(
                ghost_r,
                0.0,
                egui::Stroke::new(1.5, app.palette.accent),
                egui::StrokeKind::Inside,
            );
            fg.text(
                ghost_r.center(),
                egui::Align2::CENTER_CENTER,
                "Moving Panel Group (Drop to Dock / Merge / Float)",
                egui::FontId::proportional(10.5),
                app.palette.text,
            );
        }
        if released {
            app.dragging_group = None;
            if let Some(pos) = pointer_pos {
                if let Some((target_gid, _)) = app
                    .panel_title_rects
                    .iter()
                    .find(|(gid, r)| *gid != dragged_gid && r.contains(pos))
                    .copied()
                {
                    app.merge_groups(dragged_gid, target_gid);
                } else if right_inner_rect.is_some_and(|r| r.contains(pos)) {
                    app.move_group_to_zone(dragged_gid, crate::app::DockZone::RightInner, None);
                } else if right_outer_rect.is_some_and(|r| r.contains(pos)) {
                    app.move_group_to_zone(dragged_gid, crate::app::DockZone::RightOuter, None);
                } else {
                    let snapped = app.snap_floating_pos(
                        dragged_gid,
                        pos - Vec2::new(110.0, 10.0),
                        Vec2::new(250.0, 180.0),
                        ui.ctx().content_rect(),
                    );
                    app.move_group_to_zone(
                        dragged_gid,
                        crate::app::DockZone::Floating,
                        Some(snapped),
                    );
                }
            }
        }
    }

    if let Some(dragged) = app.dragging_panel {
        let pointer_pos = ui.ctx().pointer_latest_pos();
        let released = ui.input(|i| !i.pointer.primary_down());
        if !released && let Some(pos) = pointer_pos {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            let ghost_r =
                egui::Rect::from_min_size(pos + Vec2::new(8.0, 8.0), Vec2::new(180.0, 26.0));
            let fg = ui.ctx().layer_painter(egui::LayerId::new(
                egui::Order::Tooltip,
                ui.make_persistent_id("panel_drag_ghost"),
            ));
            fg.rect_filled(ghost_r, 0.0, app.palette.panel_alt.gamma_multiply(0.92));
            fg.rect_stroke(
                ghost_r,
                0.0,
                egui::Stroke::new(1.5, app.palette.accent),
                egui::StrokeKind::Inside,
            );
            fg.text(
                ghost_r.center(),
                egui::Align2::CENTER_CENTER,
                format!("Moving: {}", dragged.title()),
                egui::FontId::proportional(11.0),
                app.palette.text,
            );
        }
        if released {
            app.dragging_panel = None;
            if let Some(pos) = pointer_pos {
                if let Some((target_gid, _)) = app
                    .panel_title_rects
                    .iter()
                    .find(|(_, r)| r.contains(pos))
                    .copied()
                {
                    app.group_panel_into(dragged, target_gid);
                } else if right_inner_rect.is_some_and(|r| r.contains(pos)) {
                    app.ungroup_panel_to_floating(dragged, pos);
                    if let Some(last_id) = app.panel_groups.last().map(|g| g.id) {
                        app.move_group_to_zone(last_id, crate::app::DockZone::RightInner, None);
                    }
                } else if right_outer_rect.is_some_and(|r| r.contains(pos)) {
                    app.ungroup_panel_to_floating(dragged, pos);
                    if let Some(last_id) = app.panel_groups.last().map(|g| g.id) {
                        app.move_group_to_zone(last_id, crate::app::DockZone::RightOuter, None);
                    }
                } else {
                    app.ungroup_panel_to_floating(dragged, pos - Vec2::new(110.0, 10.0));
                }
            }
        }
    }

    dialogs::new_document_dialog(app);
    dialogs::canvas_size_dialog(app);
    dialogs::find_replace_dialog(app);
    dialogs::saved_commands_dialog(app);
    dialogs::command_palette_dialog(app);
    dialogs::mcp_cli_dialog(app);
    dialogs::about_dialog(app);
    dialogs::update_dialog(app);
    dialogs::unsaved_changes_dialog(app);
    dialogs::file_dialog(app);
    dialogs::welcome_overlay(app);

    screenshot(ui, app);
}

fn toolbar_sep(ui: &mut Ui, color: egui::Color32) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(7.0, 20.0), egui::Sense::hover());
    ui.painter().line_segment(
        [
            egui::pos2(r.center().x, r.top() + 3.0),
            egui::pos2(r.center().x, r.bottom() - 3.0),
        ],
        egui::Stroke::new(1.0, color),
    );
}

fn main_toolbar(ui: &mut Ui, app: &mut FireCraft) {
    let sep_color = app.palette.stroke;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        if canvas::tool_button(ui, icons::Icon::New, false, "New Document (Ctrl+N)").clicked() {
            app.show_new_dialog = true;
        }
        if canvas::tool_button(ui, icons::Icon::Open, false, "Open Document (Ctrl+O)").clicked() {
            app.begin_file_dialog(FileMode::Open);
        }
        if canvas::tool_button(ui, icons::Icon::Save, false, "Save Document (Ctrl+S)").clicked() {
            app.save_current();
        }
        if canvas::tool_button(ui, icons::Icon::Import, false, "Import (Ctrl+R)").clicked() {
            app.begin_file_dialog(FileMode::Import);
        }
        if canvas::tool_button(ui, icons::Icon::Export, false, "Export (Ctrl+Shift+R)").clicked() {
            app.begin_file_dialog(FileMode::Export);
        }
        toolbar_sep(ui, sep_color);
        if canvas::tool_button(ui, icons::Icon::Cut, false, "Cut (Ctrl+X)").clicked() {
            app.cut_selected();
        }
        if canvas::tool_button(ui, icons::Icon::Copy, false, "Copy (Ctrl+C)").clicked() {
            app.copy_selected();
        }
        if canvas::tool_button(ui, icons::Icon::Paste, false, "Paste (Ctrl+V)").clicked() {
            app.paste();
        }
        if canvas::tool_button(ui, icons::Icon::Duplicate, false, "Duplicate (Ctrl+D)").clicked() {
            app.duplicate_selected();
        }
        toolbar_sep(ui, sep_color);
        if canvas::tool_button(ui, icons::Icon::Undo, false, "Undo (Ctrl+Z)").clicked() {
            app.undo_action();
        }
        if canvas::tool_button(ui, icons::Icon::Redo, false, "Redo (Ctrl+Y)").clicked() {
            app.redo_action();
        }
        toolbar_sep(ui, sep_color);
        if canvas::tool_button(
            ui,
            icons::Icon::Group,
            false,
            "Group Selected Objects (Ctrl+G)",
        )
        .clicked()
        {
            app.group_selected();
        }
        if canvas::tool_button(
            ui,
            icons::Icon::Ungroup,
            false,
            "Ungroup Selected Group (Ctrl+Shift+G)",
        )
        .clicked()
        {
            app.ungroup_selected();
        }
        toolbar_sep(ui, sep_color);
        if canvas::tool_button(
            ui,
            icons::Icon::BringFront,
            false,
            "Bring to Front (Ctrl+Shift+Up)",
        )
        .clicked()
        {
            app.reorder_selected(true);
        }
        if canvas::tool_button(
            ui,
            icons::Icon::BringForward,
            false,
            "Bring Forward One Level (Ctrl+Up)",
        )
        .clicked()
        {
            app.reorder_selected_step(true);
        }
        if canvas::tool_button(
            ui,
            icons::Icon::SendBackward,
            false,
            "Send Backward One Level (Ctrl+Down)",
        )
        .clicked()
        {
            app.reorder_selected_step(false);
        }
        if canvas::tool_button(
            ui,
            icons::Icon::SendBack,
            false,
            "Send to Back (Ctrl+Shift+Down)",
        )
        .clicked()
        {
            app.reorder_selected(false);
        }
        ui.separator();
        if canvas::tool_button(
            ui,
            icons::Icon::RotateCCW,
            false,
            "Rotate 90° Counter-Clockwise",
        )
        .clicked()
        {
            app.rotate_ccw_90();
        }
        if canvas::tool_button(ui, icons::Icon::RotateCW, false, "Rotate 90° Clockwise").clicked()
        {
            app.rotate_90();
        }
        if canvas::tool_button(ui, icons::Icon::FlipH, false, "Flip Horizontal").clicked() {
            app.flip_horizontal();
        }
        if canvas::tool_button(ui, icons::Icon::FlipV, false, "Flip Vertical").clicked() {
            app.flip_vertical();
        }
        ui.separator();
        // Canvas Resolution & DPI Dropdown right on the Main Toolbar
        let cur_res = format!("{:.0}×{:.0}", app.doc.page().width, app.doc.page().height);
        egui::ComboBox::from_id_salt("toolbar_resolution_combo")
            .width(105.0)
            .selected_text(cur_res)
            .show_ui(ui, |ui| {
                for &(label, w, h) in constants::RESOLUTION_PRESETS {
                    let active = (app.doc.page().width - w).abs() < 0.5
                        && (app.doc.page().height - h).abs() < 0.5;
                    if ui.selectable_label(active, label).clicked() {
                        app.resize_canvas(w, h);
                    }
                }
            });
        egui::ComboBox::from_id_salt("toolbar_dpi_combo")
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
        ui.separator();
        if ui
            .small_button("Demo Showcase")
            .on_hover_text("Populate canvas with a Fireworks CS6 vector + live-effect UI showcase")
            .clicked()
        {
            app.load_demo_showcase();
        }
    });
}

fn document_tab_strip(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    egui::Frame::new()
        .fill(p.panel_alt)
        .inner_margin(egui::Margin::symmetric(4, 2))
        .show(ui, |ui| {
            // Row 1: Unified sculpted CS6 Document Tabs (middle-click or embedded '×' closes the tab)
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);
                let doc_count = app.open_docs.len();
                let mut switch_to = None;
                let mut close_idx = None;
                for i in 0..doc_count {
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
                    let tab_w = (28.0 + name.len() as f32 * 6.2).clamp(112.0, 210.0);
                    let (tab_r, tab_resp) =
                        ui.allocate_exact_size(Vec2::new(tab_w, 21.0), egui::Sense::click());
                    let close_r = egui::Rect::from_center_size(
                        egui::pos2(tab_r.right() - 11.0, tab_r.center().y),
                        Vec2::splat(14.0),
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
                    painter.text(
                        egui::pos2(tab_r.left() + 8.0, tab_r.center().y),
                        egui::Align2::LEFT_CENTER,
                        &name,
                        egui::FontId::proportional(11.0),
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
                    if close_resp
                        .on_hover_text("Close Document Tab (Middle-Click or Ctrl+W)")
                        .clicked()
                        || tab_resp.middle_clicked()
                    {
                        close_idx = Some(i);
                    } else if tab_resp
                        .on_hover_text("Left-click to switch tab · Middle-click to close tab")
                        .clicked()
                        && !is_active
                    {
                        switch_to = Some(i);
                    }
                }
                if ui
                    .small_button("+")
                    .on_hover_text("New Document Tab (1920×1080 @ 144 ppi)")
                    .clicked()
                {
                    app.new_document_tab(constants::DEFAULT_WIDTH, constants::DEFAULT_HEIGHT);
                }
                if let Some(target) = switch_to {
                    app.switch_document_tab(target);
                }
                if let Some(ci) = close_idx {
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

fn bottom_properties_dock(ui: &mut Ui, app: &mut FireCraft) {
    const TABS: &[&str] = &["◊ Properties", "Symbol Properties"];
    let current = app.bottom_tab.min(1);
    let mut tab = current;
    let palette = app.palette;
    panels::panel(
        ui,
        TABS[current],
        TABS,
        &mut tab,
        palette,
        Some(58.0),
        |ui| {
            if current == 0 {
                panels::properties_panel(ui, app);
            } else {
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
            }
        },
    );
    app.bottom_tab = tab;
}

/// The stacked right-hand dock matching Fireworks CS6's Expanded Mode, with fixed body heights
/// so switching tabs never causes adjacent dock groups to jump around:
/// 1. Optimize | History | Align | Path (122 px)
/// 2. Pages | States | Layers (160 px)
/// 3. Styles | Color Palette | Swatches | Info (145 px)
/// 4. Chars | Image Edit | Auto Shapes | Library | CSS (115 px)
fn canvas_footer_bar(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
        ui.label(
            RichText::new(format!("{} (Document)", app.doc.optimize.format))
                .small()
                .color(p.text_dim),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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

            // Resolution Dropdown (e.g. 1920 x 1080 ▾)
            let (pw, ph) = (app.doc.page().width as i32, app.doc.page().height as i32);
            egui::ComboBox::from_id_salt("canvas_footer_res")
                .width(108.0)
                .selected_text(format!("{pw} x {ph}"))
                .show_ui(ui, |ui| {
                    for (w, h, lbl) in [
                        (1920, 1080, "1920 × 1080 (Full HD)"),
                        (1440, 900, "1440 × 900 (WXGA+)"),
                        (1280, 720, "1280 × 720 (HD)"),
                        (1024, 768, "1024 × 768 (XGA)"),
                        (800, 600, "800 × 600 (SVGA)"),
                    ] {
                        if ui.selectable_label(pw == w && ph == h, lbl).clicked() {
                            app.resize_canvas(w as f32, h as f32);
                        }
                    }
                    ui.separator();
                    if ui.button("Custom Canvas Size…").clicked() {
                        app.open_canvas_size_dialog();
                    }
                });

            ui.separator();

            // Animation / State Transport Controls: |<  ▶/⏸  >|  <frame>  ◀  ▶
            let state_count = app.doc.page().states.len().max(1);
            let cur_state = app.doc.page().current_state;
            if ui.small_button("▶").on_hover_text("Next State").clicked() {
                app.select_state_step(1);
            }
            if ui
                .small_button("◀")
                .on_hover_text("Previous State")
                .clicked()
            {
                app.select_state_step(-1);
            }
            ui.label(RichText::new(format!("{}", cur_state + 1)).small().strong());
            if ui.small_button(">|").on_hover_text("Last State").clicked() {
                app.doc.page_mut().select_state(state_count - 1);
            }
            if ui
                .small_button(if app.anim_playing { "⏸" } else { "▶" })
                .on_hover_text("Play / Pause Animation States")
                .clicked()
            {
                app.anim_playing = !app.anim_playing;
                app.anim_last_tick = ui.input(|i| i.time);
            }
            if ui.small_button("|<").on_hover_text("First State").clicked() {
                app.doc.page_mut().select_state(0);
            }
        });
    });
}

fn dock_column(ui: &mut Ui, app: &mut FireCraft, zone: crate::app::DockZone) {
    let p = app.palette;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button("▸")
                .on_hover_text("Expand all panels in dock")
                .clicked()
            {
                app.collapse_all_panels(false);
            }
            if ui
                .small_button("▾")
                .on_hover_text("Collapse all panels in dock")
                .clicked()
            {
                app.collapse_all_panels(true);
            }
        });
    });
    ui.add_space(2.0);
    let indices: Vec<usize> = app
        .panel_groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.zone == zone && !g.tabs.is_empty())
        .map(|(i, _)| i)
        .collect();
    egui::ScrollArea::vertical()
        .id_salt(("dock_scroll", zone as u8))
        .show(ui, |ui| {
            for idx in indices {
                panels::panel_group_widget(ui, app, idx, false);
                ui.add_space(3.0);
            }
            if app.dragging_panel.is_some() {
                let drop_r = ui.available_rect_before_wrap();
                if drop_r.height() > 20.0 {
                    ui.painter().rect_stroke(
                        drop_r,
                        2.0,
                        egui::Stroke::new(1.5, p.accent),
                        egui::StrokeKind::Inside,
                    );
                }
            }
        });
}

fn right_dock(ui: &mut Ui, app: &mut FireCraft) {
    dock_column(ui, app, crate::app::DockZone::RightOuter);
}

/// The bottom status bar + CS6 animation playback transport controls.
fn status_bar(ui: &mut Ui, app: &mut FireCraft) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(app.status.as_str()).small());
        ui.separator();
        ui.label(
            RichText::new(format!("Tool: {}", app.tool.name()))
                .small()
                .color(app.palette.text_dim),
        );
        if !app.selected_set.is_empty() {
            ui.separator();
            ui.label(
                RichText::new(format!("Selected: {}", app.selected_set.len()))
                    .small()
                    .color(app.palette.selection),
            );
        }
        if let Some(pos) = app.cursor_doc_pos {
            ui.separator();
            ui.label(
                RichText::new(format!("X: {:.0}, Y: {:.0}", pos.x, pos.y))
                    .small()
                    .color(app.palette.text_dim),
            );
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{:.0}%", app.doc.zoom * 100.0))
                    .small()
                    .strong(),
            );
            ui.separator();
            let (w, h, dpi, cur_st, total_st) = {
                let page = app.doc.page();
                (
                    page.width as u32,
                    page.height as u32,
                    app.doc.dpi,
                    page.current_state + 1,
                    page.states.len(),
                )
            };
            ui.label(RichText::new(format!("{w} × {h} @ {dpi}ppi")).small());
            ui.separator();
            if ui
                .small_button(">|")
                .on_hover_text("Next Animation State")
                .clicked()
            {
                let next = (cur_st) % total_st.max(1);
                app.doc.page_mut().select_state(next);
            }
            ui.label(RichText::new(format!("State {cur_st} of {total_st}")).small());
            if ui
                .small_button("|<")
                .on_hover_text("Previous Animation State")
                .clicked()
            {
                let prev = if cur_st > 1 {
                    cur_st - 2
                } else {
                    total_st.saturating_sub(1)
                };
                app.doc.page_mut().select_state(prev);
            }
        });
    });
}

enum Shortcut {
    OpenMenu(usize),
    ToggleFullScreenCanvas,
    ToggleWindowFullscreen,
    ToggleRightDock,
    ToggleToolsDock,
    TogglePropertiesDock,
    FocusRightPanel(usize, usize),
    HelpDialog,
    Undo,
    Redo,
    Save,
    SaveAs,
    New,
    Open,
    CloseTab,
    Quit,
    Import,
    Export,
    ImagePreview,
    SelectAll,
    Deselect,
    SelectInverse,
    CycleSelect(bool),
    Group,
    Ungroup,
    CommandPalette,
    Copy,
    CopyCss,
    Cut,
    Paste(Option<String>),
    Duplicate,
    Clone,
    Delete,
    Find,
    FitWindow,
    FitCanvas,
    ZoomIn,
    ZoomOut,
    ZoomTo(f32),
    Tool(Tool),
    SwapColors,
    DefaultColors,
    Nudge(Vec2),
    ConvertSymbol,
    TweenSymbol,
    ReorderFront(bool),
    ReorderStep(bool),
    JoinPaths,
    SplitPaths,
    FlattenBitmap,
    ToggleLockSelection,
    ToggleRulers,
    ToggleGrid,
    ToggleGuides,
    ToggleBold,
    ToggleItalic,
    PageStep(isize),
    StateStep(isize),
}

fn shortcuts(ui: &mut Ui, app: &mut FireCraft) {
    if ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Escape)) {
        if let Some(fid) = ui.ctx().memory(|m| m.focused()) {
            ui.ctx().memory_mut(|m| m.surrender_focus(fid));
        }
        let closed_popup = app.active_menu.is_some()
            || app.context_menu_pos.is_some()
            || app.show_command_palette
            || app.show_find_replace
            || app.show_new_dialog
            || app.show_canvas_size_dialog
            || app.show_commands_dialog
            || app.show_mcp_dialog
            || app.show_about_dialog
            || app.show_update_dialog
            || app.pending_close_tab.is_some()
            || app.show_file_dialog
            || app.show_welcome;
        app.active_menu = None;
        app.context_menu_pos = None;
        app.show_command_palette = false;
        app.show_find_replace = false;
        app.show_new_dialog = false;
        app.show_canvas_size_dialog = false;
        app.show_commands_dialog = false;
        app.show_mcp_dialog = false;
        app.show_about_dialog = false;
        app.show_update_dialog = false;
        app.pending_close_tab = None;
        app.show_file_dialog = false;
        app.show_welcome = false;
        if !closed_popup {
            app.clear_selection();
        }
        ui.ctx().request_repaint();
        return;
    }
    if !ui.ctx().egui_wants_keyboard_input() {
        let tab_toggled = ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Tab));
        if tab_toggled {
            if let Some(fid) = ui.ctx().memory(|m| m.focused()) {
                ui.ctx().memory_mut(|m| m.surrender_focus(fid));
            }
            app.toggle_full_screen_canvas();
            ui.ctx().request_repaint();
            return;
        }
        let shift_tab = ui.input_mut(|i| i.consume_key(egui::Modifiers::SHIFT, Key::Tab));
        if shift_tab {
            app.show_right_dock = !app.show_right_dock;
            ui.ctx().request_repaint();
            return;
        }
        let ctrl_a = ui.input_mut(|i| i.consume_key(egui::Modifiers::CTRL, Key::A));
        if ctrl_a {
            app.select_all();
            ui.ctx().request_repaint();
            return;
        }
    }
    if ui.ctx().egui_wants_keyboard_input() {
        return;
    }
    let (ctrl, shift, alt) = ui.input(|i| (i.modifiers.ctrl, i.modifiers.shift, i.modifiers.alt));
    let step = if shift { 10.0 } else { 1.0 };
    let menu_open = app.active_menu.is_some();
    let mut action = None;
    ui.input(|i| {
        for event in &i.events {
            match event {
                egui::Event::Copy => {
                    action = Some(Shortcut::Copy);
                    break;
                }
                egui::Event::Cut => {
                    action = Some(Shortcut::Cut);
                    break;
                }
                egui::Event::Paste(text) => {
                    action = Some(Shortcut::Paste(Some(text.clone())));
                    break;
                }
                egui::Event::Key {
                    key, pressed: true, ..
                } => {
                    // 1. Alt+Letter Menu Bar Mnemonics (Alt+F, Alt+E, Alt+V, Alt+S, Alt+M, Alt+T, Alt+C, Alt+L/I, Alt+W, Alt+H)
                    if alt && !ctrl && !shift {
                        let mnemonic_menu = match key {
                            Key::F => Some(0),
                            Key::E => Some(1),
                            Key::V => Some(2),
                            Key::S => Some(3),
                            Key::M => Some(4),
                            Key::T => Some(5),
                            Key::C => Some(6),
                            Key::L | Key::I => Some(7),
                            Key::W => Some(8),
                            Key::H => Some(9),
                            _ => None,
                        };
                        if let Some(m_idx) = mnemonic_menu {
                            action = Some(Shortcut::OpenMenu(m_idx));
                            break;
                        }
                    }

                    // When a top-level menu is open, arrow keys, Enter, Space, and Escape belong to menu navigation
                    if menu_open
                        && matches!(
                            key,
                            Key::ArrowUp
                                | Key::ArrowDown
                                | Key::ArrowLeft
                                | Key::ArrowRight
                                | Key::Enter
                                | Key::Space
                                | Key::Escape
                        )
                    {
                        continue;
                    }

                    action = match (key, ctrl, shift, alt) {
                        // Workspace & Panel toggles (Tab, Shift+Tab, F1..F12)
                        (Key::Tab, false, false, false) | (Key::F4, false, false, false) => {
                            Some(Shortcut::ToggleFullScreenCanvas)
                        }
                        (Key::F11, false, false, false) => Some(Shortcut::ToggleWindowFullscreen),
                        (Key::Tab, false, true, false) => Some(Shortcut::ToggleRightDock),
                        (Key::F1, false, false, false) => Some(Shortcut::HelpDialog),
                        (Key::F2, true, false, false) => Some(Shortcut::ToggleToolsDock),
                        (Key::F3, true, false, false) => Some(Shortcut::TogglePropertiesDock),
                        (Key::F2, false, false, false) => Some(Shortcut::FocusRightPanel(1, 2)), // Layers
                        (Key::F2, false, true, false) => Some(Shortcut::FocusRightPanel(1, 1)), // States
                        (Key::F5, false, false, false) => Some(Shortcut::FocusRightPanel(1, 0)), // Pages
                        (Key::F6, false, false, false) => Some(Shortcut::FocusRightPanel(0, 0)), // Optimize
                        (Key::F7, false, false, false) => Some(Shortcut::FocusRightPanel(3, 3)), // Library
                        (Key::F7, true, false, false) => Some(Shortcut::FocusRightPanel(3, 4)), // CSS
                        (Key::F9, true, false, false) => Some(Shortcut::FocusRightPanel(2, 2)), // Swatches
                        (Key::F9, false, true, false) => Some(Shortcut::FocusRightPanel(2, 1)), // Color Mixer
                        (Key::F10, false, true, false) => Some(Shortcut::FocusRightPanel(0, 1)), // History
                        (Key::F11, true, false, false) => Some(Shortcut::FocusRightPanel(2, 0)), // Styles
                        (Key::F12, false, true, true) => Some(Shortcut::FocusRightPanel(2, 3)), // Info
                        (Key::F8, false, false, false) => Some(Shortcut::ConvertSymbol),
                        (Key::F8, false, true, true) => Some(Shortcut::TweenSymbol),

                        // File & Edit commands
                        (Key::Z, true, false, false) => Some(Shortcut::Undo),
                        (Key::Y, true, _, false) | (Key::Z, true, true, false) => {
                            Some(Shortcut::Redo)
                        }
                        (Key::S, true, false, false) => Some(Shortcut::Save),
                        (Key::S, true, true, false) => Some(Shortcut::SaveAs),
                        (Key::N, true, _, false) => Some(Shortcut::New),
                        (Key::O, true, false, _) => Some(Shortcut::Open),
                        (Key::W, true, _, false) => Some(Shortcut::CloseTab),
                        (Key::Q, true, false, false) => Some(Shortcut::Quit),
                        (Key::R, true, false, false) => Some(Shortcut::Import),
                        (Key::R, true, true, false) => Some(Shortcut::Export),
                        (Key::X, true, true, false) => Some(Shortcut::ImagePreview),
                        (Key::A, true, false, false) => Some(Shortcut::SelectAll),
                        (Key::A, true, true, false) | (Key::Escape, false, false, false) => {
                            Some(Shortcut::Deselect)
                        }
                        (Key::I, true, true, false) => Some(Shortcut::SelectInverse),
                        (Key::G, true, false, false) => Some(Shortcut::Group),
                        (Key::G, true, true, false) => Some(Shortcut::Ungroup),
                        (Key::K | Key::P, true, _, false) => Some(Shortcut::CommandPalette),
                        (Key::C, true, false, false) => Some(Shortcut::Copy),
                        (Key::C, true, false, true) => Some(Shortcut::CopyCss),
                        (Key::X, true, false, false) => Some(Shortcut::Cut),
                        (Key::V, true, _, false) => Some(Shortcut::Paste(None)),
                        (Key::D, true, false, _) => Some(Shortcut::Duplicate),
                        (Key::D, true, true, false) => Some(Shortcut::Clone),
                        (Key::F, true, false, false) => Some(Shortcut::Find),
                        (Key::F, true, false, true) => Some(Shortcut::FitCanvas),

                        // View & Magnification
                        (Key::Num0, true, _, false) => Some(Shortcut::FitWindow),
                        (Key::Insert, false, false, false) => Some(Shortcut::FitWindow),
                        (Key::Num1, true, false, false) => Some(Shortcut::ZoomTo(1.0)),
                        (Key::Num2, true, false, false) => Some(Shortcut::ZoomTo(2.0)),
                        (Key::Num5, true, false, false) => Some(Shortcut::ZoomTo(0.5)),
                        (Key::Plus | Key::Equals, true, _, false) => Some(Shortcut::ZoomIn),
                        (Key::Minus, true, _, false) => Some(Shortcut::ZoomOut),
                        (Key::R, true, false, true) => Some(Shortcut::ToggleRulers),
                        (Key::G, true, false, true) => Some(Shortcut::ToggleGrid),
                        (Key::Semicolon, true, false, false) => Some(Shortcut::ToggleGuides),

                        // Arrange, Paths & Selection cycling
                        (Key::ArrowUp, true, true, false) => Some(Shortcut::ReorderFront(true)),
                        (Key::ArrowDown, true, true, false) => Some(Shortcut::ReorderFront(false)),
                        (Key::ArrowUp, true, false, false) => Some(Shortcut::ReorderStep(true)),
                        (Key::ArrowDown, true, false, false) => Some(Shortcut::ReorderStep(false)),
                        (Key::ArrowRight, true, false, false) => Some(Shortcut::CycleSelect(true)),
                        (Key::ArrowLeft, true, false, false) => Some(Shortcut::CycleSelect(false)),
                        (Key::J, true, false, false) => Some(Shortcut::JoinPaths),
                        (Key::J, true, true, false) => Some(Shortcut::SplitPaths),
                        (Key::E, true, false, false) | (Key::Z, true, true, true) => {
                            Some(Shortcut::FlattenBitmap)
                        }
                        (Key::L, true, false, true) => Some(Shortcut::ToggleLockSelection),
                        (Key::B, true, false, false) => Some(Shortcut::ToggleBold),
                        (Key::I, true, false, false) => Some(Shortcut::ToggleItalic),

                        // Page & Animation State stepping
                        (Key::PageUp, false, false, false) => Some(Shortcut::PageStep(-1)),
                        (Key::PageDown, false, false, false) => Some(Shortcut::PageStep(1)),
                        (Key::Comma, false, false, false) => Some(Shortcut::StateStep(-1)),
                        (Key::Period, false, false, false) => Some(Shortcut::StateStep(1)),

                        // Canvas object nudging & deletion
                        (Key::Delete | Key::Backspace, false, _, false) => Some(Shortcut::Delete),
                        (Key::ArrowLeft, false, _, false) => {
                            Some(Shortcut::Nudge(Vec2::new(-step, 0.0)))
                        }
                        (Key::ArrowRight, false, _, false) => {
                            Some(Shortcut::Nudge(Vec2::new(step, 0.0)))
                        }
                        (Key::ArrowUp, false, _, false) => {
                            Some(Shortcut::Nudge(Vec2::new(0.0, -step)))
                        }
                        (Key::ArrowDown, false, _, false) => {
                            Some(Shortcut::Nudge(Vec2::new(0.0, step)))
                        }

                        // Single-key CS6 tool letters (only when no modifiers are held)
                        (Key::V, false, false, false) => Some(Shortcut::Tool(Tool::Select)),
                        (Key::A, false, false, false) => Some(Shortcut::Tool(Tool::Subselect)),
                        (Key::Q, false, false, false) => Some(Shortcut::Tool(Tool::Scale)),
                        (Key::M, false, false, false) => Some(Shortcut::Tool(Tool::Marquee)),
                        (Key::L, false, false, false) => Some(Shortcut::Tool(Tool::Lasso)),
                        (Key::W, false, false, false) => Some(Shortcut::Tool(Tool::Wand)),
                        (Key::C, false, false, false) => Some(Shortcut::Tool(Tool::Crop)),
                        (Key::B, false, false, false) => Some(Shortcut::Tool(Tool::Brush)),
                        (Key::E, false, false, false) => Some(Shortcut::Tool(Tool::Eraser)),
                        (Key::S, false, false, false) => Some(Shortcut::Tool(Tool::Stamp)),
                        (Key::N, false, false, false) => Some(Shortcut::Tool(Tool::Line)),
                        (Key::P, false, false, false) => Some(Shortcut::Tool(Tool::Path)),
                        (Key::U, false, false, false) => Some(Shortcut::Tool(Tool::Rectangle)),
                        (Key::T, false, false, false) => Some(Shortcut::Tool(Tool::Text)),
                        (Key::K, false, false, false) => Some(Shortcut::Tool(Tool::Slice)),
                        (Key::J, false, false, false) => Some(Shortcut::Tool(Tool::Hotspot)),
                        (Key::I, false, false, false) => Some(Shortcut::Tool(Tool::Picker)),
                        (Key::G, false, false, false) => Some(Shortcut::Tool(Tool::Gradient)),
                        (Key::H, false, false, false) => Some(Shortcut::Tool(Tool::Hand)),
                        (Key::Z, false, false, false) => Some(Shortcut::Tool(Tool::Zoom)),
                        (Key::X, false, false, false) => Some(Shortcut::SwapColors),
                        (Key::D, false, false, false) => Some(Shortcut::DefaultColors),
                        _ => None,
                    };
                    if action.is_some() {
                        break;
                    }
                }
                _ => {}
            }
        }
    });
    match action {
        Some(Shortcut::OpenMenu(idx)) => {
            if app.active_menu == Some(idx) {
                app.active_menu = None;
            } else {
                app.active_menu = Some(idx);
                app.active_menu_item = 0;
            }
        }
        Some(Shortcut::ToggleFullScreenCanvas) => app.toggle_full_screen_canvas(),
        Some(Shortcut::ToggleWindowFullscreen) => app.toggle_window_fullscreen(),
        Some(Shortcut::ToggleRightDock) => {
            app.show_right_dock = !app.show_right_dock;
            app.full_screen_canvas = false;
        }
        Some(Shortcut::ToggleToolsDock) => app.show_tools_dock = !app.show_tools_dock,
        Some(Shortcut::TogglePropertiesDock) => {
            app.show_bottom_inspector = !app.show_bottom_inspector
        }
        Some(Shortcut::FocusRightPanel(group, tab)) => {
            app.full_screen_canvas = false;
            app.show_right_dock = true;
            match group {
                0 => app.top_right_tab = tab,
                1 => app.layers_tab = tab,
                2 => app.middle_right_tab = tab,
                _ => app.library_tab = tab,
            }
        }
        Some(Shortcut::HelpDialog) => app.show_about_dialog = !app.show_about_dialog,
        Some(Shortcut::Undo) => app.undo_action(),
        Some(Shortcut::Redo) => app.redo_action(),
        Some(Shortcut::Save) => app.save_current(),
        Some(Shortcut::SaveAs) => app.begin_file_dialog(FileMode::Save),
        Some(Shortcut::New) => app.show_new_dialog = true,
        Some(Shortcut::Open) => app.begin_file_dialog(FileMode::Open),
        Some(Shortcut::CloseTab) => app.request_close_active_tab(),
        Some(Shortcut::Quit) => std::process::exit(0),
        Some(Shortcut::Import) => app.begin_file_dialog(FileMode::Import),
        Some(Shortcut::Export) => app.begin_file_dialog(FileMode::Export),
        Some(Shortcut::ImagePreview) => {
            app.view_mode = if app.view_mode == CanvasViewMode::TwoUp {
                CanvasViewMode::Original
            } else {
                CanvasViewMode::TwoUp
            };
        }
        Some(Shortcut::SelectAll) => app.select_all(),
        Some(Shortcut::Deselect) => {
            app.show_welcome = false;
            app.context_menu_pos = None;
            app.clear_selection();
        }
        Some(Shortcut::SelectInverse) => app.select_inverse(),
        Some(Shortcut::CycleSelect(fwd)) => app.cycle_selection(fwd),
        Some(Shortcut::Group) => app.group_selected(),
        Some(Shortcut::Ungroup) => app.ungroup_selected(),
        Some(Shortcut::CommandPalette) => app.show_command_palette = !app.show_command_palette,
        Some(Shortcut::Copy) => app.copy_selected(),
        Some(Shortcut::CopyCss) => app.copy_selected_css(),
        Some(Shortcut::Cut) => app.cut_selected(),
        Some(Shortcut::Paste(txt)) => app.paste_with_text(txt.as_deref()),
        Some(Shortcut::Duplicate) => app.duplicate_selected(),
        Some(Shortcut::Clone) => app.clone_selected(),
        Some(Shortcut::Delete) => app.delete_selected(),
        Some(Shortcut::Find) => app.show_find_replace = true,
        Some(Shortcut::FitWindow) => app.fit_in_window(),
        Some(Shortcut::FitCanvas) => app.fit_canvas_to_contents(),
        Some(Shortcut::ZoomIn) => app.zoom_by(constants::ZOOM_STEP),
        Some(Shortcut::ZoomOut) => app.zoom_by(1.0 / constants::ZOOM_STEP),
        Some(Shortcut::ZoomTo(z)) => app.set_zoom(z),
        Some(Shortcut::Tool(t)) => {
            app.tool = t;
            app.status = format!("{} selected", t.name());
        }
        Some(Shortcut::SwapColors) => std::mem::swap(&mut app.fill, &mut app.stroke),
        Some(Shortcut::DefaultColors) => {
            app.fill = egui::Color32::WHITE;
            app.stroke = egui::Color32::BLACK;
            app.status = "Reset default Stroke (Black) & Fill (White) colors (D)".to_string();
        }
        Some(Shortcut::Nudge(delta)) => app.nudge_selected(delta),
        Some(Shortcut::ConvertSymbol) => app.convert_selection_to_symbol(),
        Some(Shortcut::TweenSymbol) => app.tween_selected_across_states(),
        Some(Shortcut::ReorderFront(front)) => app.reorder_selected(front),
        Some(Shortcut::ReorderStep(fwd)) => app.reorder_selected_step(fwd),
        Some(Shortcut::JoinPaths) => app.combine_paths_op("join"),
        Some(Shortcut::SplitPaths) => app.combine_paths_op("split"),
        Some(Shortcut::FlattenBitmap) => app.flatten_selected_to_bitmap(),
        Some(Shortcut::ToggleLockSelection) => app.toggle_lock_selection(),
        Some(Shortcut::ToggleRulers) => app.show_rulers = !app.show_rulers,
        Some(Shortcut::ToggleGrid) => app.show_grid = !app.show_grid,
        Some(Shortcut::ToggleGuides) => app.show_guides = !app.show_guides,
        Some(Shortcut::ToggleBold) => app.toggle_selected_text_bold(),
        Some(Shortcut::ToggleItalic) => app.toggle_selected_text_italic(),
        Some(Shortcut::PageStep(d)) => app.select_page_step(d),
        Some(Shortcut::StateStep(d)) => app.select_state_step(d),
        None => return,
    }
    ui.ctx().request_repaint();
}

/// Capture the rendered window to `app.screenshot_path` after a few frames and exit.
fn screenshot(ui: &mut Ui, app: &mut FireCraft) {
    let Some(path) = app.screenshot_path.clone() else {
        return;
    };
    if app.frames < 10 {
        ui.ctx().request_repaint();
        return;
    }
    if app.screenshot_saved {
        return;
    }
    app.screenshot_saved = true;
    let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = done.clone();
    let waiter = done.clone();
    ui.ctx()
        .request_screenshot(move |image: std::sync::Arc<egui::ColorImage>| {
            let w = image.width();
            let h = image.height();
            let mut rgba = Vec::with_capacity(w * h * 4);
            for c in &image.pixels {
                rgba.push(c.r());
                rgba.push(c.g());
                rgba.push(c.b());
                rgba.push(c.a());
            }
            if let Some(img) = image::RgbaImage::from_raw(w as u32, h as u32, rgba) {
                if let Some(parent) = std::path::Path::new(&path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if img.save(&path).is_ok() {
                    eprintln!("Screenshot saved to {path}");
                }
            }
            flag.store(true, std::sync::atomic::Ordering::SeqCst);
        });
    ui.ctx().request_repaint();
    std::thread::spawn(move || {
        for _ in 0..150 {
            if waiter.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        std::process::exit(0);
    });
}
