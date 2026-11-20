#![allow(dead_code, unused_imports, unused_variables, unused_must_use)]
//! Properties panel - shows selection properties.

use eframe::egui::{self, Color32};
use crate::app::FireCraft;

/// Render a Properties panel showing selection properties.
pub fn properties_panel(ui: &mut egui::Ui, app: &mut FireCraft) {
    ui.vertical(|ui| {
        ui.heading("Properties");
        ui.separator();

        // Show selected object info
        if let Some(idx) = app.selected {
            let layer = app.selected_layer;
            let obj_name = {
                if let Some(obj) = app.doc.page().layers.get(layer).and_then(|l| l.objects.get(idx)) {
                    Some(obj.name.clone())
                } else {
                    None
                }
            };
            if let Some(name) = obj_name {
                let mut edit_name = name.clone();
                ui.label("Name:");
                let resp = ui.add(egui::TextEdit::singleline(&mut edit_name).desired_width(140.0));
                if resp.changed() {
                    app.set_selected_name(edit_name);
                }

                // Fill color
                let rgb: [f32; 3] = [app.fill.r() as f32 / 255.0, app.fill.g() as f32 / 255.0, app.fill.b() as f32 / 255.0];
                ui.horizontal(|ui| {
                    ui.label("Fill:");
                    let mut fill_rgb = rgb;
                    ui.color_edit_button_rgb(&mut fill_rgb);
                    app.fill = Color32::from_rgb(
                        (fill_rgb[0] * 255.0) as u8,
                        (fill_rgb[1] * 255.0) as u8,
                        (fill_rgb[2] * 255.0) as u8,
                    );
                });

                // Stroke color
                let stroke_rgb: [f32; 3] = [app.stroke.r() as f32 / 255.0, app.stroke.g() as f32 / 255.0, app.stroke.b() as f32 / 255.0];
                ui.horizontal(|ui| {
                    ui.label("Stroke:");
                    let mut s_rgb = stroke_rgb;
                    ui.color_edit_button_rgb(&mut s_rgb);
                    app.stroke = Color32::from_rgb(
                        (s_rgb[0] * 255.0) as u8,
                        (s_rgb[1] * 255.0) as u8,
                        (s_rgb[2] * 255.0) as u8,
                    );
                });

                ui.label(format!("Object: {}", name));
            }
        } else {
            ui.label("Nothing selected");
        }
    });
}
