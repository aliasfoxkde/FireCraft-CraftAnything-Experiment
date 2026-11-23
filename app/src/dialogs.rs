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
    enum PaletteAction {
        Catalog(&'static str),
        OpenPanel(crate::app::PanelId),
        Workspace(&'static str),
        FullScreen,
        ResetWorkspace,
        FindDialog,
        SelectObject(usize, usize),
        ReplaceText(String, String),
        AddFilter(&'static str),
        InsertWidget(&'static str),
    }
    struct PaletteEntry {
        badge: &'static str,
        title: String,
        desc: String,
        action: PaletteAction,
    }

    let mut open = app.show_command_palette;
    let mut close_now = false;
    egui::Window::new("Command Palette & Search (Ctrl+K)")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .fixed_size([560.0, 390.0])
        .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 48.0))
        .show(&app.ctx.clone(), |ui| {
            let p = app.palette;
            ui.horizontal(|ui| {
                ui.label(RichText::new("Search:").strong());
                let resp = ui.add(
                    egui::TextEdit::singleline(&mut app.palette_filter)
                        .desired_width(410.0)
                        .hint_text(
                            "Panels, filters, objects, find/replace (replace:old->new), commands…",
                        ),
                );
                resp.request_focus();
                if ui.small_button("Clear").clicked() {
                    app.palette_filter.clear();
                }
            });
            ui.separator();

            let raw_q = app.palette_filter.trim().to_string();
            let q = raw_q.to_lowercase();
            let mut entries: Vec<PaletteEntry> = Vec::new();

            // 1. Inline Replace: `replace:old->new`
            if let Some(rest) = raw_q
                .strip_prefix("replace:")
                .or_else(|| raw_q.strip_prefix("Replace:"))
                && let Some((from_s, to_s)) = rest.split_once("->")
                && !from_s.trim().is_empty()
            {
                let from_t = from_s.trim().to_string();
                let to_t = to_s.trim().to_string();
                entries.push(PaletteEntry {
                    badge: "[Replace]",
                    title: format!("Replace \"{from_t}\" -> \"{to_t}\""),
                    desc: "Replace across all object names and text on page".to_string(),
                    action: PaletteAction::ReplaceText(from_t, to_t),
                });
            }

            // 2. Find & Replace Dialog + Quick Actions
            entries.push(PaletteEntry {
                badge: "[Action]",
                title: "Find & Replace… (Ctrl+F)".to_string(),
                desc: "Open Find & Replace dialog for text and object names".to_string(),
                action: PaletteAction::FindDialog,
            });
            entries.push(PaletteEntry {
                badge: "[View]",
                title: "Full Screen Canvas (Tab / F4 / F11)".to_string(),
                desc: "Toggle full-screen distraction-free canvas mode".to_string(),
                action: PaletteAction::FullScreen,
            });
            entries.push(PaletteEntry {
                badge: "[View]",
                title: "Reset Workspace Layout".to_string(),
                desc: "Restore all CS6 docks, toolbars, and default panel groups".to_string(),
                action: PaletteAction::ResetWorkspace,
            });
            for preset in ["Expanded Mode", "CS6 Dual-Column", "All Panels Floating"] {
                entries.push(PaletteEntry {
                    badge: "[Layout]",
                    title: format!("Workspace: {preset}"),
                    desc: "Switch panel docking preset".to_string(),
                    action: PaletteAction::Workspace(preset),
                });
            }

            // 3. All 21 CS6 Panels
            for pid in crate::app::PanelId::ALL {
                let sc = pid.shortcut();
                let desc = if sc.is_empty() {
                    "Open or focus panel".to_string()
                } else {
                    format!("Open or focus panel ({sc})")
                };
                entries.push(PaletteEntry {
                    badge: "[Panel]",
                    title: format!("Open / Focus {}", pid.title()),
                    desc,
                    action: PaletteAction::OpenPanel(pid),
                });
            }

            // 4. Live Filters & Bitmap Filters
            for (fkey, fname, fdesc) in [
                (
                    "drop_shadow",
                    "Drop Shadow",
                    "Add non-destructive Live Drop Shadow",
                ),
                (
                    "inner_shadow",
                    "Inner Shadow",
                    "Add non-destructive Live Inner Shadow",
                ),
                (
                    "outer_glow",
                    "Outer Glow",
                    "Add non-destructive Live Outer Glow",
                ),
                (
                    "inner_glow",
                    "Inner Glow",
                    "Add non-destructive Live Inner Glow",
                ),
                (
                    "inner_bevel",
                    "Inner Bevel",
                    "Add non-destructive Live Inner Bevel",
                ),
                (
                    "gaussian_blur",
                    "Gaussian Blur",
                    "Add non-destructive Gaussian Blur",
                ),
                (
                    "brightness",
                    "Brightness / Contrast",
                    "Add Live Brightness/Contrast filter",
                ),
                ("sharpen", "Sharpen", "Apply Sharpen bitmap filter"),
                ("dodge", "Dodge (Brighten)", "Apply Dodge filter"),
                ("burn", "Burn (Darken)", "Apply Burn filter"),
                (
                    "clear",
                    "Clear All Live Filters",
                    "Remove all live effects from selection",
                ),
            ] {
                entries.push(PaletteEntry {
                    badge: "[Filter]",
                    title: format!("Filter: {fname}"),
                    desc: fdesc.to_string(),
                    action: PaletteAction::AddFilter(fkey),
                });
            }

            // 5. Common Library Widgets
            for widget in [
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
            ] {
                entries.push(PaletteEntry {
                    badge: "[Widget]",
                    title: format!("Insert {widget}"),
                    desc: "Insert Common Library UI component onto canvas".to_string(),
                    action: PaletteAction::InsertWidget(widget),
                });
            }

            // 6. Live Canvas Objects (when searching)
            if !q.is_empty() {
                let search_term = q.strip_prefix("find:").map(str::trim).unwrap_or(&q);
                if !search_term.is_empty() {
                    for (li, layer) in app.doc.page().layers.iter().enumerate() {
                        for (oi, obj) in layer.objects.iter().enumerate() {
                            let text_match = match &obj.kind {
                                crate::doc::Shape::Text { text, .. } => {
                                    text.to_lowercase().contains(search_term)
                                }
                                _ => false,
                            };
                            if obj.name.to_lowercase().contains(search_term) || text_match {
                                entries.push(PaletteEntry {
                                    badge: "[Object]",
                                    title: format!("Select \"{}\"", obj.name),
                                    desc: format!(
                                        "Layer: {} · Jump & select on canvas",
                                        layer.name
                                    ),
                                    action: PaletteAction::SelectObject(li, oi),
                                });
                            }
                        }
                    }
                }
            }

            // 7. Registered MCP / Editor Commands
            for (id, desc) in mcp::COMMAND_CATALOG {
                entries.push(PaletteEntry {
                    badge: "[Cmd]",
                    title: (*id).to_string(),
                    desc: (*desc).to_string(),
                    action: PaletteAction::Catalog(id),
                });
            }

            let hits: Vec<PaletteEntry> = entries
                .into_iter()
                .filter(|e| {
                    q.is_empty()
                        || e.title.to_lowercase().contains(&q)
                        || e.desc.to_lowercase().contains(&q)
                        || e.badge.to_lowercase().contains(&q)
                })
                .collect();

            let sel_id = ui.make_persistent_id("palette_sel_idx");
            let mut sel_idx: usize = ui.ctx().data(|d| d.get_temp(sel_id).unwrap_or(0));
            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) && !hits.is_empty() {
                sel_idx = (sel_idx + 1).min(hits.len() - 1);
            }
            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                sel_idx = sel_idx.saturating_sub(1);
            }
            sel_idx = sel_idx.min(hits.len().saturating_sub(1));
            ui.ctx().data_mut(|d| d.insert_temp(sel_id, sel_idx));

            let mut run_idx: Option<usize> =
                if ui.input(|i| i.key_pressed(egui::Key::Enter)) && !hits.is_empty() {
                    Some(sel_idx)
                } else {
                    None
                };

            egui::ScrollArea::vertical()
                .max_height(280.0)
                .show(ui, |ui| {
                    for (idx, entry) in hits.iter().enumerate() {
                        let is_sel = idx == sel_idx;
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(entry.badge)
                                    .monospace()
                                    .small()
                                    .color(p.accent),
                            );
                            if ui
                                .selectable_label(is_sel, RichText::new(&entry.title).strong())
                                .clicked()
                            {
                                run_idx = Some(idx);
                            }
                            ui.label(RichText::new(&entry.desc).small().color(p.text_dim));
                        });
                    }
                });

            if let Some(idx) = run_idx
                && let Some(entry) = hits.into_iter().nth(idx)
            {
                match entry.action {
                    PaletteAction::Catalog(id) => {
                        if let Ok(msg) = mcp::execute_catalog_command(app, id) {
                            app.status = msg;
                        }
                    }
                    PaletteAction::OpenPanel(pid) => {
                        app.full_screen_canvas = false;
                        app.show_right_dock = true;
                        app.focus_or_open_panel(pid);
                    }
                    PaletteAction::Workspace(preset) => {
                        app.apply_workspace_preset(preset);
                    }
                    PaletteAction::FullScreen => app.toggle_full_screen_canvas(),
                    PaletteAction::ResetWorkspace => {
                        app.full_screen_canvas = false;
                        app.show_main_toolbar = true;
                        app.show_tools_dock = true;
                        app.show_right_dock = true;
                        app.show_bottom_inspector = true;
                        app.apply_workspace_preset("Expanded Mode");
                    }
                    PaletteAction::FindDialog => app.show_find_replace = true,
                    PaletteAction::SelectObject(li, oi) => {
                        app.selected_layer = li;
                        app.select_single(Some(oi));
                        app.status = "Selected object from search".to_string();
                    }
                    PaletteAction::ReplaceText(from_t, to_t) => {
                        app.find_query = from_t;
                        app.replace_query = to_t;
                        app.run_find_and_replace();
                    }
                    PaletteAction::AddFilter(fkey) => match fkey {
                        "drop_shadow" => app.add_live_effect(crate::doc::LiveEffect::DropShadow {
                            distance: 6.0,
                            angle_deg: 45.0,
                            blur: 8.0,
                            color: Color32::from_black_alpha(128),
                            enabled: true,
                        }),
                        "inner_shadow" => {
                            app.add_live_effect(crate::doc::LiveEffect::InnerShadow {
                                distance: 4.0,
                                blur: 6.0,
                                color: Color32::from_black_alpha(110),
                                enabled: true,
                            })
                        }
                        "outer_glow" => app.add_live_effect(crate::doc::LiveEffect::Glow {
                            radius: 8.0,
                            color: Color32::from_rgb(244, 185, 66),
                            inner: false,
                            enabled: true,
                        }),
                        "inner_glow" => app.add_live_effect(crate::doc::LiveEffect::Glow {
                            radius: 6.0,
                            color: Color32::from_rgb(255, 224, 140),
                            inner: true,
                            enabled: true,
                        }),
                        "inner_bevel" => app.add_live_effect(crate::doc::LiveEffect::Bevel {
                            width: 4.0,
                            highlight: Color32::from_white_alpha(180),
                            shadow: Color32::from_black_alpha(140),
                            enabled: true,
                        }),
                        "gaussian_blur" => {
                            app.add_live_effect(crate::doc::LiveEffect::GaussianBlur {
                                radius: 4.0,
                                enabled: true,
                            })
                        }
                        "brightness" => app.add_live_effect(crate::doc::LiveEffect::ColorAdjust {
                            brightness: 0.12,
                            contrast: 0.10,
                            saturation: 1.0,
                            enabled: true,
                        }),
                        "sharpen" => app.run_bitmap_filter(crate::app::Tool::Sharpen),
                        "dodge" => app.run_bitmap_filter(crate::app::Tool::Dodge),
                        "burn" => app.run_bitmap_filter(crate::app::Tool::Burn),
                        _ => app.clear_live_effects(),
                    },
                    PaletteAction::InsertWidget(w) => app.insert_common_library_item(w),
                }
                close_now = true;
            }

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("↑/↓ Navigate · Enter Run · find:<text> · replace:old->new")
                        .small()
                        .color(p.text_dim),
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
                        ("Alt+F / E / V / S / M / T / C / L / W / H", "Open Top Menu + Arrow-Key Navigation"),
                        ("Tab or F4 (Shift+Tab)", "Toggle Full-Screen Canvas Mode (or Right Dock)"),
                        ("Ctrl + Mouse Wheel", "Zoom In / Out Anchored at Cursor"),
                        ("Shift + Mouse Wheel", "Horizontal Canvas Scroll (Left / Right)"),
                        ("Mouse Wheel", "Vertical Canvas Scroll (Up / Down)"),
                        ("Alt + Mouse Wheel / Middle-Drag / Space", "2D Hand Pan Across Canvas"),
                        ("Ctrl+C / Ctrl+X / Ctrl+V", "Copy / Cut / Paste Shapes, OS Images & Text"),
                        ("Right-Click on Canvas", "Adaptive CS6 Context Menu"),
                        ("Shift-Click / Marquee Drag", "Multi-Object Selection"),
                        ("Middle-Click Document Tab", "Close Document Tab"),
                        ("Ctrl+G / Ctrl+Shift+G", "Group / Ungroup Selected Objects"),
                        ("F2 / F5 / F6 / F7 / F8", "Layers / Pages / Optimize / Library / Convert Symbol"),
                        ("Ctrl+K / Ctrl+P", "Searchable Command Palette (60 CS6 Commands)"),
                        ("V / A / Q / U / P / T / B / E / I / G / D / X", "CS6 Single-Key Tools, Default & Swap Colors"),
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

/// The refactored file browser used for Open, Save, Import and Export.
pub fn file_dialog(app: &mut FireCraft) {
    if !app.show_file_dialog {
        return;
    }
    let mut open = app.show_file_dialog;
    let mut close_now = false;
    let p = app.palette;
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
        .default_size([680.0, 460.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(&app.ctx.clone(), |ui| {
            let filter_id = ui.make_persistent_id("file_dialog_filter_str");
            let mut filter_str: String =
                ui.ctx().data(|d| d.get_temp(filter_id).unwrap_or_default());

            // Top Navigation Bar: Up + Clickable Breadcrumbs + Filter
            ui.horizontal(|ui| {
                if ui.button("⬆ Up").clicked()
                    && let Some(parent) = app.file_dialog_dir.parent()
                {
                    let parent = parent.to_path_buf();
                    app.set_file_dialog_dir(parent);
                }
                if ui.button("↻").on_hover_text("Refresh folder").clicked() {
                    let cur = app.file_dialog_dir.clone();
                    app.set_file_dialog_dir(cur);
                }
                ui.separator();
                let mut nav_target: Option<std::path::PathBuf> = None;
                let mut accum = std::path::PathBuf::new();
                for comp in app.file_dialog_dir.components() {
                    accum.push(comp);
                    let lbl = comp.as_os_str().to_string_lossy();
                    if ui.small_button(lbl.as_ref()).clicked() {
                        nav_target = Some(accum.clone());
                    }
                    ui.label(RichText::new("›").small().color(p.text_dim));
                }
                if let Some(t) = nav_target {
                    app.set_file_dialog_dir(t);
                }
            });

            let type_filter_id = ui.make_persistent_id("file_dialog_type_filter");
            let mut type_filter: usize = ui.ctx().data(|d| d.get_temp(type_filter_id).unwrap_or(0));
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Filter:").small().color(p.text_dim));
                ui.add(
                    egui::TextEdit::singleline(&mut filter_str)
                        .desired_width(210.0)
                        .hint_text("Filter files by name or extension…"),
                );
                if !filter_str.is_empty() && ui.small_button("×").clicked() {
                    filter_str.clear();
                }
                if matches!(app.file_mode, FileMode::Open | FileMode::Import) {
                    ui.separator();
                    for (ti, tlbl) in ["All", "Images", "SVG", "PSD/AI"].iter().enumerate() {
                        if ui.selectable_label(type_filter == ti, *tlbl).clicked() {
                            type_filter = ti;
                        }
                    }
                }
            });
            ui.ctx().data_mut(|d| {
                d.insert_temp(filter_id, filter_str.clone());
                d.insert_temp(type_filter_id, type_filter);
            });
            ui.separator();

            // Two-Column Body: Quick Places Sidebar | File List
            let mut confirm_now = false;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(132.0);
                    ui.label(
                        RichText::new("Quick Places")
                            .strong()
                            .small()
                            .color(p.text_dim),
                    );
                    ui.add_space(4.0);
                    if let Ok(home) =
                        std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME"))
                    {
                        let hp = std::path::PathBuf::from(home);
                        for (lbl, sub) in [
                            ("🏠 Home", ""),
                            ("🖥 Desktop", "Desktop"),
                            ("📄 Documents", "Documents"),
                            ("🖼 Pictures", "Pictures"),
                            ("⬇ Downloads", "Downloads"),
                        ] {
                            let target = if sub.is_empty() {
                                hp.clone()
                            } else {
                                hp.join(sub)
                            };
                            if target.exists()
                                && ui
                                    .selectable_label(app.file_dialog_dir == target, lbl)
                                    .clicked()
                            {
                                app.set_file_dialog_dir(target);
                            }
                        }
                    }
                    if let Ok(cwd) = std::env::current_dir()
                        && ui
                            .selectable_label(app.file_dialog_dir == cwd, "📁 Workspace")
                            .clicked()
                    {
                        app.set_file_dialog_dir(cwd);
                    }
                });

                ui.separator();

                ui.vertical(|ui| {
                    let q = filter_str.trim().to_lowercase();
                    egui::ScrollArea::vertical()
                        .max_height(255.0)
                        .show(ui, |ui| {
                            for entry in app.file_dialog_entries.clone() {
                                let name = entry
                                    .file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_default();
                                if !q.is_empty() && !name.to_lowercase().contains(&q) {
                                    continue;
                                }
                                let is_dir = entry.is_dir();
                                let ext = entry
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("")
                                    .to_uppercase();
                                if !is_dir
                                    && matches!(app.file_mode, FileMode::Open | FileMode::Import)
                                {
                                    let ext_l = ext.to_lowercase();
                                    let ok = match type_filter {
                                        1 => matches!(
                                            ext_l.as_str(),
                                            "png"
                                                | "jpg"
                                                | "jpeg"
                                                | "webp"
                                                | "gif"
                                                | "bmp"
                                                | "tiff"
                                                | "tif"
                                                | "ico"
                                        ),
                                        2 => ext_l == "svg",
                                        3 => matches!(ext_l.as_str(), "psd" | "psb" | "ai"),
                                        _ => true,
                                    };
                                    if !ok {
                                        continue;
                                    }
                                }
                                let badge = if is_dir {
                                    "📁".to_string()
                                } else if ext.is_empty() {
                                    "📄".to_string()
                                } else {
                                    format!("[{ext}]")
                                };
                                let size_str = if is_dir {
                                    "Folder".to_string()
                                } else if let Ok(meta) = std::fs::metadata(&entry) {
                                    let kb = meta.len() / 1024;
                                    if kb >= 1024 {
                                        format!("{:.1} MB", kb as f32 / 1024.0)
                                    } else {
                                        format!("{} KB", kb.max(1))
                                    }
                                } else {
                                    String::new()
                                };
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(badge).monospace().small().color(if is_dir {
                                            p.accent
                                        } else {
                                            p.text_dim
                                        }),
                                    );
                                    let row_resp =
                                        ui.selectable_label(app.file_dialog_name == name, &name);
                                    if row_resp.clicked() {
                                        if is_dir {
                                            app.set_file_dialog_dir(entry);
                                        } else {
                                            app.file_dialog_name = name;
                                        }
                                    }
                                    if !is_dir && row_resp.double_clicked() {
                                        confirm_now = true;
                                    }
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.label(
                                                RichText::new(size_str).small().color(p.text_dim),
                                            );
                                        },
                                    );
                                });
                            }
                        });
                });
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label(RichText::new("File name:").strong());
                ui.add(egui::TextEdit::singleline(&mut app.file_dialog_name).desired_width(380.0));
            });
            if matches!(app.file_mode, FileMode::Save | FileMode::Export) {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new("Format:").small().color(p.text_dim));
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
                        let cur_fmt = crate::formats::Format::from_path(std::path::Path::new(
                            &app.file_dialog_name,
                        ));
                        if ui.selectable_label(cur_fmt == fmt, fmt.label()).clicked() {
                            let stem = std::path::Path::new(&app.file_dialog_name)
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .filter(|s| !s.is_empty())
                                .unwrap_or("export");
                            app.file_dialog_name = format!("{stem}.{}", fmt.extension());
                            app.doc.optimize.format = fmt.label().to_string();
                        }
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Quality:").small().color(p.text_dim));
                    ui.add(egui::Slider::new(&mut app.doc.optimize.quality, 10..=100).suffix("%"));
                    ui.separator();
                    let mut trans = app.doc.page().transparent_bg;
                    if ui.checkbox(&mut trans, "Transparent BG").changed() {
                        app.doc.page_mut().transparent_bg = trans;
                    }
                    ui.separator();
                    ui.label(
                        RichText::new(format!("Est: ~{} KB", app.estimated_export_kb()))
                            .small()
                            .color(p.accent),
                    );
                });
            }
            if confirm_now {
                app.commit_file_dialog();
                close_now = true;
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

/// The Fireworks CS6 Welcome Splash screen overlay (`screenshots/Fireworks-Tab_View.jpg`).
pub fn startup_splash(ui: &mut egui::Ui, app: &mut FireCraft) -> bool {
    let ctx = ui.ctx().clone();
    let t = ctx.input(|i| i.time);
    let skip_id = ui.make_persistent_id("splash_skipped");
    let tex_id = ui.make_persistent_id("firecraft_splash_tex");
    let area_id = ui.make_persistent_id("cs6_startup_splash");
    let skipped = ctx.data(|d| d.get_temp::<bool>(skip_id).unwrap_or(false));
    if t >= 2.2 || skipped || app.screenshot_path.is_some() {
        return false;
    }
    let alpha = if t < 0.30 {
        (t / 0.30) as f32
    } else if t > 1.8 {
        ((2.2 - t) / 0.4).clamp(0.0, 1.0) as f32
    } else {
        1.0
    };
    let tex = ctx
        .data_mut(|d| d.get_temp::<egui::TextureHandle>(tex_id))
        .unwrap_or_else(|| {
            let bytes = include_bytes!("../../screenshots/FireCraft Flame Tech Logo.png");
            let img = image::load_from_memory(bytes).expect("valid splash logo");
            let size = [img.width() as usize, img.height() as usize];
            let rgba = img.to_rgba8();
            let color_img = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            let handle = ctx.load_texture(
                "firecraft_splash_tex",
                color_img,
                egui::TextureOptions::LINEAR,
            );
            ctx.data_mut(|d| d.insert_temp(tex_id, handle.clone()));
            handle
        });
    let screen = ctx.content_rect();
    ui.painter().rect_filled(
        screen,
        0.0,
        egui::Color32::from_rgba_unmultiplied(18, 18, 22, 235),
    );
    egui::Area::new(area_id)
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(&ctx, |ui| {
            let p = app.palette;
            let (rect, resp) =
                ui.allocate_exact_size(egui::Vec2::new(420.0, 260.0), egui::Sense::click());
            let painter = ui.painter();
            let bg = egui::Color32::from_rgba_unmultiplied(28, 28, 34, (220.0 * alpha) as u8);
            painter.rect_filled(rect, 0.0, bg);
            painter.rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(1.5, p.accent.gamma_multiply(alpha)),
                egui::StrokeKind::Inside,
            );
            let pulse = 1.0 + 0.04 * (t as f32 * 5.0).sin();
            let logo_sz = egui::Vec2::splat(136.0 * pulse);
            let logo_r = egui::Rect::from_center_size(
                egui::pos2(rect.center().x, rect.top() + 92.0),
                logo_sz,
            );
            painter.image(
                tex.id(),
                logo_r,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::from_white_alpha((255.0 * alpha) as u8),
            );
            painter.text(
                egui::pos2(rect.center().x, rect.bottom() - 62.0),
                egui::Align2::CENTER_CENTER,
                "FireCraft Studio",
                egui::FontId::proportional(20.0),
                egui::Color32::from_white_alpha((245.0 * alpha) as u8),
            );
            let prog = (t / 2.0).clamp(0.0, 1.0) as f32;
            let bar_bg = egui::Rect::from_min_size(
                egui::pos2(rect.left() + 48.0, rect.bottom() - 32.0),
                egui::Vec2::new(rect.width() - 96.0, 5.0),
            );
            painter.rect_filled(
                bar_bg,
                0.0,
                egui::Color32::from_white_alpha((45.0 * alpha) as u8),
            );
            let bar_fg = egui::Rect::from_min_size(
                bar_bg.min,
                egui::Vec2::new(bar_bg.width() * prog, bar_bg.height()),
            );
            painter.rect_filled(bar_fg, 0.0, p.accent.gamma_multiply(alpha));
            if resp.clicked() {
                ctx.data_mut(|d| d.insert_temp(skip_id, true));
            }
        });
    ctx.request_repaint();
    true
}

pub fn welcome_overlay(app: &mut FireCraft) {
    if !app.show_welcome {
        return;
    }
    let ctx = app.ctx.clone();
    egui::Window::new("Welcome to FireCraft CS6")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .fixed_size([624.0, 412.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .frame(
            egui::Frame::new()
                .fill(app.palette.welcome)
                .stroke(egui::Stroke::new(1.0, app.palette.stroke))
                .inner_margin(egui::Margin::same(4)),
        )
        .show(&ctx, |ui| {
            crate::panels::start_panel(ui, app);
            ui.separator();
            ui.horizontal(|ui| {
                if ui
                    .checkbox(&mut app.dont_show_welcome_again, "Don't show again")
                    .changed()
                    && app.dont_show_welcome_again
                {
                    app.show_welcome = false;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Start Blank 1080p Canvas").clicked() {
                        app.show_welcome = false;
                    }
                    if ui.button("Load 1080p Showcase Demo").clicked() {
                        app.load_demo_showcase();
                        app.show_welcome = false;
                    }
                });
            });
        });
}
