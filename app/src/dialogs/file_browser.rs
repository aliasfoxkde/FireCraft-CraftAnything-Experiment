//! In-app file browser for Open, Save, Import, and Export.
//!
//! Keeping the browser inside the main egui context makes dragging smooth and lets the address
//! bar, type selector, and recent places share app state with the rest of the editor.

use super::*;

/// FireCraft's file browser, used for Open, Save, Import and Export. It is an in-app window rather
/// than a second immediate-mode OS viewport: dragging is smooth, the address bar is editable, and
/// the file-type selector is part of the same compact CS6-style layout.
pub fn file_dialog(app: &mut FireCraft) {
    if !app.show_file_dialog {
        return;
    }
    let title = match app.file_mode {
        FileMode::Open => "Open Document",
        FileMode::Import => "Import Image / Vector",
        FileMode::Save => "Save Document As",
        FileMode::Export => "Export Document",
    };
    if app.file_dialog_address.is_empty() {
        app.file_dialog_address = app.file_dialog_dir.display().to_string();
    }
    let mut open = app.show_file_dialog;
    let mut close_now = app.ctx.input(|i| i.key_pressed(egui::Key::Escape));
    let available = app.ctx.content_rect().size();
    let browser_save_form =
        cfg!(target_arch = "wasm32") && matches!(app.file_mode, FileMode::Save | FileMode::Export);
    let dialog_size = if browser_save_form {
        Vec2::new(
            560.0_f32.min((available.x - 48.0).max(420.0)),
            280.0_f32.min((available.y - 48.0).max(220.0)),
        )
    } else {
        Vec2::new(
            760.0_f32.min((available.x - 48.0).max(420.0)),
            500.0_f32.min((available.y - 48.0).max(300.0)),
        )
    };
    let max_size = Vec2::new(
        (available.x - 24.0).max(420.0),
        (available.y - 24.0).max(300.0),
    );
    let center = app.ctx.content_rect().center();
    egui::Window::new(title)
        .open(&mut open)
        .collapsible(false)
        .movable(true)
        .resizable(true)
        .default_size(dialog_size)
        .min_size(if browser_save_form {
            Vec2::new(420.0, 220.0).min(max_size)
        } else {
            Vec2::new(560.0, 360.0).min(max_size)
        })
        .max_size(max_size)
        .default_pos(center - dialog_size * 0.5)
        .show(&app.ctx.clone(), |ui| {
            file_dialog_body(ui, app, &mut close_now);
        });
    if close_now || !open {
        app.show_file_dialog = false;
        // A cancelled Save As must not leave a tab close or quit waiting forever.
        app.pending_close_after_save = None;
        app.pending_close_tabs.clear();
        app.pending_quit_after_save = false;
    }
}

fn file_dialog_body(ui: &mut egui::Ui, app: &mut FireCraft, close_now: &mut bool) {
    let p = app.palette;
    let is_read = matches!(app.file_mode, FileMode::Open | FileMode::Import);
    let mut confirm_now = false;

    if !cfg!(target_arch = "wasm32") {
        // Address bar: editable, Enter/Go navigates, Up and Refresh are always available.
        ui.horizontal(|ui| {
            if ui.button("↑").on_hover_text("Parent folder").clicked()
                && let Some(parent) = app.file_dialog_dir.parent()
            {
                let parent = parent.to_path_buf();
                app.set_file_dialog_dir(parent);
            }
            if ui.button("↻").on_hover_text("Refresh").clicked() {
                let cur = app.file_dialog_dir.clone();
                app.set_file_dialog_dir(cur);
            }
            // Keep space for the Go button: an infinite-width text edit consumes the
            // entire row and silently pushes the button outside the visible clip rect.
            let address_width = (ui.available_width() - 38.0).max(100.0);
            let edit = ui.add(
                egui::TextEdit::singleline(&mut app.file_dialog_address)
                    .desired_width(address_width)
                    .font(egui::TextStyle::Monospace)
                    .hint_text("Type a folder path and press Enter"),
            );
            let enter = edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            if ui.button("Go").clicked() || enter {
                let raw = app.file_dialog_address.trim().trim_matches('"');
                let path = std::path::PathBuf::from(raw);
                if path.is_dir() {
                    app.set_file_dialog_dir(path);
                } else if let Some(parent) = path.parent().filter(|p| p.is_dir()) {
                    app.file_dialog_name = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();
                    app.set_file_dialog_dir(parent.to_path_buf());
                } else {
                    app.status = format!("Folder not found: {raw}");
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label(RichText::new("Look in:").small().color(p.text_dim));
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

        ui.add_space(3.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new("Filter:").small().color(p.text_dim));
            ui.add(
                egui::TextEdit::singleline(&mut app.file_dialog_filter)
                    .desired_width(250.0)
                    .hint_text("Search this folder…"),
            );
            if !app.file_dialog_filter.is_empty() && ui.small_button("×").clicked() {
                app.file_dialog_filter.clear();
            }
            ui.separator();
            ui.label(RichText::new("File type:").small().color(p.text_dim));
            if is_read {
                let options = [
                    "All supported files",
                    "FireCraft layered (.fw.png)",
                    "Raster images",
                    "SVG vector",
                    "PSD / AI (bridge required)",
                    "All files",
                ];
                let idx = app.file_dialog_type_filter.min(options.len() - 1);
                egui::ComboBox::from_id_salt("file_browser_type")
                    .width(205.0)
                    .selected_text(options[idx])
                    .show_ui(ui, |ui| {
                        for (i, label) in options.iter().enumerate() {
                            if ui.selectable_label(idx == i, *label).clicked() {
                                app.file_dialog_type_filter = i;
                            }
                        }
                    });
            } else {
                let formats = save_formats();
                let current =
                    crate::formats::Format::from_path(std::path::Path::new(&app.file_dialog_name));
                egui::ComboBox::from_id_salt("file_browser_save_type")
                    .width(205.0)
                    .selected_text(format!("{} (.{})", current.label(), current.extension()))
                    .show_ui(ui, |ui| {
                        for fmt in formats {
                            if ui
                                .selectable_label(
                                    current == fmt,
                                    format!("{} (.{})", fmt.label(), fmt.extension()),
                                )
                                .clicked()
                            {
                                set_file_extension(app, fmt);
                                app.doc.optimize.format = fmt.label().to_string();
                            }
                        }
                    });
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{} items", app.file_dialog_entries.len()))
                        .small()
                        .color(p.text_dim),
                );
            });
        });
        ui.add_space(4.0);
    } else {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Format:").small().color(p.text_dim));
            let formats = save_formats();
            let current =
                crate::formats::Format::from_path(std::path::Path::new(&app.file_dialog_name));
            egui::ComboBox::from_id_salt("browser_save_type")
                .width(205.0)
                .selected_text(format!("{} (.{})", current.label(), current.extension()))
                .show_ui(ui, |ui| {
                    for fmt in formats {
                        if ui
                            .selectable_label(
                                current == fmt,
                                format!("{} (.{})", fmt.label(), fmt.extension()),
                            )
                            .clicked()
                        {
                            set_file_extension(app, fmt);
                            app.doc.optimize.format = fmt.label().to_string();
                        }
                    }
                });
        });
        ui.add_space(4.0);
    }

    // Quick Places rail + dense, column-aligned file list. `horizontal_top` fills all
    // remaining height by design, which pushed the filename/actions below the window.
    // Allocate the list's actual height so those persistent actions stay in the dialog.
    if !cfg!(target_arch = "wasm32") {
        let list_h = (ui.available_height() - 190.0).clamp(80.0, 320.0);
        let file_list_h = 19.0 + list_h;
        ui.allocate_ui_with_layout(
            Vec2::new(ui.available_width(), file_list_h),
            egui::Layout::left_to_right(egui::Align::Min),
            |ui| {
                ui.vertical(|ui| {
                    ui.set_width(158.0);
                    ui.label(RichText::new("PLACES").small().strong().color(p.accent));
                    ui.add_space(2.0);
                    if let Ok(home) =
                        std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME"))
                    {
                        let hp = std::path::PathBuf::from(home);
                        for (lbl, sub) in [
                            ("Home", ""),
                            ("Desktop", "Desktop"),
                            ("Documents", "Documents"),
                            ("Pictures", "Pictures"),
                            ("Downloads", "Downloads"),
                        ] {
                            let target = if sub.is_empty() {
                                hp.clone()
                            } else {
                                hp.join(sub)
                            };
                            if target.exists() {
                                let selected = app.file_dialog_dir == target;
                                if ui
                                    .add_sized(
                                        [ui.available_width(), 21.0],
                                        egui::Button::new(RichText::new(lbl).small())
                                            .selected(selected)
                                            .frame(selected),
                                    )
                                    .clicked()
                                {
                                    app.set_file_dialog_dir(target);
                                }
                            }
                        }
                    }
                    if let Ok(cwd) = std::env::current_dir()
                        && ui
                            .add_sized(
                                [ui.available_width(), 21.0],
                                egui::Button::new(RichText::new("Workspace").small())
                                    .selected(app.file_dialog_dir == cwd)
                                    .frame(app.file_dialog_dir == cwd),
                            )
                            .clicked()
                    {
                        app.set_file_dialog_dir(cwd);
                    }
                    ui.add_space(6.0);
                    ui.label(RichText::new("TIP").small().strong().color(p.accent));
                    ui.label(
                        RichText::new(
                            "Double-click a file to open it. The address bar accepts full paths.",
                        )
                        .small()
                        .color(p.text_dim),
                    );
                });

                ui.separator();

                ui.vertical(|ui| {
                    // Column header keeps the list scannable instead of the old badge + name + pushed-right size.
                    let (head, _) = ui.allocate_exact_size(
                        Vec2::new(ui.available_width(), 19.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(head, 0.0, p.panel_alt);
                    let name_x = head.left() + 24.0;
                    ui.painter().text(
                        egui::pos2(name_x, head.center().y),
                        Align2::LEFT_CENTER,
                        "Name",
                        egui::FontId::proportional(10.5),
                        p.text_dim,
                    );
                    ui.painter().text(
                        egui::pos2(head.right() - 205.0, head.center().y),
                        Align2::LEFT_CENTER,
                        "Type",
                        egui::FontId::proportional(10.5),
                        p.text_dim,
                    );
                    ui.painter().text(
                        egui::pos2(head.right() - 112.0, head.center().y),
                        Align2::RIGHT_CENTER,
                        "Size",
                        egui::FontId::proportional(10.5),
                        p.text_dim,
                    );
                    ui.painter().text(
                        egui::pos2(head.right() - 8.0, head.center().y),
                        Align2::RIGHT_CENTER,
                        "Modified",
                        egui::FontId::proportional(10.5),
                        p.text_dim,
                    );

                    let q = app.file_dialog_filter.trim().to_lowercase();
                    let visible_entries: Vec<_> = app
                        .file_dialog_entries
                        .iter()
                        .filter(|entry| {
                            let name = entry
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_default();
                            if !q.is_empty() && !name.to_lowercase().contains(&q) {
                                return false;
                            }
                            let is_dir = entry.is_dir();
                            let ext = entry
                                .extension()
                                .and_then(|e| e.to_str())
                                .unwrap_or("")
                                .to_lowercase();
                            is_dir || !is_read || file_passes_filter(app, entry, &ext)
                        })
                        .cloned()
                        .collect();
                    egui::ScrollArea::vertical()
                        .max_height(list_h)
                        .auto_shrink([false, false])
                        .show_rows(ui, 23.0, visible_entries.len(), |ui, row_range| {
                            for row_index in row_range {
                                let entry = &visible_entries[row_index];
                                let name = entry
                                    .file_name()
                                    .map(|n| n.to_string_lossy().to_string())
                                    .unwrap_or_default();
                                let is_dir = entry.is_dir();
                                let ext = entry
                                    .extension()
                                    .and_then(|e| e.to_str())
                                    .unwrap_or("")
                                    .to_lowercase();
                                let row = ui.allocate_exact_size(
                                    Vec2::new(ui.available_width(), 23.0),
                                    egui::Sense::click(),
                                );
                                let (rect, resp) = row;
                                let selected = !is_dir && app.file_dialog_name == name;
                                if selected {
                                    ui.painter().rect_filled(
                                        rect,
                                        0.0,
                                        p.selection.gamma_multiply(0.28),
                                    );
                                } else if resp.hovered() {
                                    ui.painter().rect_filled(rect, 0.0, p.well);
                                }
                                if resp.clicked() && is_dir {
                                    app.set_file_dialog_dir(entry.clone());
                                } else if resp.clicked() {
                                    app.file_dialog_name = name.clone();
                                }
                                if resp.double_clicked() && !is_dir {
                                    app.file_dialog_name = name.clone();
                                    confirm_now = true;
                                }
                                let icon_rect = egui::Rect::from_center_size(
                                    egui::pos2(rect.left() + 12.0, rect.center().y),
                                    Vec2::splat(14.0),
                                );
                                if is_dir {
                                    crate::icons::draw(
                                        ui.painter(),
                                        icon_rect,
                                        crate::icons::Icon::Folder,
                                        p.accent,
                                        1.25,
                                    );
                                } else {
                                    ui.painter().text(
                                        icon_rect.center(),
                                        Align2::CENTER_CENTER,
                                        ext.chars().take(3).collect::<String>().to_uppercase(),
                                        egui::FontId::monospace(7.5),
                                        p.text_dim,
                                    );
                                }
                                ui.painter().text(
                                    egui::pos2(rect.left() + 24.0, rect.center().y),
                                    Align2::LEFT_CENTER,
                                    &name,
                                    egui::FontId::proportional(11.5),
                                    if selected { p.text } else { p.text_dim },
                                );
                                let (type_label, size_label, date_label) = if is_dir {
                                    ("Folder".to_string(), String::new(), String::new())
                                } else {
                                    let type_label = format!("{} file", ext.to_uppercase());
                                    let meta = std::fs::metadata(entry).ok();
                                    let size_label = meta
                                        .as_ref()
                                        .map(|m| human_file_size(m.len()))
                                        .unwrap_or_default();
                                    let date_label = meta
                                        .and_then(|m| m.modified().ok())
                                        .map(relative_modified)
                                        .unwrap_or_default();
                                    (type_label, size_label, date_label)
                                };
                                ui.painter().text(
                                    egui::pos2(rect.right() - 205.0, rect.center().y),
                                    Align2::LEFT_CENTER,
                                    type_label,
                                    egui::FontId::proportional(10.5),
                                    p.text_dim,
                                );
                                ui.painter().text(
                                    egui::pos2(rect.right() - 112.0, rect.center().y),
                                    Align2::RIGHT_CENTER,
                                    size_label,
                                    egui::FontId::proportional(10.5),
                                    p.text_dim,
                                );
                                ui.painter().text(
                                    egui::pos2(rect.right() - 8.0, rect.center().y),
                                    Align2::RIGHT_CENTER,
                                    date_label,
                                    egui::FontId::proportional(10.5),
                                    p.text_dim,
                                );
                            }
                        });
                });
            },
        );
    }

    ui.add_space(3.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new("File name:").strong());
        let name_edit = ui.add(
            egui::TextEdit::singleline(&mut app.file_dialog_name).desired_width(f32::INFINITY),
        );
        if name_edit.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            confirm_now = true;
        }
    });
    if !is_read {
        ui.horizontal(|ui| {
            let format =
                crate::formats::Format::from_path(std::path::Path::new(&app.file_dialog_name));
            if matches!(
                format,
                crate::formats::Format::Jpeg | crate::formats::Format::WebP
            ) {
                ui.label(RichText::new("Quality").small().color(p.text_dim));
                ui.add(egui::Slider::new(&mut app.doc.optimize.quality, 10..=100).suffix("%"));
                ui.separator();
            }
            if !matches!(
                format,
                crate::formats::Format::Svg | crate::formats::Format::Html
            ) {
                let mut trans = app.doc.page().transparent_bg;
                if ui.checkbox(&mut trans, "Transparent background").changed() {
                    app.doc.page_mut().transparent_bg = trans;
                }
                ui.separator();
            }
            ui.label(
                RichText::new(format!("Estimated size: ~{} KB", app.estimated_export_kb()))
                    .small()
                    .color(p.accent),
            );
        });
    }

    let mut commit_clicked = false;
    let mut cancel_clicked = false;
    dialog_footer_rule(ui, p);
    ui.horizontal(|ui| {
        let action = match app.file_mode {
            FileMode::Open => "Open",
            FileMode::Import => "Import",
            FileMode::Save => "Save",
            FileMode::Export => "Export",
        };
        if primary_button(ui, p, action) {
            commit_clicked = true;
        }
        if ui.button("Cancel").clicked() {
            cancel_clicked = true;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(if is_read {
                    "Layered .fw.png files reopen with pages, layers and off-page objects intact."
                } else {
                    "PNG preserves live layers; other formats flatten to the page."
                })
                .small()
                .color(p.text_dim),
            );
        });
    });

    if confirm_now || commit_clicked {
        app.commit_file_dialog();
        if !app.show_file_dialog {
            *close_now = true;
        }
    }
    if cancel_clicked {
        *close_now = true;
    }
}

fn save_formats() -> [crate::formats::Format; 9] {
    use crate::formats::Format;
    [
        Format::Png,
        Format::Jpeg,
        Format::Gif,
        Format::WebP,
        Format::Svg,
        Format::Html,
        Format::Tiff,
        Format::Bmp,
        Format::Ico,
    ]
}

fn set_file_extension(app: &mut FireCraft, fmt: crate::formats::Format) {
    let stem = std::path::Path::new(&app.file_dialog_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("untitled");
    app.file_dialog_name = format!("{stem}.{}", fmt.extension());
}

fn file_passes_filter(app: &FireCraft, entry: &std::path::Path, ext: &str) -> bool {
    let name = entry
        .file_name()
        .map(|n| n.to_string_lossy().to_lowercase())
        .unwrap_or_default();
    match app.file_dialog_type_filter {
        0 => matches!(
            ext,
            "png"
                | "jpg"
                | "jpeg"
                | "webp"
                | "gif"
                | "bmp"
                | "tiff"
                | "tif"
                | "ico"
                | "svg"
                | "psd"
                | "psb"
                | "ai"
        ),
        1 => name.ends_with(".fw.png"),
        2 => matches!(
            ext,
            "png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp" | "tiff" | "tif" | "ico"
        ),
        3 => ext == "svg",
        4 => matches!(ext, "psd" | "psb" | "ai"),
        _ => true,
    }
}

fn human_file_size(bytes: u64) -> String {
    let kb = bytes / 1024;
    if kb >= 1024 * 1024 {
        format!("{:.1} GB", kb as f32 / (1024.0 * 1024.0))
    } else if kb >= 1024 {
        format!("{:.1} MB", kb as f32 / 1024.0)
    } else {
        format!("{} KB", kb.max(1))
    }
}

fn relative_modified(time: std::time::SystemTime) -> String {
    let Ok(age) = std::time::SystemTime::now().duration_since(time) else {
        return String::new();
    };
    let hours = age.as_secs() / 3600;
    if hours < 1 {
        "Just now".to_string()
    } else if hours < 24 {
        format!("{hours}h ago")
    } else {
        format!("{}d ago", hours / 24)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formats::Format;
    use std::path::Path;

    fn app() -> FireCraft {
        FireCraft::new(None)
    }

    fn temporary_browser_folder() -> std::path::PathBuf {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = std::env::temp_dir().join(format!(
            "firecraft-file-browser-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(dir.join("folder")).unwrap();
        for name in [
            "art.fw.png",
            "photo.jpg",
            "vector.svg",
            "bridge.psd",
            "notes.txt",
        ] {
            std::fs::write(dir.join(name), b"file browser fixture").unwrap();
        }
        dir
    }

    fn render_body(app: &mut FireCraft) -> usize {
        let ctx = app.ctx.clone();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::Vec2::new(1100.0, 760.0),
            )),
            ..Default::default()
        };
        let mut close_now = false;
        let mut output = ctx.run_ui(input, |ui| file_dialog_body(ui, app, &mut close_now));
        output.textures_delta.clear();
        output.shapes.len()
    }

    #[test]
    fn file_dialog_actions_stay_visible_at_desktop_viewport_sizes() {
        for (width, height) in [(1360.0, 860.0), (1024.0, 600.0), (800.0, 500.0)] {
            for (mode, action) in [
                (FileMode::Open, "Open"),
                (FileMode::Import, "Import"),
                (FileMode::Save, "Save"),
                (FileMode::Export, "Export"),
            ] {
                let dir = temporary_browser_folder();
                let mut app = app();
                app.set_file_dialog_dir(dir.clone());
                app.file_mode = mode;
                app.file_dialog_name = if matches!(mode, FileMode::Save | FileMode::Export) {
                    "image.png".into()
                } else {
                    String::new()
                };
                app.show_file_dialog = true;

                let screen =
                    egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::new(width, height));
                let input = egui::RawInput {
                    screen_rect: Some(screen),
                    ..Default::default()
                };
                let ctx = app.ctx.clone();
                let mut output = None;
                for _ in 0..3 {
                    let mut frame = ctx.run_pass(input.clone(), |_| file_dialog(&mut app));
                    frame.textures_delta.clear();
                    output = Some(frame);
                }
                let output = output.expect("dialog should render a frame");
                let text_bounds = |text: &str| {
                    output
                        .shapes
                        .iter()
                        .find_map(|clipped| match &clipped.shape {
                            egui::Shape::Text(shape) if shape.galley.text() == text => {
                                Some((shape.visual_bounding_rect(), clipped.clip_rect))
                            }
                            _ => None,
                        })
                };

                for label in ["Go", action, "Cancel"] {
                    let (bounds, clip) = text_bounds(label).unwrap_or_else(|| {
                        panic!("{label} action missing for {mode:?} at {width}×{height}")
                    });
                    assert!(
                        screen.contains_rect(bounds) && clip.contains_rect(bounds),
                        "{label} action is clipped for {mode:?} at {width}×{height}: \
                         bounds={bounds:?}, clip={clip:?}, screen={screen:?}"
                    );
                }

                std::fs::remove_dir_all(dir).unwrap();
            }
        }
    }

    #[test]
    fn dialog_body_renders_real_entries_for_read_and_save_modes() {
        let dir = temporary_browser_folder();
        let mut app = app();
        app.set_file_dialog_dir(dir.clone());

        for mode in [
            FileMode::Open,
            FileMode::Import,
            FileMode::Save,
            FileMode::Export,
        ] {
            app.file_mode = mode;
            app.file_dialog_name = if matches!(mode, FileMode::Save | FileMode::Export) {
                "image.jpg".into()
            } else {
                String::new()
            };
            app.file_dialog_type_filter = 0;
            app.file_dialog_filter.clear();
            assert!(
                render_body(&mut app) > 20,
                "{mode:?} dialog should paint its controls"
            );
        }

        // Search and type filtering are applied to the listing while preserving directory rows.
        app.file_mode = FileMode::Open;
        app.file_dialog_filter = "photo".into();
        app.file_dialog_type_filter = 3;
        assert!(render_body(&mut app) > 20);
        app.file_dialog_filter.clear();
        for filter in 0..=5 {
            app.file_dialog_type_filter = filter;
            assert!(render_body(&mut app) > 20);
        }

        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn long_directory_listing_only_paints_visible_rows() {
        let mut app = app();
        let dir = temporary_browser_folder();
        app.set_file_dialog_dir(dir.clone());
        app.file_mode = FileMode::Open;
        app.file_dialog_type_filter = 0;
        app.file_dialog_entries = vec![dir.join("photo.jpg"); 1_000];

        let painted_shapes = render_body(&mut app);

        assert!(
            painted_shapes < 500,
            "painted {painted_shapes} shapes for 1,000 rows"
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn escape_closes_file_dialog_and_clears_pending_close_state() {
        let mut app = app();
        app.show_file_dialog = true;
        app.pending_close_after_save = Some(0);
        app.pending_close_tabs.push(1);
        app.pending_quit_after_save = true;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::Vec2::new(1000.0, 700.0),
            )),
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let ctx = app.ctx.clone();
        let mut output = ctx.run_ui(input, |_| file_dialog(&mut app));
        output.textures_delta.clear();
        assert!(!app.show_file_dialog);
        assert_eq!(app.pending_close_after_save, None);
        assert!(app.pending_close_tabs.is_empty());
        assert!(!app.pending_quit_after_save);
    }

    #[test]
    fn save_type_selection_replaces_the_extension_and_handles_empty_names() {
        let mut app = app();
        app.file_dialog_name = "poster.jpeg".into();
        set_file_extension(&mut app, Format::Png);
        assert_eq!(app.file_dialog_name, "poster.png");

        app.file_dialog_name.clear();
        set_file_extension(&mut app, Format::Svg);
        assert_eq!(app.file_dialog_name, "untitled.svg");

        assert_eq!(save_formats().len(), 9);
        assert!(save_formats().contains(&Format::WebP));
        assert!(save_formats().contains(&Format::Ico));
    }

    #[test]
    fn open_type_filters_cover_layered_raster_vector_bridge_and_all_files() {
        let mut app = app();
        let cases = [
            (0, "art.fw.png", "png", true),
            (0, "photo.JPEG", "jpeg", true),
            (0, "design.ai", "ai", true),
            (0, "notes.txt", "txt", false),
            (1, "art.fw.png", "png", true),
            (1, "art.png", "png", false),
            (2, "photo.tiff", "tiff", true),
            (2, "design.svg", "svg", false),
            (3, "design.svg", "svg", true),
            (3, "design.SVG", "svg", true),
            (4, "design.psb", "psb", true),
            (4, "design.psd", "psd", true),
            (4, "design.ai", "ai", true),
            (4, "design.png", "png", false),
            (5, "notes.txt", "txt", true),
            (99, "notes.txt", "txt", true),
        ];

        for (filter, filename, ext, expected) in cases {
            app.file_dialog_type_filter = filter;
            assert_eq!(
                file_passes_filter(&app, Path::new(filename), ext),
                expected,
                "filter {filter} for {filename}"
            );
        }
    }

    #[test]
    fn file_size_labels_use_kilobyte_megabyte_and_gigabyte_units() {
        assert_eq!(human_file_size(0), "1 KB");
        assert_eq!(human_file_size(1024), "1 KB");
        assert_eq!(human_file_size(1024 * 1024), "1.0 MB");
        assert_eq!(human_file_size(1024_u64.pow(3)), "1.0 GB");
        assert_eq!(human_file_size(1536), "1 KB");
    }

    #[test]
    fn modified_time_labels_cover_recent_old_and_future_files() {
        use std::time::{Duration, SystemTime};
        let now = SystemTime::now();
        assert_eq!(relative_modified(now), "Just now");
        assert_eq!(
            relative_modified(now - Duration::from_secs(2 * 3600)),
            "2h ago"
        );
        assert_eq!(
            relative_modified(now - Duration::from_secs(3 * 24 * 3600)),
            "3d ago"
        );
        assert_eq!(relative_modified(now + Duration::from_secs(60)), "");
    }
}
