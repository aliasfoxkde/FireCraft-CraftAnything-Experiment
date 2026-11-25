//! Properties Inspector panel body.

use super::*;

/// The bottom Properties Inspector (CS6 bottom dock):
/// When an object is selected: Name, Type, editable X/Y/W/H, Fill, Stroke, Width, Opacity,
/// Corner Radius / Star Points / Text Font / Slice URL, plus Live Filters (+) stack.
/// When nothing is selected: Document Canvas Size, Background Color, Resolution, Fit Canvas.
pub fn properties_panel(ui: &mut Ui, app: &mut FireCraft) {
    // The inspector dock can be as short as 118 px; without a scroll area the second
    // property row was painted past the panel edge and vanished under the frame.
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| properties_body(ui, app));
}

fn properties_body(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    if app.selected_set.len() >= 2 {
        let primary = app.selected_object();
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
            ui.separator();
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Selection Appearance:")
                        .small()
                        .color(p.text_dim),
                );
                ui.horizontal(|ui| {
                    let mut fill = primary.as_ref().and_then(|o| o.fill).unwrap_or(app.fill);
                    let mut fc = [
                        fill.r() as f32 / 255.0,
                        fill.g() as f32 / 255.0,
                        fill.b() as f32 / 255.0,
                    ];
                    if ui
                        .color_edit_button_rgb(&mut fc)
                        .on_hover_text("Fill all selected objects")
                        .changed()
                    {
                        fill = Color32::from_rgb(
                            (fc[0] * 255.0) as u8,
                            (fc[1] * 255.0) as u8,
                            (fc[2] * 255.0) as u8,
                        );
                        app.fill = fill;
                        app.set_selected_fill(Some(fill));
                    }
                    let mut stroke = primary
                        .as_ref()
                        .and_then(|o| o.stroke)
                        .unwrap_or(app.stroke);
                    let mut sc = [
                        stroke.r() as f32 / 255.0,
                        stroke.g() as f32 / 255.0,
                        stroke.b() as f32 / 255.0,
                    ];
                    if ui
                        .color_edit_button_rgb(&mut sc)
                        .on_hover_text("Stroke all selected objects")
                        .changed()
                    {
                        stroke = Color32::from_rgb(
                            (sc[0] * 255.0) as u8,
                            (sc[1] * 255.0) as u8,
                            (sc[2] * 255.0) as u8,
                        );
                        app.stroke = stroke;
                        app.set_selected_stroke(Some(stroke));
                    }
                    let mut opacity = primary.as_ref().map(|o| o.opacity * 100.0).unwrap_or(100.0);
                    if ui
                        .add(
                            egui::DragValue::new(&mut opacity)
                                .range(0.0..=100.0)
                                .suffix("%"),
                        )
                        .on_hover_text("Opacity for all selected objects")
                        .changed()
                    {
                        app.set_selected_opacity(opacity / 100.0);
                    }
                    if ui.small_button("+ Shadow").clicked() {
                        app.add_live_effect(LiveEffect::DropShadow {
                            distance: 5.0,
                            angle_deg: 45.0,
                            blur: 5.0,
                            color: Color32::from_black_alpha(150),
                            enabled: true,
                        });
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
                        let mut fc = obj.fill.unwrap_or(app.fill);
                        let mut has_fill = obj.fill.is_some();
                        if cs6_color_button(
                            ui,
                            "prop_obj_fill_swatch",
                            p,
                            &mut fc,
                            Some(&mut has_fill),
                        ) {
                            if has_fill {
                                app.fill = fc;
                                app.set_selected_fill(Some(fc));
                            } else {
                                app.set_selected_fill(None);
                            }
                        }

                        ui.label(RichText::new("Stroke:").small().color(p.text_dim));
                        let mut sc = obj.stroke.unwrap_or(app.stroke);
                        let mut has_stroke = obj.stroke.is_some();
                        if cs6_color_button(
                            ui,
                            "prop_obj_stroke_swatch",
                            p,
                            &mut sc,
                            Some(&mut has_stroke),
                        ) {
                            if has_stroke {
                                app.stroke = sc;
                                app.set_selected_stroke(Some(sc));
                            } else {
                                app.set_selected_stroke(None);
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
                            font_family,
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
                            let mut family = font_family.clone();
                            egui::ComboBox::from_id_salt("prop_text_font")
                                .width(92.0)
                                .selected_text(family.as_str())
                                .show_ui(ui, |ui| {
                                    for fam in
                                        ["Sans-Serif", "Serif", "Monospace", "Display", "Geometric"]
                                    {
                                        if ui.selectable_label(family == fam, fam).clicked() {
                                            family = fam.to_string();
                                            app.set_selected_font_family(fam);
                                        }
                                    }
                                });
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
                                RichText::new(format!("Layer: {}", app.selected_layer + 1))
                                    .small()
                                    .color(p.text_dim),
                            );
                            ui.label(RichText::new("Blend:").small().color(p.text_dim));
                            egui::ComboBox::from_id_salt("prop_blend_mode")
                                .width(96.0)
                                .selected_text(obj.blend_mode.label())
                                .show_ui(ui, |ui| {
                                    for mode in BlendMode::ALL {
                                        if ui
                                            .selectable_label(obj.blend_mode == mode, mode.label())
                                            .clicked()
                                        {
                                            app.set_selected_blend_mode(mode);
                                        }
                                    }
                                });
                        }
                    });

                    // Gradient fill editor (styles with gradient_end create these automatically).
                    ui.horizontal(|ui| {
                        if let Some(g) = obj.gradient {
                            ui.label(RichText::new("Gradient:").small().color(p.text_dim));
                            let mut start = g.start;
                            let mut end = g.end;
                            let mut changed = false;
                            if cs6_color_button(ui, "prop_grad_start", p, &mut start, None) {
                                changed = true;
                            }
                            ui.label(RichText::new("→").small().color(p.text_dim));
                            if cs6_color_button(ui, "prop_grad_end", p, &mut end, None) {
                                changed = true;
                            }
                            if changed {
                                app.set_selected_gradient(Some(GradientFill { start, end }));
                            }
                            if ui
                                .small_button("Remove")
                                .on_hover_text("Switch back to a flat fill")
                                .clicked()
                            {
                                app.set_selected_gradient(None);
                            }
                        } else if ui
                            .small_button("+ Gradient")
                            .on_hover_text("Use a linear start→end gradient as this object's fill")
                            .clicked()
                        {
                            let start = obj.fill.unwrap_or(app.fill);
                            let end = obj.stroke.unwrap_or(app.stroke);
                            app.set_selected_gradient(Some(GradientFill { start, end }));
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
                        let mut bg_c = app.doc.page().bg_color;
                        let mut opaque_bg = !app.doc.page().transparent_bg;
                        if cs6_color_button(
                            ui,
                            "prop_canvas_bg_swatch",
                            p,
                            &mut bg_c,
                            Some(&mut opaque_bg),
                        ) {
                            let page = app.doc.page_mut();
                            page.bg_color = bg_c;
                            page.transparent_bg = !opaque_bg;
                        }
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
