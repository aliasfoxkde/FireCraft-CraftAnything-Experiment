//! Panel render bodies. Each public function renders one registered panel; the
//! `PANELS` descriptor table in `panels/mod.rs` is the only caller that matters.

use super::tabs::pages_panel;
use super::*;

/// Optimize panel (CS6 top-right tab 0): export format, palette, quality, dither, matte, size estimate.
pub(super) fn optimize_panel(ui: &mut Ui, app: &mut FireCraft) {
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
pub(super) fn history_panel(ui: &mut Ui, app: &mut FireCraft) {
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

pub(super) fn align_icon_btn(
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

pub(super) fn path_icon_btn(
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
pub(super) fn align_panel(ui: &mut Ui, app: &mut FireCraft) {
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
pub(super) fn paths_panel(ui: &mut Ui, app: &mut FireCraft) {
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
pub(super) fn swatches_panel(ui: &mut Ui, app: &mut FireCraft) {
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
pub(super) fn slider_row(
    ui: &mut Ui,
    label: &str,
    value: &mut f32,
    min: f32,
    max: f32,
    unit: &str,
) -> bool {
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
pub(super) fn info_panel(ui: &mut Ui, app: &mut FireCraft) {
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

pub(super) fn small_icon_cell(
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

/// The Styles panel (`screenshots/Fireworks_Layout_Design.jpg`): sharp CS6 style cards + Current Document header.
pub(super) fn styles_panel(ui: &mut Ui, app: &mut FireCraft) {
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
pub(super) fn library_panel(ui: &mut Ui, app: &mut FireCraft) {
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

pub(super) fn pages_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.layers_tab = 0;
    pages_panel(ui, app);
}
pub(super) fn states_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.layers_tab = 1;
    pages_panel(ui, app);
}
pub(super) fn layers_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.layers_tab = 2;
    pages_panel(ui, app);
}
pub(super) fn special_chars_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 0;
    library_panel(ui, app);
}
pub(super) fn image_editing_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 1;
    library_panel(ui, app);
}
pub(super) fn auto_shapes_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 2;
    library_panel(ui, app);
}
pub(super) fn document_library_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 3;
    library_panel(ui, app);
}
pub(super) fn css_properties_tab(ui: &mut Ui, app: &mut FireCraft) {
    app.library_tab = 4;
    library_panel(ui, app);
}

#[cfg(test)]
mod body_render_tests {
    use super::*;

    fn render_body(app: &mut FireCraft, mut body: impl FnMut(&mut Ui, &mut FireCraft)) -> usize {
        let ctx = egui::Context::default();
        app.ctx = ctx.clone();
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, 700.0),
                )),
                time: Some(3.0),
                ..Default::default()
            },
            |ui| body(ui, app),
        );
        let shape_count = output.shapes.len();
        output.textures_delta.clear();
        shape_count
    }

    fn assert_panel_renders(app: &mut FireCraft, body: impl FnMut(&mut Ui, &mut FireCraft)) {
        let shapes = render_body(app, body);
        assert!(shapes > 0, "panel body should paint at least one shape");
    }

    #[test]
    fn primary_panel_bodies_render_empty_and_populated_states() {
        let mut app = FireCraft::new(None);
        assert_panel_renders(&mut app, optimize_panel);
        assert_panel_renders(&mut app, history_panel);
        assert_panel_renders(&mut app, align_panel);
        assert_panel_renders(&mut app, paths_panel);
        assert_panel_renders(&mut app, swatches_panel);
        assert_panel_renders(&mut app, info_panel);

        app.load_demo_showcase();
        app.select_all();
        app.undo.push(app.doc.snapshot("Render history entry"));
        app.redo.push(app.doc.snapshot("Render redo entry"));
        assert_panel_renders(&mut app, history_panel);
        assert_panel_renders(&mut app, align_panel);
        assert_panel_renders(&mut app, info_panel);
        assert_panel_renders(&mut app, paths_panel);
    }

    #[test]
    fn styles_and_intelligence_render_each_available_state() {
        let mut app = FireCraft::new(None);
        for library in crate::doc::StyleLibrary::ALL {
            app.style_library = library;
            assert_panel_renders(&mut app, styles_panel);
        }

        app.load_demo_showcase();
        app.select_all();
        app.style_library = crate::doc::StyleLibrary::CurrentDocument;
        app.selected_style_idx = Some(0);
        assert_panel_renders(&mut app, styles_panel);

        app.show_ai_settings = false;
        app.ai_response.clear();
        assert_panel_renders(&mut app, intelligence_panel);
        app.show_ai_settings = true;
        app.ai_response = "A real panel response state".to_string();
        assert_panel_renders(&mut app, intelligence_panel);
    }

    #[test]
    fn library_tabs_render_empty_and_populated_content() {
        let mut app = FireCraft::new(None);
        for tab in 0..5 {
            app.library_tab = tab;
            assert_panel_renders(&mut app, library_panel);
        }

        app.load_demo_showcase();
        app.select_all();
        app.doc.symbols.push(crate::doc::SymbolDef {
            id: 0,
            name: "Panel symbol".to_string(),
            kind: "Graphic".to_string(),
            objects: Vec::new(),
        });
        for tab in 1..5 {
            app.library_tab = tab;
            assert_panel_renders(&mut app, library_panel);
        }
    }

    #[test]
    fn tab_adapters_select_their_document_and_library_pages() {
        let mut app = FireCraft::new(None);
        for (tab, render) in [pages_tab, states_tab, layers_tab].into_iter().enumerate() {
            assert_panel_renders(&mut app, render);
            assert_eq!(app.layers_tab, tab);
        }
        for (tab, render) in [
            special_chars_tab,
            image_editing_tab,
            auto_shapes_tab,
            document_library_tab,
            css_properties_tab,
        ]
        .into_iter()
        .enumerate()
        {
            assert_panel_renders(&mut app, render);
            assert_eq!(app.library_tab, tab);
        }
    }
}
