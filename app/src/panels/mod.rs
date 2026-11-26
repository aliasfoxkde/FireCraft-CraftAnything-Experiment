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

mod bodies;
mod tabs;
use bodies::{
    align_panel, auto_shapes_tab, css_properties_tab, document_library_tab, history_panel,
    image_editing_tab, info_panel, layers_tab, optimize_panel, pages_tab, paths_panel,
    special_chars_tab, states_tab, styles_panel, swatches_panel,
};
pub use bodies::{intelligence_panel, start_panel};
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
pub mod geometry;

#[allow(clippy::too_many_arguments)] // CS6 panel chrome; splitting would churn every caller
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
                                    .scroll_bar_visibility(
                                        egui::scroll_area::ScrollBarVisibility::AlwaysVisible,
                                    )
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
