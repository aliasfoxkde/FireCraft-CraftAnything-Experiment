#![allow(dead_code, unused_imports, unused_variables)]
//! The docked panels: tools, swatches, colour picker, paths, pages/layers,
//! styles, libraries and the properties inspector.

use crate::app::{FireCraft, Tool};
use crate::colors;
use crate::doc::{Layer, Obj};
use crate::icons;

use eframe::egui::{
    Align, Align2, Color32, FontId, Frame, Grid, Label, Layout, Rect, RichText, Sense, Stroke,
    TextEdit, Ui, Vec2, pos2,
};

/// A titled panel box, matching the Fireworks dock look.
pub fn panel(
    ui: &mut Ui,
    title: &str,
    tabs: &[&str],
    tab_index: &mut usize,
    palette: crate::theme::Palette,
    body: impl FnOnce(&mut Ui),
) {
    Frame::new()
        .fill(palette.panel)
        .stroke(Stroke::new(1.0, palette.stroke))
        .inner_margin(egui::Margin::same(3))
        .show(ui, |ui| {
            let (bar, _) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), 18.0),
                Sense::hover(),
            );
            let painter = ui.painter();
            painter.rect_filled(bar, 0.0, palette.panel_alt);
            if tabs.is_empty() {
                painter.text(
                    pos2(bar.left() + 6.0, bar.center().y),
                    Align2::LEFT_CENTER,
                    title,
                    FontId::proportional(11.0),
                    palette.text,
                );
            } else {
                let mut x = bar.left() + 4.0;
                for (i, t) in tabs.iter().enumerate() {
                    let w = 8.0 + t.len() as f32 * 6.0;
                    let r = Rect::from_min_size(pos2(x, bar.top() + 2.0), Vec2::new(w, 14.0));
                    if i == *tab_index {
                        painter.rect_filled(r, 2.0, palette.panel);
                        painter.rect_stroke(r, 2.0, Stroke::new(1.0, palette.stroke), eframe::egui::StrokeKind::Middle);
                    }
                    painter.text(
                        r.center(),
                        Align2::CENTER_CENTER,
                        *t,
                        FontId::proportional(10.5),
                        if i == *tab_index {
                            palette.text
                        } else {
                            palette.text_dim
                        },
                    );
                    let response = ui.interact(
                        r,
                        ui.make_persistent_id(("panel-tab", title, i)),
                        Sense::click(),
                    );
                    if response.clicked() {
                        *tab_index = i;
                    }
                    x += w + 2.0;
                }
            }
            ui.add_space(3.0);
            body(ui);
        });
}

/// The tool palette down the left edge, grouped like Fireworks.
pub fn tools_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let sections: [(&str, &[(icons::Icon, Tool)]); 6] = [
        (
            "Select",
            &[
                (icons::Icon::Select, Tool::Select),
                (icons::Icon::Marquee, Tool::Marquee),
                (icons::Icon::Lasso, Tool::Lasso),
                (icons::Icon::Wand, Tool::Wand),
                (icons::Icon::Crop, Tool::Crop),
            ],
        ),
        (
            "Bitmap",
            &[
                (icons::Icon::Brush, Tool::Brush),
                (icons::Icon::Pencil, Tool::Pencil),
                (icons::Icon::Eraser, Tool::Eraser),
                (icons::Icon::Blur, Tool::Blur),
                (icons::Icon::Sharpen, Tool::Sharpen),
                (icons::Icon::Dodge, Tool::Dodge),
                (icons::Icon::Burn, Tool::Burn),
            ],
        ),
        (
            "Vector",
            &[
                (icons::Icon::Line, Tool::Line),
                (icons::Icon::Rectangle, Tool::Rectangle),
                (icons::Icon::Ellipse, Tool::Ellipse),
                (icons::Icon::Polygon, Tool::Polygon),
                (icons::Icon::Pencil, Tool::Path),
                (icons::Icon::Text, Tool::Text),
            ],
        ),
        (
            "Web",
            &[
                (icons::Icon::Rectangle, Tool::Rectangle),
                (icons::Icon::Ellipse, Tool::Ellipse),
                (icons::Icon::Gradient, Tool::Gradient),
            ],
        ),
        (
            "Colors",
            &[
                (icons::Icon::Picker, Tool::Picker),
                (icons::Icon::Fill, Tool::Fill),
                (icons::Icon::Gradient, Tool::Gradient),
            ],
        ),
        (
            "View",
            &[
                (icons::Icon::Hand, Tool::Hand),
                (icons::Icon::Zoom, Tool::Zoom),
                (icons::Icon::Fit, Tool::Fit),
            ],
        ),
    ];

    for (name, tools) in sections {
        ui.add_space(2.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 15.0), Sense::hover());
        let painter = ui.painter();
        painter.text(
            pos2(rect.left() + 4.0, rect.center().y),
            Align2::LEFT_CENTER,
            name,
            FontId::proportional(10.5),
            p.text_dim,
        );
        painter.line_segment(
            [pos2(rect.left() + 2.0, rect.bottom()), pos2(rect.right() - 2.0, rect.bottom())],
            Stroke::new(1.0, p.stroke),
        );
        ui.horizontal_wrapped(|ui| {
            for (icon, tool) in tools {
                let active = app.tool == *tool;
                let response =
                    crate::canvas::tool_button(ui, *icon, active, &format!("{} tool", name));
                if response.clicked() {
                    app.tool = *tool;
                }
            }
        });
        ui.add_space(2.0);
    }
}

/// Colour swatches, laid out as Fireworks' default ramp.
pub fn swatches_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let cols = 24;
    let cell = 11.0;
    let swatches = app.swatches.clone();
    let mut index = 0;
    while index < swatches.len() {
        ui.horizontal(|ui| {
            for _ in 0..cols {
                if index >= swatches.len() {
                    break;
                }
                let color = swatches[index];
                let (rect, response) =
                    ui.allocate_exact_size(Vec2::splat(cell), Sense::click());
                let painter = ui.painter();
                painter.rect_filled(rect, 0.0, color);
                painter.rect_stroke(rect, 0.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);
                if response.clicked() {
                    app.fill = color;
                    app.stroke = color;
                }
                if response.hovered() {
                    response.on_hover_text(colors::hex_of(color));
                }
                index += 1;
            }
        });
    }
}

/// The colour palette: swatch slots, colour space tabs and sliders.
pub fn color_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;

    // Stroke and fill slots, as in the original.
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::click());
            ui.painter().rect_filled(rect, 2.0, app.stroke);
            ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);
            ui.label(RichText::new("stroke").small().color(p.text_dim));
        });
        ui.vertical(|ui| {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::click());
            ui.painter().rect_filled(rect, 2.0, app.fill);
            ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);
            ui.label(RichText::new("fill").small().color(p.text_dim));
        });
        ui.vertical(|ui| {
            let mut hex = colors::hex_of(app.fill);
            if ui
                .add(
                    TextEdit::singleline(&mut hex)
                        .desired_width(60.0)
                        .text_color(p.text),
                )
                .changed()
            {
                if let Some(c) = colors::from_hex(&hex) {
                    app.fill = c;
                }
            }
            let mut hex2 = colors::hex_of(app.stroke);
            if ui
                .add(
                    TextEdit::singleline(&mut hex2)
                        .desired_width(60.0)
                        .text_color(p.text),
                )
                .changed()
            {
                if let Some(c) = colors::from_hex(&hex2) {
                    app.stroke = c;
                }
            }
        });
    });

    ui.add_space(4.0);

    // Colour space tabs.
    let spaces = ["HLS", "HSV", "CMYK", "RGB"];
    ui.horizontal(|ui| {
        for (i, name) in spaces.iter().enumerate() {
            let selected = app.color_tab == i;
            let color = if selected { p.text } else { p.text_dim };
            if ui
                .add(
                    Label::new(RichText::new(*name).small().color(color)).sense(Sense::click()),
                )
                .clicked()
            {
                app.color_tab = i;
            }
        }
    });

    ui.add_space(4.0);

    // Saturation / value square plus the hue strip.
    let (square, _) = ui.allocate_exact_size(Vec2::new(120.0, 96.0), Sense::click());
    let painter = ui.painter();
    let hsv = colors::rgb_to_hsv(app.fill);
    let steps = 24;
    for iy in 0..steps {
        for ix in 0..steps {
            let s = ix as f32 / (steps - 1) as f32;
            let v = 1.0 - iy as f32 / (steps - 1) as f32;
            let c = colors::hsv_to_rgb(colors::Hsv {
                h: hsv.h,
                s,
                v,
            });
            let cell = Rect::from_min_size(
                pos2(
                    square.left() + square.width() * ix as f32 / steps as f32,
                    square.top() + square.height() * iy as f32 / steps as f32,
                ),
                Vec2::new(
                    square.width() / steps as f32 + 1.0,
                    square.height() / steps as f32 + 1.0,
                ),
            );
            painter.rect_filled(cell, 0.0, c);
        }
    }
    painter.rect_stroke(square, 0.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);

    ui.horizontal(|ui| {
        let (bar, _) = ui.allocate_exact_size(Vec2::new(16.0, 96.0), Sense::click());
        let painter = ui.painter();
        for iy in 0..24 {
            let h = iy as f32 / 24.0 * 360.0;
            let c = colors::hsv_to_rgb(colors::Hsv {
                h,
                s: 1.0,
                v: 1.0,
            });
            let cell = Rect::from_min_size(
                pos2(bar.left(), bar.top() + bar.height() * iy as f32 / 24.0),
                Vec2::new(bar.width(), bar.height() / 24.0 + 1.0),
            );
            painter.rect_filled(cell, 0.0, c);
        }
        painter.rect_stroke(bar, 0.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);

        let mut changed = false;
        ui.vertical(|ui| {
            match app.color_tab {
                0 => {
                    let mut hls = colors::rgb_to_hls(app.fill);
                    changed |= slider_row(ui, "H", &mut hls.h, 0.0, 360.0, "");
                    changed |= slider_row(ui, "L", &mut hls.l, 0.0, 100.0, "%");
                    changed |= slider_row(ui, "S", &mut hls.s, 0.0, 100.0, "%");
                    if changed {
                        app.fill = colors::hls_to_rgb(hls);
                    }
                }
                1 => {
                    let mut hsv = colors::rgb_to_hsv(app.fill);
                    changed |= slider_row(ui, "H", &mut hsv.h, 0.0, 360.0, "");
                    changed |= slider_row(ui, "S", &mut hsv.s, 0.0, 100.0, "%");
                    changed |= slider_row(ui, "V", &mut hsv.v, 0.0, 100.0, "%");
                    if changed {
                        app.fill = colors::hsv_to_rgb(hsv);
                    }
                }
                2 => {
                    let mut cmyk = colors::rgb_to_cmyk(app.fill);
                    changed |= slider_row(ui, "C", &mut cmyk.c, 0.0, 100.0, "%");
                    changed |= slider_row(ui, "M", &mut cmyk.m, 0.0, 100.0, "%");
                    changed |= slider_row(ui, "Y", &mut cmyk.y, 0.0, 100.0, "%");
                    changed |= slider_row(ui, "K", &mut cmyk.k, 0.0, 100.0, "%");
                    if changed {
                        app.fill = colors::cmyk_to_rgb(cmyk);
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
                    }
                }
            }
        });
    });
}

fn slider_row(ui: &mut Ui, label: &str, value: &mut f32, min: f32, max: f32, unit: &str) -> bool {
    ui.style_mut().spacing.slider_width = 70.0;
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).small());
        let mut v = *value;
        let response = ui.add(
            eframe::egui::Slider::new(&mut v, min..=max)
                .custom_formatter(|x, _| format!("{x:.0}{unit}")),
        );
        *value = v;
        response.changed()
    })
    .inner
}

/// The Optimize / History / Path panel with the path command rows.
pub fn paths_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.label(RichText::new("Combine Paths:").small().color(p.text));
    ui.horizontal_wrapped(|ui| {
        for i in 0..8 {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
            let painter = ui.painter();
            painter.rect_stroke(rect, 2.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);
            painter.circle_filled(rect.center(), 3.0, p.text_dim);
            if i == 6 {
                painter.rect_stroke(rect, 2.0, Stroke::new(1.0, p.accent), eframe::egui::StrokeKind::Middle);
            }
        }
    });
    ui.label(RichText::new("Alter Paths:").small().color(p.text));
    ui.horizontal_wrapped(|ui| {
        for i in 0..8 {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
            let painter = ui.painter();
            painter.rect_stroke(rect, 2.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);
            painter.add(egui::Shape::closed_line(
                (0..=10)
                    .map(|i| {
                        let t = std::f32::consts::PI * i as f32 / 10.0;
                        pos2(
                            rect.center().x + 5.0 * t.cos(),
                            rect.center().y + 5.0 * t.sin(),
                        )
                    })
                    .collect::<Vec<_>>(),
                Stroke::new(1.0, p.text_dim),
            ));
        }
    });
    ui.label(RichText::new("Edit Points:").small().color(p.text));
    ui.horizontal_wrapped(|ui| {
        for i in 0..8 {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
            let painter = ui.painter();
            painter.rect_stroke(rect, 2.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);
            painter.circle_filled(rect.center(), 2.5, p.text_dim);
            if i % 2 == 0 {
                painter.line_segment(
                    [rect.left_top(), rect.right_bottom()],
                    Stroke::new(1.0, p.text_dim),
                );
            }
        }
    });
    ui.label(RichText::new("Select Points:").small().color(p.text));
    ui.horizontal_wrapped(|ui| {
        for i in 0..6 {
            let (rect, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
            let painter = ui.painter();
            painter.rect_stroke(rect, 2.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);
            for (dx, dy) in [(-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25)] {
                painter.circle_filled(
                    rect.center() + Vec2::new(dx * rect.width(), dy * rect.height()),
                    2.0,
                    p.text_dim,
                );
            }
        }
    });
    ui.add_space(4.0);
    if app.selected.is_some() {
        ui.label(RichText::new("Selected path").small().color(p.accent));
    }
}

/// Pages, styles and layers.
pub fn pages_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    match app.layers_tab {
        0 => {
            ui.label(RichText::new("Pages").small().color(p.text_dim));
            let count = app.doc.pages.len();
            for i in 0..count {
                let selected = app.doc.current_page == i;
                let name = app.doc.pages[i].name.clone();
                if ui
                    .selectable_label(selected, name)
                    .clicked()
                {
                    app.doc.current_page = i;
                }
            }
            ui.horizontal(|ui| {
                if ui.button("+ Page").clicked() {
                    let page = app.doc.page().clone();
                    let mut new_page = page.clone();
                    new_page.name = format!("Page {}", app.doc.pages.len() + 1);
                    app.doc.pages.push(new_page);
                    app.doc.current_page = app.doc.pages.len() - 1;
                }
                if ui.button("Duplicate").clicked() && count > 0 {
                    let page = app.doc.page().clone();
                    let mut new_page = page.clone();
                    new_page.name = format!("{} copy", page.name);
                    app.doc.pages.push(new_page);
                    app.doc.current_page = app.doc.pages.len() - 1;
                }
            });
        }
        1 => {
            ui.label(RichText::new("Styles").small().color(p.text_dim));
            for name in [
                "Current Document",
                "FireCraft Soft",
                "FireCraft Sharp",
                "FireCraft Glow",
            ] {
                if ui.selectable_label(false, name).clicked() {
                    app.status = format!("Applied style: {name}");
                }
            }
        }
        _ => {
            let layers = app.doc.page().layers.clone();
            for (i, layer) in layers.iter().enumerate().rev() {
                ui.horizontal(|ui| {
                    let mut visible = layer.visible;
                    let mut locked = layer.locked;
                    ui.checkbox(&mut visible, "");
                    ui.checkbox(&mut locked, "");
                    let name = layer.name.clone();
                    if ui
                        .selectable_label(app.selected_layer == i, name)
                        .clicked()
                    {
                        app.selected_layer = i;
                        app.selected = None;
                    }
                });
            }
            ui.horizontal(|ui| {
                if ui.button("New Layer").clicked() {
                    let mut layer = Layer::new(&format!("Layer {}", app.doc.page().layers.len() + 1));
                    layer.opacity = 1.0;
                    app.doc.page_mut().layers.push(layer);
                    app.selected_layer = app.doc.page().layers.len() - 1;
                }
                if ui.button("Delete").clicked() {
                    let layers = app.doc.page_mut().layers.len();
                    if layers > 1 && app.selected_layer < layers {
                        app.doc.page_mut().layers.remove(app.selected_layer);
                        app.selected_layer = app.selected_layer.saturating_sub(1);
                        app.selected = None;
                    }
                }
            });
            let mut opacity = app.doc.page().layers[app.selected_layer].opacity;
            if ui
                .add(
                    eframe::egui::Slider::new(&mut opacity, 0.0..=1.0)
                        .text("Opacity"),
                )
                .changed()
            {
                app.doc.page_mut().layers[app.selected_layer].opacity = opacity;
            }
        }
    }
}

/// The styles panel under pages/layers.
pub fn styles_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    egui::ComboBox::from_id_salt("styles")
        .selected_text(&app.style_name)
        .width(ui.available_width())
        .show_ui(ui, |ui| {
            for name in ["Current Document", "FireCraft Soft", "FireCraft Sharp"] {
                if ui.selectable_label(app.style_name == name, name).clicked() {
                    app.style_name = name.to_string();
                }
            }
        });
    Frame::new()
        .fill(p.well)
        .stroke(Stroke::new(1.0, p.stroke))
        .inner_margin(egui::Margin::same(3))
        .show(ui, |ui| {
            ui.add_space(40.0);
        });
}

/// Character, image-editing and auto-shape tabs.
pub fn library_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    match app.library_tab {
        0 => {
            let glyphs = [
                "a b c d e f g h i j k l m n o p q r s t u v w x y z",
                "A B C D E F G H I J K L M N O P Q R S T U V W X Y Z",
                "0 1 2 3 4 5 6 7 8 9 . , : ; ! ? ' \" ( ) [ ] - _ +",
                "@ # $ % & * / \\ = < > | ~ ^ ` { } $ € £ ¥ © ® ™ °",
                "¡ ¢ £ ¤ ¥ ¦ § ¨ © ª « ¬ ® ¯ ° ± ² ³ ´ µ ¶ · ¸",
                "À Á Â Ã Ä Å Æ Ç È É Ê Ë Ì Í Î Ï Ð Ñ Ò Ó Ô Õ Ö × Ø Ù",
                "á â ã ä å æ ç è é ê ë ì í î ï ð ñ ò ó ô õ ö ÷ ø ù",
            ];
            for line in glyphs {
                ui.label(RichText::new(line).small().color(p.text));
            }
        }
        1 => {
            ui.label(RichText::new("Image Editing").small().color(p.text_dim));
            for name in [
                "Brightness/Contrast",
                "Levels",
                "Curves",
                "Hue/Saturation",
                "Color Balance",
                "Black and White",
                "Invert",
            ] {
                if ui.button(name).clicked() {
                    app.status = format!("Filter: {name}");
                }
            }
        }
        _ => {
            ui.label(RichText::new("Auto Shapes").small().color(p.text_dim));
            Grid::new("autoshapes")
                .spacing([6.0, 6.0])
                .show(ui, |ui| {
                    for i in 0..12 {
                        let (rect, _) = ui.allocate_exact_size(Vec2::splat(18.0), Sense::hover());
                        let painter = ui.painter();
                        painter.rect_stroke(rect, 2.0, Stroke::new(1.0, p.stroke), eframe::egui::StrokeKind::Middle);
                        painter.circle_filled(rect.center(), 4.0, p.text_dim);
                    }
                });
        }
    }
}

/// The properties inspector at the bottom left.
pub fn properties_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let obj: Option<Obj> = app
        .selected
        .and_then(|i| app.doc.page().layers.get(app.selected_layer).map(|l| l.objects[i].clone()));
    match obj {
        Some(obj) => {
            Grid::new("props")
                .num_columns(4)
                .spacing([16.0, 3.0])
                .show(ui, |ui| {
                    ui.label(RichText::new("Name").small().color(p.text_dim));
                    let mut name = obj.name.clone();
                    if ui
                        .add(TextEdit::singleline(&mut name).desired_width(120.0))
                        .changed()
                    {
                        app.set_selected_name(name);
                    }
                    ui.label(RichText::new("Type").small().color(p.text_dim));
                    ui.label(
                        RichText::new(match obj.kind {
                            crate::doc::Shape::Rect { .. } => "Rectangle",
                            crate::doc::Shape::Ellipse { .. } => "Ellipse",
                            crate::doc::Shape::Line { .. } => "Line",
                            crate::doc::Shape::Path { .. } => "Path",
                            crate::doc::Shape::Text { .. } => "Text",
                            crate::doc::Shape::Bitmap { .. } => "Bitmap",
                        })
                        .small(),
                    );
                    if let Some((lo, hi)) = obj.bounds() {
                        ui.label(RichText::new("W").small().color(p.text_dim));
                        ui.label(
                            RichText::new(format!("{:.0}", hi.x - lo.x)).small().color(p.text),
                        );
                        ui.label(RichText::new("H").small().color(p.text_dim));
                        ui.label(
                            RichText::new(format!("{:.0}", hi.y - lo.y)).small().color(p.text),
                        );
                        ui.label(RichText::new("X").small().color(p.text_dim));
                        ui.label(
                            RichText::new(format!("{:.0}", lo.x)).small().color(p.text),
                        );
                        ui.label(RichText::new("Y").small().color(p.text_dim));
                        ui.label(
                            RichText::new(format!("{:.0}", lo.y)).small().color(p.text),
                        );
                    }
                    ui.label(RichText::new("Fill").small().color(p.text_dim));
                    let mut hex = obj
                        .fill
                        .map(colors::hex_of)
                        .unwrap_or_else(|| "none".to_string());
                    if ui.add(TextEdit::singleline(&mut hex).desired_width(70.0)).changed() {
                        app.set_selected_fill(colors::from_hex(&hex));
                    }
                    ui.label(RichText::new("Stroke").small().color(p.text_dim));
                    let mut hex2 = obj
                        .stroke
                        .map(colors::hex_of)
                        .unwrap_or_else(|| "none".to_string());
                    if ui.add(TextEdit::singleline(&mut hex2).desired_width(70.0)).changed() {
                        app.set_selected_stroke(colors::from_hex(&hex2));
                    }
                    ui.label(RichText::new("Opacity").small().color(p.text_dim));
                    let mut width = obj.width;
                    if ui
                        .add(
                            eframe::egui::Slider::new(&mut width, 0.0..=12.0)
                                .text(""),
                        )
                        .changed()
                    {
                        app.set_selected_width(width);
                    }
                });
            if let crate::doc::Shape::Text { text, .. } = &obj.kind {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Text").small().color(p.text_dim));
                    let mut edit = text.clone();
                    if ui
                        .add(TextEdit::singleline(&mut edit).desired_width(200.0))
                        .changed()
                    {
                        app.set_selected_text(edit);
                    }
                });
            }
        }
        None => {
            ui.label(
                RichText::new("No object selected.")
                    .small()
                    .color(p.text_dim),
            );
        }
    }
}

pub fn start_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    let width = 640.0;
    let height = 400.0;
    let rect = ui.available_rect_before_wrap();
    let center = rect.center();
    let panel_rect = Rect::from_center_size(center, Vec2::new(width, height));
    
    // Transparent splash background (like original Fireworks CS6)
    let painter = ui.painter();
    painter.rect_filled(panel_rect, 8.0, p.well.gamma_multiply(0.85));
    painter.rect_stroke(panel_rect, 4.0, Stroke::new(1.0, p.accent.gamma_multiply(0.5)), eframe::egui::StrokeKind::Middle);
    
    // Title bar
    let title_bar = Rect::from_min_size(panel_rect.left_top(), Vec2::new(width, 40.0));
    painter.rect_filled(title_bar, 8.0, p.banner);
    painter.text(
        pos2(title_bar.left() + 12.0, title_bar.center().y),
        Align2::LEFT_CENTER,
        "FireCraft",
        FontId::proportional(18.0),
        Color32::WHITE,
    );
    
    // Logo if available
    if let Some(ref tex) = app.logo_tex {
        let logo_rect = Rect::from_center_size(
            pos2(center.x, panel_rect.center().y - 20.0),
            Vec2::new(160.0, 45.0),
        );
        painter.text(
            pos2(logo_rect.left() + logo_rect.width()/2.0, logo_rect.center().y),
            Align2::CENTER_CENTER,
            "FireCraft",
            FontId::proportional(18.0),
            p.accent,
        );
    } else {
        painter.text(
            pos2(center.x, panel_rect.center().y),
            Align2::CENTER_CENTER,
            "FireCraft",
            FontId::proportional(32.0),
            p.accent,
        );
    }
    
    // Description
    painter.text(
        pos2(center.x, panel_rect.bottom() - 50.0),
        Align2::CENTER_TOP,
        "A modern vector and bitmap editor — based on Adobe Fireworks CS6",
        FontId::proportional(12.0),
        p.text_dim,
    );
    
    // Close button
    let btn = Rect::from_min_size(
        pos2(panel_rect.right() - 45.0, title_bar.top() + 8.0),
        Vec2::new(32.0, 20.0),
    );
    let response = ui.interact(btn, ui.make_persistent_id("close-btn"), Sense::click());
    painter.rect_filled(btn, 4.0, p.stroke);
    painter.text(
        btn.center(),
        Align2::CENTER_CENTER,
        "✕",
        FontId::proportional(10.0),
        p.text,
    );
    if response.clicked() {
        app.show_start = false;
        app.status = "Editor ready".to_string();
    }
    
    ui.advance_cursor_after_rect(panel_rect);
}

/// Small helper for the status line text.
pub fn status_text(text: &str, p: crate::theme::Palette) -> Label {
    Label::new(
        RichText::new(text).small().color(p.text_dim),
    )
}
