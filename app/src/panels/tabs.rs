//! Pages / States / Layers and related tab bodies (CS6 right-dock groups 2-4).

use super::*;

/// Pages, States (Animation frames), and Hierarchical Layers + Objects panel (`screenshots/Fireworks-All_Window_Panels.jpg`).
pub(super) fn pages_panel(ui: &mut Ui, app: &mut FireCraft) {
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
