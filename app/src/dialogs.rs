//! Modal dialogs: New Document, Canvas Size, Open/Save/Export/Import browser,
//! Find and Replace, Manage Saved Commands, About/Help, and the CS6 Welcome Splash.

use crate::app::{FileMode, FireCraft};
use crate::constants;

use eframe::egui::{self, Align2, Color32, RichText, Vec2};

/// The Fireworks CS6 New Document dialog.
pub fn new_document_dialog(app: &mut FireCraft) {
    if !app.show_new_dialog {
        return;
    }
    let mut open = app.show_new_dialog;
    egui::Window::new("New Document")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            let bytes = (app.new_doc_width * app.new_doc_height * 4.0) / (1024.0 * 1024.0);
            ui.label(
                RichText::new(format!("Canvas Size: {bytes:.2}M"))
                    .small()
                    .color(app.palette.text_dim),
            );
            ui.add_space(4.0);
            egui::Grid::new("new_doc_grid")
                .num_columns(3)
                .spacing([10.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Width:");
                    ui.add(
                        egui::DragValue::new(&mut app.new_doc_width)
                            .speed(2.0)
                            .range(1.0..=16384.0),
                    );
                    ui.label("Pixels");
                    ui.end_row();

                    ui.label("Height:");
                    ui.add(
                        egui::DragValue::new(&mut app.new_doc_height)
                            .speed(2.0)
                            .range(1.0..=16384.0),
                    );
                    ui.label("Pixels");
                    ui.end_row();

                    ui.label("Resolution:");
                    ui.add(
                        egui::DragValue::new(&mut app.new_doc_dpi)
                            .speed(1.0)
                            .range(36..=1200),
                    );
                    ui.label("Pixels/Inch");
                    ui.end_row();
                });

            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                ui.label("Presets:");
                for (w, h, name) in [
                    (800.0, 600.0, "800×600"),
                    (1024.0, 768.0, "1024×768"),
                    (1280.0, 720.0, "1280×720"),
                    (1920.0, 1080.0, "1920×1080"),
                    (512.0, 512.0, "512×512"),
                ] {
                    if ui.small_button(name).clicked() {
                        app.new_doc_width = w;
                        app.new_doc_height = h;
                    }
                }
            });

            ui.separator();
            ui.label(RichText::new("Canvas color").strong());
            ui.horizontal(|ui| {
                if ui
                    .radio(
                        !app.new_doc_transparent && app.new_doc_bg == [1.0, 1.0, 1.0],
                        "White",
                    )
                    .clicked()
                {
                    app.new_doc_transparent = false;
                    app.new_doc_bg = [1.0, 1.0, 1.0];
                }
                if ui.radio(app.new_doc_transparent, "Transparent").clicked() {
                    app.new_doc_transparent = true;
                }
                if ui
                    .radio(
                        !app.new_doc_transparent && app.new_doc_bg != [1.0, 1.0, 1.0],
                        "Custom:",
                    )
                    .clicked()
                {
                    app.new_doc_transparent = false;
                }
                if ui.color_edit_button_rgb(&mut app.new_doc_bg).changed() {
                    app.new_doc_transparent = false;
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Templates…").clicked() {
                    app.create_template_document(
                        "Web Hero Banner",
                        app.new_doc_width.max(800.0),
                        app.new_doc_height.max(600.0),
                    );
                    app.show_new_dialog = false;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        app.show_new_dialog = false;
                    }
                    if ui.button("   OK   ").clicked() {
                        let w = app.new_doc_width.max(1.0);
                        let h = app.new_doc_height.max(1.0);
                        let bg = Color32::from_rgb(
                            (app.new_doc_bg[0] * 255.0) as u8,
                            (app.new_doc_bg[1] * 255.0) as u8,
                            (app.new_doc_bg[2] * 255.0) as u8,
                        );
                        app.new_document_with_options(
                            w,
                            h,
                            app.new_doc_dpi,
                            bg,
                            app.new_doc_transparent,
                        );
                        app.show_new_dialog = false;
                    }
                });
            });
        });
    app.show_new_dialog = open;
}

/// Canvas Size dialog (Modify > Canvas > Canvas Size).
pub fn canvas_size_dialog(app: &mut FireCraft) {
    if !app.show_canvas_size_dialog {
        return;
    }
    let mut open = app.show_canvas_size_dialog;
    egui::Window::new("Canvas Size")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.add(
                    egui::DragValue::new(&mut app.new_doc_width)
                        .speed(2.0)
                        .range(1.0..=16384.0)
                        .suffix(" px"),
                );
                ui.label("Height:");
                ui.add(
                    egui::DragValue::new(&mut app.new_doc_height)
                        .speed(2.0)
                        .range(1.0..=16384.0)
                        .suffix(" px"),
                );
            });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Fit to Objects").clicked() {
                    app.fit_canvas_to_contents();
                    app.show_canvas_size_dialog = false;
                }
                if ui.button("Cancel").clicked() {
                    app.show_canvas_size_dialog = false;
                }
                if ui.button("Apply").clicked() {
                    app.resize_canvas(app.new_doc_width.max(1.0), app.new_doc_height.max(1.0));
                    app.show_canvas_size_dialog = false;
                }
            });
        });
    app.show_canvas_size_dialog = open;
}

/// Find and Replace dialog (Edit > Find and Replace).
pub fn find_replace_dialog(app: &mut FireCraft) {
    if !app.show_find_replace {
        return;
    }
    let mut open = app.show_find_replace;
    egui::Window::new("Find and Replace")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            egui::Grid::new("find_replace_grid")
                .num_columns(2)
                .spacing([8.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Find text / name:");
                    ui.text_edit_singleline(&mut app.find_query);
                    ui.end_row();
                    ui.label("Replace with:");
                    ui.text_edit_singleline(&mut app.replace_query);
                    ui.end_row();
                });
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Replace All in Document").clicked() {
                    app.run_find_and_replace();
                }
                if ui.button("Close").clicked() {
                    app.show_find_replace = false;
                }
            });
        });
    app.show_find_replace = open;
}

/// Manage Saved Commands dialog (Commands > Manage Saved Commands).
pub fn saved_commands_dialog(app: &mut FireCraft) {
    if !app.show_commands_dialog {
        return;
    }
    let mut open = app.show_commands_dialog;
    egui::Window::new("Manage Saved Commands")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_size([380.0, 260.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            ui.label(
                RichText::new("Saved Automation & Batch Commands")
                    .small()
                    .color(app.palette.text_dim),
            );
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    for cmd in app.saved_commands.clone() {
                        ui.horizontal(|ui| {
                            ui.label(&cmd);
                            if ui.small_button("▶ Run").clicked() {
                                app.execute_saved_command(&cmd);
                            }
                        });
                    }
                });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("New command name:");
                ui.text_edit_singleline(&mut app.new_command_name);
                if ui.button("Save Current Step").clicked()
                    && !app.new_command_name.trim().is_empty()
                {
                    let name = app.new_command_name.trim().to_string();
                    app.saved_commands.push(name.clone());
                    app.new_command_name.clear();
                    app.status = format!("Saved command: {name}");
                }
            });
        });
    app.show_commands_dialog = open;
}

/// About & Keyboard Shortcuts dialog (Help > About FireCraft).
pub fn about_dialog(app: &mut FireCraft) {
    if !app.show_about_dialog {
        return;
    }
    let mut open = app.show_about_dialog;
    egui::Window::new("About FireCraft — Adobe Fireworks CS6 Reimagined")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_size([460.0, 320.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            ui.heading(RichText::new("FireCraft Studio (CS6 Parity Edition)").color(app.palette.accent));
            ui.label("Clean-room hybrid vector + bitmap & web graphics editor inspired by Adobe Fireworks CS6 (v12.0).");
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.hyperlink_to("FireCraft Wiki", "https://github.com/aliasfoxkde/FireCraft/wiki");
                ui.separator();
                ui.hyperlink_to("GitHub Repository", "https://github.com/aliasfoxkde/FireCraft/blob/main/README.md");
            });
            ui.separator();
            ui.label(RichText::new("Key Shortcuts & Workflow").strong());
            egui::Grid::new("shortcuts_help_grid")
                .num_columns(2)
                .spacing([18.0, 3.0])
                .show(ui, |ui| {
                    for (k, d) in [
                        ("V / A / Q", "Pointer / Subselection / Scale Tool"),
                        ("U / L / N / P / T", "Rectangle / Ellipse / Line / Pen / Text"),
                        ("B / E / I / G", "Brush / Eraser / Eyedropper / Gradient"),
                        ("K / J", "Web Slice / Web Hotspot Tool"),
                        ("H / Z", "Hand Pan / Zoom Tool (Alt-click to zoom out)"),
                        ("Ctrl+N / Ctrl+O / Ctrl+S", "New / Open / Save Document"),
                        ("Ctrl+Z / Ctrl+Y", "Undo / Redo (up to 32 steps)"),
                        ("Ctrl+C / Ctrl+X / Ctrl+V / Ctrl+D", "Copy / Cut / Paste / Duplicate"),
                        ("Arrow Keys (Shift = 10px)", "Nudge Selected Object"),
                    ] {
                        ui.label(RichText::new(k).monospace().color(app.palette.accent));
                        ui.label(d);
                        ui.end_row();
                    }
                });
        });
    app.show_about_dialog = open;
}

/// The file browser used for Open, Save, Import and Export.
pub fn file_dialog(app: &mut FireCraft) {
    if !app.show_file_dialog {
        return;
    }
    let mut open = app.show_file_dialog;
    let title = match app.file_mode {
        FileMode::Open => "Open Document",
        FileMode::Import => "Import Image / Vector into Layer",
        FileMode::Save => "Save Document As",
        FileMode::Export => "Export Document",
    };
    egui::Window::new(title)
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .default_size([540.0, 410.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("<- Up").clicked()
                    && let Some(parent) = app.file_dialog_dir.parent()
                {
                    let parent = parent.to_path_buf();
                    app.set_file_dialog_dir(parent);
                }
                ui.label(
                    RichText::new(app.file_dialog_dir.to_string_lossy().to_string())
                        .small()
                        .color(app.palette.text_dim),
                );
            });
            ui.separator();

            egui::ScrollArea::vertical()
                .max_height(230.0)
                .show(ui, |ui| {
                    for entry in app.file_dialog_entries.clone() {
                        let name = entry
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        let is_dir = entry.is_dir();
                        let label = if is_dir {
                            format!("[DIR] {name}")
                        } else {
                            name.clone()
                        };
                        if ui
                            .selectable_label(app.file_dialog_name == name, label)
                            .clicked()
                        {
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
            if matches!(app.file_mode, FileMode::Save | FileMode::Export) {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Format:").small().color(app.palette.text_dim));
                    for fmt in [
                        crate::formats::Format::Png,
                        crate::formats::Format::Jpeg,
                        crate::formats::Format::Gif,
                        crate::formats::Format::WebP,
                        crate::formats::Format::Svg,
                        crate::formats::Format::Html,
                        crate::formats::Format::Tiff,
                        crate::formats::Format::Bmp,
                        crate::formats::Format::Ico,
                    ] {
                        if ui.small_button(fmt.label()).clicked() {
                            let stem = std::path::Path::new(&app.file_dialog_name)
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .filter(|s| !s.is_empty())
                                .unwrap_or("export");
                            app.file_dialog_name = format!("{stem}.{}", fmt.extension());
                        }
                    }
                });
            }
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    app.show_file_dialog = false;
                }
                let label = match app.file_mode {
                    FileMode::Open => "Open",
                    FileMode::Import => "Import",
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

/// The Fireworks CS6 Welcome Splash screen overlay (`reference/picture-1.jpg`).
pub fn welcome_overlay(app: &mut FireCraft) {
    if !app.show_welcome {
        return;
    }
    let ctx = app.ctx.clone();
    egui::Window::new("Welcome to FireCraft CS6")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .fixed_size([620.0, 390.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .frame(
            egui::Frame::new()
                .fill(app.palette.welcome)
                .stroke(egui::Stroke::new(1.5, app.palette.accent)),
        )
        .show(&ctx, |ui| {
            crate::panels::start_panel(ui, app);
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Start Editing").clicked() {
                    app.show_welcome = false;
                }
                if ui.button("Load Showcase Demo").clicked() {
                    app.load_demo_showcase();
                    app.show_welcome = false;
                }
                ui.label(
                    RichText::new(format!(
                        "Default canvas: {}×{} px @ {} dpi",
                        constants::DEFAULT_WIDTH,
                        constants::DEFAULT_HEIGHT,
                        constants::EXPORT_DEFAULT_DPI
                    ))
                    .small()
                    .color(app.palette.text_dim),
                );
            });
        });
}
