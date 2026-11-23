//! Modal dialogs: New Document, Canvas Size, Open/Save/Export/Import browser,
//! Find and Replace, Manage Saved Commands, Command Palette (Ctrl+K), MCP/CLI Console,
//! About/Help, and the CS6 Welcome Splash.
//!
//! Notice: Every dialog uses a local `should_close` flag combined with `open` so clicking
//! `Cancel` or `OK` inside the window body is never overwritten by `open = true` at the end.

use crate::app::{FileMode, FireCraft};
use crate::constants;
use crate::mcp;

use eframe::egui::{self, Align2, Color32, RichText, Vec2};

/// The Fireworks CS6 New Document dialog.
pub fn new_document_dialog(app: &mut FireCraft) {
    if !app.show_new_dialog {
        return;
    }
    let mut open = app.show_new_dialog;
    let mut close_now = false;
    egui::Window::new("New Document")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            let bytes = (app.new_doc_width * app.new_doc_height * 4.0) / (1024.0 * 1024.0);
            ui.label(
                RichText::new(format!(
                    "Canvas Size: {bytes:.2}M ({:.0}×{:.0} @ {} ppi)",
                    app.new_doc_width, app.new_doc_height, app.new_doc_dpi
                ))
                .small()
                .color(app.palette.text_dim),
            );
            ui.add_space(4.0);
            egui::Grid::new("new_doc_grid")
                .num_columns(3)
                .spacing([10.0, 6.0])
                .show(ui, |ui| {
                    ui.label("Preset Size:");
                    let current_preset =
                        format!("{:.0}×{:.0} px", app.new_doc_width, app.new_doc_height);
                    egui::ComboBox::from_id_salt("new_doc_size_preset_combo")
                        .width(175.0)
                        .selected_text(current_preset)
                        .show_ui(ui, |ui| {
                            for &(label, w, h) in constants::RESOLUTION_PRESETS {
                                let active = (app.new_doc_width - w).abs() < 0.5
                                    && (app.new_doc_height - h).abs() < 0.5;
                                if ui.selectable_label(active, label).clicked() {
                                    app.new_doc_width = w;
                                    app.new_doc_height = h;
                                }
                            }
                        });
                    ui.label("");
                    ui.end_row();

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
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut app.new_doc_dpi)
                                .speed(1.0)
                                .range(36..=1200),
                        );
                        egui::ComboBox::from_id_salt("new_doc_dpi_preset_combo")
                            .width(90.0)
                            .selected_text(format!("{} ppi", app.new_doc_dpi))
                            .show_ui(ui, |ui| {
                                for &dpi in constants::DPI_PRESETS {
                                    if ui
                                        .selectable_label(
                                            app.new_doc_dpi == dpi,
                                            format!("{dpi} Pixels/Inch"),
                                        )
                                        .clicked()
                                    {
                                        app.new_doc_dpi = dpi;
                                    }
                                }
                            });
                    });
                    ui.label("Pixels/Inch");
                    ui.end_row();
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
                    close_now = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        close_now = true;
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
                        close_now = true;
                    }
                });
            });
        });
    app.show_new_dialog = open && !close_now;
}

/// Canvas Size & Resolution dialog (Modify > Canvas > Canvas Size).
pub fn canvas_size_dialog(app: &mut FireCraft) {
    if !app.show_canvas_size_dialog {
        return;
    }
    let mut open = app.show_canvas_size_dialog;
    let mut close_now = false;
    egui::Window::new("Canvas Size & Resolution")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Preset:");
                egui::ComboBox::from_id_salt("canvas_size_preset_combo")
                    .width(185.0)
                    .selected_text(format!(
                        "{:.0}×{:.0} px",
                        app.new_doc_width, app.new_doc_height
                    ))
                    .show_ui(ui, |ui| {
                        for &(label, w, h) in constants::RESOLUTION_PRESETS {
                            let active = (app.new_doc_width - w).abs() < 0.5
                                && (app.new_doc_height - h).abs() < 0.5;
                            if ui.selectable_label(active, label).clicked() {
                                app.new_doc_width = w;
                                app.new_doc_height = h;
                            }
                        }
                    });
            });
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
                ui.label("Resolution:");
                egui::ComboBox::from_id_salt("canvas_size_dpi_combo")
                    .width(85.0)
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
            ui.separator();
            ui.horizontal(|ui| {
                if ui.button("Fit to Objects").clicked() {
                    app.fit_canvas_to_contents();
                    close_now = true;
                }
                if ui.button("Cancel").clicked() {
                    close_now = true;
                }
                if ui.button("Apply").clicked() {
                    app.resize_canvas(app.new_doc_width.max(1.0), app.new_doc_height.max(1.0));
                    close_now = true;
                }
            });
        });
    app.show_canvas_size_dialog = open && !close_now;
}

/// Find and Replace dialog (Edit > Find and Replace).
pub fn find_replace_dialog(app: &mut FireCraft) {
    if !app.show_find_replace {
        return;
    }
    let mut open = app.show_find_replace;
    let mut close_now = false;
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
                    close_now = true;
                }
            });
        });
    app.show_find_replace = open && !close_now;
}

/// Manage Saved Commands dialog (Commands > Manage Saved Commands).
pub fn saved_commands_dialog(app: &mut FireCraft) {
    if !app.show_commands_dialog {
        return;
    }
    let mut open = app.show_commands_dialog;
    let mut close_now = false;
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
                            if ui.small_button("Run").clicked() {
                                app.execute_saved_command(&cmd);
                            }
                        });
                    }
                });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("New command:");
                ui.text_edit_singleline(&mut app.new_command_name);
                if ui.button("Save Step").clicked() && !app.new_command_name.trim().is_empty() {
                    let name = app.new_command_name.trim().to_string();
                    app.saved_commands.push(name.clone());
                    app.new_command_name.clear();
                    app.status = format!("Saved command: {name}");
                }
                if ui.button("Close").clicked() {
                    close_now = true;
                }
            });
        });
    app.show_commands_dialog = open && !close_now;
}

/// Searchable Command Palette (`Ctrl+K` / `Ctrl+P`, extracted from `C:\Repos\FireCraft\crates\app\src\shell\palette.rs`).
pub fn command_palette_dialog(app: &mut FireCraft) {
    if !app.show_command_palette {
        return;
    }
    let mut open = app.show_command_palette;
    let mut close_now = false;
    egui::Window::new("Command Palette (Ctrl+K)")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .fixed_size([460.0, 320.0])
        .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 56.0))
        .show(&app.ctx.clone(), |ui| {
            ui.horizontal(|ui| {
                ui.label("Search:");
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut app.palette_filter)
                        .desired_width(310.0)
                        .hint_text("Type a command (e.g. group, shadow, slice, align, theme)…"),
                );
                resp.request_focus();
                if ui.small_button("Clear").clicked() {
                    app.palette_filter.clear();
                }
            });
            ui.separator();

            let q = app.palette_filter.trim().to_lowercase();
            let hits: Vec<(&str, &str)> = mcp::COMMAND_CATALOG
                .iter()
                .copied()
                .filter(|(id, desc)| {
                    q.is_empty()
                        || id.to_lowercase().contains(&q)
                        || desc.to_lowercase().contains(&q)
                })
                .collect();

            let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
            if enter_pressed && let Some((first_id, _)) = hits.first() {
                if let Ok(msg) = mcp::execute_catalog_command(app, first_id) {
                    app.status = msg;
                }
                close_now = true;
            }

            egui::ScrollArea::vertical()
                .max_height(225.0)
                .show(ui, |ui| {
                    for (id, desc) in hits {
                        ui.horizontal(|ui| {
                            if ui
                                .button(RichText::new(id).monospace().color(app.palette.selection))
                                .clicked()
                            {
                                if let Ok(msg) = mcp::execute_catalog_command(app, id) {
                                    app.status = msg;
                                }
                                close_now = true;
                            }
                            ui.label(RichText::new(desc).small().color(app.palette.text_dim));
                        });
                    }
                });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "{} registered commands · Press Enter to run top match",
                        mcp::COMMAND_CATALOG.len()
                    ))
                    .small()
                    .color(app.palette.text_dim),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("Close (Esc)").clicked() {
                        close_now = true;
                    }
                });
            });
        });
    if app.ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        close_now = true;
    }
    app.show_command_palette = open && !close_now;
}

/// Interactive Model Context Protocol (MCP) JSON-RPC 2.0 & Headless CLI Console.
pub fn mcp_cli_dialog(app: &mut FireCraft) {
    if !app.show_mcp_dialog {
        return;
    }
    let mut open = app.show_mcp_dialog;
    let mut close_now = false;
    egui::Window::new("MCP JSON-RPC 2.0 Server & Headless CLI Console")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .default_size([560.0, 380.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            ui.label(
                RichText::new(format!(
                    "MCP Protocol {} · Tools: firecraft.info, firecraft.apply, firecraft.export, firecraft.undo, firecraft.redo, firecraft.listCommands",
                    mcp::PROTOCOL_VERSION
                ))
                .small()
                .color(app.palette.text_dim),
            );
            ui.add_space(4.0);
            ui.label(RichText::new("Headless CLI Commands (run on firecraft.exe):").strong());
            ui.code(
                "firecraft.exe info --demo\n\
                 firecraft.exe export --demo out.svg\n\
                 firecraft.exe batch ./folder --format webp --out ./converted\n\
                 firecraft.exe list-commands\n\
                 firecraft.exe mcp --demo",
            );
            ui.separator();
            ui.label(RichText::new("Live MCP JSON-RPC 2.0 Test Request:").strong());
            ui.horizontal(|ui| {
                for (label, sample) in [
                    (
                        "tools/list",
                        r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#,
                    ),
                    (
                        "firecraft.info",
                        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"firecraft.info"}}"#,
                    ),
                    (
                        "firecraft.apply (Demo)",
                        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"firecraft.apply","arguments":{"command":"document.demoShowcase"}}}"#,
                    ),
                    (
                        "resources/read",
                        r#"{"jsonrpc":"2.0","id":4,"method":"resources/read","params":{"uri":"document://current"}}"#,
                    ),
                ] {
                    if ui.small_button(label).clicked() {
                        app.mcp_input = sample.to_string();
                        app.mcp_output =
                            mcp::handle_mcp_jsonrpc(app, sample).unwrap_or_default();
                    }
                }
            });
            ui.add(
                egui::TextEdit::singleline(&mut app.mcp_input)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace),
            );
            ui.horizontal(|ui| {
                if ui.button("Dispatch JSON-RPC Request").clicked() {
                    let req = app.mcp_input.clone();
                    app.mcp_output = mcp::handle_mcp_jsonrpc(app, &req)
                        .unwrap_or_else(|| "(notification - no reply)".to_string());
                }
                if ui.button("Copy Reply").clicked() {
                    app.ctx.copy_text(app.mcp_output.clone());
                }
                if ui.button("Close").clicked() {
                    close_now = true;
                }
            });
            if !app.mcp_output.is_empty() {
                ui.separator();
                egui::ScrollArea::vertical().max_height(110.0).show(ui, |ui| {
                    ui.code(&app.mcp_output);
                });
            }
        });
    app.show_mcp_dialog = open && !close_now;
}

/// About & Keyboard Shortcuts dialog (Help > About FireCraft).
pub fn about_dialog(app: &mut FireCraft) {
    if !app.show_about_dialog {
        return;
    }
    let mut open = app.show_about_dialog;
    let mut close_now = false;
    egui::Window::new("About FireCraft — Adobe Fireworks CS6 Reimagined")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .default_size([480.0, 340.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            ui.heading(RichText::new("FireCraft Studio (CS6 Parity Edition)").color(app.palette.selection));
            ui.label("Clean-room hybrid vector + bitmap & web graphics editor inspired by Adobe Fireworks CS6 (v12.0).");
            ui.add_space(4.0);
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
                        ("Right-Click on Canvas", "Adaptive CS6 Context Menu"),
                        ("Shift-Click / Marquee Drag", "Multi-Object Selection"),
                        ("Middle-Click Document Tab", "Close Document Tab"),
                        ("Ctrl+G / Ctrl+Shift+G", "Group / Ungroup Selected Objects"),
                        ("Ctrl+K / Ctrl+P", "Searchable Command Palette"),
                        ("V / A / Q", "Pointer / Subselection / Scale Tool"),
                        ("U / L / N / P / T", "Rectangle / Ellipse / Line / Pen / Text"),
                        ("B / E / I / G", "Brush / Eraser / Eyedropper / Gradient"),
                        ("K / J", "Web Slice / Web Hotspot Tool"),
                        ("Ctrl+Z / Ctrl+Y", "Undo / Redo (up to 32 steps)"),
                    ] {
                        ui.label(RichText::new(k).monospace().color(app.palette.selection));
                        ui.label(d);
                        ui.end_row();
                    }
                });
            ui.separator();
            if ui.button("Close").clicked() {
                close_now = true;
            }
        });
    app.show_about_dialog = open && !close_now;
}

/// The file browser used for Open, Save, Import and Export.
pub fn file_dialog(app: &mut FireCraft) {
    if !app.show_file_dialog {
        return;
    }
    let mut open = app.show_file_dialog;
    let mut close_now = false;
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
                    close_now = true;
                }
                let label = match app.file_mode {
                    FileMode::Open => "Open",
                    FileMode::Import => "Import",
                    FileMode::Save => "Save",
                    FileMode::Export => "Export",
                };
                if ui.button(label).clicked() {
                    app.commit_file_dialog();
                    if !app.show_file_dialog {
                        close_now = true;
                    }
                }
            });
        });
    app.show_file_dialog = open && !close_now;
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
                        "Default canvas: {:.0}×{:.0} px @ {} ppi",
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
