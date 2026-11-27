//! Modal dialogs: New Document, Canvas Size, Open/Save/Export/Import browser,
//! Find and Replace, Manage Saved Commands, Command Palette (Ctrl+K), MCP/CLI Console,
//! About/Help, and the CS6 Welcome Splash.
//!
//! Notice: Every dialog uses a local `should_close` flag combined with `open` so clicking
//! `Cancel` or `OK` inside the window body is never overwritten by `open = true` at the end.

use crate::app::{FileMode, FindReplaceMode, FireCraft};
use crate::constants;
use crate::mcp;

use eframe::egui::{self, Align2, Color32, RichText, Vec2};

mod file_browser;
mod find_replace;

pub use file_browser::file_dialog;
pub use find_replace::find_replace_dialog;

/// Shared dialog header: a compact accent rule, title, and one-line explanation. Keeping this in
/// one place makes every modal read as part of the same application instead of a collection of
/// unrelated windows.
fn dialog_header(ui: &mut egui::Ui, p: crate::theme::Palette, title: &str, subtitle: &str) {
    let (rule, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 2.0), egui::Sense::hover());
    ui.painter().rect_filled(rule, 0.0, p.accent);
    ui.add_space(7.0);
    ui.label(RichText::new(title).size(16.0).strong().color(p.text));
    if !subtitle.is_empty() {
        ui.label(RichText::new(subtitle).small().color(p.text_dim));
    }
    ui.add_space(8.0);
}

fn primary_button(ui: &mut egui::Ui, p: crate::theme::Palette, label: &str) -> bool {
    ui.add(
        egui::Button::new(RichText::new(label).strong().color(p.accent_text))
            .fill(p.accent)
            .stroke(egui::Stroke::new(1.0, p.selection)),
    )
    .clicked()
}

fn dialog_footer_rule(ui: &mut egui::Ui, p: crate::theme::Palette) {
    ui.add_space(8.0);
    let (rule, _) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().rect_filled(rule, 0.0, p.stroke);
    ui.add_space(7.0);
}

/// The Fireworks CS6 New Document dialog.
pub fn new_document_dialog(app: &mut FireCraft) {
    if !app.show_new_dialog {
        return;
    }
    let mut open = app.show_new_dialog;
    let mut close_now = false;
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("New Document")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .default_width(430.0)
        .default_pos(center - Vec2::new(215.0, 190.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "New Document",
                "Set up the canvas and color space. Presets are editable before creation.",
            );
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

            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if ui.button("Template: Web Hero").clicked() {
                    app.create_template_document(
                        "Web Hero Banner",
                        app.new_doc_width.max(800.0),
                        app.new_doc_height.max(600.0),
                    );
                    close_now = true;
                }
                if ui.button("Cancel").clicked() {
                    close_now = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, p, "Create") {
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
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("Canvas Size & Resolution")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .default_width(500.0)
        .default_pos(center - Vec2::new(250.0, 150.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "Canvas Size & Resolution",
                "Resize the document canvas without scaling its objects.",
            );
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
            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if ui.button("Fit to Objects").clicked() {
                    app.fit_canvas_to_contents();
                    close_now = true;
                }
                if ui.button("Cancel").clicked() {
                    close_now = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, p, "Apply") {
                        app.resize_canvas(app.new_doc_width.max(1.0), app.new_doc_height.max(1.0));
                        close_now = true;
                    }
                });
            });
        });
    app.show_canvas_size_dialog = open && !close_now;
}

/// HTML Setup dialog (File > HTML Setup…).
pub fn html_setup_dialog(app: &mut FireCraft) {
    if !app.show_html_setup_dialog {
        return;
    }
    let mut open = app.show_html_setup_dialog;
    let mut close_now = app.ctx.input(|i| i.key_pressed(egui::Key::Escape));
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("HTML Setup")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .default_width(440.0)
        .default_pos(center - Vec2::new(220.0, 180.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "HTML Setup",
                "Choose the markup style and web overlays included in HTML export.",
            );
            ui.horizontal(|ui| {
                ui.label("HTML Style:");
                let style_label = match app.html_setup.style {
                    crate::formats::HtmlStyle::Html5 => "HTML5 Semantic",
                    crate::formats::HtmlStyle::Xhtml1 => "XHTML 1.0 Transitional",
                    crate::formats::HtmlStyle::CssGrid => "HTML5 + CSS Grid",
                };
                egui::ComboBox::from_id_salt("html_setup_style_combo")
                    .width(190.0)
                    .selected_text(style_label)
                    .show_ui(ui, |ui| {
                        for (st, lbl) in [
                            (crate::formats::HtmlStyle::Html5, "HTML5 Semantic"),
                            (crate::formats::HtmlStyle::Xhtml1, "XHTML 1.0 Transitional"),
                            (crate::formats::HtmlStyle::CssGrid, "HTML5 + CSS Grid"),
                        ] {
                            if ui
                                .selectable_label(app.html_setup.style == st, lbl)
                                .clicked()
                            {
                                app.html_setup.style = st;
                            }
                        }
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Extension:");
                egui::ComboBox::from_id_salt("html_setup_ext_combo")
                    .width(90.0)
                    .selected_text(app.html_setup.extension)
                    .show_ui(ui, |ui| {
                        for ext in [".html", ".htm"] {
                            if ui
                                .selectable_label(app.html_setup.extension == ext, ext)
                                .clicked()
                            {
                                app.html_setup.extension = ext;
                            }
                        }
                    });
            });
            ui.checkbox(
                &mut app.html_setup.include_slices,
                "Include Slices & Hotspots",
            );
            ui.checkbox(
                &mut app.html_setup.center_in_browser,
                "Center Stage in Browser",
            );
            ui.checkbox(
                &mut app.html_setup.include_viewport_meta,
                "Include Responsive Viewport Meta",
            );
            ui.checkbox(
                &mut app.html_setup.export_doc_bg,
                "Export Document Background Color",
            );
            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if ui.button("Preview in Browser (F12)").clicked() {
                    app.preview_in_browser();
                    close_now = true;
                }
                if ui.button("Cancel").clicked() {
                    close_now = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, p, "OK") {
                        app.status = "Updated HTML Setup options".to_string();
                        close_now = true;
                    }
                });
            });
        });
    app.show_html_setup_dialog = open && !close_now;
}

/// Page Setup dialog (File > Page Setup…).
pub fn page_setup_dialog(app: &mut FireCraft) {
    if !app.show_page_setup_dialog {
        return;
    }
    let mut open = app.show_page_setup_dialog;
    let mut close_now = app.ctx.input(|i| i.key_pressed(egui::Key::Escape));
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("Page Setup")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .default_width(440.0)
        .default_pos(center - Vec2::new(220.0, 180.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "Page Setup",
                "Print layout, margins, and guide output. These settings do not resize the canvas.",
            );
            ui.horizontal(|ui| {
                ui.label("Paper Size:");
                egui::ComboBox::from_id_salt("page_setup_paper_combo")
                    .width(195.0)
                    .selected_text(app.page_setup.paper.label())
                    .show_ui(ui, |ui| {
                        for paper in [
                            crate::app::PaperSize::Letter,
                            crate::app::PaperSize::A4,
                            crate::app::PaperSize::Legal,
                            crate::app::PaperSize::Tabloid,
                        ] {
                            if ui
                                .selectable_label(app.page_setup.paper == paper, paper.label())
                                .clicked()
                            {
                                app.page_setup.paper = paper;
                            }
                        }
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Orientation:");
                ui.radio_value(&mut app.page_setup.landscape, false, "Portrait");
                ui.radio_value(&mut app.page_setup.landscape, true, "Landscape");
            });
            ui.horizontal(|ui| {
                ui.label("Margins:");
                ui.add(
                    egui::DragValue::new(&mut app.page_setup.margin_in)
                        .speed(0.05)
                        .range(0.0..=3.0)
                        .suffix(" in"),
                );
            });
            ui.checkbox(&mut app.page_setup.fit_to_page, "Fit Artwork to Page");
            if !app.page_setup.fit_to_page {
                ui.horizontal(|ui| {
                    ui.label("Scale:");
                    ui.add(
                        egui::DragValue::new(&mut app.page_setup.scale_pct)
                            .speed(1.0)
                            .range(10.0..=400.0)
                            .suffix("%"),
                    );
                });
            }
            ui.checkbox(
                &mut app.page_setup.print_guides_slices,
                "Print Guides & Slice Overlays",
            );
            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if ui.button("Print…").clicked() {
                    app.print_document();
                    close_now = true;
                }
                if ui.button("Cancel").clicked() {
                    close_now = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, p, "OK") {
                        app.status =
                            format!("Page Setup updated ({})", app.page_setup.paper.label());
                        close_now = true;
                    }
                });
            });
        });
    app.show_page_setup_dialog = open && !close_now;
}

/// Manage Saved Commands dialog (Commands > Manage Saved Commands).
pub fn saved_commands_dialog(app: &mut FireCraft) {
    if !app.show_commands_dialog {
        return;
    }
    let mut open = app.show_commands_dialog;
    let mut close_now = false;
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("Manage Saved Commands")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .movable(true)
        .default_width(470.0)
        .default_height(330.0)
        .default_pos(center - Vec2::new(235.0, 165.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "Saved Commands",
                "Reusable automation steps and batch actions for the current document.",
            );
            egui::ScrollArea::vertical()
                .max_height(170.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for cmd in app.saved_commands.clone() {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&cmd).color(p.text));
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui.small_button("Run").clicked() {
                                        app.execute_saved_command(&cmd);
                                    }
                                },
                            );
                        });
                        ui.separator();
                    }
                });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label("New command:");
                ui.add(
                    egui::TextEdit::singleline(&mut app.new_command_name)
                        .desired_width(f32::INFINITY)
                        .hint_text("Name this command"),
                );
            });
            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if primary_button(ui, p, "Save Step") && !app.new_command_name.trim().is_empty() {
                    let name = app.new_command_name.trim().to_string();
                    app.saved_commands.push(name.clone());
                    app.new_command_name.clear();
                    app.status = format!("Saved command: {name}");
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Close").clicked() {
                        close_now = true;
                    }
                });
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
    let screen = app.ctx.content_rect();
    egui::Window::new("Command Palette")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .movable(true)
        .default_width(600.0)
        .default_height(410.0)
        .min_width(480.0)
        .default_pos(egui::pos2(screen.center().x - 300.0, screen.top() + 54.0))
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
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("MCP & Headless CLI Console")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .movable(true)
        .default_width(660.0)
        .default_height(480.0)
        .min_width(520.0)
        .default_pos(center - Vec2::new(330.0, 240.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "MCP & Headless CLI Console",
                "Test the live JSON-RPC server or copy a headless command for scripts and CI.",
            );
            ui.label(
                RichText::new(format!(
                    "MCP Protocol {} · Tools: firecraft.info, firecraft.apply, firecraft.export, firecraft.undo, firecraft.redo, firecraft.listCommands",
                    mcp::PROTOCOL_VERSION
                ))
                .small()
                .color(p.text_dim),
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
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, p, "Close") {
                        close_now = true;
                    }
                });
            });
            if !app.mcp_output.is_empty() {
                ui.separator();
                egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
                    ui.code(&app.mcp_output);
                });
            }
        });
    app.show_mcp_dialog = open && !close_now;
}

/// Per-tab close confirmation. The window is deliberately not fixed-height: the old 145 px
/// fixed size clipped the entire action row, which made Save / Don't Save / Cancel appear dead.
pub fn unsaved_changes_dialog(app: &mut FireCraft) {
    let Some(idx) = app.pending_close_tab else {
        return;
    };
    let doc_name = if idx == app.active_doc {
        app.doc.name.clone()
    } else {
        app.open_docs
            .get(idx)
            .map(|d| d.name.clone())
            .unwrap_or_else(|| "Untitled".to_string())
    };
    let has_path = app
        .open_docs
        .get(idx)
        .is_some_and(|d| d.file_path.is_some());
    let mut open = true;
    let mut close_now = false;
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("Save Changes?")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .default_width(432.0)
        .min_width(400.0)
        .default_pos(center - Vec2::new(216.0, 120.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "Save changes?",
                &format!("\"{doc_name}\" has unsaved edits."),
            );
            ui.label(
                RichText::new(if has_path {
                    "Save the document before closing this tab?"
                } else {
                    "This document has not been saved yet. Save will ask for a filename."
                })
                .color(p.text),
            );
            ui.label(
                RichText::new("Closing without saving discards the changes.")
                    .small()
                    .color(p.text_dim),
            );
            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if primary_button(ui, p, "Save") {
                    if idx != app.active_doc {
                        app.switch_document_tab(idx);
                    }
                    if app.current_path.is_some() {
                        app.save_current();
                        if !app.doc.dirty {
                            app.close_document_tab(idx);
                        }
                        app.pending_close_tab = None;
                        app.continue_pending_tab_closes();
                    } else {
                        // Keep the request alive while the Save As browser is open.
                        app.pending_close_after_save = Some(idx);
                        app.pending_close_tab = None;
                        app.begin_file_dialog(FileMode::Save);
                    }
                    close_now = true;
                }
                if ui.button("Don't Save").clicked() {
                    app.close_document_tab(idx);
                    app.pending_close_tab = None;
                    close_now = true;
                    app.continue_pending_tab_closes();
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        app.pending_close_tab = None;
                        app.pending_close_tabs.clear();
                        close_now = true;
                    }
                });
            });
        });
    if !open {
        // Dismissing the dialog with its window close button cancels the whole bulk-close queue,
        // not just the tab currently being asked about.
        app.pending_close_tab = None;
        app.pending_close_tabs.clear();
    } else if close_now {
        app.pending_close_tab = None;
    }
}

/// Session-close confirmation. Buttons are visible, clickable, and never discard an untitled
/// document silently: Save All walks through Save As dialogs one document at a time.
pub fn quit_confirm_dialog(app: &mut FireCraft) {
    if !app.show_quit_confirm {
        return;
    }
    let p = app.palette;
    let dirty = app
        .open_docs
        .iter()
        .filter(|d| d.dirty)
        .map(|d| d.name.clone())
        .collect::<Vec<_>>();
    let dirty_count = dirty.len().max(usize::from(app.doc.dirty));
    let mut open = true;
    let mut close = false;
    let center = app.ctx.content_rect().center();
    egui::Window::new("Unsaved Changes")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .default_width(460.0)
        .min_width(420.0)
        .default_pos(center - Vec2::new(230.0, 130.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "Unsaved changes",
                if dirty_count == 1 {
                    "One document has edits that are not on disk."
                } else {
                    "Several documents have edits that are not on disk."
                },
            );
            ui.label(
                RichText::new(if dirty_count == 1 {
                    "There is 1 document with unsaved changes.".to_string()
                } else {
                    format!("There are {dirty_count} documents with unsaved changes.")
                })
                .strong(),
            );
            ui.add_space(3.0);
            ui.label(
                RichText::new(if dirty.is_empty() {
                    &app.doc.name
                } else {
                    dirty.first().unwrap_or(&app.doc.name)
                })
                .small()
                .color(p.text_dim),
            );
            ui.label(
                RichText::new("Save all documents before quitting? Untitled documents will each ask for a filename.")
                    .small()
                    .color(p.text_dim),
            );
            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if primary_button(ui, p, "Save All & Quit") {
                    app.show_quit_confirm = false;
                    match app.save_all_and_quit_step() {
                        Ok(true) => {}
                        Ok(false) => close = true,
                        Err(err) => app.status = err,
                    }
                }
                if ui.button("Discard & Quit").clicked() {
                    app.confirm_discard_and_quit();
                    close = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Cancel").clicked() {
                        app.show_quit_confirm = false;
                        app.pending_quit_after_save = false;
                    }
                });
            });
        });
    if close {
        // The user already made the close decision; do not let the unsaved-work interceptor
        // cancel the command on the next frame.
        app.allow_close = true;
        app.ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }
    if !open {
        app.show_quit_confirm = false;
        app.pending_quit_after_save = false;
    }
}

pub fn update_dialog(app: &mut FireCraft) {
    if !app.show_update_dialog {
        return;
    }
    let mut open = app.show_update_dialog;
    let mut close_now = false;
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("Check for Updates")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .movable(true)
        .default_width(470.0)
        .default_pos(center - Vec2::new(235.0, 130.0))
        .show(&app.ctx.clone(), |ui| {
            dialog_header(
                ui,
                p,
                "Check for Updates",
                "FireCraft can check GitHub Releases for a newer build.",
            );
            ui.label(
                RichText::new(format!("FireCraft Studio v{}", env!("CARGO_PKG_VERSION")))
                    .strong()
                    .color(p.accent),
            );
            ui.add_space(6.0);
            ui.label(&app.update_status);
            if let Some(tag) = &app.update_latest_version {
                ui.label(
                    RichText::new(format!("Latest GitHub Release: {tag}"))
                        .small()
                        .color(p.text_dim),
                );
            }
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if primary_button(ui, p, "Check Now") {
                    app.check_for_updates();
                }
                if ui.button("Open Releases Page").clicked() {
                    app.open_update_release_page();
                }
            });
            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                if ui.button("Copy Version Info").clicked() {
                    app.ctx.copy_text(format!(
                        "FireCraft Studio v{} ({})",
                        env!("CARGO_PKG_VERSION"),
                        std::env::consts::OS
                    ));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, p, "Close") {
                        close_now = true;
                    }
                });
            });
        });
    app.show_update_dialog = open && !close_now;
}

pub fn about_dialog(app: &mut FireCraft) {
    if !app.show_about_dialog {
        return;
    }
    let mut open = app.show_about_dialog;
    let mut close_now = false;
    let p = app.palette;
    let center = app.ctx.content_rect().center();
    egui::Window::new("About FireCraft")
        .open(&mut open)
        .collapsible(false)
        .resizable(true)
        .movable(true)
        .default_width(610.0)
        .min_width(520.0)
        .default_height(440.0)
        .min_height(360.0)
        .default_pos(center - Vec2::new(305.0, 230.0))
        .show(&app.ctx.clone(), |ui| {
            ui.horizontal(|ui| {
                if let Some(tex) = &app.logo_tex {
                    let (rect, _) = ui.allocate_exact_size(Vec2::splat(64.0), egui::Sense::hover());
                    ui.painter().image(
                        tex.id(),
                        rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                }
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new("FireCraft Studio")
                            .size(22.0)
                            .strong()
                            .color(p.text),
                    );
                    ui.label(
                        RichText::new(format!(
                            "CS6 Parity Edition · v{} · {}",
                            env!("CARGO_PKG_VERSION"),
                            std::env::consts::OS
                        ))
                        .small()
                        .color(p.accent),
                    );
                    ui.label(
                        RichText::new("Hybrid vector, bitmap, and web graphics editor")
                            .small()
                            .color(p.text_dim),
                    );
                });
            });
            ui.add_space(8.0);
            let (rule, _) = ui.allocate_exact_size(
                Vec2::new(ui.available_width(), 1.0),
                egui::Sense::hover(),
            );
            ui.painter().rect_filled(rule, 0.0, p.stroke);
            ui.add_space(8.0);

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .max_height(280.0)
                .show(ui, |ui| {
                    ui.label(
                        RichText::new("A clean-room reimagining of Adobe Fireworks CS6, built for modern displays, larger documents, and cross-application workflows.")
                            .color(p.text),
                    );
                    ui.add_space(4.0);
                    ui.horizontal_wrapped(|ui| {
                        ui.hyperlink_to("Project Wiki", "https://github.com/aliasfoxkde/FireCraft/wiki");
                        ui.label(RichText::new("·").color(p.text_dim));
                        ui.hyperlink_to(
                            "Source & Releases",
                            "https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment",
                        );
                    });
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new("WORKFLOW SHORTCUTS")
                            .small()
                            .strong()
                            .color(p.accent),
                    );
                    ui.add_space(2.0);
                    egui::Grid::new("about_shortcuts_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            for (k, d) in [
                                ("V  A  Q  U  P  T  B  E  G  I", "Core CS6 tool switches"),
                                ("Tab / F4", "Full-screen canvas; Shift+Tab toggles right dock"),
                                ("Ctrl+0 / Ctrl+1 / Ctrl+2", "Fit, 100%, and 200% zoom"),
                                ("Ctrl+F", "Find & Replace (text, names, fills, strokes, any color)"),
                                ("Ctrl+Shift+F", "God-view infinite canvas"),
                                ("Ctrl+G / Ctrl+Shift+G", "Group / ungroup selection"),
                                ("F2 / F5 / F6", "Layers / Pages / Optimize panels"),
                                ("Ctrl+K / Ctrl+P", "Command palette"),
                                ("Space / middle-drag / Alt+wheel", "Pan without changing tools"),
                            ] {
                                ui.label(RichText::new(k).monospace().color(p.selection));
                                ui.label(RichText::new(d).color(p.text));
                                ui.end_row();
                            }
                        });
                });

            dialog_footer_rule(ui, p);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("Built with Rust + egui · No Adobe code used")
                        .small()
                        .color(p.text_dim),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, p, "Close") {
                        close_now = true;
                    }
                });
            });
        });
    app.show_about_dialog = open && !close_now;
}

/// Brief, transparent brand overlay while the real workspace remains visible and responsive.
pub fn startup_splash(ui: &mut egui::Ui, app: &mut FireCraft) {
    let ctx = ui.ctx().clone();
    let t = ctx.input(|i| i.time);
    let skip_id = ui.make_persistent_id("splash_skipped");
    let tex_id = ui.make_persistent_id("firecraft_splash_tex");
    let area_id = ui.make_persistent_id("cs6_startup_splash");
    let skipped = ctx.data(|d| d.get_temp::<bool>(skip_id).unwrap_or(false));
    let skip_requested = ctx.input(|i| i.pointer.any_click() || i.key_pressed(egui::Key::Escape));
    if skip_requested {
        ctx.data_mut(|d| d.insert_temp(skip_id, true));
    }
    const DURATION: f64 = 2.4;
    if t >= DURATION || skipped || skip_requested || app.screenshot_path.is_some() {
        return;
    }
    let alpha = startup_splash_alpha(t);
    let tex = ctx
        .data_mut(|d| d.get_temp::<egui::TextureHandle>(tex_id))
        .unwrap_or_else(|| {
            let bytes = startup_splash_image_bytes();
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
    egui::Area::new(area_id)
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .interactable(false)
        .show(&ctx, |ui| {
            let (rect, _) =
                ui.allocate_exact_size(egui::Vec2::new(330.0, 220.0), egui::Sense::hover());
            ui.painter().image(
                tex.id(),
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::from_white_alpha((255.0 * alpha) as u8),
            );
        });
    ctx.request_repaint_after(std::time::Duration::from_millis(16));
}

fn startup_splash_alpha(time: f64) -> f32 {
    const DURATION: f64 = 2.4;
    if time < 0.25 {
        (time / 0.25).clamp(0.0, 1.0) as f32
    } else if time > 1.9 {
        ((DURATION - time) / (DURATION - 1.9)).clamp(0.0, 1.0) as f32
    } else {
        1.0
    }
}

fn startup_splash_image_bytes() -> &'static [u8] {
    include_bytes!("../../assets/splash.png")
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
                    if ui.button("Load Studio Showcase Demo").clicked() {
                        app.load_demo_showcase();
                        app.show_welcome = false;
                    }
                });
            });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn startup_splash_fades_in_and_out_within_two_point_four_seconds() {
        assert_eq!(startup_splash_alpha(0.0), 0.0);
        assert_eq!(startup_splash_alpha(0.25), 1.0);
        assert_eq!(startup_splash_alpha(1.9), 1.0);
        assert!((startup_splash_alpha(2.15) - 0.5).abs() < 0.01);
        assert_eq!(startup_splash_alpha(2.4), 0.0);
    }

    #[test]
    fn startup_splash_artwork_has_a_transparent_background() {
        let image = image::load_from_memory(startup_splash_image_bytes())
            .expect("startup splash image must decode")
            .to_rgba8();
        for (x, y) in [
            (0, 0),
            (image.width() - 1, 0),
            (0, image.height() - 1),
            (image.width() - 1, image.height() - 1),
            (image.width() / 2, 0),
            (0, image.height() / 2),
            (image.width() - 1, image.height() / 2),
            (image.width() / 2, image.height() - 1),
        ] {
            assert_eq!(
                image.get_pixel(x, y).0[3],
                0,
                "splash artwork must leave background pixel ({x}, {y}) transparent"
            );
        }
    }

    fn press_key(app: &mut FireCraft, key: egui::Key) {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(1200.0, 800.0),
            )),
            time: Some(3.0),
            events: vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let ctx = app.ctx.clone();
        let mut output = ctx.run_ui(input, |_| match key {
            egui::Key::Escape if app.show_html_setup_dialog => html_setup_dialog(app),
            egui::Key::Escape if app.show_page_setup_dialog => page_setup_dialog(app),
            _ => command_palette_dialog(app),
        });
        output.textures_delta.clear();
    }

    #[test]
    fn escape_closes_html_setup_without_changing_export_preferences() {
        let mut app = FireCraft::new(None);
        let original = app.html_setup.clone();
        app.show_html_setup_dialog = true;

        press_key(&mut app, egui::Key::Escape);

        assert!(!app.show_html_setup_dialog);
        assert_eq!(app.html_setup.style, original.style);
        assert_eq!(app.html_setup.extension, original.extension);
        assert_eq!(app.html_setup.include_slices, original.include_slices);
        assert_eq!(app.html_setup.center_in_browser, original.center_in_browser);
        assert_eq!(
            app.html_setup.include_viewport_meta,
            original.include_viewport_meta
        );
        assert_eq!(app.html_setup.export_doc_bg, original.export_doc_bg);
    }

    #[test]
    fn escape_closes_page_setup_without_changing_print_preferences() {
        let mut app = FireCraft::new(None);
        let original = app.page_setup.clone();
        app.show_page_setup_dialog = true;

        press_key(&mut app, egui::Key::Escape);

        assert!(!app.show_page_setup_dialog);
        assert_eq!(app.page_setup.paper, original.paper);
        assert_eq!(app.page_setup.landscape, original.landscape);
        assert_eq!(app.page_setup.margin_in, original.margin_in);
        assert_eq!(app.page_setup.fit_to_page, original.fit_to_page);
        assert_eq!(app.page_setup.scale_pct, original.scale_pct);
        assert_eq!(
            app.page_setup.print_guides_slices,
            original.print_guides_slices
        );
    }

    #[test]
    fn command_palette_escape_closes_without_running_an_action() {
        let mut app = FireCraft::new(None);
        app.show_command_palette = true;

        press_key(&mut app, egui::Key::Escape);

        assert!(!app.show_command_palette);
        assert!(!app.show_find_replace);
    }

    #[test]
    fn command_palette_enter_runs_the_initial_find_action() {
        let mut app = FireCraft::new(None);
        app.show_command_palette = true;

        press_key(&mut app, egui::Key::Enter);

        assert!(!app.show_command_palette);
        assert!(app.show_find_replace);
    }
}
