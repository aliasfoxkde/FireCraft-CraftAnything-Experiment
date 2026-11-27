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

use eframe::egui::{self, Key, RichText, Sense, Ui, Vec2};

mod chrome;

/// Draw the whole editor for one frame.
pub fn show(ui: &mut Ui, app: &mut FireCraft) {
    let ctx = ui.ctx().clone();
    app.ctx = ctx.clone();
    app.frames = app.frames.saturating_add(1);
    #[cfg(target_arch = "wasm32")]
    for payload in crate::take_browser_pastes() {
        app.paste_with_text(Some(&payload));
    }
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

    // Intercept the OS close button before eframe tears the window down. CancelClose is issued
    // only while there is unsaved work so a clean session still closes normally.
    if ui.ctx().input(|i| i.viewport().close_requested()) && app.should_intercept_close() {
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::CancelClose);
        app.show_quit_confirm = true;
    }

    shortcuts(ui, app);

    // 1. CS6 menu bar plus familiar file, hand, zoom and workspace controls.
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

    // 4. Bottom Dock: Properties, Symbol Properties & Intelligence Inspector.
    // Height is owned by the app (not egui panel memory) so a drag cannot snap back when the
    // nested panel re-measures its content.
    if app.show_bottom_inspector {
        let frame =
            egui::Frame::new()
                .fill(app.palette.panel)
                .inner_margin(egui::Margin::symmetric(
                    8,
                    if app.bottom_collapsed { 2 } else { 5 },
                ));
        let dock_h = if app.bottom_collapsed {
            30.0
        } else {
            app.bottom_height.clamp(118.0, 340.0)
        };
        egui::Panel::bottom("properties_dock")
            .exact_size(dock_h)
            .resizable(false)
            .frame(frame)
            .show(ui, |ui| chrome::bottom_properties_dock(ui, app));
    }

    // 5. Left/Right/Floating CS6 Tool Strip (with top grip bar to drag/float/snap + >> / << collapse)
    if app.show_tools_dock {
        let tools_w = if app.tools_collapsed { 40.0 } else { 90.0 };
        match app.tools_zone {
            crate::app::ToolsDockMode::Left => {
                egui::Panel::left("tools")
                    .exact_size(tools_w)
                    .resizable(false)
                    .frame(
                        egui::Frame::new()
                            .fill(app.palette.panel)
                            .inner_margin(egui::Margin::symmetric(4, 2)),
                    )
                    .show(ui, |ui| {
                        tools_top_bar(ui, app);
                        tools_dock_contents(ui, app, "tools_scroll_left");
                    });
            }
            crate::app::ToolsDockMode::Right => {
                egui::Panel::right("tools_right")
                    .exact_size(tools_w)
                    .resizable(false)
                    .frame(
                        egui::Frame::new()
                            .fill(app.palette.panel)
                            .inner_margin(egui::Margin::symmetric(4, 2)),
                    )
                    .show(ui, |ui| {
                        tools_top_bar(ui, app);
                        tools_dock_contents(ui, app, "tools_scroll_right");
                    });
            }
            crate::app::ToolsDockMode::Floating => {}
        }
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

    // 6. Right/Left/Floating Panel Dock: CS6 Stacked Tab-Groups (or Collapsed Iconic Strip)
    let mut right_outer_rect = None;
    let mut right_inner_rect = None;
    if app.show_right_dock {
        let collapsed_w = if app.workspace_preset == "Iconic Mode" {
            36.0
        } else {
            94.0
        };
        match app.right_dock_zone {
            crate::app::ToolsDockMode::Right => {
                let resp = if app.right_dock_collapsed {
                    egui::Panel::right("dock")
                        .exact_size(collapsed_w)
                        .resizable(false)
                        .frame(
                            egui::Frame::new()
                                .fill(app.palette.panel)
                                .inner_margin(egui::Margin::same(3)),
                        )
                        .show(ui, |ui| right_dock(ui, app))
                } else {
                    egui::Panel::right("dock")
                        .default_size(275.0)
                        .min_size(240.0)
                        .max_size(360.0)
                        .resizable(true)
                        .frame(
                            egui::Frame::new()
                                .fill(app.palette.panel)
                                .inner_margin(egui::Margin::same(3)),
                        )
                        .show(ui, |ui| right_dock(ui, app))
                };
                right_outer_rect = Some(resp.response.rect);
            }
            crate::app::ToolsDockMode::Left => {
                let resp = if app.right_dock_collapsed {
                    egui::Panel::left("dock_on_left")
                        .exact_size(collapsed_w)
                        .resizable(false)
                        .frame(
                            egui::Frame::new()
                                .fill(app.palette.panel)
                                .inner_margin(egui::Margin::same(3)),
                        )
                        .show(ui, |ui| right_dock(ui, app))
                } else {
                    egui::Panel::left("dock_on_left")
                        .default_size(275.0)
                        .min_size(240.0)
                        .max_size(360.0)
                        .resizable(true)
                        .frame(
                            egui::Frame::new()
                                .fill(app.palette.panel)
                                .inner_margin(egui::Margin::same(3)),
                        )
                        .show(ui, |ui| right_dock(ui, app))
                };
                right_outer_rect = Some(resp.response.rect);
            }
            crate::app::ToolsDockMode::Floating => {
                let screen = ui.ctx().content_rect();
                let dock_w = if app.right_dock_collapsed {
                    94.0
                } else {
                    268.0
                };
                let max_h = (screen.height() - 160.0).max(260.0);
                let resp = egui::Area::new(ui.make_persistent_id("floating_right_dock"))
                    .order(egui::Order::Middle)
                    .fixed_pos(app.right_dock_floating_pos)
                    .constrain(true)
                    .show(ui.ctx(), |ui| {
                        egui::Frame::new()
                            .fill(app.palette.panel)
                            .stroke(egui::Stroke::new(1.0, app.palette.stroke))
                            .shadow(egui::Shadow {
                                offset: [2, 3],
                                blur: 8,
                                spread: 0,
                                color: egui::Color32::from_black_alpha(100),
                            })
                            .inner_margin(egui::Margin::same(3))
                            .show(ui, |ui| {
                                ui.set_width(dock_w);
                                ui.set_max_height(max_h);
                                right_dock(ui, app);
                            });
                    });
                right_outer_rect = Some(resp.response.rect);
            }
        }

        let has_right_inner = app.right_dock_zone == crate::app::ToolsDockMode::Right
            && !app.right_dock_collapsed
            && app
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
    if app.floating_doc_tabs.contains(&app.active_doc)
        && let Some(first_docked) =
            (0..app.open_docs.len()).find(|i| !app.floating_doc_tabs.contains(i))
    {
        app.switch_document_tab(first_docked);
    }
    let all_floating = !app.open_docs.is_empty()
        && (0..app.open_docs.len()).all(|i| app.floating_doc_tabs.contains(&i));
    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(app.palette.bg))
        .show(ui, |ui| {
            chrome::document_tab_strip(ui, app);
            if all_floating {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(40.0);
                        ui.label(
                            RichText::new("Canvas floated to external window (Multi-Monitor Mode)")
                                .size(14.0)
                                .color(app.palette.text),
                        );
                        ui.add_space(8.0);
                        if ui.button("Dock All Canvases Back to Main Window").clicked() {
                            app.floating_doc_tabs.clear();
                            app.status =
                                "Docked floating canvas window(s) back to main window".to_string();
                        }
                    });
                });
            } else {
                egui::Panel::bottom("canvas_footer_bar")
                    .exact_size(24.0)
                    .frame(
                        egui::Frame::new()
                            .fill(app.palette.panel)
                            .stroke(egui::Stroke::new(1.0, app.palette.stroke))
                            .inner_margin(egui::Margin::symmetric(8, 2)),
                    )
                    .show(ui, |ui| chrome::canvas_footer_bar(ui, app));
                canvas::show(ui, app);
            }
        });

    // 8. Floating CS6 Tools Palette (when undocked via its top grip bar)
    if app.show_tools_dock && app.tools_zone == crate::app::ToolsDockMode::Floating {
        let tools_w = if app.tools_collapsed { 40.0 } else { 90.0 };
        let screen = ui.ctx().content_rect();
        let max_h = (screen.height() - 160.0).max(240.0);
        egui::Area::new(ui.make_persistent_id("floating_tools_palette"))
            .order(egui::Order::Middle)
            .fixed_pos(app.tools_floating_pos)
            .constrain(true)
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(app.palette.panel)
                    .stroke(egui::Stroke::new(1.0, app.palette.stroke))
                    .shadow(egui::Shadow {
                        offset: [2, 3],
                        blur: 8,
                        spread: 0,
                        color: egui::Color32::from_black_alpha(100),
                    })
                    .inner_margin(egui::Margin::symmetric(4, 3))
                    .show(ui, |ui| {
                        ui.set_width(tools_w);
                        ui.set_max_height(max_h);
                        tools_top_bar(ui, app);
                        tools_dock_contents(ui, app, "tools_scroll_floating");
                    });
            });
    }

    // 9. Floating Panel Windows with Magnetic Snapping
    let floating_indices: Vec<usize> = app
        .panel_groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.zone == crate::app::DockZone::Floating && !g.tabs.is_empty())
        .map(|(i, _)| i)
        .collect();
    // Resolve stack geometry before drawing: each stacked group sits flush below its parent and
    // inherits the parent's width, so dragging the root moves the whole column as one unit.
    for _ in 0..app.panel_groups.len() {
        for i in 0..app.panel_groups.len() {
            let Some(parent_id) = app.panel_groups[i].stack_parent else {
                continue;
            };
            let Some((pp, pw, ph)) = app
                .panel_groups
                .iter()
                .find(|g| g.id == parent_id)
                .map(|g| {
                    (
                        g.floating_pos,
                        g.floating_width,
                        FireCraft::panel_group_height(g),
                    )
                })
            else {
                continue;
            };
            let g = &mut app.panel_groups[i];
            g.floating_pos = egui::pos2(pp.x, pp.y + ph + 2.0);
            g.floating_width = pw;
        }
    }

    for idx in floating_indices {
        let (gid, fpos, fwidth, fheight, collapsed, stacked) = {
            let g = &app.panel_groups[idx];
            (
                g.id,
                g.floating_pos,
                g.floating_width,
                g.body_height,
                g.collapsed,
                g.stack_parent.is_some(),
            )
        };
        let area_resp = egui::Area::new(ui.make_persistent_id(("floating_pg", gid)))
            .order(egui::Order::Middle)
            .current_pos(fpos)
            .movable(!stacked)
            .show(ui.ctx(), |ui| {
                ui.set_width(fwidth);
                panels::panel_group_widget(ui, app, idx, true);
            });
        let new_pos = area_resp.response.rect.min;
        if !stacked && (new_pos - fpos).length() > 0.5 {
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
            let (width, height, titles) = app
                .panel_groups
                .iter()
                .find(|g| g.id == dragged_gid)
                .map(|g| {
                    (
                        g.floating_width.clamp(180.0, 360.0),
                        if g.collapsed {
                            24.0
                        } else {
                            (g.body_height + 30.0).clamp(80.0, 420.0)
                        },
                        g.tabs.iter().map(|t| t.title().to_string()).collect(),
                    )
                })
                .unwrap_or((260.0, 160.0, vec!["Panel".to_string()]));
            let ghost_r = egui::Rect::from_min_size(
                pos - Vec2::new(width * 0.5, 10.0),
                Vec2::new(width, height),
            );
            draw_widget_drag_ghost(ui, app, ghost_r, &titles);
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
                } else if let Some((target_id, drop_above)) = app
                    .panel_groups
                    .iter()
                    .filter(|g| {
                        g.id != dragged_gid
                            && g.zone == crate::app::DockZone::Floating
                            && !g.tabs.is_empty()
                    })
                    .find_map(|g| {
                        let r = egui::Rect::from_min_size(
                            g.floating_pos,
                            Vec2::new(g.floating_width, FireCraft::panel_group_height(g)),
                        );
                        r.contains(pos).then(|| (g.id, pos.y < r.center().y))
                    })
                {
                    app.stack_group(dragged_gid, target_id, !drop_above);
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
            let width = app
                .panel_groups
                .iter()
                .find(|g| g.tabs.contains(&dragged))
                .map(|g| g.floating_width.clamp(180.0, 360.0))
                .unwrap_or(260.0);
            let ghost_r = egui::Rect::from_min_size(
                pos - Vec2::new(width * 0.5, 10.0),
                Vec2::new(width, 170.0),
            );
            draw_widget_drag_ghost(ui, app, ghost_r, &[dragged.title().to_string()]);
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
    dialogs::html_setup_dialog(app);
    dialogs::page_setup_dialog(app);
    dialogs::find_replace_dialog(app);
    dialogs::saved_commands_dialog(app);
    dialogs::quit_confirm_dialog(app);
    // 10. Detached Floating Canvas Windows (Multi-Monitor Native OS Viewports)
    #[cfg(not(target_arch = "wasm32"))]
    if !app.floating_doc_tabs.is_empty() {
        app.sync_active_doc_slot();
        let floating_list = app.floating_doc_tabs.clone();
        let mut redock_list: Vec<usize> = Vec::new();
        let saved_active = app.active_doc;
        let saved_doc = app.doc.clone();
        for idx in floating_list {
            let Some(doc_clone) = app.open_docs.get(idx).cloned() else {
                redock_list.push(idx);
                continue;
            };
            app.active_doc = idx;
            app.doc = doc_clone;
            let title = if app.doc.dirty {
                format!("*{} — FireCraft Canvas", app.doc.name)
            } else {
                format!("{} — FireCraft Canvas", app.doc.name)
            };
            let vp_id = egui::ViewportId::from_hash_of(("firecraft_floating_doc", idx));
            let builder = egui::ViewportBuilder::default()
                .with_title(&title)
                .with_inner_size([860.0, 620.0])
                .with_min_inner_size([360.0, 260.0])
                .with_resizable(true);
            let mut dock_back = false;
            ui.ctx()
                .show_viewport_immediate(vp_id, builder, |vui, _class| {
                    if vui.ctx().input(|i| i.viewport().close_requested()) {
                        dock_back = true;
                    }
                    egui::Panel::top(("floating_doc_top", idx))
                        .exact_size(24.0)
                        .frame(
                            egui::Frame::new()
                                .fill(app.palette.panel_alt)
                                .stroke(egui::Stroke::new(1.0, app.palette.stroke))
                                .inner_margin(egui::Margin::symmetric(6, 2)),
                        )
                        .show(vui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(&app.doc.name)
                                        .small()
                                        .strong()
                                        .color(app.palette.text),
                                );
                                ui.separator();
                                if ui
                                    .selectable_label(
                                        app.view_mode == CanvasViewMode::Original,
                                        RichText::new("Original").small(),
                                    )
                                    .clicked()
                                {
                                    app.view_mode = CanvasViewMode::Original;
                                }
                                if ui
                                    .selectable_label(
                                        app.view_mode == CanvasViewMode::Preview,
                                        RichText::new("Preview").small(),
                                    )
                                    .clicked()
                                {
                                    app.view_mode = CanvasViewMode::Preview;
                                }
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui
                                            .small_button("Dock to Main Window")
                                            .on_hover_text(
                                                "Re-dock this canvas back into the main tab bar",
                                            )
                                            .clicked()
                                        {
                                            dock_back = true;
                                        }
                                    },
                                );
                            });
                        });
                    egui::Panel::bottom(("floating_doc_footer", idx))
                        .exact_size(24.0)
                        .frame(
                            egui::Frame::new()
                                .fill(app.palette.panel)
                                .stroke(egui::Stroke::new(1.0, app.palette.stroke))
                                .inner_margin(egui::Margin::symmetric(8, 2)),
                        )
                        .show(vui, |ui| chrome::canvas_footer_bar(ui, app));
                    egui::CentralPanel::default()
                        .frame(egui::Frame::new().fill(app.palette.bg))
                        .show(vui, |ui| canvas::show(ui, app));
                });
            app.open_docs[idx] = app.doc.clone();
            if dock_back {
                redock_list.push(idx);
            }
        }
        app.active_doc = saved_active;
        app.doc = saved_doc;
        if let Some(&last_redock) = redock_list.last() {
            app.floating_doc_tabs.retain(|i| !redock_list.contains(i));
            if last_redock < app.open_docs.len() {
                app.switch_document_tab(last_redock);
            }
            app.status = "Re-docked floating canvas window".to_string();
        }
    }

    dialogs::command_palette_dialog(app);
    dialogs::mcp_cli_dialog(app);
    dialogs::about_dialog(app);
    dialogs::update_dialog(app);
    dialogs::unsaved_changes_dialog(app);
    dialogs::file_dialog(app);
    dialogs::welcome_overlay(app);
    app.handle_dropped_files(ui.ctx());

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
            app.run_file_intent(crate::app::FileIntent::SaveCurrent);
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
    });
}

/// Full-widget drag preview: a translucent facsimile of the panel header, tabs, and body so the
/// user sees the whole object moving, while the real window stays in place until drop.
fn draw_widget_drag_ghost(ui: &Ui, app: &FireCraft, rect: egui::Rect, titles: &[String]) {
    let p = app.palette;
    let fg = ui.ctx().layer_painter(egui::LayerId::new(
        egui::Order::Tooltip,
        ui.make_persistent_id("widget_drag_ghost"),
    ));
    fg.rect_filled(
        rect.translate(Vec2::new(3.0, 4.0)),
        2.0,
        egui::Color32::from_black_alpha(70),
    );
    fg.rect_filled(rect, 2.0, p.panel.gamma_multiply(0.93));
    fg.rect_stroke(
        rect,
        2.0,
        egui::Stroke::new(1.5, p.accent),
        egui::StrokeKind::Inside,
    );
    let header = egui::Rect::from_min_size(rect.min, Vec2::new(rect.width(), 22.0));
    fg.rect_filled(header, 2.0, p.panel_alt.gamma_multiply(0.96));
    fg.line_segment(
        [header.left_bottom(), header.right_bottom()],
        egui::Stroke::new(1.0, p.stroke),
    );
    let mut x = header.left() + 8.0;
    for (i, title) in titles.iter().take(4).enumerate() {
        let tw = (title.len() as f32 * 5.4 + 12.0).min(96.0);
        let tab_r = egui::Rect::from_min_size(
            egui::pos2(x, header.top() + 2.0),
            Vec2::new(tw, header.height() - 3.0),
        );
        if i == 0 {
            fg.rect_filled(tab_r, 0.0, p.panel);
            fg.line_segment(
                [tab_r.left_top(), tab_r.right_top()],
                egui::Stroke::new(1.5, p.accent),
            );
        }
        fg.text(
            tab_r.center(),
            egui::Align2::CENTER_CENTER,
            title,
            egui::FontId::proportional(10.0),
            if i == 0 { p.text } else { p.text_dim },
        );
        x += tw + 2.0;
    }
    let body = rect.shrink2(Vec2::new(4.0, 0.0));
    let body_top = header.bottom() + 6.0;
    if rect.height() > 48.0 {
        for row in 0..4 {
            let y = body_top + row as f32 * 15.0;
            if y > rect.bottom() - 10.0 {
                break;
            }
            let w = (body.width() * (0.8 - row as f32 * 0.12)).max(40.0);
            fg.rect_filled(
                egui::Rect::from_min_size(egui::pos2(body.left() + 6.0, y), Vec2::new(w, 7.0)),
                0.0,
                p.well.gamma_multiply(0.85),
            );
        }
        fg.text(
            egui::pos2(rect.center().x, rect.bottom() - 12.0),
            egui::Align2::CENTER_CENTER,
            "Drop on a tab bar to group · edge to dock · elsewhere to float",
            egui::FontId::proportional(9.5),
            p.text_dim,
        );
    }
}

fn panel_mini_icon(id: crate::app::PanelId) -> icons::Icon {
    use crate::app::PanelId;
    match id {
        PanelId::Optimize => icons::Icon::Slice,
        PanelId::History => icons::Icon::Undo,
        PanelId::Align => icons::Icon::Group,
        PanelId::Path => icons::Icon::Pen,
        PanelId::Pages => icons::Icon::New,
        PanelId::States => icons::Icon::Duplicate,
        PanelId::Layers => icons::Icon::Folder,
        PanelId::Styles => icons::Icon::Star,
        PanelId::ColorPalette | PanelId::ColorMixer | PanelId::Swatches => icons::Icon::Picker,
        PanelId::Info => icons::Icon::Select,
        PanelId::SpecialChars => icons::Icon::Text,
        PanelId::ImageEditing => icons::Icon::Brush,
        PanelId::AutoShapes | PanelId::AutoShapeProps => icons::Icon::Polygon,
        PanelId::DocumentLibrary | PanelId::CommonLibrary => icons::Icon::Open,
        PanelId::CssProperties => icons::Icon::Export,
        PanelId::Url => icons::Icon::Hotspot,
        PanelId::Behaviors => icons::Icon::Wand,
    }
}

fn tools_top_bar(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let w = ui.available_width().max(26.0);
    let (bar_r, _) = ui.allocate_exact_size(Vec2::new(w, 15.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(bar_r, 0.0, p.panel_alt);
    painter.line_segment(
        [bar_r.left_bottom(), bar_r.right_bottom()],
        egui::Stroke::new(1.0, p.stroke),
    );

    // Collapse / Expand chevron button (<< collapse to one column, >> show the full three-column palette)
    let btn_r = egui::Rect::from_min_max(
        egui::pos2(bar_r.right() - 16.0, bar_r.top() + 1.0),
        egui::pos2(bar_r.right() - 1.0, bar_r.bottom() - 1.0),
    );
    let btn_resp = ui.interact(
        btn_r,
        ui.make_persistent_id("tools_collapse_btn"),
        egui::Sense::click(),
    );
    if btn_resp.hovered() {
        painter.rect_filled(btn_r, 0.0, p.well);
    }
    let chevron = match (app.tools_zone, app.tools_collapsed) {
        (crate::app::ToolsDockMode::Right, false) => ">>",
        (crate::app::ToolsDockMode::Right, true) => "<<",
        (_, false) => "<<",
        (_, true) => ">>",
    };
    painter.text(
        btn_r.center(),
        egui::Align2::CENTER_CENTER,
        chevron,
        egui::FontId::proportional(9.0),
        if btn_resp.hovered() {
            p.accent
        } else {
            p.text_dim
        },
    );
    if btn_resp.clicked() {
        app.tools_collapsed = !app.tools_collapsed;
    }
    btn_resp.on_hover_text(if app.tools_collapsed {
        "Expand tool palette"
    } else {
        "Collapse tool palette"
    });

    // Draggable CS6 dotted grip (drag to float anywhere or snap to left/right edge)
    let grip_r =
        egui::Rect::from_min_max(bar_r.left_top(), egui::pos2(btn_r.left(), bar_r.bottom()));
    let grip_resp = ui.interact(
        grip_r,
        ui.make_persistent_id("tools_drag_grip"),
        egui::Sense::click_and_drag(),
    );
    let dot_col = if grip_resp.hovered() || grip_resp.dragged() {
        p.accent
    } else {
        p.text_dim
    };
    let dots = ((grip_r.width() - 6.0) / 4.0).floor().clamp(3.0, 6.0) as usize;
    let start_x = grip_r.left() + 4.0;
    for i in 0..dots {
        painter.rect_filled(
            egui::Rect::from_center_size(
                egui::pos2(start_x + i as f32 * 4.0, grip_r.center().y),
                Vec2::splat(1.5),
            ),
            0.0,
            dot_col,
        );
    }
    if grip_resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
    }
    if grip_resp.double_clicked() {
        app.tools_collapsed = !app.tools_collapsed;
    } else if grip_resp.dragged() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
        let screen = ui.ctx().content_rect();
        if let Some(ptr) = ui.ctx().pointer_interact_pos() {
            let delta = grip_resp.drag_delta();
            match app.tools_zone {
                crate::app::ToolsDockMode::Left if ptr.x > screen.left() + 48.0 => {
                    app.tools_zone = crate::app::ToolsDockMode::Floating;
                    app.tools_floating_pos = egui::pos2(ptr.x - 16.0, ptr.y - 6.0);
                }
                crate::app::ToolsDockMode::Right if ptr.x < screen.right() - 48.0 => {
                    app.tools_zone = crate::app::ToolsDockMode::Floating;
                    app.tools_floating_pos = egui::pos2(ptr.x - 16.0, ptr.y - 6.0);
                }
                crate::app::ToolsDockMode::Floating => {
                    app.tools_floating_pos += delta;
                    app.tools_floating_pos.x = app
                        .tools_floating_pos
                        .x
                        .clamp(screen.left(), (screen.right() - 90.0).max(screen.left()));
                    app.tools_floating_pos.y = app.tools_floating_pos.y.clamp(
                        screen.top() + 50.0,
                        (screen.bottom() - 120.0).max(screen.top() + 50.0),
                    );
                    if ptr.x <= screen.left() + 28.0 {
                        app.tools_zone = crate::app::ToolsDockMode::Left;
                    } else if ptr.x >= screen.right() - 28.0 {
                        app.tools_zone = crate::app::ToolsDockMode::Right;
                    }
                }
                _ => {}
            }
        }
    }
    grip_resp.on_hover_text(
        "Drag to float Tools or snap to Left/Right edge · Double-click to collapse/expand",
    );
}

fn dock_column(ui: &mut Ui, app: &mut FireCraft, zone: crate::app::DockZone) {
    let p = app.palette;
    let is_right_outer = zone == crate::app::DockZone::RightOuter;
    let collapsed_iconic = is_right_outer && app.right_dock_collapsed;

    // CS6 Dock Header Bar with >> / << collapse-to-iconic-strip toggle
    let bar_w = ui.available_width();
    let (bar_r, _) = ui.allocate_exact_size(Vec2::new(bar_w, 16.0), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(bar_r, 0.0, p.panel_alt);
    painter.line_segment(
        [bar_r.left_bottom(), bar_r.right_bottom()],
        egui::Stroke::new(1.0, p.stroke),
    );
    if is_right_outer {
        let chev_r = egui::Rect::from_min_max(
            egui::pos2(bar_r.left() + 1.0, bar_r.top() + 1.0),
            egui::pos2(bar_r.left() + 20.0, bar_r.bottom() - 1.0),
        );
        let chev_resp = ui.interact(
            chev_r,
            ui.make_persistent_id("right_dock_collapse_btn"),
            egui::Sense::click(),
        );
        if chev_resp.hovered() {
            painter.rect_filled(chev_r, 0.0, p.well);
        }
        let chev_txt = match (app.right_dock_zone, collapsed_iconic) {
            (crate::app::ToolsDockMode::Left, false) => "<<",
            (crate::app::ToolsDockMode::Left, true) => ">>",
            (_, false) => ">>",
            (_, true) => "<<",
        };
        painter.text(
            chev_r.center(),
            egui::Align2::CENTER_CENTER,
            chev_txt,
            egui::FontId::proportional(9.5),
            if chev_resp.hovered() {
                p.accent
            } else {
                p.text_dim
            },
        );
        if chev_resp.clicked() {
            app.right_dock_collapsed = !app.right_dock_collapsed;
        }
        chev_resp.on_hover_text(if collapsed_iconic {
            "Expand Panels Dock"
        } else {
            "Collapse Panels Dock to Iconic Strip"
        });

        // Identify the bar: an unlabelled strip above the tabs reads as a rendering bug.
        painter.text(
            egui::pos2(chev_r.right() + 6.0, bar_r.center().y),
            egui::Align2::LEFT_CENTER,
            "Panels",
            egui::FontId::proportional(9.5),
            p.text_dim,
        );

        // One state-aware toggle replaces separate expand and collapse buttons.
        let btn_w = 18.0;
        let toggle_r = egui::Rect::from_min_max(
            egui::pos2(bar_r.right() - btn_w - 2.0, bar_r.top() + 1.0),
            egui::pos2(bar_r.right() - 2.0, bar_r.bottom() - 1.0),
        );
        #[cfg(test)]
        {
            app.panels_toggle_rect = Some(toggle_r);
        }
        let all_collapsed = app.panel_groups.iter().all(|group| group.collapsed);
        let toggle_resp = ui.interact(
            toggle_r,
            ui.make_persistent_id("right_dock_toggle_all"),
            egui::Sense::click(),
        );
        if toggle_resp.hovered() {
            painter.rect_filled(toggle_r, 0.0, p.well);
        }
        painter.text(
            toggle_r.center(),
            egui::Align2::CENTER_CENTER,
            if all_collapsed { "▸" } else { "▾" },
            egui::FontId::proportional(9.5),
            if toggle_resp.hovered() {
                p.accent
            } else {
                p.text_dim
            },
        );
        if toggle_resp.clicked() {
            app.toggle_all_panels();
        }
        toggle_resp.on_hover_text(if all_collapsed {
            "Expand all panels"
        } else {
            "Collapse all panels"
        });

        // Draggable CS6 dotted grip (drag to float panel block or snap to Left/Right; double-click to toggle collapse)
        let grip_r = egui::Rect::from_min_max(
            egui::pos2(chev_r.right() + 56.0, bar_r.top()),
            egui::pos2(toggle_r.left() - 2.0, bar_r.bottom()),
        );
        let grip_resp = ui.interact(
            grip_r,
            ui.make_persistent_id("right_dock_drag_grip"),
            egui::Sense::click_and_drag(),
        );
        let dot_col = if grip_resp.hovered() || grip_resp.dragged() {
            p.accent
        } else {
            p.text_dim
        };
        let dots = ((grip_r.width() - 8.0) / 5.0).floor().clamp(4.0, 12.0) as usize;
        let start_x = grip_r.center().x - (dots as f32 * 2.5);
        for i in 0..dots {
            painter.rect_filled(
                egui::Rect::from_center_size(
                    egui::pos2(start_x + i as f32 * 5.0, grip_r.center().y),
                    Vec2::splat(1.5),
                ),
                0.0,
                dot_col,
            );
        }
        if grip_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
        }
        if grip_resp.double_clicked() {
            app.right_dock_collapsed = !app.right_dock_collapsed;
        } else if grip_resp.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            let screen = ui.ctx().content_rect();
            if let Some(ptr) = ui.ctx().pointer_interact_pos() {
                let delta = grip_resp.drag_delta();
                match app.right_dock_zone {
                    crate::app::ToolsDockMode::Right if ptr.x < screen.right() - 60.0 => {
                        app.right_dock_zone = crate::app::ToolsDockMode::Floating;
                        app.right_dock_floating_pos = egui::pos2(ptr.x - 120.0, ptr.y - 6.0);
                    }
                    crate::app::ToolsDockMode::Left if ptr.x > screen.left() + 60.0 => {
                        app.right_dock_zone = crate::app::ToolsDockMode::Floating;
                        app.right_dock_floating_pos = egui::pos2(ptr.x - 120.0, ptr.y - 6.0);
                    }
                    crate::app::ToolsDockMode::Floating => {
                        app.right_dock_floating_pos += delta;
                        app.right_dock_floating_pos.x = app
                            .right_dock_floating_pos
                            .x
                            .clamp(screen.left(), (screen.right() - 120.0).max(screen.left()));
                        app.right_dock_floating_pos.y = app.right_dock_floating_pos.y.clamp(
                            screen.top() + 50.0,
                            (screen.bottom() - 120.0).max(screen.top() + 50.0),
                        );
                        if ptr.x >= screen.right() - 28.0 {
                            app.right_dock_zone = crate::app::ToolsDockMode::Right;
                        } else if ptr.x <= screen.left() + 28.0 {
                            app.right_dock_zone = crate::app::ToolsDockMode::Left;
                        }
                    }
                    _ => {}
                }
            }
        }
        grip_resp.on_hover_text(
            "Drag to move/float Panels Dock or snap to Left/Right edge · Double-click to collapse/expand",
        );
    }
    if !collapsed_iconic && !is_right_outer {
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
    }
    ui.add_space(2.0);
    let indices: Vec<usize> = app
        .panel_groups
        .iter()
        .enumerate()
        .filter(|(_, g)| g.zone == zone && !g.tabs.is_empty())
        .map(|(i, _)| i)
        .collect();
    if collapsed_iconic {
        let icon_only = app.workspace_preset == "Iconic Mode";
        let mut flyout_anchor_y: Option<f32> = None;
        egui::ScrollArea::vertical()
            .id_salt(("dock_iconic_scroll", zone as u8))
            .show(ui, |ui| {
                let mut expand_to: Option<(usize, usize)> = None;
                for (gi, idx) in indices.iter().copied().enumerate() {
                    if gi > 0 {
                        let (div_r, _) = ui.allocate_exact_size(
                            Vec2::new(ui.available_width(), 3.0),
                            egui::Sense::hover(),
                        );
                        ui.painter().line_segment(
                            [
                                egui::pos2(div_r.left() + 2.0, div_r.center().y),
                                egui::pos2(div_r.right() - 2.0, div_r.center().y),
                            ],
                            egui::Stroke::new(1.0, p.stroke),
                        );
                    }
                    // Dotted group separator matching CS6 Iconic Strip
                    let (sep_r, _) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 7.0),
                        egui::Sense::hover(),
                    );
                    let dot_start_x = if icon_only {
                        sep_r.center().x - 6.0
                    } else {
                        sep_r.left() + 6.0
                    };
                    for d in 0..4 {
                        ui.painter().rect_filled(
                            egui::Rect::from_center_size(
                                egui::pos2(dot_start_x + d as f32 * 4.0, sep_r.center().y),
                                Vec2::splat(1.5),
                            ),
                            0.0,
                            p.text_dim,
                        );
                    }
                    let tabs = app.panel_groups[idx].tabs.clone();
                    let active_tab = app.panel_groups[idx].active_tab;
                    for (ti, pid) in tabs.into_iter().enumerate() {
                        if icon_only {
                            let (btn_r, btn_resp) = ui.allocate_exact_size(
                                Vec2::new(ui.available_width(), 24.0),
                                egui::Sense::click(),
                            );
                            let is_open = app.iconic_flyout_panel == Some(pid);
                            if is_open {
                                flyout_anchor_y = Some(btn_r.top());
                                ui.painter().rect_filled(
                                    btn_r,
                                    0.0,
                                    p.selection.gamma_multiply(0.32),
                                );
                                ui.painter().rect_stroke(
                                    btn_r,
                                    0.0,
                                    egui::Stroke::new(1.0, p.selection),
                                    egui::StrokeKind::Middle,
                                );
                            } else if btn_resp.hovered() {
                                ui.painter().rect_filled(btn_r, 0.0, p.well);
                            }
                            let icon_r =
                                egui::Rect::from_center_size(btn_r.center(), Vec2::splat(15.0));
                            icons::draw(
                                ui.painter(),
                                icon_r,
                                panel_mini_icon(pid),
                                if is_open || btn_resp.hovered() {
                                    p.accent
                                } else {
                                    p.text
                                },
                                1.25,
                            );
                            if btn_resp.clicked() {
                                app.iconic_flyout_panel = if is_open { None } else { Some(pid) };
                            }
                            btn_resp.on_hover_text(pid.title());
                        } else {
                            let (row_r, row_resp) = ui.allocate_exact_size(
                                Vec2::new(ui.available_width(), 20.0),
                                egui::Sense::click(),
                            );
                            if row_resp.hovered() {
                                ui.painter().rect_filled(row_r, 0.0, p.well);
                            } else if ti == active_tab {
                                ui.painter().rect_filled(
                                    row_r,
                                    0.0,
                                    p.panel_alt.gamma_multiply(0.7),
                                );
                            }
                            let icon_r = egui::Rect::from_center_size(
                                egui::pos2(row_r.left() + 10.0, row_r.center().y),
                                Vec2::splat(13.0),
                            );
                            icons::draw(
                                ui.painter(),
                                icon_r,
                                panel_mini_icon(pid),
                                if row_resp.hovered() { p.accent } else { p.text },
                                1.1,
                            );
                            let mut title = pid.title().to_string();
                            if title.len() > 10 {
                                title.truncate(9);
                                title.push('…');
                            }
                            ui.painter().text(
                                egui::pos2(row_r.left() + 20.0, row_r.center().y),
                                egui::Align2::LEFT_CENTER,
                                title,
                                egui::FontId::proportional(10.0),
                                if row_resp.hovered() {
                                    p.text
                                } else {
                                    p.text_dim
                                },
                            );
                            if row_resp.clicked() {
                                expand_to = Some((idx, ti));
                            }
                            row_resp.on_hover_text(format!("Expand {}", pid.title()));
                        }
                    }
                }
                if let Some((gidx, tidx)) = expand_to {
                    app.panel_groups[gidx].active_tab = tidx;
                    app.panel_groups[gidx].collapsed = false;
                    app.right_dock_collapsed = false;
                }
            });
        if icon_only && let Some(pid) = app.iconic_flyout_panel {
            let dock_r = ui.min_rect();
            let screen = ui.ctx().content_rect();
            let fly_w = 268.0;
            let fly_x = if app.right_dock_zone == crate::app::ToolsDockMode::Left {
                (dock_r.right() + 4.0).min(screen.right() - fly_w - 8.0)
            } else {
                (dock_r.left() - fly_w - 4.0).max(screen.left() + 8.0)
            };
            let fly_y = flyout_anchor_y.unwrap_or(dock_r.top() + 20.0).clamp(
                screen.top() + 56.0,
                (screen.bottom() - 280.0).max(screen.top() + 56.0),
            );
            egui::Area::new(ui.make_persistent_id(("iconic_flyout", pid as u8)))
                .order(egui::Order::Foreground)
                .fixed_pos(egui::pos2(fly_x, fly_y))
                .constrain(true)
                .show(ui.ctx(), |ui| {
                    egui::Frame::new()
                        .fill(p.panel)
                        .stroke(egui::Stroke::new(1.0, p.stroke))
                        .shadow(egui::Shadow {
                            offset: [2, 3],
                            blur: 8,
                            spread: 0,
                            color: egui::Color32::from_black_alpha(110),
                        })
                        .inner_margin(egui::Margin::same(6))
                        .show(ui, |ui| {
                            ui.set_width(fly_w);
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(pid.title()).small().strong().color(p.text));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.small_button("×").clicked() {
                                            app.iconic_flyout_panel = None;
                                        }
                                    },
                                );
                            });
                            ui.separator();
                            egui::ScrollArea::vertical()
                                .id_salt(("iconic_flyout_scroll", pid as u8))
                                .max_height(280.0)
                                .show(ui, |ui| {
                                    panels::render_panel_by_id(ui, app, pid);
                                });
                        });
                });
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                app.iconic_flyout_panel = None;
            }
        }
        return;
    }
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

/// Scrolling tool sections plus an always-visible footer pinned to the bottom of the rail.
fn tools_dock_contents(ui: &mut Ui, app: &mut FireCraft, scroll_id: &str) {
    let footer_h = ui.available_height().min(30.0);
    let total_h = ui.available_height();
    let body_h = (total_h - footer_h).max(0.0);
    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), body_h),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_min_height(body_h);
            ui.set_max_height(body_h);
            egui::ScrollArea::vertical()
                .id_salt(scroll_id)
                .auto_shrink([false, false])
                .max_height(body_h)
                .show(ui, |ui| panels::tools_panel_body(ui, app));
        },
    );
    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), footer_h),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.set_min_height(footer_h);
            ui.set_max_height(footer_h);
            panels::tools_panel_footer(ui, app);
        },
    );
}

/// The bottom status bar + CS6 animation playback transport controls.
fn status_bar(ui: &mut Ui, app: &mut FireCraft) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 6.0;
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

        // Zoom, resolution, DPI, and state transport live only in the canvas footer so the
        // status bar never duplicates document controls.
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
    StateFirst,
    StateLast,
    BrushSize(f32),
    FocusIntelligence,
    InfiniteCanvas,
}

fn shortcuts(ui: &mut Ui, app: &mut FireCraft) {
    // Check the pen state first: `consume_key` removes the event, so consuming it here and then
    // testing `pen_nodes` would swallow Escape whenever no pen path is being drawn.
    if !app.pen_nodes.is_empty()
        && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Escape))
    {
        app.finish_pen_path(false);
        ui.ctx().request_repaint();
        return;
    }
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
            // Cancel any in-progress creation/drag/text edit, then return to the Pointer tool.
            // Selection is preserved so users can immediately continue editing.
            app.drag = None;
            app.inline_text_edit_obj = None;
            app.alt_hand_pan = false;
            if app.tool != Tool::Select {
                app.status = "Pointer tool (Esc)".to_string();
            }
            app.set_tool(Tool::Select);
        }
        ui.ctx().request_repaint();
        return;
    }
    // `egui_wants_keyboard_input` is true for *any* focused widget, including a clicked tab or
    // toolbar button. Only a text edit should block single-key tool shortcuts; otherwise the
    // first tab click silently disables the keyboard for the rest of the session.
    if !ui.ctx().text_edit_focused() {
        if !app.pen_nodes.is_empty()
            && ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Enter))
        {
            app.finish_pen_path(false);
            ui.ctx().request_repaint();
            return;
        }
        // Shift+Tab must be consumed first: `consume_key(NONE, ..)` matches logically and would
        // otherwise swallow the shifted chord, making the dock toggle unreachable.
        let shift_tab = ui.input_mut(|i| i.consume_key(egui::Modifiers::SHIFT, Key::Tab));
        if shift_tab {
            app.show_right_dock = !app.show_right_dock;
            ui.ctx().request_repaint();
            return;
        }
        let tab_toggled = ui.input_mut(|i| i.consume_key(egui::Modifiers::NONE, Key::Tab));
        if tab_toggled {
            if let Some(fid) = ui.ctx().memory(|m| m.focused()) {
                ui.ctx().memory_mut(|m| m.surrender_focus(fid));
            }
            app.toggle_full_screen_canvas();
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
    if ui.ctx().text_edit_focused() {
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
                        (Key::I, true, false, true) => Some(Shortcut::FocusIntelligence),
                        (Key::F, true, true, false) => Some(Shortcut::InfiniteCanvas),
                        (Key::OpenBracket, false, false, false) => Some(Shortcut::BrushSize(-1.0)),
                        (Key::CloseBracket, false, false, false) => Some(Shortcut::BrushSize(1.0)),
                        (Key::Plus | Key::Equals, false, false, false) => Some(Shortcut::ZoomIn),
                        (Key::Minus, false, false, false) => Some(Shortcut::ZoomOut),
                        (Key::Home, false, false, false) => Some(Shortcut::StateFirst),
                        (Key::End, false, false, false) => Some(Shortcut::StateLast),

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
                        (Key::O, false, false, false) => Some(Shortcut::Tool(Tool::OvalMarquee)),
                        (Key::L, false, false, false) => Some(Shortcut::Tool(Tool::Lasso)),
                        (Key::W, false, false, false) => Some(Shortcut::Tool(Tool::Wand)),
                        (Key::C, false, false, false) => Some(Shortcut::Tool(Tool::Crop)),
                        (Key::B, false, false, false) => Some(Shortcut::Tool(Tool::Brush)),
                        (Key::E, false, false, false) => Some(Shortcut::Tool(Tool::Eraser)),
                        (Key::S, false, false, false) => Some(Shortcut::Tool(Tool::Stamp)),
                        (Key::R, false, false, false) => Some(Shortcut::Tool(Tool::ReplaceColor)),
                        (Key::Y, false, false, false) => Some(Shortcut::Tool(Tool::RedEye)),
                        (Key::N, false, false, false) => Some(Shortcut::Tool(Tool::Line)),
                        (Key::P, false, false, false) => Some(Shortcut::Tool(Tool::Path)),
                        (Key::F, false, false, false) => Some(Shortcut::Tool(Tool::Freeform)),
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
        Some(Shortcut::Save) => {
            app.run_file_intent(crate::app::FileIntent::SaveCurrent);
        }
        Some(Shortcut::SaveAs) => app.begin_file_dialog(FileMode::Save),
        Some(Shortcut::New) => app.show_new_dialog = true,
        Some(Shortcut::Open) => app.begin_file_dialog(FileMode::Open),
        Some(Shortcut::CloseTab) => {
            app.run_file_intent(crate::app::FileIntent::CloseActiveTab);
        }
        Some(Shortcut::Quit) => {
            app.run_file_intent(crate::app::FileIntent::Quit);
        }
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
            let next = match (t, app.tool) {
                (Tool::Rectangle, Tool::Rectangle) => Tool::RoundedRect,
                (Tool::Rectangle, Tool::RoundedRect) => Tool::Ellipse,
                (Tool::Rectangle, Tool::Ellipse) => Tool::Polygon,
                (Tool::Rectangle, Tool::Polygon) => Tool::Star,
                (Tool::Rectangle, Tool::Star) => Tool::Rectangle,
                (Tool::Brush, Tool::Brush) => Tool::Pencil,
                (Tool::Brush, Tool::Pencil) => Tool::Brush,
                (Tool::Marquee, Tool::Marquee) => Tool::Lasso,
                (Tool::Marquee, Tool::Lasso) => Tool::Marquee,
                (Tool::Gradient, Tool::Gradient) => Tool::Fill,
                (Tool::Gradient, Tool::Fill) => Tool::Gradient,
                _ => t,
            };
            app.set_tool(next);
            app.status = format!("{} selected", app.tool.name());
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
        Some(Shortcut::StateFirst) => {
            let count = app.doc.page().states.len();
            app.doc.page_mut().select_state(0);
            app.clear_selection();
            app.status = format!("First Animation State (1 of {count})");
        }
        Some(Shortcut::StateLast) => {
            let count = app.doc.page().states.len();
            app.doc.page_mut().select_state(count.saturating_sub(1));
            app.clear_selection();
            app.status = format!("Last Animation State ({count} of {count})");
        }
        Some(Shortcut::BrushSize(d)) => {
            app.brush_size = (app.brush_size + d).clamp(1.0, 128.0);
            app.status = format!("Brush size: {:.0} px", app.brush_size);
        }
        Some(Shortcut::FocusIntelligence) => {
            app.show_bottom_inspector = true;
            app.bottom_collapsed = false;
            app.bottom_tab = 2;
            app.status = "Opened Intelligence inspector".to_string();
        }
        Some(Shortcut::InfiniteCanvas) => {
            app.infinite_canvas = !app.infinite_canvas;
            app.status = if app.infinite_canvas {
                "Infinite canvas ON — flat exports crop to page bounds".to_string()
            } else {
                "Infinite canvas OFF".to_string()
            };
        }
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
