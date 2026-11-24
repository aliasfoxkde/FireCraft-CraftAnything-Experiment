//! Left tool palette: complete CS6 tool inventory, grouped sections, and pinned color/display footer.

use super::*;

type ToolSection = (&'static str, &'static [(icons::Icon, Tool, &'static str)]);

/// The tool palette down the left edge, grouped into Fireworks CS6's 6 sections.
/// A single tool slot in the left palette. The rail uses three columns at its normal width so the
/// entire CS6 tool inventory stays visible without hidden flyout menus.
fn palette_tool_button(ui: &mut Ui, icon: icons::Icon, active: bool, tip: &str) -> egui::Response {
    let p = ui.visuals().clone();
    let size = Vec2::new(24.0, 24.0);
    let (rect, response) = ui.allocate_exact_size(size, Sense::click());
    if ui.is_rect_visible(rect) {
        if active {
            ui.painter()
                .rect_filled(rect, 2.0, p.selection.bg_fill.gamma_multiply(0.38));
            ui.painter().rect_stroke(
                rect,
                2.0,
                Stroke::new(1.0, p.selection.stroke.color),
                egui::StrokeKind::Inside,
            );
        } else if response.hovered() {
            ui.painter()
                .rect_filled(rect, 2.0, p.widgets.hovered.bg_fill);
            ui.painter().rect_stroke(
                rect,
                2.0,
                Stroke::new(1.0, p.widgets.hovered.bg_stroke.color),
                egui::StrokeKind::Inside,
            );
        }
        let color = if active {
            Color32::WHITE
        } else {
            p.text_color()
        };
        icons::draw(ui.painter(), rect, icon, color, 1.35);
    }
    response.on_hover_text(tip)
}

pub fn tools_panel_body(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let sections: [ToolSection; 6] = [
        (
            "SELECT",
            &[
                (
                    icons::Icon::Select,
                    Tool::Select,
                    "Pointer (V) — select and move",
                ),
                (
                    icons::Icon::Subselect,
                    Tool::Subselect,
                    "Subselection (A) — edit path points",
                ),
                (icons::Icon::Scale, Tool::Scale, "Scale / Transform (Q)"),
                (
                    icons::Icon::Crop,
                    Tool::Crop,
                    "Crop (C) — drag to crop the canvas",
                ),
                (icons::Icon::Marquee, Tool::Marquee, "Marquee Select (M)"),
                (icons::Icon::Marquee, Tool::OvalMarquee, "Oval Marquee (O)"),
                (icons::Icon::Lasso, Tool::Lasso, "Lasso Select (L)"),
                (
                    icons::Icon::Wand,
                    Tool::Wand,
                    "Magic Wand (W) — select matching colors",
                ),
            ],
        ),
        (
            "BITMAP",
            &[
                (icons::Icon::Brush, Tool::Brush, "Brush (B)"),
                (icons::Icon::Pencil, Tool::Pencil, "Pencil"),
                (icons::Icon::Eraser, Tool::Eraser, "Eraser (E)"),
                (icons::Icon::Blur, Tool::Blur, "Blur"),
                (icons::Icon::Sharpen, Tool::Sharpen, "Sharpen"),
                (icons::Icon::Dodge, Tool::Dodge, "Dodge — lighten"),
                (icons::Icon::Burn, Tool::Burn, "Burn — darken"),
                (
                    icons::Icon::Stamp,
                    Tool::Stamp,
                    "Rubber Stamp (S) — duplicate at cursor",
                ),
                (
                    icons::Icon::Fill,
                    Tool::ReplaceColor,
                    "Replace Color (R) — click a fill to find/replace it",
                ),
                (
                    icons::Icon::Eye,
                    Tool::RedEye,
                    "Red Eye Removal — correct red-dominant pixels",
                ),
            ],
        ),
        (
            "VECTOR",
            &[
                (icons::Icon::Line, Tool::Line, "Line (N)"),
                (icons::Icon::Pen, Tool::Path, "Pen (P) — draw a vector path"),
                (
                    icons::Icon::Pencil,
                    Tool::Freeform,
                    "Freeform Path (F) — draw with the current stroke",
                ),
                (icons::Icon::Rectangle, Tool::Rectangle, "Rectangle (U)"),
                (icons::Icon::Rounded, Tool::RoundedRect, "Rounded Rectangle"),
                (icons::Icon::Ellipse, Tool::Ellipse, "Ellipse"),
                (icons::Icon::Polygon, Tool::Polygon, "Polygon"),
                (icons::Icon::Star, Tool::Star, "Star Auto Shape"),
                (icons::Icon::Text, Tool::Text, "Text (T)"),
                (
                    icons::Icon::Knife,
                    Tool::Knife,
                    "Knife — split the selected path",
                ),
            ],
        ),
        (
            "WEB",
            &[
                (icons::Icon::Slice, Tool::Slice, "Rectangle Slice (K)"),
                (icons::Icon::Hotspot, Tool::Hotspot, "Rectangle Hotspot (J)"),
            ],
        ),
        (
            "COLORS",
            &[
                (icons::Icon::Picker, Tool::Picker, "Eyedropper (I)"),
                (icons::Icon::Fill, Tool::Fill, "Paint Bucket (G)"),
                (icons::Icon::Gradient, Tool::Gradient, "Gradient"),
            ],
        ),
        (
            "VIEW",
            &[
                (icons::Icon::Hand, Tool::Hand, "Hand (H) — pan"),
                (
                    icons::Icon::Zoom,
                    Tool::Zoom,
                    "Zoom (Z) — click to zoom in, Alt+click out",
                ),
                (icons::Icon::Fit, Tool::Fit, "Fit in Window (Ctrl+0)"),
            ],
        ),
    ];

    for (name, tools) in sections {
        ui.add_space(4.0);
        let (head, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), 16.0), Sense::hover());
        ui.painter().rect_filled(head, 0.0, p.panel_alt);
        ui.painter().rect_filled(
            Rect::from_min_size(head.left_top(), Vec2::new(2.0, head.height())),
            0.0,
            p.accent,
        );
        ui.painter().text(
            pos2(head.left() + 6.0, head.center().y),
            Align2::LEFT_CENTER,
            name,
            FontId::proportional(9.5),
            p.text_dim,
        );
        ui.painter().text(
            pos2(head.right() - 4.0, head.center().y),
            Align2::RIGHT_CENTER,
            tools.len().to_string(),
            FontId::proportional(8.5),
            p.text_dim.gamma_multiply(0.7),
        );
        ui.add_space(2.0);
        for row in tools.chunks(3) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(3.0, 3.0);
                for &(icon, tool, tip) in row {
                    let active = app.tool == tool;
                    if palette_tool_button(ui, icon, active, tip).clicked() {
                        if tool == Tool::Fit {
                            app.fit_in_window();
                        } else {
                            app.tool = tool;
                            app.status = format!("{} selected", tool.name());
                        }
                    }
                }
            });
        }
    }
    ui.add_space(5.0);
    ui.label(
        RichText::new("Hover any tool for its shortcut and behavior.")
            .small()
            .color(p.text_dim.gamma_multiply(0.75)),
    );
}

/// Pinned footer of the tool rail: color chips, swap/default/no-fill, and Full Screen. It sits
/// outside the scrolling tool list so it can never be clipped at the bottom of the window.
pub fn tools_panel_footer(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    if app.tools_collapsed {
        // Single-column variant so the palette and Full Screen stay reachable when the rail is
        // collapsed to one icon column.
        ui.vertical_centered(|ui| {
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
            if ui
                .small_button("⇄")
                .on_hover_text("Swap Fill & Stroke (X)")
                .clicked()
            {
                std::mem::swap(&mut app.fill, &mut app.stroke);
            }
            if ui
                .small_button("▣")
                .on_hover_text("Default Colors (D)")
                .clicked()
            {
                app.fill = Color32::WHITE;
                app.stroke = Color32::BLACK;
            }
            if ui
                .small_button("⊘")
                .on_hover_text("No Fill on Selection")
                .clicked()
            {
                app.set_selected_fill(None);
            }
            if ui
                .small_button("⛶")
                .on_hover_text("Toggle Full-Screen Canvas (Tab / F4)")
                .clicked()
            {
                app.toggle_full_screen_canvas();
            }
        });
        return;
    }
    ui.painter().line_segment(
        [
            ui.available_rect_before_wrap().left_bottom(),
            ui.available_rect_before_wrap().right_bottom(),
        ],
        Stroke::new(1.0, p.stroke),
    );
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
                .small_button("▣")
                .on_hover_text("Default Stroke & Fill Colors (D)")
                .clicked()
            {
                app.fill = Color32::WHITE;
                app.stroke = Color32::BLACK;
            }
            if ui
                .small_button("⊘")
                .on_hover_text("Set No Fill on Selection")
                .clicked()
            {
                app.set_selected_fill(None);
            }
        });
        // The rail is only wide enough for two icon columns; a text button would clip. Use the
        // same full-screen glyph as the collapsed rail, stretched to the available width.
        let full_w = ui.available_width();
        if ui
            .add_sized(
                [full_w, 20.0],
                egui::Button::new(RichText::new("⛶").small()).frame(false),
            )
            .on_hover_text("Toggle Full-Screen Canvas / Hide Panels (Tab or F4)")
            .clicked()
        {
            app.toggle_full_screen_canvas();
        }
    });
}
