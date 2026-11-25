//! Left tool palette: CS6's six tool categories, grouped tools with flyout triangles,
//! Auto Shape controls, and the pinned color/display footer.

use super::*;

#[derive(Clone, Copy)]
struct ToolSpec {
    icon: icons::Icon,
    tool: Tool,
    tip: &'static str,
}

struct ToolGroup {
    name: &'static str,
    members: &'static [ToolSpec],
}

impl ToolGroup {
    fn leader(&self) -> ToolSpec {
        self.members[0]
    }

    fn displayed(&self, active: Tool) -> ToolSpec {
        self.members
            .iter()
            .copied()
            .find(|spec| spec.tool == active)
            .unwrap_or_else(|| self.leader())
    }

    fn contains(&self, tool: Tool) -> bool {
        self.members.iter().any(|spec| spec.tool == tool)
    }
}

struct ToolSection {
    name: &'static str,
    groups: &'static [ToolGroup],
}

const fn spec(icon: icons::Icon, tool: Tool, tip: &'static str) -> ToolSpec {
    ToolSpec { icon, tool, tip }
}

static SECTIONS: &[ToolSection] = &[
    ToolSection {
        name: "SELECT",
        groups: &[
            ToolGroup {
                name: "Pointer",
                members: &[spec(
                    icons::Icon::Select,
                    Tool::Select,
                    "Pointer (V) — select and move",
                )],
            },
            ToolGroup {
                name: "Subselection",
                members: &[spec(
                    icons::Icon::Subselect,
                    Tool::Subselect,
                    "Subselection (A) — edit path nodes",
                )],
            },
            ToolGroup {
                name: "Transform",
                members: &[spec(
                    icons::Icon::Scale,
                    Tool::Scale,
                    "Scale / Transform (Q)",
                )],
            },
            ToolGroup {
                name: "Crop",
                members: &[spec(
                    icons::Icon::Crop,
                    Tool::Crop,
                    "Crop (C) — drag to crop the canvas",
                )],
            },
            ToolGroup {
                name: "Marquee",
                members: &[
                    spec(icons::Icon::Marquee, Tool::Marquee, "Marquee Select (M)"),
                    spec(icons::Icon::Marquee, Tool::OvalMarquee, "Oval Marquee"),
                ],
            },
            ToolGroup {
                name: "Lasso",
                members: &[spec(icons::Icon::Lasso, Tool::Lasso, "Lasso Select (L)")],
            },
            ToolGroup {
                name: "Magic Wand",
                members: &[spec(icons::Icon::Wand, Tool::Wand, "Magic Wand (W)")],
            },
        ],
    },
    ToolSection {
        name: "BITMAP",
        groups: &[
            ToolGroup {
                name: "Brush",
                members: &[
                    spec(icons::Icon::Brush, Tool::Brush, "Brush (B)"),
                    spec(icons::Icon::Pencil, Tool::Pencil, "Pencil (B)"),
                ],
            },
            ToolGroup {
                name: "Eraser",
                members: &[spec(icons::Icon::Eraser, Tool::Eraser, "Eraser (E)")],
            },
            ToolGroup {
                name: "Retouch",
                members: &[
                    spec(icons::Icon::Blur, Tool::Blur, "Blur"),
                    spec(icons::Icon::Sharpen, Tool::Sharpen, "Sharpen"),
                    spec(icons::Icon::Dodge, Tool::Dodge, "Dodge — lighten"),
                    spec(icons::Icon::Burn, Tool::Burn, "Burn — darken"),
                ],
            },
            ToolGroup {
                name: "Rubber Stamp",
                members: &[
                    spec(
                        icons::Icon::Stamp,
                        Tool::Stamp,
                        "Rubber Stamp (S) — duplicate at cursor",
                    ),
                    spec(
                        icons::Icon::Fill,
                        Tool::ReplaceColor,
                        "Replace Color — click a fill to find/replace it",
                    ),
                    spec(
                        icons::Icon::Eye,
                        Tool::RedEye,
                        "Red Eye Removal — correct red-dominant pixels",
                    ),
                ],
            },
        ],
    },
    ToolSection {
        name: "VECTOR",
        groups: &[
            ToolGroup {
                name: "Line",
                members: &[spec(icons::Icon::Line, Tool::Line, "Line (N)")],
            },
            ToolGroup {
                name: "Pen",
                members: &[
                    spec(
                        icons::Icon::Pen,
                        Tool::Path,
                        "Pen (P) — click points; double-click or Enter finishes",
                    ),
                    spec(
                        icons::Icon::Pencil,
                        Tool::Freeform,
                        "Freeform Path (F) — draw with the current stroke",
                    ),
                ],
            },
            ToolGroup {
                name: "Auto Shapes",
                members: &[
                    spec(icons::Icon::Rectangle, Tool::Rectangle, "Rectangle (U)"),
                    spec(icons::Icon::Rounded, Tool::RoundedRect, "Rounded Rectangle"),
                    spec(icons::Icon::Ellipse, Tool::Ellipse, "Ellipse"),
                    spec(icons::Icon::Polygon, Tool::Polygon, "Polygon Auto Shape"),
                    spec(icons::Icon::Star, Tool::Star, "Star Auto Shape"),
                ],
            },
            ToolGroup {
                name: "Text",
                members: &[spec(icons::Icon::Text, Tool::Text, "Text (T)")],
            },
            ToolGroup {
                name: "Knife",
                members: &[spec(
                    icons::Icon::Knife,
                    Tool::Knife,
                    "Knife — split the selected path",
                )],
            },
        ],
    },
    ToolSection {
        name: "WEB",
        groups: &[ToolGroup {
            name: "Web Objects",
            members: &[
                spec(icons::Icon::Slice, Tool::Slice, "Rectangle Slice (K)"),
                spec(icons::Icon::Hotspot, Tool::Hotspot, "Rectangle Hotspot (J)"),
            ],
        }],
    },
    ToolSection {
        name: "COLORS",
        groups: &[
            ToolGroup {
                name: "Eyedropper",
                members: &[spec(icons::Icon::Picker, Tool::Picker, "Eyedropper (I)")],
            },
            ToolGroup {
                name: "Paint Bucket",
                members: &[
                    spec(icons::Icon::Fill, Tool::Fill, "Paint Bucket (G)"),
                    spec(icons::Icon::Gradient, Tool::Gradient, "Gradient (G)"),
                ],
            },
        ],
    },
    ToolSection {
        name: "VIEW",
        groups: &[
            ToolGroup {
                name: "Hand",
                members: &[spec(icons::Icon::Hand, Tool::Hand, "Hand (H) — pan")],
            },
            ToolGroup {
                name: "Zoom",
                members: &[spec(
                    icons::Icon::Zoom,
                    Tool::Zoom,
                    "Zoom (Z) — click to zoom in, Alt+click out",
                )],
            },
            ToolGroup {
                name: "Fit",
                members: &[spec(icons::Icon::Fit, Tool::Fit, "Fit in Window (Ctrl+0)")],
            },
        ],
    },
];

fn all_groups() -> Vec<&'static ToolGroup> {
    SECTIONS
        .iter()
        .flat_map(|section| section.groups.iter())
        .collect()
}

fn palette_tool_button(
    ui: &mut Ui,
    spec: ToolSpec,
    active: bool,
    flyout: bool,
    compact: bool,
) -> (egui::Response, egui::Rect) {
    let p = ui.visuals().clone();
    let side = if compact { 20.0 } else { 24.0 };
    let size = Vec2::new(side, side);
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
        icons::draw(ui.painter(), rect, spec.icon, color, 1.35);
        // CS6's small corner triangle marks a group leader. It is also the click target
        // that opens the flyout, so the grouping is visible rather than a hidden gesture.
        if flyout {
            let a = pos2(rect.right() - 9.0, rect.bottom() - 1.5);
            ui.painter().add(egui::Shape::convex_polygon(
                vec![a, pos2(a.x + 7.0, a.y), pos2(a.x + 7.0, a.y - 7.0)],
                p.selection.stroke.color,
                Stroke::NONE,
            ));
        }
    }
    (response, rect)
}

fn vector_options(ui: &mut Ui, app: &mut FireCraft, compact: bool) {
    let p = app.palette;
    ui.add_space(if compact { 1.0 } else { 2.0 });
    let mut sides = app.polygon_sides;
    let mut percent = app.star_ratio * 100.0;
    if compact {
        // One narrow row keeps the Auto Shape controls visible without pushing the
        // Colors and View categories out of a 960x640 dock.
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);
            ui.label(RichText::new("S").small().color(p.text_dim));
            ui.add(
                egui::DragValue::new(&mut sides)
                    .speed(1.0)
                    .range(3..=24)
                    .max_decimals(0),
            )
            .on_hover_text("Polygon / Star point count");
            ui.label(RichText::new("★").small().color(p.text_dim));
            ui.add(
                egui::DragValue::new(&mut percent)
                    .speed(1.0)
                    .range(10.0..=100.0)
                    .suffix("%")
                    .max_decimals(0),
            )
            .on_hover_text("Star inner radius percentage");
        });
    } else {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Sides").small().color(p.text_dim));
            ui.add(
                egui::DragValue::new(&mut sides)
                    .speed(1.0)
                    .range(3..=24)
                    .max_decimals(0),
            )
            .on_hover_text("Polygon / Star point count");
        });
        ui.horizontal(|ui| {
            ui.label(RichText::new("Star").small().color(p.text_dim));
            ui.add(
                egui::DragValue::new(&mut percent)
                    .speed(1.0)
                    .range(10.0..=100.0)
                    .suffix("%")
                    .max_decimals(0),
            )
            .on_hover_text("Star inner radius as a percentage of the outer radius");
        });
        ui.add(
            egui::Label::new(
                RichText::new("Drag a shape on canvas; the amber diamond adjusts it.")
                    .small()
                    .color(p.text_dim.gamma_multiply(0.8)),
            )
            .wrap(),
        );
    }
    app.polygon_sides = sides;
    app.star_ratio = percent / 100.0;
}

pub fn tools_panel_body(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    // At a 960x640 window the dock body is under ~500 px. Tighten rows and drop the
    // explanatory footer so all six categories, including View, stay visible without
    // scrolling or clipping.
    let compact = ui.available_height() < 500.0;
    let mut flat_index = 0usize;
    let mut open_flyout: Option<(usize, egui::Pos2)> = None;
    let mut group_count = 0usize;
    let mut tool_count = 0usize;

    for section in SECTIONS {
        ui.add_space(if compact { 1.0 } else { 4.0 });
        let head_h = if compact { 12.0 } else { 16.0 };
        let (head, _) =
            ui.allocate_exact_size(Vec2::new(ui.available_width(), head_h), Sense::hover());
        ui.painter().rect_filled(head, 0.0, p.panel_alt);
        ui.painter().rect_filled(
            Rect::from_min_size(head.left_top(), Vec2::new(2.0, head.height())),
            0.0,
            p.accent,
        );
        ui.painter().text(
            pos2(head.left() + 6.0, head.center().y),
            Align2::LEFT_CENTER,
            section.name,
            FontId::proportional(9.5),
            p.text_dim,
        );
        let members = section
            .groups
            .iter()
            .map(|g| g.members.len())
            .sum::<usize>();
        ui.painter().text(
            pos2(head.right() - 4.0, head.center().y),
            Align2::RIGHT_CENTER,
            format!("{}·{}", section.groups.len(), members),
            FontId::proportional(8.0),
            p.text_dim.gamma_multiply(0.7),
        );
        ui.add_space(if compact { 1.0 } else { 2.0 });

        for row in section.groups.chunks(3) {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(3.0, 3.0);
                for group in row {
                    let group_index = flat_index;
                    flat_index += 1;
                    group_count += 1;
                    tool_count += group.members.len();
                    let shown = group.displayed(app.tool);
                    let active = group.contains(app.tool);
                    let flyout = group.members.len() > 1;
                    let (response, rect) =
                        palette_tool_button(ui, shown, active, flyout, compact);
                    let tip = if flyout {
                        format!(
                            "{} — {} tools. Click the corner triangle or right-click for the group.",
                            group.name,
                            group.members.len()
                        )
                    } else {
                        format!("{} · {}", shown.tip, group.name)
                    };
                    let response = response.on_hover_text(tip);
                    let corner_click = response.interact_pointer_pos().is_some_and(|pos| {
                        pos.x >= rect.right() - 10.0 && pos.y >= rect.bottom() - 10.0
                    });
                    if response.clicked() {
                        if flyout && corner_click {
                            open_flyout = Some((
                                group_index,
                                pos2(rect.right() + 2.0, rect.top() - 2.0),
                            ));
                        } else if shown.tool == Tool::Fit {
                            app.fit_in_window();
                        } else {
                            app.set_tool(shown.tool);
                            app.status = format!("{} selected", shown.tool.name());
                        }
                    }
                    if flyout && response.secondary_clicked() {
                        open_flyout =
                            Some((group_index, pos2(rect.right() + 2.0, rect.top() - 2.0)));
                    }
                }
            });
        }
        if section.name == "VECTOR" {
            vector_options(ui, app, compact);
        }
    }

    let opened_now = open_flyout.is_some();
    if let Some(open) = open_flyout {
        app.tool_flyout = Some(open);
    }
    tool_group_flyout(ui, app, opened_now);

    if !compact {
        ui.add_space(4.0);
        ui.label(
            RichText::new(format!(
                "{group_count} tool groups · {tool_count} tools · corner triangle opens a group"
            ))
            .small()
            .color(p.text_dim.gamma_multiply(0.75)),
        );
    }
}

/// The open group flyout. It is anchored beside the tool strip and lists every member with
/// its shortcut, including members hidden behind the leader button.
fn tool_group_flyout(ui: &mut Ui, app: &mut FireCraft, opened_now: bool) {
    if app.tool_flyout.is_none() {
        return;
    }
    let groups = all_groups();
    let Some(&(group_index, anchor)) = app.tool_flyout.as_ref() else {
        return;
    };
    let Some(group) = groups.get(group_index).copied() else {
        app.tool_flyout = None;
        return;
    };
    let p = app.palette;
    let mut chosen = None;
    let mut close = false;
    let area = egui::Area::new(ui.make_persistent_id("tool_group_flyout"))
        .order(egui::Order::Foreground)
        .fixed_pos(anchor)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style())
                .fill(p.panel)
                .stroke(Stroke::new(1.0, p.accent))
                .show(ui, |ui| {
                    ui.set_min_width(188.0);
                    ui.label(RichText::new(group.name).small().strong().color(p.text_dim));
                    ui.separator();
                    for spec in group.members {
                        let selected = app.tool == spec.tool;
                        if ui
                            .selectable_label(
                                selected,
                                format!("{}  ·  {}", spec.tip, spec.tool.name()),
                            )
                            .clicked()
                        {
                            chosen = Some(spec.tool);
                        }
                    }
                });
        });
    if let Some(tool) = chosen {
        app.set_tool(tool);
        app.status = format!("{} selected from {} group", tool.name(), group.name);
        close = true;
    } else if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
        close = true;
    } else if !opened_now
        && let Some(pointer) = ui.ctx().pointer_interact_pos()
        && ui.input(|i| i.pointer.any_click())
        && !area.response.rect.contains(pointer)
    {
        close = true;
    }
    if close {
        app.tool_flyout = None;
    }
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
                .small_button("F")
                .on_hover_text("Full-Screen Canvas (Tab / F4)")
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
        if ui
            .small_button(if app.full_screen_canvas {
                "Exit Full"
            } else {
                "Full Screen"
            })
            .on_hover_text("Full-Screen Canvas / Hide Panels (Tab or F4)")
            .clicked()
        {
            app.toggle_full_screen_canvas();
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn categories_are_cs6s_six_in_order() {
        let names: Vec<&str> = SECTIONS.iter().map(|section| section.name).collect();
        assert_eq!(
            names,
            ["SELECT", "BITMAP", "VECTOR", "WEB", "COLORS", "VIEW"]
        );
    }

    #[test]
    fn every_tool_appears_once_and_groups_resolve_the_active_member() {
        let mut seen = Vec::new();
        for group in all_groups() {
            assert!(!group.members.is_empty());
            for member in group.members {
                assert!(
                    !seen.contains(&member.tool),
                    "{} appears in more than one tool group",
                    member.tool.name()
                );
                seen.push(member.tool);
            }
            for member in group.members {
                assert_eq!(group.displayed(member.tool).tool, member.tool);
            }
        }
    }

    #[test]
    fn auto_shapes_group_exposes_the_dynamic_shape_tools() {
        let group = all_groups()
            .into_iter()
            .find(|group| group.name == "Auto Shapes")
            .expect("Auto Shapes group");
        for tool in [
            Tool::Rectangle,
            Tool::RoundedRect,
            Tool::Ellipse,
            Tool::Polygon,
            Tool::Star,
        ] {
            assert!(group.contains(tool), "{} missing", tool.name());
        }
    }
}
