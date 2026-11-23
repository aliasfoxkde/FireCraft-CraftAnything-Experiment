//! The docked Fireworks CS6 panels:
//! - Left 2-column tool palette (Select, Bitmap, Vector, Web, Colors, View)
//! - Right stacked tab-groups:
//!   - Group 1: Optimize | History | Align | Path
//!   - Group 2: Pages | States | Layers
//!   - Group 3: Styles | Color Palette | Swatches | Info
//!   - Group 4: Special Characters | Image Editing | Auto Shapes | Document Library | CSS / URL
//! - Bottom dock: Properties & Symbol Properties inspector with Live Filters (+) stack.

use crate::app::{FileMode, FireCraft, Tool};
use crate::colors;
use crate::constants;
use crate::doc::{AutoShapeKind, BlendMode, LiveEffect, Obj, Shape};
use crate::icons;

use eframe::egui::{
    self, Align2, Color32, FontId, Frame, Grid, Label, Rect, RichText, Sense, Stroke, TextEdit, Ui,
    Vec2, pos2,
};

/// A titled, tabbed panel container matching the Fireworks CS6 dock look (`screenshots/Fireworks_Layout_Design.jpg`).
pub fn panel(
    ui: &mut Ui,
    title: &str,
    tabs: &[&str],
    tab_index: &mut usize,
    palette: crate::theme::Palette,
    body_height: Option<f32>,
    body: impl FnOnce(&mut Ui),
) {
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

            if tabs.is_empty() {
                painter.text(
                    pos2(bar.left() + 9.0, bar.center().y),
                    Align2::LEFT_CENTER,
                    title,
                    FontId::proportional(11.0),
                    palette.text,
                );
            } else {
                let mut x = bar.left() + 8.0;
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
}

type ToolSection = (&'static str, &'static [(icons::Icon, Tool, &'static str)]);

/// The tool palette down the left edge, grouped into Fireworks CS6's 6 sections.
pub fn tools_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let sections: [ToolSection; 6] = [
        (
            "Select",
            &[
                (icons::Icon::Select, Tool::Select, "Pointer Tool (V)"),
                (
                    icons::Icon::Subselect,
                    Tool::Subselect,
                    "Subselection Tool (A)",
                ),
                (
                    icons::Icon::Scale,
                    Tool::Scale,
                    "Scale / Transform Tool (Q)",
                ),
                (icons::Icon::Crop, Tool::Crop, "Crop Tool (C)"),
            ],
        ),
        (
            "Bitmap",
            &[
                (icons::Icon::Marquee, Tool::Marquee, "Marquee Tool (M)"),
                (icons::Icon::Lasso, Tool::Lasso, "Lasso Tool (L)"),
                (icons::Icon::Wand, Tool::Wand, "Magic Wand Tool (W)"),
                (icons::Icon::Brush, Tool::Brush, "Brush Tool (B)"),
                (icons::Icon::Pencil, Tool::Pencil, "Pencil Tool"),
                (icons::Icon::Eraser, Tool::Eraser, "Eraser Tool (E)"),
                (icons::Icon::Blur, Tool::Blur, "Blur Tool"),
                (icons::Icon::Sharpen, Tool::Sharpen, "Sharpen Tool"),
                (icons::Icon::Dodge, Tool::Dodge, "Dodge Tool"),
                (icons::Icon::Burn, Tool::Burn, "Burn Tool"),
                (
                    icons::Icon::Stamp,
                    Tool::Stamp,
                    "Rubber Stamp / Clone Tool (S)",
                ),
            ],
        ),
        (
            "Vector",
            &[
                (icons::Icon::Line, Tool::Line, "Line Tool (N)"),
                (icons::Icon::Pen, Tool::Path, "Pen / Vector Path Tool (P)"),
                (
                    icons::Icon::Rectangle,
                    Tool::Rectangle,
                    "Rectangle Tool (U)",
                ),
                (
                    icons::Icon::Rounded,
                    Tool::RoundedRect,
                    "Rounded Rectangle Tool",
                ),
                (icons::Icon::Ellipse, Tool::Ellipse, "Ellipse Tool"),
                (icons::Icon::Polygon, Tool::Polygon, "Polygon Tool"),
                (icons::Icon::Star, Tool::Star, "Star Auto Shape Tool"),
                (icons::Icon::Text, Tool::Text, "Text Tool (T)"),
                (icons::Icon::Knife, Tool::Knife, "Knife / Split Path Tool"),
            ],
        ),
        (
            "Web",
            &[
                (icons::Icon::Slice, Tool::Slice, "Rectangle Slice Tool (K)"),
                (
                    icons::Icon::Hotspot,
                    Tool::Hotspot,
                    "Rectangle Hotspot Tool (J)",
                ),
            ],
        ),
        (
            "Colors",
            &[
                (icons::Icon::Picker, Tool::Picker, "Eyedropper Tool (I)"),
                (icons::Icon::Fill, Tool::Fill, "Paint Bucket Tool (G)"),
                (icons::Icon::Gradient, Tool::Gradient, "Gradient Tool"),
            ],
        ),
        (
            "View",
            &[
                (icons::Icon::Hand, Tool::Hand, "Hand Tool (H)"),
                (icons::Icon::Zoom, Tool::Zoom, "Zoom Tool (Z)"),
                (icons::Icon::Fit, Tool::Fit, "Fit in Window (Ctrl+0)"),
            ],
        ),
    ];

    for (name, tools) in sections {
        ui.add_space(2.0);
        let (rect, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 14.0), Sense::hover());
        let painter = ui.painter();
        painter.text(
            pos2(rect.left() + 3.0, rect.center().y),
            Align2::LEFT_CENTER,
            name,
            FontId::proportional(10.0),
            p.text_dim,
        );
        painter.line_segment(
            [
                pos2(rect.left() + 2.0, rect.bottom()),
                pos2(rect.right() - 2.0, rect.bottom()),
            ],
            Stroke::new(1.0, p.stroke),
        );
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(2.0);
            for (icon, tool, tip) in tools {
                let active = app.tool == *tool;
                let response = crate::canvas::tool_button(ui, *icon, active, tip);
                if response.clicked() {
                    if *tool == Tool::Fit {
                        app.fit_in_window();
                    } else {
                        app.tool = *tool;
                        app.status = format!("{} selected", tool.name());
                    }
                }
            }
        });
    }

    ui.separator();
    // Active Stroke / Fill color chips + Swap (X) + Default (D) in the Colors section of the tool strip
    ui.vertical_centered(|ui| {
        ui.horizontal(|ui| {
            let mut stroke_rgb = [
                app.stroke.r() as f32 / 255.0,
                app.stroke.g() as f32 / 255.0,
                app.stroke.b() as f32 / 255.0,
            ];
            if ui
                .color_edit_button_rgb(&mut stroke_rgb)
                .on_hover_text("Stroke Color")
                .changed()
            {
                let c = Color32::from_rgb(
                    (stroke_rgb[0] * 255.0) as u8,
                    (stroke_rgb[1] * 255.0) as u8,
                    (stroke_rgb[2] * 255.0) as u8,
                );
                app.stroke = c;
                app.set_selected_stroke(Some(c));
            }
            let mut fill_rgb = [
                app.fill.r() as f32 / 255.0,
                app.fill.g() as f32 / 255.0,
                app.fill.b() as f32 / 255.0,
            ];
            if ui
                .color_edit_button_rgb(&mut fill_rgb)
                .on_hover_text("Fill Color")
                .changed()
            {
                let c = Color32::from_rgb(
                    (fill_rgb[0] * 255.0) as u8,
                    (fill_rgb[1] * 255.0) as u8,
                    (fill_rgb[2] * 255.0) as u8,
                );
                app.fill = c;
                app.set_selected_fill(Some(c));
            }
        });
        ui.horizontal(|ui| {
            if ui
                .small_button("⇄")
                .on_hover_text("Swap Fill & Stroke Colors (X)")
                .clicked()
            {
                std::mem::swap(&mut app.fill, &mut app.stroke);
            }
            if ui
                .small_button("⊘")
                .on_hover_text("Set No Fill on Selection")
                .clicked()
            {
                app.set_selected_fill(None);
            }
        });
    });
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

    ui.horizontal(|ui| {
        ui.checkbox(&mut app.doc.optimize.transparency, "Transparency");
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
            app.tool = Tool::Marquee;
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
                app.tool = Tool::Subselect;
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
pub fn color_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);

    // Top CS6 segmented sub-tabs: Selector | Mixers | Blender
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);
        for (i, label) in ["Selector", "Mixers", "Blender"].iter().enumerate() {
            if ui
                .selectable_label(app.color_subtab == i, RichText::new(*label).small())
                .clicked()
            {
                app.color_subtab = i;
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button("⇄")
                .on_hover_text("Swap Fill & Stroke Colors (X)")
                .clicked()
            {
                std::mem::swap(&mut app.fill, &mut app.stroke);
            }
            if ui
                .small_button("⊘")
                .on_hover_text("Set No Fill on Selection")
                .clicked()
            {
                app.set_selected_fill(None);
            }
        });
    });

    // Stroke & Fill icon + swatch + hex rows matching CS6 Color Palette
    ui.horizontal(|ui| {
        // Stroke box
        let (pen_r, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::hover());
        icons::draw(ui.painter(), pen_r, icons::Icon::Pencil, p.text_dim, 1.0);
        let (st_rect, st_resp) = ui.allocate_exact_size(Vec2::new(18.0, 14.0), Sense::click());
        ui.painter().rect_filled(st_rect, 0.0, app.stroke);
        ui.painter().rect_stroke(
            st_rect,
            0.0,
            Stroke::new(1.0, p.stroke),
            egui::StrokeKind::Middle,
        );
        if st_resp
            .on_hover_text("Stroke Color (click to swap with Fill)")
            .clicked()
        {
            std::mem::swap(&mut app.fill, &mut app.stroke);
        }
        let mut hex2 = colors::hex_of(app.stroke);
        if ui
            .add(
                TextEdit::singleline(&mut hex2)
                    .desired_width(68.0)
                    .font(FontId::monospace(10.5))
                    .text_color(p.text),
            )
            .changed()
            && let Some(c) = colors::from_hex(&hex2)
        {
            app.stroke = c;
            app.set_selected_stroke(Some(c));
        }

        ui.add_space(4.0);

        // Fill box
        let (fill_ic_r, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::hover());
        icons::draw(ui.painter(), fill_ic_r, icons::Icon::Fill, p.text_dim, 1.0);
        let (fl_rect, _) = ui.allocate_exact_size(Vec2::new(18.0, 14.0), Sense::click());
        ui.painter().rect_filled(fl_rect, 0.0, app.fill);
        ui.painter().rect_stroke(
            fl_rect,
            0.0,
            Stroke::new(1.0, p.accent),
            egui::StrokeKind::Middle,
        );
        let mut hex = colors::hex_of(app.fill);
        if ui
            .add(
                TextEdit::singleline(&mut hex)
                    .desired_width(68.0)
                    .font(FontId::monospace(10.5))
                    .text_color(p.text),
            )
            .changed()
            && let Some(c) = colors::from_hex(&hex)
        {
            app.fill = c;
            app.set_selected_fill(Some(c));
        }
    });

    if app.color_subtab == 2 {
        // Blender tab: live 12-step interpolation ramp between Stroke and Fill
        ui.label(
            RichText::new("Blend Ramp (Left-click: Fill, Right-click: Stroke):")
                .small()
                .color(p.text_dim),
        );
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 2.0);
            for step in 0..12 {
                let t = step as f32 / 11.0;
                let r = (app.stroke.r() as f32 * (1.0 - t) + app.fill.r() as f32 * t) as u8;
                let g = (app.stroke.g() as f32 * (1.0 - t) + app.fill.g() as f32 * t) as u8;
                let b = (app.stroke.b() as f32 * (1.0 - t) + app.fill.b() as f32 * t) as u8;
                let c = Color32::from_rgb(r, g, b);
                let (cell, resp) = ui.allocate_exact_size(Vec2::new(20.0, 52.0), Sense::click());
                ui.painter().rect_filled(cell, 0.0, c);
                ui.painter().rect_stroke(
                    cell,
                    0.0,
                    Stroke::new(1.0, p.stroke),
                    egui::StrokeKind::Middle,
                );
                if resp.clicked() {
                    app.fill = c;
                    app.set_selected_fill(Some(c));
                }
                if resp.secondary_clicked() {
                    app.stroke = c;
                    app.set_selected_stroke(Some(c));
                }
                resp.on_hover_text(colors::hex_of(c));
            }
        });
        return;
    }

    let spaces = ["HLS", "HSV", "CMYK", "RGB"];
    ui.horizontal(|ui| {
        for (i, name) in spaces.iter().enumerate() {
            let selected = app.color_tab == i;
            if ui
                .selectable_label(selected, RichText::new(*name).small())
                .clicked()
            {
                app.color_tab = i;
            }
        }
    });

    ui.horizontal(|ui| {
        let mut hsv = colors::rgb_to_hsv(app.fill);
        let (square, sq_resp) =
            ui.allocate_exact_size(Vec2::new(92.0, 68.0), Sense::click_and_drag());
        let painter = ui.painter();
        let steps_x = 24;
        let steps_y = 18;
        for iy in 0..steps_y {
            for ix in 0..steps_x {
                let s = ix as f32 / (steps_x - 1) as f32;
                let v = 1.0 - iy as f32 / (steps_y - 1) as f32;
                let c = colors::hsv_to_rgb(colors::Hsv { h: hsv.h, s, v });
                let cell = Rect::from_min_size(
                    pos2(
                        square.left() + square.width() * ix as f32 / steps_x as f32,
                        square.top() + square.height() * iy as f32 / steps_y as f32,
                    ),
                    Vec2::new(
                        square.width() / steps_x as f32 + 0.8,
                        square.height() / steps_y as f32 + 0.8,
                    ),
                );
                painter.rect_filled(cell, 0.0, c);
            }
        }
        painter.rect_stroke(
            square,
            0.0,
            Stroke::new(1.0, p.stroke),
            egui::StrokeKind::Middle,
        );
        // Active SV crosshair ring
        let cur_pt = pos2(
            square.left() + hsv.s.clamp(0.0, 1.0) * square.width(),
            square.top() + (1.0 - hsv.v.clamp(0.0, 1.0)) * square.height(),
        );
        painter.circle_stroke(cur_pt, 3.5, Stroke::new(1.2, Color32::WHITE));
        painter.circle_stroke(cur_pt, 4.5, Stroke::new(1.0, Color32::BLACK));

        if (sq_resp.clicked() || sq_resp.dragged())
            && let Some(pos) = sq_resp.interact_pointer_pos()
        {
            hsv.s = ((pos.x - square.left()) / square.width()).clamp(0.0, 1.0);
            hsv.v = (1.0 - (pos.y - square.top()) / square.height()).clamp(0.0, 1.0);
            app.fill = colors::hsv_to_rgb(hsv);
            app.set_selected_fill(Some(app.fill));
        }

        let (bar, bar_resp) =
            ui.allocate_exact_size(Vec2::new(14.0, 68.0), Sense::click_and_drag());
        let painter = ui.painter();
        for iy in 0..32 {
            let h = iy as f32 / 32.0 * 360.0;
            let c = colors::hsv_to_rgb(colors::Hsv { h, s: 1.0, v: 1.0 });
            let cell = Rect::from_min_size(
                pos2(bar.left(), bar.top() + bar.height() * iy as f32 / 32.0),
                Vec2::new(bar.width(), bar.height() / 32.0 + 0.8),
            );
            painter.rect_filled(cell, 0.0, c);
        }
        painter.rect_stroke(
            bar,
            0.0,
            Stroke::new(1.0, p.stroke),
            egui::StrokeKind::Middle,
        );
        // Hue indicator tick
        let hy = bar.top() + (hsv.h / 360.0).clamp(0.0, 1.0) * bar.height();
        painter.line_segment(
            [pos2(bar.left(), hy), pos2(bar.right(), hy)],
            Stroke::new(1.5, Color32::WHITE),
        );
        if (bar_resp.clicked() || bar_resp.dragged())
            && let Some(pos) = bar_resp.interact_pointer_pos()
        {
            hsv.h = (((pos.y - bar.top()) / bar.height()).clamp(0.0, 1.0)) * 360.0;
            app.fill = colors::hsv_to_rgb(hsv);
            app.set_selected_fill(Some(app.fill));
        }

        let mut changed = false;
        ui.vertical(|ui| match app.color_tab {
            0 => {
                let mut hls = colors::rgb_to_hls(app.fill);
                let mut l_pct = hls.l * 100.0;
                let mut s_pct = hls.s * 100.0;
                changed |= slider_row(ui, "H", &mut hls.h, 0.0, 360.0, "°");
                changed |= slider_row(ui, "L", &mut l_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "S", &mut s_pct, 0.0, 100.0, "%");
                if changed {
                    hls.l = l_pct / 100.0;
                    hls.s = s_pct / 100.0;
                    app.fill = colors::hls_to_rgb(hls);
                    app.set_selected_fill(Some(app.fill));
                }
            }
            1 => {
                let mut hsv = colors::rgb_to_hsv(app.fill);
                let mut s_pct = hsv.s * 100.0;
                let mut v_pct = hsv.v * 100.0;
                changed |= slider_row(ui, "H", &mut hsv.h, 0.0, 360.0, "°");
                changed |= slider_row(ui, "S", &mut s_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "V", &mut v_pct, 0.0, 100.0, "%");
                if changed {
                    hsv.s = s_pct / 100.0;
                    hsv.v = v_pct / 100.0;
                    app.fill = colors::hsv_to_rgb(hsv);
                    app.set_selected_fill(Some(app.fill));
                }
            }
            2 => {
                let mut cmyk = colors::rgb_to_cmyk(app.fill);
                let mut c_pct = cmyk.c * 100.0;
                let mut m_pct = cmyk.m * 100.0;
                let mut y_pct = cmyk.y * 100.0;
                let mut k_pct = cmyk.k * 100.0;
                changed |= slider_row(ui, "C", &mut c_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "M", &mut m_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "Y", &mut y_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "K", &mut k_pct, 0.0, 100.0, "%");
                if changed {
                    cmyk.c = c_pct / 100.0;
                    cmyk.m = m_pct / 100.0;
                    cmyk.y = y_pct / 100.0;
                    cmyk.k = k_pct / 100.0;
                    app.fill = colors::cmyk_to_rgb(cmyk);
                    app.set_selected_fill(Some(app.fill));
                }
            }
            _ => {
                let mut r = app.fill.r() as f32;
                let mut g = app.fill.g() as f32;
                let mut b = app.fill.b() as f32;
                changed |= slider_row(ui, "R", &mut r, 0.0, 255.0, "");
                changed |= slider_row(ui, "G", &mut g, 0.0, 255.0, "");
                changed |= slider_row(ui, "B", &mut b, 0.0, 255.0, "");
                if changed {
                    app.fill = Color32::from_rgb(r as u8, g as u8, b as u8);
                    app.set_selected_fill(Some(app.fill));
                }
            }
        });
    });
}

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
            app.doc.page_mut().sync_state();
            let state_count = app.doc.page().states.len();
            let current_st = app.doc.page().current_state;
            egui::ScrollArea::vertical()
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
                .max_height(84.0)
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
        ui.label(RichText::new("Current Document").small().color(p.text_dim));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
    let styles = app.doc.styles.clone();
    egui::ScrollArea::vertical()
        .max_height(112.0)
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);
                for (idx, preset) in styles.iter().enumerate() {
                    let (rect, resp) =
                        ui.allocate_exact_size(Vec2::new(134.0, 26.0), Sense::click());
                    let painter = ui.painter();
                    painter.rect_filled(rect, 0.0, p.well);
                    let swatch_r = Rect::from_min_size(
                        pos2(rect.left() + 4.0, rect.top() + 4.0),
                        Vec2::splat(18.0),
                    );
                    painter.rect_filled(
                        swatch_r,
                        0.0,
                        preset.fill.unwrap_or(Color32::from_rgb(90, 90, 90)),
                    );
                    if let Some(st) = preset.stroke {
                        painter.rect_stroke(
                            swatch_r,
                            0.0,
                            Stroke::new(1.5, st),
                            egui::StrokeKind::Middle,
                        );
                    }
                    painter.text(
                        pos2(swatch_r.right() + 6.0, rect.center().y),
                        Align2::LEFT_CENTER,
                        &preset.name,
                        FontId::proportional(10.5),
                        p.text,
                    );
                    painter.rect_stroke(
                        rect,
                        0.0,
                        Stroke::new(1.0, if resp.hovered() { p.accent } else { p.stroke }),
                        egui::StrokeKind::Middle,
                    );
                    if resp.clicked() {
                        app.apply_style_preset(idx);
                    }
                }
            });
        });
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
                        app.tool = tool;
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

/// The bottom Properties Inspector (CS6 bottom dock):
/// When an object is selected: Name, Type, editable X/Y/W/H, Fill, Stroke, Width, Opacity,
/// Corner Radius / Star Points / Text Font / Slice URL, plus Live Filters (+) stack.
/// When nothing is selected: Document Canvas Size, Background Color, Resolution, Fit Canvas.
pub fn properties_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    if app.selected_set.len() >= 2 {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new(format!("{} Objects Selected", app.selected_set.len()))
                        .strong()
                        .color(p.selection),
                );
                ui.horizontal(|ui| {
                    if ui.small_button("Group (Ctrl+G)").clicked() {
                        app.group_selected();
                    }
                    if ui.small_button("Duplicate").clicked() {
                        app.duplicate_selected();
                    }
                    if ui.small_button("Delete").clicked() {
                        app.delete_selected();
                    }
                });
            });
            ui.separator();
            ui.vertical(|ui| {
                ui.label(RichText::new("Align Selection:").small().color(p.text_dim));
                ui.horizontal(|ui| {
                    for (mode, lbl) in [
                        ("left", "Left"),
                        ("center_x", "Center H"),
                        ("right", "Right"),
                        ("top", "Top"),
                        ("center_y", "Center V"),
                        ("bottom", "Bottom"),
                    ] {
                        if ui.small_button(lbl).clicked() {
                            app.align_selected_to_canvas(mode);
                        }
                    }
                });
            });
        });
        return;
    }
    let obj: Option<Obj> = app.selected_object();
    match obj {
        Some(obj) => {
            ui.horizontal(|ui| {
                // Column 0: CS6 Object Thumbnail Box + Type & Name
                let (thumb_r, _) = ui.allocate_exact_size(Vec2::new(30.0, 38.0), Sense::hover());
                ui.painter().rect_filled(thumb_r, 0.0, p.well);
                ui.painter().rect_stroke(
                    thumb_r,
                    0.0,
                    Stroke::new(1.0, p.stroke),
                    egui::StrokeKind::Middle,
                );
                let thumb_ic = match obj.kind {
                    Shape::Polygon { .. } => icons::Icon::Star,
                    Shape::Ellipse { .. } => icons::Icon::Ellipse,
                    Shape::Line { .. } => icons::Icon::Line,
                    Shape::Path { .. } => icons::Icon::Pen,
                    Shape::Text { .. } => icons::Icon::Text,
                    Shape::Slice { .. } => icons::Icon::Slice,
                    Shape::Group { .. } => icons::Icon::Group,
                    _ => icons::Icon::Rectangle,
                };
                icons::draw(
                    ui.painter(),
                    thumb_r.shrink(3.0),
                    thumb_ic,
                    obj.fill.unwrap_or(p.accent),
                    1.2,
                );

                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(obj.type_label())
                            .small()
                            .strong()
                            .color(p.accent),
                    );
                    let mut name = obj.name.clone();
                    if ui
                        .add(TextEdit::singleline(&mut name).desired_width(108.0))
                        .changed()
                    {
                        app.set_selected_name(name);
                    }
                });

                if let Some((lo, hi)) = obj.bounds() {
                    let mut x = lo.x;
                    let mut y = lo.y;
                    let mut w = (hi.x - lo.x).max(1.0);
                    let mut h = (hi.y - lo.y).max(1.0);
                    let mut geom_changed = false;
                    // CS6 2×2 W/H and X/Y stacked columns
                    Grid::new("prop_wh_xy_grid")
                        .num_columns(4)
                        .spacing([4.0, 2.0])
                        .show(ui, |ui| {
                            ui.label(RichText::new("W:").small().color(p.text_dim));
                            geom_changed |= ui
                                .add(egui::DragValue::new(&mut w).speed(1.0).range(1.0..=16384.0))
                                .changed();
                            ui.label(RichText::new("X:").small().color(p.text_dim));
                            geom_changed |=
                                ui.add(egui::DragValue::new(&mut x).speed(1.0)).changed();
                            ui.end_row();

                            ui.label(RichText::new("H:").small().color(p.text_dim));
                            geom_changed |= ui
                                .add(egui::DragValue::new(&mut h).speed(1.0).range(1.0..=16384.0))
                                .changed();
                            ui.label(RichText::new("Y:").small().color(p.text_dim));
                            geom_changed |=
                                ui.add(egui::DragValue::new(&mut y).speed(1.0)).changed();
                            ui.end_row();
                        });
                    if geom_changed {
                        app.set_selected_bounds(Vec2::new(x, y), Vec2::new(x + w, y + h));
                    }
                }

                ui.separator();

                // Column 2: Fill, Stroke, Stroke Width, Opacity & Shape-specific parameters
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Fill:").small().color(p.text_dim));
                        let mut has_fill = obj.fill.is_some();
                        if ui.checkbox(&mut has_fill, "").changed() {
                            if has_fill {
                                app.set_selected_fill(Some(app.fill));
                            } else {
                                app.set_selected_fill(None);
                            }
                        }
                        if let Some(fc) = obj.fill {
                            let mut rgb = [
                                fc.r() as f32 / 255.0,
                                fc.g() as f32 / 255.0,
                                fc.b() as f32 / 255.0,
                            ];
                            if ui.color_edit_button_rgb(&mut rgb).changed() {
                                let c = Color32::from_rgb(
                                    (rgb[0] * 255.0) as u8,
                                    (rgb[1] * 255.0) as u8,
                                    (rgb[2] * 255.0) as u8,
                                );
                                app.fill = c;
                                app.set_selected_fill(Some(c));
                            }
                        }

                        ui.label(RichText::new("Stroke:").small().color(p.text_dim));
                        let mut has_stroke = obj.stroke.is_some();
                        if ui.checkbox(&mut has_stroke, "").changed() {
                            if has_stroke {
                                app.set_selected_stroke(Some(app.stroke));
                            } else {
                                app.set_selected_stroke(None);
                            }
                        }
                        if let Some(sc) = obj.stroke {
                            let mut rgb = [
                                sc.r() as f32 / 255.0,
                                sc.g() as f32 / 255.0,
                                sc.b() as f32 / 255.0,
                            ];
                            if ui.color_edit_button_rgb(&mut rgb).changed() {
                                let c = Color32::from_rgb(
                                    (rgb[0] * 255.0) as u8,
                                    (rgb[1] * 255.0) as u8,
                                    (rgb[2] * 255.0) as u8,
                                );
                                app.stroke = c;
                                app.set_selected_stroke(Some(c));
                            }
                        }
                        let mut width = obj.width;
                        if ui
                            .add(
                                egui::DragValue::new(&mut width)
                                    .speed(0.5)
                                    .range(0.0..=64.0)
                                    .suffix(" px"),
                            )
                            .on_hover_text("Stroke Width")
                            .changed()
                        {
                            app.set_selected_width(width);
                        }

                        ui.label(RichText::new("Opacity:").small().color(p.text_dim));
                        let mut op = obj.opacity * 100.0;
                        if ui
                            .add(
                                egui::DragValue::new(&mut op)
                                    .speed(1.0)
                                    .range(0.0..=100.0)
                                    .suffix("%"),
                            )
                            .changed()
                        {
                            app.set_selected_opacity(op / 100.0);
                        }
                    });

                    // Shape-specific secondary row
                    ui.horizontal(|ui| match &obj.kind {
                        Shape::Rect { corner_radius, .. } => {
                            ui.label(RichText::new("Roundness:").small().color(p.text_dim));
                            let mut cr = *corner_radius;
                            if ui
                                .add(egui::Slider::new(&mut cr, 0.0..=120.0).suffix(" px"))
                                .changed()
                            {
                                app.set_selected_corner_radius(cr);
                            }
                        }
                        Shape::Polygon {
                            sides, star_inner, ..
                        } => {
                            let mut s = *sides;
                            let mut inner = *star_inner;
                            ui.label(RichText::new("Sides:").small().color(p.text_dim));
                            if ui.add(egui::DragValue::new(&mut s).range(3..=36)).changed() {
                                app.set_selected_polygon_params(s, inner);
                            }
                            ui.label(RichText::new("Star Ratio:").small().color(p.text_dim));
                            if ui.add(egui::Slider::new(&mut inner, 0.15..=1.0)).changed() {
                                app.set_selected_polygon_params(s, inner);
                            }
                        }
                        Shape::Text {
                            text,
                            size,
                            bold,
                            italic,
                            ..
                        } => {
                            ui.label(RichText::new("Text:").small().color(p.text_dim));
                            let mut edit = text.clone();
                            if ui
                                .add(TextEdit::singleline(&mut edit).desired_width(140.0))
                                .changed()
                            {
                                app.set_selected_text(edit);
                            }
                            let mut sz = *size;
                            if ui
                                .add(
                                    egui::DragValue::new(&mut sz)
                                        .range(6.0..=240.0)
                                        .suffix(" pt"),
                                )
                                .changed()
                            {
                                app.set_selected_text_size(sz);
                            }
                            if ui
                                .selectable_label(*bold, RichText::new("B").strong())
                                .clicked()
                            {
                                app.toggle_selected_text_bold();
                            }
                            if ui
                                .selectable_label(*italic, RichText::new("I").italics())
                                .clicked()
                            {
                                app.toggle_selected_text_italic();
                            }
                        }
                        Shape::Slice { url, alt, .. } => {
                            ui.label(RichText::new("Link URL:").small().color(p.text_dim));
                            let mut u_edit = url.clone();
                            let mut a_edit = alt.clone();
                            let mut sl_changed = false;
                            sl_changed |= ui
                                .add(TextEdit::singleline(&mut u_edit).desired_width(140.0))
                                .changed();
                            ui.label(RichText::new("Alt:").small().color(p.text_dim));
                            sl_changed |= ui
                                .add(TextEdit::singleline(&mut a_edit).desired_width(100.0))
                                .changed();
                            if sl_changed {
                                app.set_selected_slice_meta(u_edit, a_edit);
                            }
                        }
                        Shape::Gradient { radial, .. } => {
                            let mut r = *radial;
                            if ui.checkbox(&mut r, "Radial Gradient").changed() {
                                app.set_selected_gradient_radial(r);
                            }
                        }
                        Shape::Group { children } => {
                            ui.label(
                                RichText::new(format!("Group ({} members)", children.len()))
                                    .small()
                                    .color(p.text_dim),
                            );
                            if ui.small_button("Ungroup (Ctrl+Shift+G)").clicked() {
                                app.ungroup_selected();
                            }
                        }
                        _ => {
                            ui.label(
                                RichText::new(format!(
                                    "Layer: {} · Blend: {}",
                                    app.selected_layer + 1,
                                    obj.blend_mode.label()
                                ))
                                .small()
                                .color(p.text_dim),
                            );
                        }
                    });
                });

                ui.separator();

                // Column 3: CS6 Live Filters (+) stack
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Filters:").small().strong());
                        ui.menu_button("+ Add", |ui| {
                            if ui.button("Drop Shadow").clicked() {
                                app.add_live_effect(LiveEffect::DropShadow {
                                    distance: 5.0,
                                    angle_deg: 45.0,
                                    blur: 5.0,
                                    color: Color32::from_black_alpha(150),
                                    enabled: true,
                                });
                                ui.close();
                            }
                            if ui.button("Outer Glow").clicked() {
                                app.add_live_effect(LiveEffect::Glow {
                                    radius: 6.0,
                                    color: Color32::from_rgba_unmultiplied(232, 163, 61, 180),
                                    inner: false,
                                    enabled: true,
                                });
                                ui.close();
                            }
                            if ui.button("Inner Bevel").clicked() {
                                app.add_live_effect(LiveEffect::Bevel {
                                    width: 2.0,
                                    highlight: Color32::from_white_alpha(190),
                                    shadow: Color32::from_black_alpha(150),
                                    enabled: true,
                                });
                                ui.close();
                            }
                            if ui.button("Gaussian Blur").clicked() {
                                app.add_live_effect(LiveEffect::GaussianBlur {
                                    radius: 4.0,
                                    enabled: true,
                                });
                                ui.close();
                            }
                        });
                        if !obj.effects.is_empty() && ui.small_button("- Clear").clicked() {
                            app.clear_live_effects();
                        }
                    });
                    ui.horizontal_wrapped(|ui| {
                        if obj.effects.is_empty() {
                            ui.label(
                                RichText::new("(No live filters applied)")
                                    .small()
                                    .color(p.text_dim),
                            );
                        } else {
                            for (ei, eff) in obj.effects.iter().enumerate() {
                                let mut en = eff.is_enabled();
                                if ui.checkbox(&mut en, eff.label()).changed() {
                                    app.toggle_live_effect(ei, en);
                                }
                            }
                        }
                    });
                });
            });
        }
        None => {
            // Document-level Properties Inspector when nothing is selected (exact CS6 layout from screenshot)
            ui.horizontal(|ui| {
                let (doc_thumb, _) = ui.allocate_exact_size(Vec2::new(30.0, 38.0), Sense::hover());
                ui.painter()
                    .rect_filled(doc_thumb, 0.0, app.doc.page().bg_color);
                ui.painter().rect_stroke(
                    doc_thumb,
                    0.0,
                    Stroke::new(1.0, p.stroke),
                    egui::StrokeKind::Middle,
                );
                ui.vertical(|ui| {
                    ui.label(RichText::new("Document").small().strong().color(p.accent));
                    ui.label(RichText::new(&app.doc.name).small().color(p.text_dim));
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Canvas:").small().color(p.text_dim));
                        let mut bg = [
                            app.doc.page().bg_color.r() as f32 / 255.0,
                            app.doc.page().bg_color.g() as f32 / 255.0,
                            app.doc.page().bg_color.b() as f32 / 255.0,
                        ];
                        if ui.color_edit_button_rgb(&mut bg).changed() {
                            let page = app.doc.page_mut();
                            page.bg_color = Color32::from_rgb(
                                (bg[0] * 255.0) as u8,
                                (bg[1] * 255.0) as u8,
                                (bg[2] * 255.0) as u8,
                            );
                            page.transparent_bg = false;
                        }
                        ui.checkbox(&mut app.doc.page_mut().transparent_bg, "Transparent");
                        if ui.button("Canvas Size…").clicked() {
                            app.open_canvas_size_dialog();
                        }
                        if ui.button("Image Size…").clicked() {
                            app.open_canvas_size_dialog();
                        }
                        if ui.button("Fit Canvas").clicked() {
                            app.fit_canvas_to_contents();
                        }
                        ui.label(RichText::new("Size:").small().color(p.text_dim));
                        egui::ComboBox::from_id_salt("inspector_res_preset_combo")
                            .width(150.0)
                            .selected_text(format!(
                                "{:.0}×{:.0} px",
                                app.doc.page().width,
                                app.doc.page().height
                            ))
                            .show_ui(ui, |ui| {
                                for &(label, w, h) in constants::RESOLUTION_PRESETS {
                                    let active = (app.doc.page().width - w).abs() < 0.5
                                        && (app.doc.page().height - h).abs() < 0.5;
                                    if ui.selectable_label(active, label).clicked() {
                                        app.resize_canvas(w, h);
                                    }
                                }
                            });
                        ui.label(RichText::new("DPI:").small().color(p.text_dim));
                        egui::ComboBox::from_id_salt("inspector_dpi_preset_combo")
                            .width(78.0)
                            .selected_text(format!("{} ppi", app.doc.dpi))
                            .show_ui(ui, |ui| {
                                for &dpi in constants::DPI_PRESETS {
                                    if ui
                                        .selectable_label(app.doc.dpi == dpi, format!("{dpi} ppi"))
                                        .clicked()
                                    {
                                        app.doc.dpi = dpi;
                                    }
                                }
                            });
                    });
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!(
                                "Dimensions: {:.0}×{:.0} px @ {} dpi · Export: {} · State {} of {}",
                                app.doc.page().width,
                                app.doc.page().height,
                                app.doc.dpi,
                                app.doc.optimize.format,
                                app.doc.page().current_state + 1,
                                app.doc.page().states.len()
                            ))
                            .small()
                            .color(p.text_dim),
                        );
                    });
                });
                ui.separator();
                if matches!(app.tool, Tool::Brush | Tool::Pencil | Tool::Eraser) {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Brush Size:").small().color(p.text_dim));
                        ui.add(egui::Slider::new(&mut app.brush_size, 1.0..=64.0).suffix(" px"));
                    });
                }
            });
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
