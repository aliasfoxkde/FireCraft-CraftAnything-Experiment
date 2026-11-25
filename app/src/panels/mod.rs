//! The docked Fireworks CS6 panels:
//! - Left three-column tool palette (Select, Bitmap, Vector, Web, Colors, View)
//! - Right stacked tab-groups:
//!   - Group 1: Optimize | History | Align | Path
//!   - Group 2: Pages | States | Layers
//!   - Group 3: Styles | Color Palette | Swatches | Info
//!   - Group 4: Special Characters | Image Editing | Auto Shapes | Document Library | CSS / URL
//! - Bottom dock: Properties & Symbol Properties inspector with Live Filters (+) stack.

use crate::app::{DockZone, FileMode, FireCraft, PanelId, Tool};
use crate::colors;
use crate::constants;
use crate::doc::{AutoShapeKind, BlendMode, GradientFill, LiveEffect, Obj, Shape};
use crate::icons;

mod color;
mod properties;
mod tools;
pub use color::*;
pub use properties::*;
pub use tools::*;

use eframe::egui::{
    self, Align2, Color32, FontId, Frame, Grid, Label, Rect, RichText, Sense, Stroke, TextEdit, Ui,
    Vec2, pos2,
};

/// A titled, tabbed panel container matching the Fireworks CS6 dock look
/// (`screenshots/Fireworks_Layout_Design.jpg`). Includes a leading collapse chevron and shared
/// double-click-to-collapse
/// behavior. Returns `true` when the header requested a collapse/expand toggle.
#[allow(clippy::too_many_arguments)]
pub fn panel_collapsible(
    ui: &mut Ui,
    title: &str,
    tabs: &[&str],
    tab_index: &mut usize,
    palette: crate::theme::Palette,
    body_height: Option<f32>,
    collapsed: bool,
    body: impl FnOnce(&mut Ui),
) -> bool {
    panel_impl(
        ui,
        title,
        tabs,
        tab_index,
        palette,
        body_height,
        Some(collapsed),
        body,
    )
}

#[allow(clippy::too_many_arguments)]
fn panel_impl(
    ui: &mut Ui,
    title: &str,
    tabs: &[&str],
    tab_index: &mut usize,
    palette: crate::theme::Palette,
    body_height: Option<f32>,
    collapsed: Option<bool>,
    body: impl FnOnce(&mut Ui),
) -> bool {
    let mut toggle_requested = false;
    Frame::new()
        .fill(palette.panel)
        .stroke(Stroke::new(1.0, palette.stroke))
        .inner_margin(egui::Margin::same(3))
        .show(ui, |ui| {
            let (bar, _) =
                ui.allocate_exact_size(Vec2::new(ui.available_width(), 20.0), Sense::hover());
            let painter = ui.painter();
            painter.rect_filled(bar, 0.0, palette.panel_alt);
            // Bottom border of the tab strip
            painter.line_segment(
                [
                    pos2(bar.left(), bar.bottom()),
                    pos2(bar.right(), bar.bottom()),
                ],
                Stroke::new(1.0, palette.stroke),
            );
            // Left CS6 gripper dots
            for dy in [-3.0f32, 0.0, 3.0] {
                painter.rect_filled(
                    Rect::from_center_size(
                        pos2(bar.left() + 3.5, bar.center().y + dy),
                        Vec2::splat(1.4),
                    ),
                    0.0,
                    palette.text_dim,
                );
            }

            let mut tabs_x = bar.left() + 8.0;
            if let Some(is_collapsed) = collapsed {
                // Keep the collapse affordance left of the tabs, matching the dock/tool strip
                // direction language (open points right, collapsed points down).
                let chev_r =
                    Rect::from_min_size(pos2(bar.left() + 3.0, bar.top() + 3.0), Vec2::splat(14.0));
                let chev_resp = ui.interact(
                    chev_r,
                    ui.make_persistent_id(("panel-collapse-btn", title)),
                    Sense::click(),
                );
                if chev_resp.hovered() {
                    painter.rect_filled(chev_r, 0.0, palette.well);
                }
                let (cx, cy) = (chev_r.center().x, chev_r.center().y);
                let col = if chev_resp.hovered() {
                    palette.accent
                } else {
                    palette.text_dim
                };
                if is_collapsed {
                    painter.add(egui::Shape::convex_polygon(
                        vec![
                            pos2(cx - 2.5, cy - 3.5),
                            pos2(cx + 2.5, cy + 0.5),
                            pos2(cx - 2.5, cy + 3.5),
                        ],
                        col,
                        Stroke::NONE,
                    ));
                } else {
                    painter.add(egui::Shape::convex_polygon(
                        vec![
                            pos2(cx - 3.5, cy - 2.5),
                            pos2(cx + 3.5, cy - 2.5),
                            pos2(cx, cy + 2.5),
                        ],
                        col,
                        Stroke::NONE,
                    ));
                }
                if chev_resp
                    .on_hover_text(if is_collapsed {
                        "Expand panel"
                    } else {
                        "Collapse panel"
                    })
                    .clicked()
                {
                    toggle_requested = true;
                }
                tabs_x = bar.left() + 23.0;
            }
            if tabs.is_empty() {
                painter.text(
                    pos2(tabs_x + 1.0, bar.center().y),
                    Align2::LEFT_CENTER,
                    title,
                    FontId::proportional(11.0),
                    palette.text,
                );
            } else {
                let mut x = tabs_x;
                for (i, t) in tabs.iter().enumerate() {
                    let w = (10.0 + t.len() as f32 * 5.2).min(88.0);
                    let r = Rect::from_min_size(pos2(x, bar.top() + 1.0), Vec2::new(w, 19.0));
                    let response = ui.interact(
                        r,
                        ui.make_persistent_id(("panel-tab", title, i)),
                        Sense::click(),
                    );
                    let is_active = i == *tab_index;
                    if is_active {
                        painter.rect_filled(r, 0.0, palette.panel);
                        // Top + left + right border so active tab merges into panel body
                        painter.line_segment(
                            [r.left_top(), r.right_top()],
                            Stroke::new(1.5, palette.accent),
                        );
                        painter.line_segment(
                            [r.left_top(), r.left_bottom()],
                            Stroke::new(1.0, palette.stroke),
                        );
                        painter.line_segment(
                            [r.right_top(), r.right_bottom()],
                            Stroke::new(1.0, palette.stroke),
                        );
                    } else {
                        let tab_bg = if response.hovered() {
                            palette.well
                        } else {
                            palette.panel_alt
                        };
                        painter.rect_filled(r.shrink2(Vec2::new(0.0, 0.5)), 0.0, tab_bg);
                        painter.rect_stroke(
                            r,
                            0.0,
                            Stroke::new(1.0, palette.stroke),
                            egui::StrokeKind::Middle,
                        );
                    }
                    painter.text(
                        r.center(),
                        Align2::CENTER_CENTER,
                        *t,
                        FontId::proportional(10.5),
                        if is_active {
                            palette.text
                        } else {
                            palette.text_dim
                        },
                    );
                    if response.clicked() {
                        *tab_index = i;
                    }
                    x += w + 1.0;
                }
            }

            // Right-end CS6 panel options menu glyph (≡▾)
            let opt_r = Rect::from_min_size(
                pos2(bar.right() - 18.0, bar.top() + 2.0),
                Vec2::new(16.0, 16.0),
            );
            let opt_resp = ui.interact(
                opt_r,
                ui.make_persistent_id(("panel-opt-btn", title)),
                Sense::click(),
            );
            if opt_resp.hovered() {
                painter.rect_filled(opt_r, 0.0, palette.well);
            }
            let gx = opt_r.left() + 3.0;
            for row in 0..3 {
                let y = opt_r.top() + 5.0 + row as f32 * 2.5;
                painter.line_segment(
                    [pos2(gx, y), pos2(gx + 6.5, y)],
                    Stroke::new(1.0, palette.text_dim),
                );
            }
            painter.add(egui::Shape::convex_polygon(
                vec![
                    pos2(opt_r.right() - 5.5, opt_r.top() + 7.0),
                    pos2(opt_r.right() - 2.0, opt_r.top() + 7.0),
                    pos2(opt_r.right() - 3.75, opt_r.top() + 10.0),
                ],
                palette.text_dim,
                Stroke::NONE,
            ));
            if opt_resp
                .on_hover_text("Panel Options — Click to cycle panel tab")
                .clicked()
                && !tabs.is_empty()
            {
                *tab_index = (*tab_index + 1) % tabs.len();
            }

            if collapsed.is_some() {
                let bar_resp = ui.interact(
                    bar,
                    ui.make_persistent_id(("panel-double-click", title)),
                    Sense::click(),
                );
                if bar_resp.double_clicked() {
                    toggle_requested = true;
                }
            }
            if collapsed == Some(true) {
                return;
            }
            ui.add_space(3.0);
            if let Some(h) = body_height {
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), h),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_min_height(h);
                        ui.set_max_height(h);
                        egui::ScrollArea::vertical()
                            .id_salt(("panel_scroll", title, *tab_index))
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                body(ui);
                            });
                    },
                );
            } else {
                body(ui);
            }
        });
    toggle_requested
}

/// Optimize panel (CS6 top-right tab 0): export format, palette, quality, dither, matte, size estimate.
pub fn optimize_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    Grid::new("optimize_grid")
        .num_columns(2)
        .spacing([8.0, 4.0])
        .show(ui, |ui| {
            ui.label(RichText::new("Format:").small().color(p.text_dim));
            egui::ComboBox::from_id_salt("opt_format")
                .width(145.0)
                .selected_text(&app.doc.optimize.format)
                .show_ui(ui, |ui| {
                    for fmt in [
                        "PNG32",
                        "PNG24",
                        "PNG8",
                        "JPEG - Better Quality",
                        "JPEG - Smaller File",
                        "GIF WebSnap 128",
                        "Animated GIF",
                        "WebP Lossless",
                        "SVG Vector",
                        "HTML5 + Images",
                        "TIFF",
                        "BMP",
                    ] {
                        if ui
                            .selectable_label(app.doc.optimize.format == fmt, fmt)
                            .clicked()
                        {
                            app.doc.optimize.format = fmt.to_string();
                        }
                    }
                });
            ui.end_row();

            ui.label(RichText::new("Palette:").small().color(p.text_dim));
            egui::ComboBox::from_id_salt("opt_palette")
                .width(145.0)
                .selected_text(&app.doc.optimize.palette)
                .show_ui(ui, |ui| {
                    for pal in ["Adaptive", "WebAdaptive", "Web 216", "Exact", "Grayscale"] {
                        if ui
                            .selectable_label(app.doc.optimize.palette == pal, pal)
                            .clicked()
                        {
                            app.doc.optimize.palette = pal.to_string();
                        }
                    }
                });
            ui.end_row();

            ui.label(RichText::new("Quality:").small().color(p.text_dim));
            let mut q = app.doc.optimize.quality as f32;
            if ui
                .add(egui::Slider::new(&mut q, 1.0..=100.0).suffix("%"))
                .changed()
            {
                app.doc.optimize.quality = q as u8;
            }
            ui.end_row();

            ui.label(RichText::new("Dither:").small().color(p.text_dim));
            let mut d = app.doc.optimize.dither as f32;
            if ui
                .add(egui::Slider::new(&mut d, 0.0..=100.0).suffix("%"))
                .changed()
            {
                app.doc.optimize.dither = d as u8;
            }
            ui.end_row();
        });

    ui.checkbox(&mut app.doc.optimize.transparency, "Transparent");
    ui.horizontal(|ui| {
        ui.label(RichText::new("Matte:").small().color(p.text_dim));
        let mut matte = [
            app.doc.optimize.matte.r() as f32 / 255.0,
            app.doc.optimize.matte.g() as f32 / 255.0,
            app.doc.optimize.matte.b() as f32 / 255.0,
        ];
        if ui.color_edit_button_rgb(&mut matte).changed() {
            app.doc.optimize.matte = Color32::from_rgb(
                (matte[0] * 255.0) as u8,
                (matte[1] * 255.0) as u8,
                (matte[2] * 255.0) as u8,
            );
        }
    });

    ui.horizontal(|ui| {
        let kb = app.estimated_export_kb();
        ui.label(
            RichText::new(format!("Estimated size: ~{kb} KB"))
                .small()
                .color(p.accent),
        );
        if ui.small_button("Export Now…").clicked() {
            app.begin_file_dialog(FileMode::Export);
        }
    });
}

/// History panel (CS6 top-right tab 1): interactive Undo/Redo command list + Replay.
pub fn history_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    if app.undo.is_empty() && app.redo.is_empty() {
        ui.label(
            RichText::new("No history steps recorded yet.")
                .small()
                .color(p.text_dim),
        );
    } else {
        let labels: Vec<String> = app.undo.iter().map(|s| s.label.clone()).collect();
        egui::ScrollArea::vertical()
            .id_salt("history_panel_scroll")
            .max_height(100.0)
            .show(ui, |ui| {
                for (idx, lbl) in labels.iter().enumerate() {
                    let is_last = idx + 1 == labels.len();
                    if ui
                        .selectable_label(is_last, format!("{}. {}", idx + 1, lbl))
                        .clicked()
                    {
                        let steps_back = labels.len() - idx;
                        for _ in 0..steps_back {
                            app.undo_action();
                        }
                    }
                }
            });
    }
    ui.horizontal(|ui| {
        if ui
            .add_enabled(!app.undo.is_empty(), egui::Button::new("Undo").small())
            .clicked()
        {
            app.undo_action();
        }
        if ui
            .add_enabled(!app.redo.is_empty(), egui::Button::new("Redo").small())
            .clicked()
        {
            app.redo_action();
        }
        if ui
            .add_enabled(app.selected.is_some(), egui::Button::new("Replay").small())
            .on_hover_text("Duplicate and repeat last transformation")
            .clicked()
        {
            app.replay_last_action();
        }
        if ui.small_button("Clear").clicked() {
            app.undo.clear();
            app.redo.clear();
            app.status = "History cleared".to_string();
        }
    });
}

fn align_icon_btn(
    ui: &mut Ui,
    palette: crate::theme::Palette,
    icon: icons::AlignIcon,
    enabled: bool,
    active: bool,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(22.0, 20.0), Sense::click());
    let painter = ui.painter();
    let bg = if active {
        palette.accent.gamma_multiply(0.32)
    } else if response.hovered() && enabled {
        palette.well
    } else {
        palette.panel_alt
    };
    painter.rect_filled(rect, 0.0, bg);
    painter.rect_stroke(
        rect,
        0.0,
        Stroke::new(
            1.0,
            if active {
                palette.selection
            } else if response.hovered() && enabled {
                palette.accent
            } else {
                palette.stroke
            },
        ),
        egui::StrokeKind::Middle,
    );
    let fg = if enabled {
        palette.text
    } else {
        palette.text_dim.gamma_multiply(0.6)
    };
    icons::draw_align(painter, rect, icon, fg);
    response.on_hover_text(tooltip)
}

fn path_icon_btn(
    ui: &mut Ui,
    palette: crate::theme::Palette,
    icon: icons::PathIcon,
    enabled: bool,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(22.0, 18.0), Sense::click());
    let painter = ui.painter();
    let bg = if response.hovered() && enabled {
        palette.well
    } else {
        palette.panel_alt
    };
    painter.rect_filled(rect, 0.0, bg);
    painter.rect_stroke(
        rect,
        0.0,
        Stroke::new(
            1.0,
            if response.hovered() && enabled {
                palette.accent
            } else {
                palette.stroke
            },
        ),
        egui::StrokeKind::Middle,
    );
    let fg = if enabled {
        palette.text
    } else {
        palette.text_dim.gamma_multiply(0.6)
    };
    icons::draw_path(painter, rect, icon, fg);
    response.on_hover_text(tooltip)
}

/// Align panel (CS6 top-right tab 2): exact Fireworks CS6 icon layout (`screenshots/Fireworks_Layout_Design.jpg`).
pub fn align_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let has_sel = app.selected.is_some() || !app.selected_set.is_empty();
    ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);

    // Top row: Relative To Object / Relative To Canvas right-aligned toggle with CS6 icon
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let rel_icon = if app.align_to_canvas {
                icons::AlignIcon::RelativeToCanvas
            } else {
                icons::AlignIcon::RelativeToObject
            };
            let tip = if app.align_to_canvas {
                "Relative To Canvas (click to switch to Relative To Object)"
            } else {
                "Relative To Object (click to switch to Relative To Canvas)"
            };
            if align_icon_btn(ui, p, rel_icon, true, app.align_to_canvas, tip).clicked() {
                app.align_to_canvas = !app.align_to_canvas;
                app.status = if app.align_to_canvas {
                    "Align mode: Relative To Canvas".to_string()
                } else {
                    "Align mode: Relative To Object".to_string()
                };
            }
            let rel_label = if app.align_to_canvas {
                "Relative To Canvas"
            } else {
                "Relative To Object"
            };
            if ui
                .add(
                    egui::Label::new(RichText::new(rel_label).small().color(p.text))
                        .sense(Sense::click()),
                )
                .clicked()
            {
                app.align_to_canvas = !app.align_to_canvas;
            }
        });
    });

    // Align row: [Left, Center H, Right]   [Top, Center V, Bottom]
    ui.label(RichText::new("Align:").small().color(p.text_dim));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        for (icon, mode, tip) in [
            (icons::AlignIcon::AlignLeft, "left", "Align Left Edge"),
            (
                icons::AlignIcon::AlignCenterH,
                "center_x",
                "Align Horizontal Center",
            ),
            (icons::AlignIcon::AlignRight, "right", "Align Right Edge"),
        ] {
            if align_icon_btn(ui, p, icon, has_sel, false, tip).clicked() && has_sel {
                app.align_selected_to_canvas(mode);
            }
        }
        ui.add_space(10.0);
        for (icon, mode, tip) in [
            (icons::AlignIcon::AlignTop, "top", "Align Top Edge"),
            (
                icons::AlignIcon::AlignCenterV,
                "center_y",
                "Align Vertical Center",
            ),
            (icons::AlignIcon::AlignBottom, "bottom", "Align Bottom Edge"),
        ] {
            if align_icon_btn(ui, p, icon, has_sel, false, tip).clicked() && has_sel {
                app.align_selected_to_canvas(mode);
            }
        }
    });

    // Distribute row: [Top, Center V, Bottom]   [Left, Center H, Right]
    ui.label(RichText::new("Distribute:").small().color(p.text_dim));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        for (icon, mode, tip) in [
            (
                icons::AlignIcon::DistributeTop,
                "top",
                "Distribute Top Edges",
            ),
            (
                icons::AlignIcon::DistributeCenterV,
                "center_v",
                "Distribute Vertical Centers",
            ),
            (
                icons::AlignIcon::DistributeBottom,
                "bottom",
                "Distribute Bottom Edges",
            ),
        ] {
            if align_icon_btn(ui, p, icon, has_sel, false, tip).clicked() && has_sel {
                app.distribute_selected(mode);
            }
        }
        ui.add_space(10.0);
        for (icon, mode, tip) in [
            (
                icons::AlignIcon::DistributeLeft,
                "left",
                "Distribute Left Edges",
            ),
            (
                icons::AlignIcon::DistributeCenterH,
                "center_h",
                "Distribute Horizontal Centers",
            ),
            (
                icons::AlignIcon::DistributeRight,
                "right",
                "Distribute Right Edges",
            ),
        ] {
            if align_icon_btn(ui, p, icon, has_sel, false, tip).clicked() && has_sel {
                app.distribute_selected(mode);
            }
        }
    });

    // Match Size & Space 2-column row
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.label(RichText::new("Match Size:").small().color(p.text_dim));
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
                if align_icon_btn(
                    ui,
                    p,
                    icons::AlignIcon::MatchWidth,
                    has_sel,
                    false,
                    "Match Width",
                )
                .clicked()
                    && has_sel
                {
                    app.match_canvas_dimension(true, false);
                }
                if align_icon_btn(
                    ui,
                    p,
                    icons::AlignIcon::MatchHeight,
                    has_sel,
                    false,
                    "Match Height",
                )
                .clicked()
                    && has_sel
                {
                    app.match_canvas_dimension(false, true);
                }
                if align_icon_btn(
                    ui,
                    p,
                    icons::AlignIcon::MatchBoth,
                    has_sel,
                    false,
                    "Match Width and Height",
                )
                .clicked()
                    && has_sel
                {
                    app.match_canvas_dimension(true, true);
                }
            });
        });

        ui.add_space(8.0);

        ui.vertical(|ui| {
            ui.label(RichText::new("Space:").small().color(p.text_dim));
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
                if align_icon_btn(
                    ui,
                    p,
                    icons::AlignIcon::SpaceVertical,
                    has_sel,
                    false,
                    "Space Vertically",
                )
                .clicked()
                    && has_sel
                {
                    app.space_selected(false);
                }
                if align_icon_btn(
                    ui,
                    p,
                    icons::AlignIcon::SpaceHorizontal,
                    has_sel,
                    false,
                    "Space Horizontally",
                )
                .clicked()
                    && has_sel
                {
                    app.space_selected(true);
                }
                egui::ComboBox::from_id_salt("align_space_mode_combo")
                    .width(64.0)
                    .selected_text(&app.space_mode)
                    .show_ui(ui, |ui| {
                        for opt in ["Evenly", "4 px", "8 px", "16 px", "24 px", "32 px"] {
                            if ui.selectable_label(app.space_mode == opt, opt).clicked() {
                                app.space_mode = opt.to_string();
                            }
                        }
                    });
            });
        });
    });
}

/// Path panel (CS6 top-right tab 3): exact Fireworks CS6 4-group icon grid (`screenshots/Fireworks_Layout_Design.jpg`).
pub fn paths_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let has_sel = app.selected.is_some() || !app.selected_set.is_empty();
    ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);

    // 1. Combine Paths (6 icons)
    ui.label(RichText::new("Combine Paths:").small().color(p.text_dim));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        for (icon, op, tip) in [
            (icons::PathIcon::Join, "join", "Join Paths (Ctrl+J)"),
            (
                icons::PathIcon::Split,
                "split",
                "Split Paths (Ctrl+Shift+J)",
            ),
            (icons::PathIcon::Union, "union", "Union Paths"),
            (icons::PathIcon::Intersect, "intersect", "Intersect Paths"),
            (icons::PathIcon::Punch, "punch", "Punch Paths"),
            (icons::PathIcon::Crop, "crop", "Crop Paths"),
        ] {
            if path_icon_btn(ui, p, icon, has_sel, tip).clicked() && has_sel {
                app.combine_paths_op(op);
            }
        }
    });

    // 2. Alter Paths (2 rows: 7 icons + 6 icons)
    ui.label(RichText::new("Alter Paths:").small().color(p.text_dim));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        if path_icon_btn(ui, p, icons::PathIcon::Simplify, has_sel, "Simplify Path").clicked()
            && has_sel
        {
            app.simplify_selected_path();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::ExpandStroke,
            has_sel,
            "Expand Stroke (+4 px)",
        )
        .clicked()
            && has_sel
        {
            app.expand_selected_bounds(4.0);
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::InsetExpand,
            has_sel,
            "Inset Path (-4 px)",
        )
        .clicked()
            && has_sel
        {
            app.expand_selected_bounds(-4.0);
        }
        if path_icon_btn(ui, p, icons::PathIcon::HardEdge, has_sel, "Hard Path Edge").clicked()
            && has_sel
        {
            app.clear_live_effects();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::AntiAlias,
            has_sel,
            "Anti-Alias Edge",
        )
        .clicked()
            && has_sel
        {
            app.status = "Path edge mode set to Anti-Alias".to_string();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::FeatherEdge,
            has_sel,
            "Feather Path Edge (Gaussian Blur)",
        )
        .clicked()
            && has_sel
        {
            app.add_live_effect(LiveEffect::GaussianBlur {
                radius: 3.0,
                enabled: true,
            });
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::StrokeToFill,
            has_sel,
            "Convert Stroke to Fill",
        )
        .clicked()
            && has_sel
        {
            app.convert_stroke_to_fill();
        }
    });
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::ReverseDirection,
            has_sel,
            "Reverse Path Direction",
        )
        .clicked()
            && has_sel
        {
            app.reverse_selected_path();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::ClosePath,
            has_sel,
            "Close Path Contour",
        )
        .clicked()
            && has_sel
        {
            app.toggle_selected_path_closed();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::OpenPath,
            has_sel,
            "Open Path Contour",
        )
        .clicked()
            && has_sel
        {
            app.toggle_selected_path_closed();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::PathToMarquee,
            has_sel,
            "Convert Path to Marquee",
        )
        .clicked()
            && has_sel
        {
            app.set_tool(Tool::Marquee);
            app.status = "Converted path bounds to Marquee selection".to_string();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::SnapToPixel,
            has_sel,
            "Snap to Pixel (Ctrl+K)",
        )
        .clicked()
            && has_sel
        {
            app.snap_selected_to_pixel();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::FlattenPath,
            has_sel,
            "Flatten Path to Editable Vector Contour",
        )
        .clicked()
            && has_sel
        {
            app.convert_selected_to_path();
        }
    });

    // 3. Edit Points (2 rows: 7 icons + 7 icons)
    ui.label(RichText::new("Edit Points:").small().color(p.text_dim));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        if path_icon_btn(ui, p, icons::PathIcon::AddPoints, has_sel, "Add Points").clicked()
            && has_sel
        {
            app.add_points_to_selected_path();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::DeletePoints,
            has_sel,
            "Delete Points",
        )
        .clicked()
            && has_sel
        {
            app.delete_point_from_selected_path();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::CornerPoint,
            has_sel,
            "Convert to Corner Point",
        )
        .clicked()
            && has_sel
        {
            app.round_selected_path_points(-4.0);
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::SmoothPoint,
            has_sel,
            "Convert to Smooth Point",
        )
        .clicked()
            && has_sel
        {
            app.round_selected_path_points(4.0);
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::RoundPoints,
            has_sel,
            "Round Points (+6 px)",
        )
        .clicked()
            && has_sel
        {
            app.round_selected_path_points(6.0);
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::FilletPoints,
            has_sel,
            "Fillet Points (+10 px)",
        )
        .clicked()
            && has_sel
        {
            app.round_selected_path_points(10.0);
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::ChamferPoints,
            has_sel,
            "Chamfer Points",
        )
        .clicked()
            && has_sel
        {
            app.convert_selected_to_path();
        }
    });
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::BreakPath,
            has_sel,
            "Break Path at Points",
        )
        .clicked()
            && has_sel
        {
            app.split_selected_path();
        }
        if path_icon_btn(ui, p, icons::PathIcon::JoinPoints, has_sel, "Join Points").clicked()
            && has_sel
        {
            app.toggle_selected_path_closed();
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::AlignPointsH,
            has_sel,
            "Align Points Horizontally",
        )
        .clicked()
            && has_sel
        {
            app.align_selected_path_points(true);
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::AlignPointsV,
            has_sel,
            "Align Points Vertically",
        )
        .clicked()
            && has_sel
        {
            app.align_selected_path_points(false);
        }
        if path_icon_btn(
            ui,
            p,
            icons::PathIcon::KnifeSplit,
            has_sel,
            "Knife / Split Contour",
        )
        .clicked()
            && has_sel
        {
            app.split_selected_path();
        }
        if path_icon_btn(ui, p, icons::PathIcon::ExtendPath, has_sel, "Extend Path").clicked()
            && has_sel
        {
            app.add_points_to_selected_path();
        }
        if path_icon_btn(ui, p, icons::PathIcon::SharpApex, has_sel, "Sharpen Corner").clicked()
            && has_sel
        {
            app.simplify_selected_path();
        }
    });

    // 4. Select Points (5 icons + CS6 point-chain visual indicator)
    ui.label(RichText::new("Select Points:").small().color(p.text_dim));
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
        for (icon, tip) in [
            (
                icons::PathIcon::InversePoints,
                "Inverse Point Selection (Subselect Tool)",
            ),
            (
                icons::PathIcon::SelectAllPoints,
                "Select All Points on Path",
            ),
            (icons::PathIcon::FirstPoint, "Select First Point on Contour"),
            (
                icons::PathIcon::PrevPoint,
                "Select Previous Point on Contour",
            ),
            (icons::PathIcon::NextPoint, "Select Next Point on Contour"),
        ] {
            if path_icon_btn(ui, p, icon, has_sel, tip).clicked() && has_sel {
                app.convert_selected_to_path();
                app.set_tool(Tool::Subselect);
                app.status = format!("{tip} active");
            }
        }
        // CS6 node-chain schematic graphic on the right side of Select Points
        let (diag_r, _) = ui.allocate_exact_size(Vec2::new(42.0, 20.0), Sense::hover());
        let painter = ui.painter();
        let y_top = diag_r.top() + 5.0;
        let y_bot = diag_r.bottom() - 5.0;
        painter.line_segment(
            [
                pos2(diag_r.left() + 4.0, y_top),
                pos2(diag_r.right() - 4.0, y_top),
            ],
            Stroke::new(1.0, p.text_dim),
        );
        painter.rect_stroke(
            Rect::from_min_max(
                pos2(diag_r.left() + 8.0, y_top),
                pos2(diag_r.right() - 8.0, y_bot),
            ),
            0.0,
            Stroke::new(1.0, p.text_dim),
            egui::StrokeKind::Middle,
        );
        for nx in [diag_r.left() + 4.0, diag_r.center().x, diag_r.right() - 4.0] {
            painter.rect_filled(
                Rect::from_center_size(pos2(nx, y_top), Vec2::splat(3.0)),
                0.0,
                p.text,
            );
        }
    });
}

/// Colour swatches, laid out as Fireworks CS6's 21×12 Color Cubes ramp.
pub fn swatches_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let cols = constants::SWATCH_COLS;
    let cell = 11.0;
    let swatches = app.swatches.clone();
    let mut index = 0;
    ui.spacing_mut().item_spacing = Vec2::splat(1.0);
    while index < swatches.len() {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(1.0);
            for _ in 0..cols {
                if index >= swatches.len() {
                    break;
                }
                let color = swatches[index];
                let (rect, response) = ui.allocate_exact_size(Vec2::splat(cell), Sense::click());
                let painter = ui.painter();
                painter.rect_filled(rect, 0.0, color);
                painter.rect_stroke(
                    rect,
                    0.0,
                    Stroke::new(0.5, p.stroke),
                    egui::StrokeKind::Middle,
                );
                if response.clicked() {
                    app.fill = color;
                    app.set_selected_fill(Some(color));
                }
                if response.secondary_clicked() {
                    app.stroke = color;
                    app.set_selected_stroke(Some(color));
                }
                if response.hovered() {
                    response.on_hover_text(format!(
                        "{} (Left-click: Fill, Right-click: Stroke)",
                        colors::hex_of(color)
                    ));
                }
                index += 1;
            }
        });
    }
}

/// The Color Palette / Color Mixer panel (`screenshots/Fireworks_Layout_Design.jpg`):
/// Selector | Mixers | Blender tabs, Stroke/Fill icon+hex rows, HLS/HSV/CMYK/RGB field + sliders.
fn slider_row(ui: &mut Ui, label: &str, value: &mut f32, min: f32, max: f32, unit: &str) -> bool {
    ui.style_mut().spacing.slider_width = 58.0;
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).small());
        let mut v = *value;
        let response = ui.add(
            egui::Slider::new(&mut v, min..=max).custom_formatter(|x, _| format!("{x:.0}{unit}")),
        );
        *value = v;
        response.changed()
    })
    .inner
}

/// Info panel (CS6 middle-right tab 3): live cursor coordinates, RGBA readout, selection bounds.
pub fn info_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    Grid::new("info_panel_grid")
        .num_columns(4)
        .spacing([10.0, 4.0])
        .show(ui, |ui| {
            ui.label(RichText::new("R:").small().color(p.text_dim));
            ui.label(RichText::new(format!("{}", app.fill.r())).small());
            let (cx, cy) = app
                .cursor_doc_pos
                .map(|v| (format!("{:.0}", v.x), format!("{:.0}", v.y)))
                .unwrap_or_else(|| ("—".to_string(), "—".to_string()));
            ui.label(RichText::new("X:").small().color(p.text_dim));
            ui.label(RichText::new(cx).small());
            ui.end_row();

            ui.label(RichText::new("G:").small().color(p.text_dim));
            ui.label(RichText::new(format!("{}", app.fill.g())).small());
            ui.label(RichText::new("Y:").small().color(p.text_dim));
            ui.label(RichText::new(cy).small());
            ui.end_row();

            ui.label(RichText::new("B:").small().color(p.text_dim));
            ui.label(RichText::new(format!("{}", app.fill.b())).small());
            let (w_str, h_str) = app
                .selected_object()
                .and_then(|o| o.bounds())
                .map(|(lo, hi)| (format!("{:.0}", hi.x - lo.x), format!("{:.0}", hi.y - lo.y)))
                .unwrap_or_else(|| {
                    (
                        format!("{:.0}", app.doc.page().width),
                        format!("{:.0}", app.doc.page().height),
                    )
                });
            ui.label(RichText::new("W:").small().color(p.text_dim));
            ui.label(RichText::new(w_str).small());
            ui.end_row();

            ui.label(RichText::new("A:").small().color(p.text_dim));
            ui.label(RichText::new(format!("{}", app.fill.a())).small());
            ui.label(RichText::new("H:").small().color(p.text_dim));
            ui.label(RichText::new(h_str).small());
            ui.end_row();
        });
}

fn small_icon_cell(
    ui: &mut Ui,
    palette: crate::theme::Palette,
    icon: icons::Icon,
    active: bool,
    tooltip: &str,
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(rect, 0.0, palette.well);
    }
    let fg = if active {
        palette.text
    } else {
        palette.text_dim
    };
    icons::draw(ui.painter(), rect, icon, fg, 1.0);
    resp.on_hover_text(tooltip)
}

/// Pages, States (Animation frames), and Hierarchical Layers + Objects panel (`screenshots/Fireworks-All_Window_Panels.jpg`).
pub fn pages_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    match app.layers_tab {
        0 => {
            // Pages tab with CS6 page preview thumbnail box + bottom status/action bar
            let count = app.doc.pages.len();
            egui::ScrollArea::vertical()
                .id_salt("pages_tab_scroll")
                .max_height(114.0)
                .show(ui, |ui| {
                    for i in 0..count {
                        let selected = app.doc.current_page == i;
                        let (pname, pw, ph, pbg) = {
                            let page = &app.doc.pages[i];
                            (page.name.clone(), page.width, page.height, page.bg_color)
                        };
                        ui.horizontal(|ui| {
                            let (thumb, _) =
                                ui.allocate_exact_size(Vec2::new(22.0, 16.0), Sense::hover());
                            ui.painter().rect_filled(thumb, 0.0, pbg);
                            ui.painter().rect_stroke(
                                thumb,
                                0.0,
                                Stroke::new(1.0, if selected { p.selection } else { p.stroke }),
                                egui::StrokeKind::Middle,
                            );
                            let label = format!("{:02}  {} ({:.0}×{:.0})", i + 1, pname, pw, ph);
                            if ui
                                .selectable_label(selected, RichText::new(label).small())
                                .clicked()
                            {
                                app.doc.current_page = i;
                                app.selected = None;
                                app.selected_layer = 0;
                            }
                        });
                    }
                });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{count} Page{}", if count == 1 { "" } else { "s" }))
                        .small()
                        .color(p.text_dim),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if small_icon_cell(ui, p, icons::Icon::Trash, count > 1, "Delete Page")
                        .clicked()
                        && count > 1
                    {
                        app.doc.pages.remove(app.doc.current_page);
                        app.doc.current_page = app.doc.current_page.saturating_sub(1);
                    }
                    if small_icon_cell(ui, p, icons::Icon::New, true, "New Page").clicked() {
                        app.add_page();
                    }
                    if small_icon_cell(ui, p, icons::Icon::Duplicate, true, "Duplicate Page")
                        .clicked()
                        && count > 0
                    {
                        let page = app.doc.page().clone();
                        let mut new_page = page.clone();
                        new_page.name = format!("{} copy", page.name);
                        app.doc.pages.push(new_page);
                        app.doc.current_page = app.doc.pages.len() - 1;
                    }
                });
            });
        }
        1 => {
            // States (Animation / Rollover states) tab
            let state_count = app.doc.page().states.len();
            let current_st = app.doc.page().current_state;
            egui::ScrollArea::vertical()
                .id_salt("states_tab_scroll")
                .max_height(110.0)
                .show(ui, |ui| {
                    for i in 0..state_count {
                        let (name, delay, obj_cnt) = {
                            let st = &app.doc.page().states[i];
                            let cnt: usize = st.layers.iter().map(|l| l.objects.len()).sum();
                            (st.name.clone(), st.delay_cs, cnt)
                        };
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{:02}", i + 1))
                                    .small()
                                    .color(p.text_dim),
                            );
                            if ui
                                .selectable_label(
                                    current_st == i,
                                    RichText::new(format!("{name} ({obj_cnt})")).small(),
                                )
                                .clicked()
                            {
                                app.doc.page_mut().select_state(i);
                                app.selected = None;
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let mut d = delay;
                                    if ui
                                        .add(
                                            egui::DragValue::new(&mut d)
                                                .range(1..=1000)
                                                .suffix(" /100s"),
                                        )
                                        .changed()
                                    {
                                        app.doc.page_mut().states[i].delay_cs = d;
                                    }
                                },
                            );
                        });
                    }
                });
            ui.separator();
            ui.horizontal(|ui| {
                ui.checkbox(&mut app.onion_skin, "Onion");
                if ui.small_button("Tween 3x").clicked() {
                    app.tween_selected_across_states();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if small_icon_cell(ui, p, icons::Icon::New, true, "New Animation State")
                        .clicked()
                    {
                        app.add_state();
                    }
                    if small_icon_cell(ui, p, icons::Icon::Duplicate, true, "Duplicate State")
                        .clicked()
                    {
                        app.duplicate_state();
                    }
                });
            });
        }
        _ => {
            // Layers & Objects tree tab (matching CS6 Layers panel with Blend Mode + Opacity header + Eye/Lock columns)
            ui.horizontal(|ui| {
                let cur_blend = app
                    .doc
                    .page()
                    .layers
                    .get(app.selected_layer)
                    .map(|l| l.blend_mode)
                    .unwrap_or(BlendMode::Normal);
                egui::ComboBox::from_id_salt("layer_blend_combo")
                    .width(96.0)
                    .selected_text(cur_blend.label())
                    .show_ui(ui, |ui| {
                        for bm in BlendMode::ALL {
                            if ui.selectable_label(cur_blend == bm, bm.label()).clicked()
                                && let Some(l) =
                                    app.doc.page_mut().layers.get_mut(app.selected_layer)
                            {
                                l.blend_mode = bm;
                            }
                        }
                    });
                ui.label(RichText::new("Opacity:").small().color(p.text_dim));
                let mut opacity_pct = app
                    .doc
                    .page()
                    .layers
                    .get(app.selected_layer)
                    .map(|l| l.opacity * 100.0)
                    .unwrap_or(100.0);
                if ui
                    .add(
                        egui::DragValue::new(&mut opacity_pct)
                            .range(0.0..=100.0)
                            .suffix("%"),
                    )
                    .on_hover_text("Layer Opacity")
                    .changed()
                    && let Some(layer) = app.doc.page_mut().layers.get_mut(app.selected_layer)
                {
                    layer.opacity = opacity_pct / 100.0;
                }
            });

            let layers = app.doc.page().layers.clone();
            egui::ScrollArea::vertical()
                .id_salt("layers_tab_scroll")
                .max_height(128.0)
                .show(ui, |ui| {
                    for (li, layer) in layers.iter().enumerate().rev() {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(3.0, 1.0);
                            let eye_icon = if layer.visible {
                                icons::Icon::Eye
                            } else {
                                icons::Icon::EyeOff
                            };
                            if small_icon_cell(
                                ui,
                                p,
                                eye_icon,
                                layer.visible,
                                "Toggle Layer Visibility",
                            )
                            .clicked()
                                && let Some(l) = app.doc.page_mut().layers.get_mut(li)
                            {
                                l.visible = !l.visible;
                            }
                            let lock_icon = if layer.locked {
                                icons::Icon::Lock
                            } else {
                                icons::Icon::Unlock
                            };
                            if small_icon_cell(ui, p, lock_icon, layer.locked, "Toggle Layer Lock")
                                .clicked()
                                && let Some(l) = app.doc.page_mut().layers.get_mut(li)
                            {
                                l.locked = !l.locked;
                            }
                            let (fold_r, _) =
                                ui.allocate_exact_size(Vec2::splat(14.0), Sense::hover());
                            let folder_ic = if layer.is_web_layer {
                                icons::Icon::Slice
                            } else {
                                icons::Icon::Folder
                            };
                            icons::draw(ui.painter(), fold_r, folder_ic, p.accent, 1.0);

                            let label = format!("{} ({})", layer.name, layer.objects.len());
                            if ui
                                .selectable_label(
                                    app.selected_layer == li,
                                    RichText::new(label).small().strong(),
                                )
                                .clicked()
                            {
                                app.selected_layer = li;
                                app.selected = None;
                            }
                        });
                        // Expand objects inside the layer with Eye toggle + mini color swatch
                        for (oi, obj) in layer.objects.iter().enumerate().rev() {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(3.0, 1.0);
                                ui.add_space(16.0);
                                let o_eye = if obj.visible {
                                    icons::Icon::Eye
                                } else {
                                    icons::Icon::EyeOff
                                };
                                if small_icon_cell(
                                    ui,
                                    p,
                                    o_eye,
                                    obj.visible,
                                    "Toggle Object Visibility",
                                )
                                .clicked()
                                    && let Some(o) = app
                                        .doc
                                        .page_mut()
                                        .layers
                                        .get_mut(li)
                                        .and_then(|l| l.objects.get_mut(oi))
                                {
                                    o.visible = !o.visible;
                                }
                                let (sw_r, _) =
                                    ui.allocate_exact_size(Vec2::splat(10.0), Sense::hover());
                                let sw_col = obj
                                    .fill
                                    .or(obj.stroke)
                                    .unwrap_or(Color32::from_rgb(70, 185, 95));
                                ui.painter().rect_filled(sw_r, 0.0, sw_col);
                                ui.painter().rect_stroke(
                                    sw_r,
                                    0.0,
                                    Stroke::new(0.8, p.stroke),
                                    egui::StrokeKind::Middle,
                                );

                                let is_sel = app.selected_layer == li && app.is_selected(oi);
                                let obj_label = format!("{} [{}]", obj.name, obj.type_label());
                                let resp =
                                    ui.selectable_label(is_sel, RichText::new(obj_label).small());
                                if resp.clicked() {
                                    let shift_held =
                                        ui.input(|i| i.modifiers.shift || i.modifiers.ctrl);
                                    app.selected_layer = li;
                                    if shift_held {
                                        app.toggle_select(oi);
                                    } else {
                                        app.select_single(Some(oi));
                                    }
                                }
                            });
                        }
                    }
                });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.small_button("+ Web Slice").clicked() {
                    app.insert_slice(false);
                }
                if ui.small_button("+ Layer").clicked() {
                    app.add_layer();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if small_icon_cell(
                        ui,
                        p,
                        icons::Icon::Trash,
                        true,
                        "Delete Selected Object or Layer",
                    )
                    .clicked()
                    {
                        if app.selected.is_some() || !app.selected_set.is_empty() {
                            app.delete_selected();
                        } else {
                            let count = app.doc.page().layers.len();
                            if count > 1 && app.selected_layer < count {
                                let is_web = app.doc.page().layers[app.selected_layer].is_web_layer;
                                if !is_web {
                                    app.doc.page_mut().layers.remove(app.selected_layer);
                                    app.selected_layer = app.selected_layer.saturating_sub(1);
                                    app.selected = None;
                                } else {
                                    app.status = "Web Layer cannot be deleted".to_string();
                                }
                            }
                        }
                    }
                    if small_icon_cell(
                        ui,
                        p,
                        icons::Icon::Duplicate,
                        app.selected.is_some(),
                        "Duplicate Selection",
                    )
                    .clicked()
                    {
                        app.duplicate_selected();
                    }
                });
            });
        }
    }
}

/// The Styles panel (`screenshots/Fireworks_Layout_Design.jpg`): sharp CS6 style cards + Current Document header.
pub fn styles_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);
        let icon_r = ui.allocate_exact_size(Vec2::splat(16.0), Sense::hover()).0;
        icons::draw(
            ui.painter(),
            icon_r,
            icons::Icon::Duplicate,
            p.text_dim,
            1.1,
        );
        egui::ComboBox::from_id_salt("styles_library_combo")
            .width((ui.available_width() - 8.0).max(140.0))
            .selected_text(app.style_library.label())
            .show_ui(ui, |ui| {
                for lib in crate::doc::StyleLibrary::ALL {
                    if ui
                        .selectable_label(app.style_library == lib, lib.label())
                        .clicked()
                    {
                        app.style_library = lib;
                        app.selected_style_idx = None;
                    }
                }
            });
    });
    ui.add_space(3.0);

    let styles = if app.style_library == crate::doc::StyleLibrary::CurrentDocument {
        app.doc.styles.clone()
    } else {
        crate::doc::StylePreset::library_styles(app.style_library)
    };

    egui::Frame::new()
        .fill(p.well)
        .stroke(Stroke::new(1.0, p.stroke))
        .inner_margin(egui::Margin::same(4))
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("styles_panel_scroll")
                .max_height(132.0)
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                        for (idx, preset) in styles.iter().enumerate() {
                            let (rect, resp) =
                                ui.allocate_exact_size(Vec2::splat(34.0), Sense::click());
                            let painter = ui.painter();
                            let is_sel = app.selected_style_idx == Some(idx);

                            // Outer CS6 thumbnail frame
                            painter.rect_filled(rect, 0.0, Color32::WHITE);
                            let inner = rect.shrink(2.0);
                            let top_c = preset.fill.unwrap_or(Color32::from_rgb(76, 80, 88));
                            let bot_c = preset.gradient_end.unwrap_or(top_c);

                            // Multi-band vertical gradient preview (chrome/gloss/plastic)
                            let bands = 6usize;
                            let band_h = inner.height() / bands as f32;
                            for b in 0..bands {
                                let t = (b as f32 + 0.5) / bands as f32;
                                let r = (top_c.r() as f32 * (1.0 - t) + bot_c.r() as f32 * t) as u8;
                                let g = (top_c.g() as f32 * (1.0 - t) + bot_c.g() as f32 * t) as u8;
                                let bl =
                                    (top_c.b() as f32 * (1.0 - t) + bot_c.b() as f32 * t) as u8;
                                let al =
                                    (top_c.a() as f32 * (1.0 - t) + bot_c.a() as f32 * t) as u8;
                                painter.rect_filled(
                                    Rect::from_min_size(
                                        pos2(inner.left(), inner.top() + b as f32 * band_h),
                                        Vec2::new(inner.width(), band_h + 0.6),
                                    ),
                                    0.0,
                                    Color32::from_rgba_unmultiplied(r, g, bl, al),
                                );
                            }

                            // Effect indicators (Bevel highlight / Glow / DropShadow / InnerShadow)
                            for eff in &preset.effects {
                                match eff {
                                    crate::doc::LiveEffect::Bevel { .. } => {
                                        painter.line_segment(
                                            [
                                                pos2(inner.left() + 1.0, inner.top() + 1.5),
                                                pos2(inner.right() - 1.0, inner.top() + 1.5),
                                            ],
                                            Stroke::new(1.5, Color32::from_white_alpha(150)),
                                        );
                                        painter.line_segment(
                                            [
                                                pos2(inner.left() + 1.0, inner.bottom() - 1.5),
                                                pos2(inner.right() - 1.0, inner.bottom() - 1.5),
                                            ],
                                            Stroke::new(1.5, Color32::from_black_alpha(150)),
                                        );
                                    }
                                    crate::doc::LiveEffect::Glow { color, .. } => {
                                        painter.rect_stroke(
                                            inner.shrink(1.5),
                                            1.0,
                                            Stroke::new(1.5, *color),
                                            egui::StrokeKind::Inside,
                                        );
                                    }
                                    crate::doc::LiveEffect::DropShadow { .. } => {
                                        painter.line_segment(
                                            [
                                                pos2(inner.left() + 2.0, inner.bottom() - 0.5),
                                                pos2(inner.right(), inner.bottom() - 0.5),
                                            ],
                                            Stroke::new(2.0, Color32::from_black_alpha(120)),
                                        );
                                    }
                                    crate::doc::LiveEffect::InnerShadow { color, .. } => {
                                        painter.line_segment(
                                            [
                                                pos2(inner.left() + 1.0, inner.top() + 1.5),
                                                pos2(inner.right() - 1.0, inner.top() + 1.5),
                                            ],
                                            Stroke::new(1.5, *color),
                                        );
                                    }
                                    crate::doc::LiveEffect::GaussianBlur { .. }
                                    | crate::doc::LiveEffect::ColorAdjust { .. } => {}
                                }
                            }

                            if let Some(st) = preset.stroke {
                                painter.rect_stroke(
                                    inner,
                                    0.0,
                                    Stroke::new(preset.stroke_width.clamp(1.0, 2.5), st),
                                    egui::StrokeKind::Inside,
                                );
                            }
                            painter.rect_stroke(
                                rect,
                                0.0,
                                Stroke::new(
                                    if is_sel { 1.5 } else { 1.0 },
                                    if is_sel {
                                        p.selection
                                    } else if resp.hovered() {
                                        p.accent
                                    } else {
                                        p.stroke
                                    },
                                ),
                                egui::StrokeKind::Middle,
                            );
                            if resp.on_hover_text(&preset.name).clicked() {
                                app.selected_style_idx = Some(idx);
                                app.apply_style_preset_value(preset);
                            }
                        }
                    });
                });
        });

    ui.add_space(2.0);
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let can_del = app.style_library == crate::doc::StyleLibrary::CurrentDocument
                && app.selected_style_idx.is_some();
            if ui
                .add_enabled(can_del, egui::Button::new("🗑").small())
                .on_hover_text("Delete selected custom style from Current Document")
                .clicked()
            {
                app.delete_selected_style();
            }
            if ui
                .add_enabled(
                    app.selected.is_some(),
                    egui::Button::new("+ New Style").small(),
                )
                .on_hover_text("Capture selected object's appearance as a Style Preset")
                .clicked()
            {
                app.save_selection_as_style();
            }
        });
    });
}

/// Intelligence tab: real AI-assistant integration plus the existing MCP server status.
/// When no endpoint/key is configured the panel explains what is missing and never fabricates
/// a response.
pub fn intelligence_panel(ui: &mut Ui, app: &mut FireCraft) {
    app.poll_ai();
    let p = app.palette;
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("AI Assistant")
                .small()
                .strong()
                .color(p.accent),
        );
        let configured = app.ai_key_present() && !app.ai_model.trim().is_empty();
        ui.label(
            RichText::new(if configured {
                "Configured"
            } else {
                "Not configured"
            })
            .small()
            .color(if configured { p.text } else { p.text_dim }),
        );
        ui.separator();
        ui.label(
            RichText::new("MCP: firecraft mcp (stdio JSON-RPC 2.0)")
                .small()
                .color(p.text_dim),
        );
        if ui
            .small_button("Copy MCP config")
            .on_hover_text("Copy a client configuration for the FireCraft MCP stdio server")
            .clicked()
        {
            app.ctx.copy_text(
                r#"{"mcpServers":{"firecraft":{"command":"firecraft","args":["mcp"]}}}"#
                    .to_string(),
            );
            app.status = "Copied FireCraft MCP client configuration".to_string();
        }
        if ui
            .small_button(if app.show_ai_settings {
                "Hide settings"
            } else {
                "Settings…"
            })
            .clicked()
        {
            app.show_ai_settings = !app.show_ai_settings;
        }
    });

    if app.show_ai_settings {
        Grid::new("ai_settings_grid")
            .num_columns(2)
            .spacing([6.0, 3.0])
            .show(ui, |ui| {
                ui.label(RichText::new("Endpoint:").small().color(p.text_dim));
                ui.add(
                    TextEdit::singleline(&mut app.ai_base_url)
                        .desired_width(230.0)
                        .hint_text("https://api.openai.com/v1"),
                );
                ui.end_row();
                ui.label(RichText::new("Model:").small().color(p.text_dim));
                ui.add(
                    TextEdit::singleline(&mut app.ai_model)
                        .desired_width(230.0)
                        .hint_text("Provider chat model id"),
                );
                ui.end_row();
                ui.label(RichText::new("API key:").small().color(p.text_dim));
                ui.add(
                    TextEdit::singleline(&mut app.ai_key)
                        .password(true)
                        .desired_width(230.0)
                        .hint_text("or set FIRECRAFT_AI_API_KEY"),
                );
                ui.end_row();
            });
    }

    ui.horizontal(|ui| {
        ui.label(RichText::new("Prompt:").small().color(p.text_dim));
        let can_run = app.ai_rx.is_none()
            && app.ai_key_present()
            && !app.ai_model.trim().is_empty()
            && !app.ai_prompt.trim().is_empty();
        if ui
            .add_enabled(can_run, egui::Button::new("Ask about selection").small())
            .on_hover_text(
                "Send the prompt plus real selection/document context to the configured endpoint",
            )
            .clicked()
        {
            app.start_ai_request();
        }
        if app.ai_rx.is_some() {
            ui.spinner();
        }
        if ui
            .add_enabled(
                !app.ai_response.is_empty(),
                egui::Button::new("Clear").small(),
            )
            .clicked()
        {
            app.ai_response.clear();
            app.ai_status = "Response cleared.".to_string();
        }
        ui.label(RichText::new(&app.ai_status).small().color(p.text_dim));
    });
    ui.add(
        TextEdit::multiline(&mut app.ai_prompt)
            .desired_rows(2)
            .desired_width(f32::INFINITY)
            .hint_text("Describe what you want to know or change…"),
    );
    if !app.ai_response.is_empty() {
        egui::Frame::new()
            .fill(p.well)
            .stroke(Stroke::new(1.0, p.stroke))
            .inner_margin(egui::Margin::same(4))
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("ai_response_scroll")
                    .max_height(90.0)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.label(RichText::new(&app.ai_response).small().color(p.text));
                    });
            });
    }
}

/// Bottom-right dock group: Special Characters | Image Editing | Auto Shapes | Document Library | CSS/URL.
pub fn library_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    match app.library_tab {
        0 => {
            // Special Characters — uniform 9-column CS6 character matrix (`screenshots/Fireworks-All_Window_Panels.jpg`)
            let glyphs = [
                "¡", "¢", "£", "¥", "€", "§", "©", "®", "™", "«", "»", "°", "±", "×", "÷", "≈",
                "≠", "≤", "≥", "∞", "‰", "†", "‡", "•", "…", "¶", "←", "↑", "→", "↓", "↔", "⇒",
                "Δ", "Ω", "π", "µ", "♠", "♣", "♥", "♦", "★", "☆", "✓", "✕", "◊",
            ];
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 2.0);
            for row in glyphs.chunks(9) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(2.0, 2.0);
                    for g in row {
                        let (r, resp) =
                            ui.allocate_exact_size(Vec2::new(28.0, 20.0), Sense::click());
                        ui.painter().rect_filled(
                            r,
                            0.0,
                            if resp.hovered() { p.well } else { p.panel_alt },
                        );
                        ui.painter().rect_stroke(
                            r,
                            0.0,
                            Stroke::new(1.0, if resp.hovered() { p.accent } else { p.stroke }),
                            egui::StrokeKind::Middle,
                        );
                        ui.painter().text(
                            r.center(),
                            Align2::CENTER_CENTER,
                            *g,
                            FontId::proportional(11.5),
                            p.text,
                        );
                        if resp
                            .on_hover_text(format!("Insert '{g}' into canvas or selected text"))
                            .clicked()
                        {
                            app.insert_special_glyph(g);
                        }
                    }
                });
            }
        }
        1 => {
            // Image Editing — CS6 top icon bar + categorized action rows (`screenshots/Fireworks-All_Window_Panels.jpg`)
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(4.0, 2.0);
                for (ic, tool, tip) in [
                    (icons::Icon::Wand, Tool::Wand, "Magic Wand Tool (W)"),
                    (icons::Icon::Crop, Tool::Crop, "Crop Tool (C)"),
                    (icons::Icon::Blur, Tool::Blur, "Blur Tool"),
                    (icons::Icon::Sharpen, Tool::Sharpen, "Sharpen Tool"),
                    (icons::Icon::Dodge, Tool::Dodge, "Dodge / Burn Tool"),
                ] {
                    if crate::canvas::tool_button(ui, ic, app.tool == tool, tip).clicked() {
                        app.set_tool(tool);
                    }
                }
                if ui
                    .small_button("Flatten Bitmap")
                    .on_hover_text("Rasterize selection into editable RGBA Bitmap")
                    .clicked()
                {
                    app.flatten_selected_to_bitmap();
                }
            });
            ui.separator();
            ui.label(RichText::new("ADJUST COLORS ▾").small().color(p.text_dim));
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
                for (cmd, label) in [
                    ("brighten", "Brighten +"),
                    ("darken", "Darken -"),
                    ("contrast", "Contrast +"),
                    ("invert", "Invert"),
                    ("grayscale", "Grayscale"),
                    ("sepia", "Sepia"),
                ] {
                    if ui.small_button(label).clicked() {
                        app.apply_color_command(cmd);
                    }
                }
            });
            ui.label(
                RichText::new("FILTERS & TRANSFORM ▾")
                    .small()
                    .color(p.text_dim),
            );
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
                if ui.small_button("Blur Pass").clicked() {
                    app.run_bitmap_filter(Tool::Blur);
                }
                if ui.small_button("Sharpen Pass").clicked() {
                    app.run_bitmap_filter(Tool::Sharpen);
                }
                if ui.small_button("Flip H").clicked() {
                    app.flip_horizontal();
                }
                if ui.small_button("Flip V").clicked() {
                    app.flip_vertical();
                }
                if ui.small_button("Rot 90°").clicked() {
                    app.rotate_90();
                }
            });
        }
        2 => {
            // Auto Shapes — uniform 2-column CS6 grid with vector shape glyph + label
            ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
            for pair in AutoShapeKind::ALL.chunks(2) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
                    for kind in pair {
                        let (r, resp) =
                            ui.allocate_exact_size(Vec2::new(136.0, 20.0), Sense::click());
                        let painter = ui.painter();
                        painter.rect_filled(
                            r,
                            0.0,
                            if resp.hovered() { p.well } else { p.panel_alt },
                        );
                        painter.rect_stroke(
                            r,
                            0.0,
                            Stroke::new(1.0, if resp.hovered() { p.accent } else { p.stroke }),
                            egui::StrokeKind::Middle,
                        );
                        let ic_r = Rect::from_min_size(
                            pos2(r.left() + 3.0, r.top() + 2.0),
                            Vec2::splat(16.0),
                        );
                        let shape_ic = match kind {
                            AutoShapeKind::RoundedRect | AutoShapeKind::ChamferRect => {
                                icons::Icon::Rounded
                            }
                            AutoShapeKind::Star | AutoShapeKind::Gear => icons::Icon::Star,
                            AutoShapeKind::Polygon => icons::Icon::Polygon,
                            AutoShapeKind::Pie | AutoShapeKind::Donut | AutoShapeKind::Spiral => {
                                icons::Icon::Ellipse
                            }
                            _ => icons::Icon::Pen,
                        };
                        icons::draw(painter, ic_r, shape_ic, p.accent, 1.0);
                        painter.text(
                            pos2(ic_r.right() + 4.0, r.center().y),
                            Align2::LEFT_CENTER,
                            kind.label(),
                            FontId::proportional(10.5),
                            p.text,
                        );
                        if resp
                            .on_hover_text(format!("Insert {} onto canvas", kind.label()))
                            .clicked()
                        {
                            app.insert_auto_shape(*kind);
                        }
                    }
                });
            }
        }
        3 => {
            // Document Library & Symbols
            if app.doc.symbols.is_empty() {
                ui.label(
                    RichText::new("No symbols in Document Library yet.")
                        .small()
                        .color(p.text_dim),
                );
            } else {
                for sym in app.doc.symbols.clone() {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(format!("◊ {} ({})", sym.name, sym.kind)).small());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("Place Instance").clicked() {
                                app.place_symbol_instance(sym.id);
                            }
                        });
                    });
                }
            }
            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(
                        app.selected.is_some(),
                        egui::Button::new("+ Convert Selection to Symbol (F8)").small(),
                    )
                    .clicked()
                {
                    app.convert_selection_to_symbol();
                }
            });
        }
        _ => {
            // CSS Properties & URL panel
            if let Some(obj) = app.selected_object() {
                let css = crate::formats::generate_css_for_object(&obj);
                egui::ScrollArea::vertical()
                    .id_salt("library_panel_scroll")
                    .max_height(88.0)
                    .show(ui, |ui| {
                        ui.code(&css);
                    });
                if ui.small_button("Copy CSS to Clipboard").clicked() {
                    app.ctx.copy_text(css);
                    app.status = "CSS properties copied to system clipboard".to_string();
                }
            } else {
                ui.label(
                    RichText::new("Select an object to inspect its live CSS3 rules.")
                        .small()
                        .color(p.text_dim),
                );
                for url in app.doc.urls.clone() {
                    ui.label(RichText::new(format!("-> {url}")).small());
                }
            }
        }
    }
}

/// The Fireworks CS6 Welcome Splash Screen card (`screenshots/Fireworks-Tab_View.jpg`).
pub fn start_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let width = 610.0;
    let height = 365.0;
    let (panel_rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

    let painter = ui.painter();
    painter.rect_filled(panel_rect, 0.0, p.welcome);
    painter.rect_stroke(
        panel_rect,
        0.0,
        Stroke::new(1.0, p.stroke),
        egui::StrokeKind::Middle,
    );

    // 1. Top CS6 Welcome Banner (76px tall, textured diagonal stripes + Fc logo)
    let title_bar = Rect::from_min_size(panel_rect.left_top(), Vec2::new(width, 76.0));
    painter.rect_filled(title_bar, 0.0, p.banner);
    // Subtle diagonal CS6 banner texture lines
    for i in 0..24 {
        let x0 = title_bar.left() + 180.0 + i as f32 * 18.0;
        painter.line_segment(
            [
                pos2(x0.min(title_bar.right()), title_bar.top()),
                pos2(
                    (x0 - 38.0).clamp(title_bar.left(), title_bar.right()),
                    title_bar.bottom(),
                ),
            ],
            Stroke::new(1.0, Color32::from_white_alpha(10)),
        );
    }

    let mut text_left = title_bar.left() + 20.0;
    if let Some(tex) = &app.logo_tex {
        let logo_r = Rect::from_min_size(
            pos2(title_bar.left() + 18.0, title_bar.top() + 12.0),
            Vec2::splat(44.0),
        );
        painter.image(
            tex.id(),
            logo_r,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        text_left = logo_r.right() + 10.0;
    }
    painter.text(
        pos2(text_left, title_bar.top() + 26.0),
        Align2::LEFT_CENTER,
        "Fc",
        FontId::proportional(32.0),
        p.banner_text,
    );
    painter.text(
        pos2(title_bar.left() + 20.0, title_bar.top() + 58.0),
        Align2::LEFT_CENTER,
        "Adobe® Fireworks® CS6 Parity · FireCraft Studio",
        FontId::proportional(12.5),
        p.banner_text,
    );

    let close_btn = Rect::from_min_size(
        pos2(panel_rect.right() - 30.0, title_bar.top() + 8.0),
        Vec2::new(22.0, 20.0),
    );
    let close_resp = ui.interact(
        close_btn,
        ui.make_persistent_id("welcome-close"),
        Sense::click(),
    );
    painter.rect_filled(
        close_btn,
        0.0,
        if close_resp.hovered() {
            p.accent.gamma_multiply(0.45)
        } else {
            Color32::from_black_alpha(90)
        },
    );
    painter.text(
        close_btn.center(),
        Align2::CENTER_CENTER,
        "✕",
        FontId::proportional(11.0),
        Color32::WHITE,
    );
    if close_resp.on_hover_text("Close Welcome Screen").clicked() {
        app.show_welcome = false;
    }

    let draw_fc_mini_badge = |pt: egui::Pos2| {
        let b = Rect::from_min_size(pt, Vec2::splat(14.0));
        painter.rect_filled(b, 0.0, Color32::from_rgb(36, 34, 20));
        painter.rect_stroke(
            b,
            0.0,
            Stroke::new(1.0, p.banner_text),
            egui::StrokeKind::Middle,
        );
        painter.text(
            b.center(),
            Align2::CENTER_CENTER,
            "Fc",
            FontId::proportional(8.5),
            p.banner_text,
        );
    };

    // 2. Main 2-Column Body: "Open a Recent Item" | "Create New" + "Extend"
    let body_top = title_bar.bottom() + 14.0;
    let body_bottom = panel_rect.bottom() - 88.0;
    let mid_x = panel_rect.center().x;
    let col1_x = panel_rect.left() + 24.0;
    let col2_x = mid_x + 18.0;

    // Vertical divider line between columns
    painter.line_segment(
        [pos2(mid_x, body_top - 4.0), pos2(mid_x, body_bottom - 6.0)],
        Stroke::new(1.0, p.stroke),
    );

    painter.text(
        pos2(col1_x, body_top),
        Align2::LEFT_TOP,
        "Open a Recent Item",
        FontId::proportional(13.0),
        p.text,
    );
    for (idx, (label, is_folder)) in [
        ("Showcase_1080p_UI_Kit.fw.png", false),
        ("Web_Hero_Banner.fw.png", false),
        ("Amortyx_Logo.fw.png", false),
        ("Open…", true),
    ]
    .iter()
    .enumerate()
    {
        let r = Rect::from_min_size(
            pos2(col1_x, body_top + 24.0 + idx as f32 * 24.0),
            Vec2::new(250.0, 20.0),
        );
        let resp = ui.interact(
            r,
            ui.make_persistent_id(("welcome-recent", idx)),
            Sense::click(),
        );
        if resp.hovered() {
            painter.rect_filled(r, 0.0, p.accent.gamma_multiply(0.22));
        }
        let ic_r = Rect::from_min_size(pos2(r.left() + 2.0, r.top() + 3.0), Vec2::splat(14.0));
        if *is_folder {
            icons::draw(painter, ic_r, icons::Icon::Folder, p.banner_text, 1.1);
        } else {
            icons::draw(painter, ic_r, icons::Icon::New, p.text_dim, 1.0);
        }
        painter.text(
            pos2(ic_r.right() + 6.0, r.center().y),
            Align2::LEFT_CENTER,
            *label,
            FontId::proportional(11.5),
            p.text,
        );
        if resp.clicked() {
            app.show_welcome = false;
            if *is_folder {
                app.begin_file_dialog(FileMode::Open);
            } else {
                app.load_demo_showcase();
            }
        }
    }

    painter.text(
        pos2(col2_x, body_top),
        Align2::LEFT_TOP,
        "Create New",
        FontId::proportional(13.0),
        p.text,
    );
    for (idx, label) in ["FireCraft Document (PNG)", "From Template (1080p Showcase)"]
        .iter()
        .enumerate()
    {
        let r = Rect::from_min_size(
            pos2(col2_x, body_top + 24.0 + idx as f32 * 24.0),
            Vec2::new(255.0, 20.0),
        );
        let resp = ui.interact(
            r,
            ui.make_persistent_id(("welcome-create", idx)),
            Sense::click(),
        );
        if resp.hovered() {
            painter.rect_filled(r, 0.0, p.accent.gamma_multiply(0.22));
        }
        draw_fc_mini_badge(pos2(r.left() + 2.0, r.top() + 3.0));
        painter.text(
            pos2(r.left() + 22.0, r.center().y),
            Align2::LEFT_CENTER,
            *label,
            FontId::proportional(11.5),
            p.text,
        );
        if resp.clicked() {
            app.show_welcome = false;
            if idx == 0 {
                app.show_new_dialog = true;
            } else {
                app.load_demo_showcase();
            }
        }
    }

    let extend_top = body_top + 82.0;
    painter.text(
        pos2(col2_x, extend_top),
        Align2::LEFT_TOP,
        "Extend",
        FontId::proportional(13.0),
        p.text,
    );
    for (idx, label) in [
        "FireCraft Exchange (MCP / CLI) »",
        "Command Palette (Ctrl+K) »",
    ]
    .iter()
    .enumerate()
    {
        let r = Rect::from_min_size(
            pos2(col2_x, extend_top + 22.0 + idx as f32 * 22.0),
            Vec2::new(255.0, 19.0),
        );
        let resp = ui.interact(
            r,
            ui.make_persistent_id(("welcome-extend", idx)),
            Sense::click(),
        );
        if resp.hovered() {
            painter.rect_filled(r, 0.0, p.accent.gamma_multiply(0.22));
        }
        let ic_r = Rect::from_min_size(pos2(r.left() + 2.0, r.top() + 2.5), Vec2::splat(14.0));
        icons::draw(painter, ic_r, icons::Icon::Group, p.accent, 1.0);
        painter.text(
            pos2(r.left() + 22.0, r.center().y),
            Align2::LEFT_CENTER,
            *label,
            FontId::proportional(11.0),
            p.text,
        );
        if resp.clicked() {
            app.show_welcome = false;
            if idx == 0 {
                app.show_mcp_dialog = true;
            } else {
                app.show_command_palette = true;
            }
        }
    }

    // 3. Lower CS6 Resources & Brand Badge Section (`screenshots/Fireworks-Tab_View.jpg`)
    painter.line_segment(
        [
            pos2(panel_rect.left() + 8.0, body_bottom),
            pos2(panel_rect.right() - 8.0, body_bottom),
        ],
        Stroke::new(1.0, p.stroke),
    );
    painter.line_segment(
        [
            pos2(mid_x, body_bottom + 6.0),
            pos2(mid_x, panel_rect.bottom() - 6.0),
        ],
        Stroke::new(1.0, p.stroke),
    );

    for (idx, label) in ["Getting Started »", "New Features »", "Resources »"]
        .iter()
        .enumerate()
    {
        let r = Rect::from_min_size(
            pos2(col1_x, body_bottom + 8.0 + idx as f32 * 23.0),
            Vec2::new(220.0, 19.0),
        );
        let resp = ui.interact(
            r,
            ui.make_persistent_id(("welcome-links", idx)),
            Sense::click(),
        );
        if resp.hovered() {
            painter.rect_filled(r, 0.0, p.accent.gamma_multiply(0.22));
        }
        draw_fc_mini_badge(pos2(r.left() + 2.0, r.top() + 2.5));
        painter.text(
            pos2(r.left() + 22.0, r.center().y),
            Align2::LEFT_CENTER,
            *label,
            FontId::proportional(11.0),
            p.text,
        );
        if resp.clicked() {
            app.show_welcome = false;
            match idx {
                0 => app.load_demo_showcase(),
                1 => app.show_command_palette = true,
                _ => app.show_about_dialog = true,
            }
        }
    }

    // Bottom-right CS6 square Fc Emblem Badge (`screenshots/Fireworks-Tab_View.jpg`)
    let badge_r = Rect::from_min_size(pos2(col2_x + 4.0, body_bottom + 18.0), Vec2::splat(42.0));
    painter.rect_filled(badge_r, 0.0, Color32::from_rgb(36, 34, 20));
    painter.rect_stroke(
        badge_r,
        0.0,
        Stroke::new(2.0, p.banner_text),
        egui::StrokeKind::Middle,
    );
    painter.text(
        badge_r.center(),
        Align2::CENTER_CENTER,
        "Fc",
        FontId::proportional(20.0),
        p.banner_text,
    );
    if let Some(tex) = &app.logo_tex {
        let flame_r = Rect::from_min_size(
            pos2(badge_r.right() + 12.0, body_bottom + 15.0),
            Vec2::splat(48.0),
        );
        painter.image(
            tex.id(),
            flame_r,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }

    let _ = Label::new(RichText::new("").small());
}

// ============================================================================
// Dynamic CS6 Panel Group Container & 5 New CS6 Panels
// ============================================================================

/// One panel registry row: the host chrome needs the tab identity, a default body
/// height, and a renderer. New panels are added here instead of in a second match.
pub struct PanelDescriptor {
    pub id: PanelId,
    pub default_height: f32,
    pub render: fn(&mut Ui, &mut FireCraft),
}

fn pages_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.layers_tab = 0;
    pages_panel(ui, app);
}
fn states_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.layers_tab = 1;
    pages_panel(ui, app);
}
fn layers_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.layers_tab = 2;
    pages_panel(ui, app);
}
fn special_chars_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 0;
    library_panel(ui, app);
}
fn image_editing_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 1;
    library_panel(ui, app);
}
fn auto_shapes_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 2;
    library_panel(ui, app);
}
fn document_library_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 3;
    library_panel(ui, app);
}
fn css_properties_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 4;
    library_panel(ui, app);
}

static PANELS: &[PanelDescriptor] = &[
    PanelDescriptor {
        id: PanelId::Optimize,
        default_height: 220.0,
        render: optimize_panel,
    },
    PanelDescriptor {
        id: PanelId::History,
        default_height: 148.0,
        render: history_panel,
    },
    PanelDescriptor {
        id: PanelId::Align,
        default_height: 152.0,
        render: align_panel,
    },
    PanelDescriptor {
        id: PanelId::Path,
        default_height: 220.0,
        render: paths_panel,
    },
    PanelDescriptor {
        id: PanelId::Pages,
        default_height: 188.0,
        render: pages_tab,
    },
    PanelDescriptor {
        id: PanelId::States,
        default_height: 188.0,
        render: states_tab,
    },
    PanelDescriptor {
        id: PanelId::Layers,
        default_height: 188.0,
        render: layers_tab,
    },
    PanelDescriptor {
        id: PanelId::Styles,
        default_height: 118.0,
        render: styles_panel,
    },
    PanelDescriptor {
        id: PanelId::ColorPalette,
        default_height: 186.0,
        render: color_panel,
    },
    PanelDescriptor {
        id: PanelId::ColorMixer,
        default_height: 186.0,
        render: color_mixer_panel,
    },
    PanelDescriptor {
        id: PanelId::Swatches,
        default_height: 134.0,
        render: swatches_panel,
    },
    PanelDescriptor {
        id: PanelId::Info,
        default_height: 118.0,
        render: info_panel,
    },
    PanelDescriptor {
        id: PanelId::SpecialChars,
        default_height: 118.0,
        render: special_chars_tab,
    },
    PanelDescriptor {
        id: PanelId::ImageEditing,
        default_height: 118.0,
        render: image_editing_tab,
    },
    PanelDescriptor {
        id: PanelId::AutoShapes,
        default_height: 118.0,
        render: auto_shapes_tab,
    },
    PanelDescriptor {
        id: PanelId::AutoShapeProps,
        default_height: 118.0,
        render: auto_shape_props_panel,
    },
    PanelDescriptor {
        id: PanelId::DocumentLibrary,
        default_height: 118.0,
        render: document_library_tab,
    },
    PanelDescriptor {
        id: PanelId::CommonLibrary,
        default_height: 118.0,
        render: common_library_panel,
    },
    PanelDescriptor {
        id: PanelId::CssProperties,
        default_height: 118.0,
        render: css_properties_tab,
    },
    PanelDescriptor {
        id: PanelId::Url,
        default_height: 118.0,
        render: url_panel,
    },
    PanelDescriptor {
        id: PanelId::Behaviors,
        default_height: 118.0,
        render: behaviors_panel,
    },
];

pub fn panel_descriptor(id: PanelId) -> Option<&'static PanelDescriptor> {
    PANELS.iter().find(|panel| panel.id == id)
}

pub fn default_panel_height(id: PanelId) -> f32 {
    panel_descriptor(id).map_or(118.0, |panel| panel.default_height)
}

pub fn render_panel_by_id(ui: &mut Ui, app: &mut FireCraft, id: PanelId) {
    if let Some(panel) = panel_descriptor(id) {
        (panel.render)(ui, app);
    } else {
        ui.label(RichText::new(format!("No renderer registered for {}", id.title())).small());
    }
}

fn cs6_color_button(
    ui: &mut Ui,
    id_salt: &str,
    p: crate::theme::Palette,
    color: &mut Color32,
    has_color: Option<&mut bool>,
) -> bool {
    let is_none = has_color.as_ref().is_some_and(|h| !**h);
    let (well_r, well_resp) = ui.allocate_exact_size(Vec2::new(26.0, 18.0), Sense::click());
    let painter = ui.painter();
    if is_none || color.a() < 255 {
        // Checkerboard background
        for cy in 0..3 {
            for cx in 0..4 {
                let c = if (cx + cy) % 2 == 0 {
                    Color32::from_gray(230)
                } else {
                    Color32::from_gray(180)
                };
                painter.rect_filled(
                    Rect::from_min_size(
                        pos2(
                            well_r.left() + cx as f32 * 6.5,
                            well_r.top() + cy as f32 * 6.0,
                        ),
                        Vec2::new(6.5, 6.0),
                    ),
                    0.0,
                    c,
                );
            }
        }
    }
    if is_none {
        painter.rect_filled(well_r.shrink(1.0), 0.0, Color32::from_white_alpha(200));
        painter.line_segment(
            [
                pos2(well_r.left() + 2.0, well_r.bottom() - 2.0),
                pos2(well_r.right() - 2.0, well_r.top() + 2.0),
            ],
            Stroke::new(1.5, Color32::from_rgb(200, 36, 36)),
        );
    } else {
        painter.rect_filled(well_r.shrink(1.0), 0.0, *color);
    }
    painter.rect_stroke(
        well_r,
        0.0,
        Stroke::new(
            1.0,
            if well_resp.hovered() {
                p.accent
            } else {
                p.stroke
            },
        ),
        egui::StrokeKind::Middle,
    );
    // CS6 bottom-right dropdown triangle
    let br = well_r.right_bottom();
    painter.add(egui::Shape::convex_polygon(
        vec![
            pos2(br.x - 1.5, br.y - 1.5),
            pos2(br.x - 6.0, br.y - 1.5),
            pos2(br.x - 1.5, br.y - 6.0),
        ],
        p.text,
        Stroke::NONE,
    ));

    let open_id = ui.make_persistent_id((id_salt, "open"));
    let wheel_id = ui.make_persistent_id((id_salt, "wheel"));
    let hex_id = ui.make_persistent_id((id_salt, "hex"));
    let mut is_open: bool = ui.ctx().data(|d| d.get_temp(open_id).unwrap_or(false));
    let mut just_opened = false;
    if well_resp
        .on_hover_text("Click to open CS6 Color Swatch & Transparency Picker")
        .clicked()
    {
        is_open = !is_open;
        just_opened = is_open;
        if is_open {
            ui.ctx()
                .data_mut(|d| d.insert_temp(hex_id, colors::hex_of(*color)));
        }
    }

    let mut changed = false;
    if is_open {
        let mut show_wheel: bool = ui.ctx().data(|d| d.get_temp(wheel_id).unwrap_or(false));
        let screen = ui.ctx().content_rect();
        let pop_w = 224.0;
        let pop_h = if show_wheel { 390.0 } else { 254.0 };
        let pop_pos = pos2(
            well_r
                .left()
                .clamp(screen.left() + 4.0, (screen.right() - pop_w - 8.0).max(4.0)),
            if well_r.bottom() + pop_h + 6.0 <= screen.bottom() {
                well_r.bottom() + 3.0
            } else {
                (well_r.top() - pop_h - 4.0).max(screen.top() + 4.0)
            },
        );
        let mut close_pop = ui.input(|i| i.key_pressed(egui::Key::Escape));
        let mut set_none = false;
        let area_resp = egui::Area::new(ui.make_persistent_id((id_salt, "cs6_swatch_popover")))
            .fixed_pos(pop_pos)
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                egui::Frame::menu(ui.style())
                    .fill(p.panel)
                    .stroke(Stroke::new(1.0, p.stroke))
                    .inner_margin(egui::Margin::same(6))
                    .show(ui, |ui| {
                        ui.set_width(pop_w);
                        // 1. CS6 18-column Swatch Cube Matrix
                        let swatches = colors::swatch_grid();
                        let cols = 18usize;
                        let cell = 11.0;
                        let rows = swatches.len().div_ceil(cols);
                        let (grid_r, grid_resp) = ui.allocate_exact_size(
                            Vec2::new(cols as f32 * cell, rows as f32 * cell),
                            Sense::click(),
                        );
                        let gp = ui.painter();
                        gp.rect_filled(grid_r, 0.0, Color32::BLACK);
                        for (idx, &sc) in swatches.iter().enumerate() {
                            let gx = idx % cols;
                            let gy = idx / cols;
                            let cr = Rect::from_min_size(
                                pos2(
                                    grid_r.left() + gx as f32 * cell + 0.5,
                                    grid_r.top() + gy as f32 * cell + 0.5,
                                ),
                                Vec2::splat(cell - 1.0),
                            );
                            gp.rect_filled(cr, 0.0, sc);
                        }
                        if grid_resp.clicked()
                            && let Some(pos) = grid_resp.interact_pointer_pos()
                        {
                            let gx = ((pos.x - grid_r.left()) / cell).floor() as usize;
                            let gy = ((pos.y - grid_r.top()) / cell).floor() as usize;
                            let idx = gy * cols + gx;
                            if let Some(&picked) = swatches.get(idx) {
                                *color = Color32::from_rgba_unmultiplied(
                                    picked.r(),
                                    picked.g(),
                                    picked.b(),
                                    color.a().max(if is_none { 255 } else { color.a() }),
                                );
                                changed = true;
                                close_pop = true;
                            }
                        }

                        ui.add_space(5.0);
                        // 2. Preview + HEX + RGB + Opacity %
                        ui.horizontal(|ui| {
                            let (prev_r, _) =
                                ui.allocate_exact_size(Vec2::new(28.0, 22.0), Sense::hover());
                            ui.painter().rect_filled(prev_r, 0.0, *color);
                            ui.painter().rect_stroke(
                                prev_r,
                                0.0,
                                Stroke::new(1.0, p.stroke),
                                egui::StrokeKind::Middle,
                            );
                            ui.vertical(|ui| {
                                ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new("HEX:").small().color(p.text_dim));
                                    let mut hex_str: String = ui.ctx().data(|d| {
                                        d.get_temp(hex_id).unwrap_or_else(|| colors::hex_of(*color))
                                    });
                                    let te = ui.add(
                                        egui::TextEdit::singleline(&mut hex_str)
                                            .desired_width(68.0)
                                            .font(FontId::monospace(10.5)),
                                    );
                                    if te.changed()
                                        && let Some(parsed) = colors::from_hex(&hex_str)
                                    {
                                        *color = Color32::from_rgba_unmultiplied(
                                            parsed.r(),
                                            parsed.g(),
                                            parsed.b(),
                                            color.a(),
                                        );
                                        changed = true;
                                    }
                                    ui.ctx().data_mut(|d| d.insert_temp(hex_id, hex_str));
                                    if ui.small_button("📋").on_hover_text("Copy HEX").clicked() {
                                        ui.ctx().copy_text(colors::hex_of(*color));
                                    }
                                });
                                ui.horizontal(|ui| {
                                    let (mut r, mut g, mut b) =
                                        (color.r() as i32, color.g() as i32, color.b() as i32);
                                    ui.label(RichText::new("RGB:").small().color(p.text_dim));
                                    let rc = ui
                                        .add(egui::DragValue::new(&mut r).range(0..=255).speed(1.0))
                                        .changed();
                                    let gc = ui
                                        .add(egui::DragValue::new(&mut g).range(0..=255).speed(1.0))
                                        .changed();
                                    let bc = ui
                                        .add(egui::DragValue::new(&mut b).range(0..=255).speed(1.0))
                                        .changed();
                                    if rc || gc || bc {
                                        *color = Color32::from_rgba_unmultiplied(
                                            r as u8,
                                            g as u8,
                                            b as u8,
                                            color.a(),
                                        );
                                        ui.ctx().data_mut(|d| {
                                            d.insert_temp(hex_id, colors::hex_of(*color))
                                        });
                                        changed = true;
                                    }
                                });
                            });
                        });

                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Opacity:").small().color(p.text_dim));
                            let mut pct = (color.a() as f32 / 255.0 * 100.0).round();
                            if ui
                                .add(egui::Slider::new(&mut pct, 0.0..=100.0).suffix("%"))
                                .changed()
                            {
                                let a = (pct / 100.0 * 255.0).round().clamp(0.0, 255.0) as u8;
                                *color = Color32::from_rgba_unmultiplied(
                                    color.r(),
                                    color.g(),
                                    color.b(),
                                    a,
                                );
                                changed = true;
                            }
                        });

                        ui.separator();
                        ui.horizontal(|ui| {
                            if has_color.is_some()
                                && ui
                                    .small_button("╱ Transparent")
                                    .on_hover_text("Set to Transparent / No Fill")
                                    .clicked()
                            {
                                set_none = true;
                                close_pop = true;
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .selectable_label(show_wheel, "🌈 Wheel")
                                        .on_hover_text("Toggle full HSV/RGBA Color Wheel")
                                        .clicked()
                                    {
                                        show_wheel = !show_wheel;
                                    }
                                },
                            );
                        });
                        if show_wheel {
                            ui.add_space(4.0);
                            if ui.color_edit_button_srgba(color).changed() {
                                ui.ctx()
                                    .data_mut(|d| d.insert_temp(hex_id, colors::hex_of(*color)));
                                changed = true;
                            }
                        }
                    });
            });
        ui.ctx().data_mut(|d| d.insert_temp(wheel_id, show_wheel));
        if set_none && let Some(h) = has_color {
            *h = false;
            changed = true;
        } else if changed && let Some(h) = has_color {
            *h = true;
        }
        if !just_opened
            && ui.input(|i| {
                i.pointer.any_pressed()
                    && i.pointer.interact_pos().is_some_and(|pp| {
                        !area_resp.response.rect.contains(pp) && !well_r.contains(pp)
                    })
            })
        {
            close_pop = true;
        }
        if close_pop {
            is_open = false;
        }
    }
    ui.ctx().data_mut(|d| d.insert_temp(open_id, is_open));
    changed
}

pub fn color_mixer_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.horizontal(|ui| {
        ui.label(RichText::new("Fill:").small().color(p.text_dim));
        let mut fill_c = app.fill;
        if cs6_color_button(ui, "mixer_fill_swatch", p, &mut fill_c, None) {
            app.fill = fill_c;
            app.set_selected_fill(Some(fill_c));
        }
        ui.add_space(6.0);
        ui.label(RichText::new("Stroke:").small().color(p.text_dim));
        let mut stroke_c = app.stroke;
        if cs6_color_button(ui, "mixer_stroke_swatch", p, &mut stroke_c, None) {
            app.stroke = stroke_c;
            app.set_selected_stroke(Some(stroke_c));
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(colors::hex_of(app.fill)).monospace().small());
        });
    });
    ui.add_space(4.0);
    let mut r = app.fill.r() as i32;
    let mut g = app.fill.g() as i32;
    let mut b = app.fill.b() as i32;
    let mut changed = false;
    for (label, val) in [("R:", &mut r), ("G:", &mut g), ("B:", &mut b)] {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).small().color(p.text_dim));
            if ui
                .add(egui::Slider::new(val, 0..=255).show_value(true))
                .changed()
            {
                changed = true;
            }
        });
    }
    if changed {
        let c = Color32::from_rgb(r as u8, g as u8, b as u8);
        app.fill = c;
        app.set_selected_fill(Some(c));
    }
    ui.add_space(4.0);
    let (bar_rect, bar_resp) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), 16.0),
        Sense::click_and_drag(),
    );
    let painter = ui.painter();
    let steps = 24;
    let step_w = bar_rect.width() / steps as f32;
    for i in 0..steps {
        let hue = i as f32 / steps as f32;
        let c: Color32 = egui::ecolor::Hsva::new(hue, 0.9, 0.95, 1.0).into();
        let r = Rect::from_min_size(
            pos2(bar_rect.left() + i as f32 * step_w, bar_rect.top()),
            Vec2::new(step_w + 0.5, bar_rect.height()),
        );
        painter.rect_filled(r, 0.0, c);
    }
    painter.rect_stroke(
        bar_rect,
        0.0,
        Stroke::new(1.0, p.stroke),
        egui::StrokeKind::Inside,
    );
    if (bar_resp.clicked() || bar_resp.dragged())
        && let Some(pos) = bar_resp.interact_pointer_pos()
    {
        let t = ((pos.x - bar_rect.left()) / bar_rect.width()).clamp(0.0, 1.0);
        let c: Color32 = egui::ecolor::Hsva::new(t, 0.9, 0.95, 1.0).into();
        app.fill = c;
        app.set_selected_fill(Some(c));
    }
}

pub fn auto_shape_props_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let Some(obj) = app.selected_object() else {
        ui.label(
            RichText::new("Select an Auto Shape (Rect, Polygon, Star) to edit live geometry.")
                .small()
                .color(p.text_dim),
        );
        return;
    };
    ui.label(
        RichText::new(format!("Shape: {}", obj.name))
            .strong()
            .small(),
    );
    ui.add_space(4.0);
    match obj.kind {
        Shape::Rect { corner_radius, .. } => {
            let mut r = corner_radius;
            ui.horizontal(|ui| {
                ui.label(RichText::new("Corner Radius:").small());
                if ui
                    .add(
                        egui::DragValue::new(&mut r)
                            .range(0.0..=120.0)
                            .suffix(" px"),
                    )
                    .changed()
                {
                    app.set_selected_corner_radius(r);
                }
            });
        }
        Shape::Polygon {
            sides, star_inner, ..
        } => {
            let mut s = sides;
            let mut inner = star_inner;
            let mut changed = false;
            ui.horizontal(|ui| {
                ui.label(RichText::new("Sides:").small());
                if ui.add(egui::DragValue::new(&mut s).range(3..=36)).changed() {
                    changed = true;
                }
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new("Inner Ratio:").small());
                if ui
                    .add(egui::Slider::new(&mut inner, 0.1..=1.0).fixed_decimals(2))
                    .changed()
                {
                    changed = true;
                }
            });
            if changed {
                app.set_selected_polygon_params(s, inner);
            }
        }
        _ => {
            let mut w = obj.width;
            ui.horizontal(|ui| {
                ui.label(RichText::new("Stroke Width:").small());
                if ui
                    .add(egui::DragValue::new(&mut w).range(0.0..=64.0).suffix(" px"))
                    .changed()
                {
                    app.set_selected_width(w);
                }
            });
        }
    }
}

pub fn common_library_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.label(
        RichText::new("Click a UI component to insert onto canvas:")
            .small()
            .color(p.text_dim),
    );
    ui.add_space(4.0);
    let items = [
        "Primary Button",
        "Checkbox (Checked)",
        "Radio Button",
        "Text Input Field",
        "Dropdown ComboBox",
        "Toggle Switch",
        "Modal Card",
        "Progress Bar",
        "Navigation Bar",
        "Badge Pill",
    ];
    egui::ScrollArea::vertical()
        .id_salt("common_library_scroll")
        .show(ui, |ui| {
            for item in items {
                if ui.selectable_label(false, format!("+ {item}")).clicked() {
                    app.insert_common_library_item(item);
                }
            }
        });
}

pub fn url_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.horizontal(|ui| {
        ui.label(RichText::new("URL:").small().color(p.text_dim));
        ui.add(
            egui::TextEdit::singleline(&mut app.new_url_input)
                .desired_width(130.0)
                .hint_text("https://..."),
        );
        if ui
            .button("+")
            .on_hover_text("Assign URL to selection")
            .clicked()
        {
            let url = app.new_url_input.trim().to_string();
            if !url.is_empty() {
                app.set_selected_slice_meta(url.clone(), "Linked Slice".to_string());
                app.status = format!("Assigned URL: {url}");
            }
        }
    });
    ui.add_space(4.0);
    let presets = [
        "https://github.com/aliasfoxkde/FireCraft",
        "index.html",
        "about.html",
        "mailto:hello@firecraft.app",
    ];
    for preset in presets {
        if ui
            .selectable_label(app.new_url_input == preset, preset)
            .clicked()
        {
            app.new_url_input = preset.to_string();
            app.set_selected_slice_meta(preset.to_string(), "Linked Slice".to_string());
        }
    }
}

pub fn behaviors_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.horizontal(|ui| {
        ui.menu_button("+ Add", |ui| {
            for (ev, act, info) in [
                ("onMouseOver", "Simple Rollover", "State 2"),
                ("onMouseOver", "Swap Image", "image.png"),
                ("onClick", "Pop-up Menu", "Nav Menu"),
                ("onClick", "Set Nav Bar Image", "Down"),
                ("onMouseOver", "Set Text of Status Bar", "Ready"),
            ] {
                if ui.button(act).clicked() {
                    app.behaviors.push(crate::app::WebBehavior {
                        event: ev.to_string(),
                        action: act.to_string(),
                        info: info.to_string(),
                    });
                    ui.close();
                }
            }
        });
        if ui.button("- Remove").clicked() && !app.behaviors.is_empty() {
            app.behaviors.pop();
        }
    });
    ui.separator();
    if app.behaviors.is_empty() {
        ui.label(
            RichText::new("No web behaviors attached.")
                .small()
                .color(p.text_dim),
        );
    } else {
        for b in &app.behaviors {
            ui.horizontal(|ui| {
                ui.label(RichText::new(&b.event).small().color(p.accent));
                ui.label(RichText::new(&b.action).small());
                ui.label(RichText::new(&b.info).small().color(p.text_dim));
            });
        }
    }
}

fn tab_click_without_drag(resp: &egui::Response) -> bool {
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

pub fn panel_group_widget(ui: &mut Ui, app: &mut FireCraft, group_idx: usize, is_floating: bool) {
    let Some(group) = app.panel_groups.get(group_idx).cloned() else {
        return;
    };
    if group.tabs.is_empty() {
        return;
    }
    let p = app.palette;
    let active_idx = group.active_tab.min(group.tabs.len() - 1);
    let active_panel = group.tabs[active_idx];
    let other_groups: Vec<(u64, String)> = app
        .panel_groups
        .iter()
        .filter(|g| g.id != group.id && !g.tabs.is_empty())
        .map(|g| {
            let title = g
                .tabs
                .get(g.active_tab.min(g.tabs.len() - 1))
                .map(|t| t.title().to_string())
                .unwrap_or_else(|| "Panel".to_string());
            (g.id, title)
        })
        .collect();

    egui::Frame::NONE
        .fill(p.panel)
        .stroke(Stroke::new(1.0, p.stroke))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let header_h = 22.0;
            let (header_rect, header_resp) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), header_h),
                Sense::click_and_drag(),
            );
            app.panel_title_rects.push((group.id, header_rect));

            let is_drop_target = (app.dragging_panel.is_some()
                || app.dragging_group.is_some_and(|gid| gid != group.id))
                && ui
                    .ctx()
                    .pointer_latest_pos()
                    .is_some_and(|pos| header_rect.contains(pos));
            ui.painter().rect_filled(
                header_rect,
                0.0,
                if is_drop_target {
                    p.accent.gamma_multiply(0.35)
                } else {
                    p.panel_alt
                },
            );
            ui.painter().line_segment(
                [header_rect.left_bottom(), header_rect.right_bottom()],
                Stroke::new(
                    if is_drop_target { 2.0 } else { 1.0 },
                    if is_drop_target { p.accent } else { p.stroke },
                ),
            );

            if header_resp.double_clicked()
                && let Some(g) = app.panel_groups.get_mut(group_idx)
            {
                g.collapsed = !g.collapsed;
            }
            if header_resp.drag_started() {
                app.dragging_group = Some(group.id);
            }
            if header_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }

            // Left collapse chevron
            let chev_r = Rect::from_min_size(header_rect.min, Vec2::new(18.0, header_h));
            let chev_resp = ui.interact(
                chev_r,
                ui.make_persistent_id(("pg_chev", group.id)),
                Sense::click(),
            );
            ui.painter().text(
                chev_r.center(),
                Align2::CENTER_CENTER,
                if group.collapsed { "▸" } else { "▾" },
                FontId::proportional(11.0),
                p.text_dim,
            );

            if chev_resp.clicked()
                && let Some(g) = app.panel_groups.get_mut(group_idx)
            {
                g.collapsed = !g.collapsed;
            }

            // Right-side header controls
            let ctrl_w = 56.0;
            let close_r = Rect::from_min_size(
                pos2(header_rect.right() - 18.0, header_rect.top()),
                Vec2::new(18.0, header_h),
            );
            let menu_r = Rect::from_min_size(
                pos2(header_rect.right() - 36.0, header_rect.top()),
                Vec2::new(18.0, header_h),
            );
            let dock_r = Rect::from_min_size(
                pos2(header_rect.right() - 54.0, header_rect.top()),
                Vec2::new(18.0, header_h),
            );

            ui.painter().text(
                close_r.center(),
                Align2::CENTER_CENTER,
                "×",
                FontId::proportional(12.0),
                p.text_dim,
            );
            if ui
                .interact(
                    close_r,
                    ui.make_persistent_id(("pg_close", group.id)),
                    Sense::click(),
                )
                .on_hover_text("Close active tab")
                .clicked()
            {
                app.close_panel(active_panel);
            }

            ui.painter().text(
                dock_r.center(),
                Align2::CENTER_CENTER,
                if is_floating { "📌" } else { "⧉" },
                FontId::proportional(10.0),
                p.text_dim,
            );
            if ui
                .interact(
                    dock_r,
                    ui.make_persistent_id(("pg_dock", group.id)),
                    Sense::click(),
                )
                .on_hover_text(if is_floating {
                    "Pin to Right Dock"
                } else {
                    "Undock to Floating Window"
                })
                .clicked()
            {
                if is_floating {
                    app.move_group_to_zone(group.id, DockZone::RightOuter, None);
                } else {
                    app.ungroup_panel_to_floating(
                        active_panel,
                        header_rect.left_top() - Vec2::new(260.0, 0.0),
                    );
                }
            }

            // Panel Options menu button
            let mut menu_ui = ui.new_child(egui::UiBuilder::new().max_rect(menu_r));
            menu_ui.menu_button(RichText::new("≡").small().color(p.text_dim), |ui| {
                if ui
                    .button(if group.collapsed {
                        "Expand Panel"
                    } else {
                        "Collapse Panel"
                    })
                    .clicked()
                {
                    if let Some(g) = app.panel_groups.get_mut(group_idx) {
                        g.collapsed = !g.collapsed;
                    }
                    ui.close();
                }
                if ui.button("Undock to Floating Window").clicked() {
                    app.ungroup_panel_to_floating(
                        active_panel,
                        header_rect.left_top() - Vec2::new(260.0, 0.0),
                    );
                    ui.close();
                }
                if ui.button("Pin to Primary Right Dock").clicked() {
                    app.move_group_to_zone(group.id, DockZone::RightOuter, None);
                    ui.close();
                }
                if ui.button("Pin to Inner-Right Dock (2-Col)").clicked() {
                    app.move_group_to_zone(group.id, DockZone::RightInner, None);
                    ui.close();
                }
                if ui.button("Pin to Left Dock").clicked() {
                    app.move_group_to_zone(group.id, DockZone::LeftInner, None);
                    ui.close();
                }
                if group.tabs.len() > 1 && ui.button("Ungroup Active Tab Below").clicked() {
                    app.ungroup_panel_below(active_panel);
                    ui.close();
                }
                if group.stack_parent.is_some() && ui.button("Detach from Window Stack").clicked() {
                    app.detach_stack(group.id);
                    ui.close();
                }
                ui.menu_button("Group With...", |ui| {
                    for (tid, ttitle) in &other_groups {
                        if ui.button(ttitle).clicked() {
                            app.group_panel_into(active_panel, *tid);
                            ui.close();
                        }
                    }
                });
                ui.separator();
                if ui.button("Collapse All Panels").clicked() {
                    app.collapse_all_panels(true);
                    ui.close();
                }
                if ui.button("Expand All Panels").clicked() {
                    app.collapse_all_panels(false);
                    ui.close();
                }
            });

            // Draggable Tabs. Narrow dock columns can hold many tabs; never draw labels on top
            // of each other. Fit as many as the minimum width allows, keep the active tab
            // visible, and expose every tab through an overflow menu.
            let tabs_area_w = (header_rect.width() - 18.0 - ctrl_w).max(40.0);
            let max_visible = ((tabs_area_w / 42.0).floor() as usize).max(1);
            let needs_overflow = group.tabs.len() > max_visible;
            let overflow_w = if needs_overflow { 20.0 } else { 0.0 };
            let visible_count = if needs_overflow {
                max_visible.saturating_sub(1).max(1)
            } else {
                group.tabs.len()
            };
            let tab_w = ((tabs_area_w - overflow_w) / visible_count as f32).clamp(24.0, 130.0);
            let mut shown: Vec<usize> = (0..visible_count).collect();
            if !shown.contains(&active_idx) {
                shown.pop();
                shown.push(active_idx);
                shown.sort_unstable();
            }
            for (slot, &ti) in shown.iter().enumerate() {
                let tab_id = group.tabs[ti];
                let tab_r = Rect::from_min_size(
                    pos2(
                        chev_r.right() + slot as f32 * tab_w,
                        header_rect.top() + 2.0,
                    ),
                    Vec2::new(tab_w - 1.0, header_h - 2.0),
                );
                let is_active = ti == active_idx;
                if is_active {
                    ui.painter().rect_filled(tab_r, 0.0, p.panel);
                }
                let label = if group.tabs.len() > 1 {
                    tab_id.short_tab()
                } else {
                    tab_id.title()
                };
                ui.painter()
                    .with_clip_rect(tab_r.shrink2(Vec2::new(2.0, 0.0)))
                    .text(
                        tab_r.center(),
                        Align2::CENTER_CENTER,
                        label,
                        FontId::proportional(10.5),
                        if is_active { p.text } else { p.text_dim },
                    );
                let tab_resp = ui.interact(
                    tab_r,
                    ui.make_persistent_id(("pg_tab", group.id, ti)),
                    Sense::click_and_drag(),
                );
                if tab_click_without_drag(&tab_resp)
                    && let Some(g) = app.panel_groups.get_mut(group_idx)
                {
                    g.active_tab = ti;
                    g.collapsed = false;
                    app.focus_canvas = true;
                }
                if tab_resp.drag_started() {
                    app.dragging_panel = Some(tab_id);
                }
            }

            if needs_overflow {
                let overflow_r = Rect::from_min_size(
                    pos2(
                        chev_r.right() + shown.len() as f32 * tab_w,
                        header_rect.top() + 2.0,
                    ),
                    Vec2::new(overflow_w, header_h - 2.0),
                );
                let mut overflow_ui = ui.new_child(egui::UiBuilder::new().max_rect(overflow_r));
                overflow_ui
                    .menu_button(RichText::new("»").small().color(p.text_dim), |ui| {
                        for (ti, &tab_id) in group.tabs.iter().enumerate() {
                            if ui
                                .selectable_label(ti == active_idx, tab_id.title())
                                .clicked()
                            {
                                if let Some(g) = app.panel_groups.get_mut(group_idx) {
                                    g.active_tab = ti;
                                    g.collapsed = false;
                                }
                                ui.close();
                            }
                        }
                    })
                    .response
                    .on_hover_text("All tabs in this panel group");
            }

            if !group.collapsed {
                let body_h = group.body_height;
                ui.allocate_ui_with_layout(
                    Vec2::new(ui.available_width(), body_h),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_min_height(body_h);
                        ui.set_max_height(body_h);
                        egui::Frame::NONE
                            .inner_margin(egui::Margin::symmetric(8, 6))
                            .show(ui, |ui| {
                                egui::ScrollArea::vertical()
                                    .id_salt(("pg_scroll", group.id, active_panel))
                                    .auto_shrink([false, false])
                                    .max_height((body_h - 12.0).max(40.0))
                                    .show(ui, |ui| {
                                        ui.push_id(("panel_body", group.id, active_panel), |ui| {
                                            render_panel_by_id(ui, app, active_panel);
                                        });
                                    });
                            });
                    },
                );

                // Bottom resize splitter bar
                let (split_r, split_resp) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 5.0), Sense::drag());
                ui.painter()
                    .rect_filled(split_r, 0.0, p.stroke.gamma_multiply(0.55));
                if split_resp.hovered() || split_resp.dragged() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
                }
                if split_resp.dragged()
                    && let Some(g) = app.panel_groups.get_mut(group_idx)
                {
                    g.body_height = (g.body_height + split_resp.drag_delta().y).clamp(72.0, 520.0);
                }
            }
        });
}

#[cfg(test)]
mod registry_tests {
    use super::*;

    #[test]
    fn every_panel_id_has_one_registry_entry() {
        for id in PanelId::ALL {
            let matches = PANELS.iter().filter(|panel| panel.id == id).count();
            assert_eq!(
                matches,
                1,
                "{} registry entries for {}",
                matches,
                id.title()
            );
        }
        assert_eq!(PANELS.len(), PanelId::ALL.len());
    }

    #[test]
    fn registry_height_is_the_public_default() {
        assert_eq!(default_panel_height(PanelId::Layers), 188.0);
        assert_eq!(default_panel_height(PanelId::Info), 118.0);
    }
}
