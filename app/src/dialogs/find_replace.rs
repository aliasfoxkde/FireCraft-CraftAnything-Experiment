//! Find & Replace dialog: text/name and exact color modes with a live scope preview.

use super::*;

/// Find and Replace dialog (Edit > Find and Replace), with Fireworks-style color replacement.
pub fn find_replace_dialog(app: &mut FireCraft) {
    if !app.show_find_replace {
        return;
    }
    let p = app.palette;
    let mut open = app.show_find_replace;
    let mut close_now = false;
    let center = app.ctx.content_rect().center();
    egui::Window::new("Find and Replace")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .default_width(486.0)
        .min_width(460.0)
        .default_pos(center - Vec2::new(243.0, 190.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "Find and Replace",
                "Text, object names, and exact Fireworks color channels — all pages, including hidden objects.",
            );
            ui.horizontal(|ui| {
                ui.label(RichText::new("Search for").strong());
                egui::ComboBox::from_id_salt("find_replace_mode")
                    .width(190.0)
                    .selected_text(app.find_replace_mode.label())
                    .show_ui(ui, |ui| {
                        for mode in FindReplaceMode::ALL {
                            if ui
                                .selectable_label(app.find_replace_mode == mode, mode.label())
                                .clicked()
                            {
                                app.find_replace_mode = mode;
                            }
                        }
                    });
                if app.find_replace_mode == FindReplaceMode::Text {
                    ui.checkbox(&mut app.find_case_sensitive, "Match case");
                }
            });
            ui.add_space(4.0);

            if app.find_replace_mode == FindReplaceMode::Text {
                egui::Grid::new("find_replace_text_grid")
                    .num_columns(2)
                    .spacing([10.0, 7.0])
                    .show(ui, |ui| {
                        ui.label("Find:");
                        ui.add(
                            egui::TextEdit::singleline(&mut app.find_query)
                                .desired_width(360.0)
                                .hint_text("Text or object name"),
                        );
                        ui.end_row();
                        ui.label("Replace with:");
                        ui.add(
                            egui::TextEdit::singleline(&mut app.replace_query)
                                .desired_width(360.0)
                                .hint_text("Replacement text"),
                        );
                        ui.end_row();
                    });
            } else {
                egui::Grid::new("find_replace_color_grid")
                    .num_columns(2)
                    .spacing([10.0, 9.0])
                    .show(ui, |ui| {
                        ui.label("Find color:");
                        ui.horizontal(|ui| {
                            ui.color_edit_button_srgba(&mut app.find_color)
                                .on_hover_text("Choose the exact color to find");
                            ui.label(
                                RichText::new(crate::colors::hex_of(app.find_color))
                                    .monospace()
                                    .color(p.text_dim),
                            );
                            if ui.small_button("Use fill").clicked() {
                                app.find_color = app.fill;
                            }
                            if ui.small_button("Use stroke").clicked() {
                                app.find_color = app.stroke;
                            }
                        });
                        ui.end_row();
                        ui.label("Replace with:");
                        ui.horizontal(|ui| {
                            ui.color_edit_button_srgba(&mut app.replace_color)
                                .on_hover_text("Choose the replacement color");
                            ui.label(
                                RichText::new(crate::colors::hex_of(app.replace_color))
                                    .monospace()
                                    .color(p.text_dim),
                            );
                            if ui.small_button("Use fill").clicked() {
                                app.replace_color = app.fill;
                            }
                            if ui.small_button("Use stroke").clicked() {
                                app.replace_color = app.stroke;
                            }
                        });
                        ui.end_row();
                    });
                ui.add_space(3.0);
                let scope = match app.find_replace_mode {
                    FindReplaceMode::FillColor => {
                        "Fills and gradient start/end stops. Useful for retheming templates in one pass."
                    }
                    FindReplaceMode::StrokeColor => {
                        "Stroke colors only, including vector outlines and shape borders."
                    }
                    FindReplaceMode::AnyColor => {
                        "Fills, strokes, gradients, and live-effect colors (shadows, glows, bevels)."
                    }
                    FindReplaceMode::Text => "",
                };
                ui.label(RichText::new(scope).small().color(p.text_dim));
            }

            ui.add_space(8.0);
            let matches = app.find_replace_match_count();
            let pages = app.doc.pages.len();
            ui.horizontal(|ui| {
                let (dot, _) = ui.allocate_exact_size(Vec2::splat(8.0), egui::Sense::hover());
                ui.painter().circle_filled(
                    dot.center(),
                    3.5,
                    if matches > 0 { p.selection } else { p.text_dim },
                );
                ui.label(
                    RichText::new(format!(
                        "{matches} matching object(s) across {pages} page(s)"
                    ))
                    .small()
                    .color(if matches > 0 { p.text } else { p.text_dim }),
                );
            });

            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if ui.button("Find Next").clicked() {
                    app.find_replace_find_next();
                }
                if primary_button(ui, p, "Replace All") {
                    app.run_find_and_replace();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Close").clicked() {
                        close_now = true;
                    }
                });
            });
        });
    app.show_find_replace = open && !close_now;
}
