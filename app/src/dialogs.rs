//! Modal dialogs: New Document, the file browser, and the welcome splash.

use crate::app::{FileMode, FireCraft};
use crate::constants;

use eframe::egui::{self, Align2, Color32, RichText, Vec2};

/// The New Document dialog.
pub fn new_document_dialog(app: &mut FireCraft) {
    let mut open = app.show_new_dialog;
    egui::Window::new("New Document")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Width:");
                let mut w = app.new_doc_width;
                if ui
                    .add(egui::DragValue::new(&mut w).speed(1.0).range(1.0..=16000.0))
                    .changed()
                {
                    app.new_doc_width = w;
                }
                ui.label("px");
            });
            ui.horizontal(|ui| {
                ui.label("Height:");
                let mut h = app.new_doc_height;
                if ui
                    .add(egui::DragValue::new(&mut h).speed(1.0).range(1.0..=16000.0))
                    .changed()
                {
                    app.new_doc_height = h;
                }
                ui.label("px");
            });
            ui.horizontal(|ui| {
                ui.label("Presets:");
                for (w, h, name) in [
                    (800.0, 600.0, "800×600"),
                    (1024.0, 768.0, "1024×768"),
                    (1920.0, 1080.0, "1920×1080"),
                ] {
                    if ui.small_button(name).clicked() {
                        app.new_doc_width = w;
                        app.new_doc_height = h;
                    }
                }
            });
            ui.horizontal(|ui| {
                ui.label("Background:");
                ui.color_edit_button_rgb(&mut app.new_doc_bg);
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    app.show_new_dialog = false;
                }
                if ui.button("OK").clicked() {
                    let w = app.new_doc_width.max(1.0);
                    let h = app.new_doc_height.max(1.0);
                    app.new_document(w, h);
                    app.show_new_dialog = false;
                }
            });
        });
    app.show_new_dialog = open;
}

/// The file browser used for Open, Save and Export.
pub fn file_dialog(app: &mut FireCraft) {
    let mut open = app.show_file_dialog;
    let title = match app.file_mode {
        FileMode::Open => "Open",
        FileMode::Save => "Save As",
        FileMode::Export => "Export",
    };
    egui::Window::new(title)
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_size([520.0, 380.0])
        .show(&app.ctx.clone(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("⬅ Up").clicked() {
                    if let Some(parent) = app.file_dialog_dir.parent() {
                        let parent = parent.to_path_buf();
                        app.set_file_dialog_dir(parent);
                    }
                }
                ui.label(
                    RichText::new(app.file_dialog_dir.to_string_lossy().to_string())
                        .small()
                        .color(app.palette.text_dim),
                );
            });
            ui.separator();

            egui::ScrollArea::vertical().max_height(230.0).show(ui, |ui| {
                for entry in app.file_dialog_entries.clone() {
                    let name = entry
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let is_dir = entry.is_dir();
                    let label = if is_dir {
                        format!("📁 {name}")
                    } else {
                        format!("📄 {name}")
                    };
                    if ui.selectable_label(false, label).clicked() {
                        if is_dir {
                            app.set_file_dialog_dir(entry);
                        } else {
                            app.file_dialog_name = name;
                        }
                    }
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("File name:");
                ui.text_edit_singleline(&mut app.file_dialog_name);
            });
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    app.show_file_dialog = false;
                }
                let label = match app.file_mode {
                    FileMode::Open => "Open",
                    FileMode::Save => "Save",
                    FileMode::Export => "Export",
                };
                if ui.button(label).clicked() {
                    app.commit_file_dialog();
                }
            });
        });
    app.show_file_dialog = open;
}

/// The welcome splash, drawn over the editor.
pub fn welcome_overlay(app: &mut FireCraft) {
    if !app.show_welcome {
        return;
    }
    let ctx = app.ctx.clone();
    egui::Window::new("Welcome")
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&ctx, |ui| {
            crate::panels::start_panel(ui, app);
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Start").clicked() {
                    app.show_welcome = false;
                }
                ui.label(
                    RichText::new(format!(
                        "New documents start at {}×{} px.",
                        constants::DEFAULT_WIDTH, constants::DEFAULT_HEIGHT
                    ))
                    .small(),
                );
            });
        });
}
