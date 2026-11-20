//! The window layout: menu bar, docks, canvas, status bar and dialogs.
//!
//! This module owns the whole frame. `FireCraft::ui` delegates to it.

use crate::app::{FileMode, FireCraft};
use crate::canvas;
use crate::dialogs;
use crate::menus;
use crate::panels;
use crate::theme;

use eframe::egui::{self, Key, Ui};

/// Draw the whole editor for one frame.
pub fn show(ui: &mut Ui, app: &mut FireCraft) {
    let ctx = ui.ctx().clone();
    app.ctx = ctx.clone();
    if app.logo_tex.is_none() {
        app.logo_tex = FireCraft::load_logo(&ctx);
    }
    ctx.set_visuals(theme::visuals(&app.palette, app.palette.is_dark()));

    shortcuts(ui, app);

    egui::Panel::top("menu")
        .frame(
            egui::Frame::new()
                .fill(app.palette.panel)
                .inner_margin(egui::Margin::symmetric(4, 2)),
        )
        .show(ui, |ui| menus::menu_bar(ui, app));

    egui::Panel::left("tools")
        .exact_size(72.0)
        .resizable(false)
        .frame(egui::Frame::new().fill(app.palette.panel))
        .show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| panels::tools_panel(ui, app));
        });

    egui::Panel::right("dock")
        .default_size(300.0)
        .resizable(true)
        .frame(egui::Frame::new().fill(app.palette.panel))
        .show(ui, |ui| right_dock(ui, app));

    egui::CentralPanel::default()
        .frame(egui::Frame::new().fill(app.palette.bg))
        .show(ui, |ui| canvas::show(ui, app));

    egui::Panel::bottom("status")
        .frame(egui::Frame::new().fill(app.palette.panel))
        .show(ui, |ui| status_bar(ui, app));

    dialogs::new_document_dialog(app);
    dialogs::file_dialog(app);
    dialogs::welcome_overlay(app);

    screenshot(ui, app);
}

/// The tabbed dock on the right: Properties, Color, Paths, Pages, Styles,
/// Library and Swatches.
fn right_dock(ui: &mut Ui, app: &mut FireCraft) {
    const TABS: &[&str] = &[
        "Properties",
        "Color",
        "Paths",
        "Pages",
        "Styles",
        "Library",
        "Swatches",
    ];
    let current = app.right_tab.min(TABS.len() - 1);
    let mut tab = current;
    let palette = app.palette;
    panels::panel(ui, TABS[current], TABS, &mut tab, palette, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| match current {
            1 => panels::color_panel(ui, app),
            2 => panels::paths_panel(ui, app),
            3 => panels::pages_panel(ui, app),
            4 => panels::styles_panel(ui, app),
            5 => panels::library_panel(ui, app),
            6 => panels::swatches_panel(ui, app),
            _ => panels::properties_panel(ui, app),
        });
    });
    app.right_tab = tab;
}

/// The status bar along the bottom.
fn status_bar(ui: &mut Ui, app: &mut FireCraft) {
    ui.horizontal(|ui| {
        ui.label(app.status.as_str());
        ui.separator();
        let (w, h) = {
            let page = app.doc.page();
            (page.width as u32, page.height as u32)
        };
        ui.label(format!("{w} × {h} px"));
        ui.separator();
        ui.label(format!("Layer {}", app.selected_layer + 1));
        ui.separator();
        ui.label(app.tool.name());
        ui.separator();
        ui.label(format!("Zoom {:.0}%", app.doc.zoom * 100.0));
        if let Some(sel) = app.selected {
            let name = app
                .doc
                .page()
                .layers
                .get(app.selected_layer)
                .and_then(|l| l.objects.get(sel))
                .map(|o| o.name.clone())
                .unwrap_or_default();
            ui.separator();
            ui.label(name);
        }
    });
}

/// The keyboard shortcuts handled here rather than in the menus.
enum Shortcut {
    Undo,
    Redo,
    Save,
    New,
    Open,
    SelectAll,
    Copy,
    Cut,
    Paste,
    Duplicate,
    Delete,
}

/// Keyboard shortcuts. Skipped while a text field has focus so typing a
/// filename never triggers a command.
fn shortcuts(ui: &mut Ui, app: &mut FireCraft) {
    if ui.memory(|m| m.focused()).is_some() {
        return;
    }
    let ctrl = ui.input(|i| i.modifiers.ctrl);
    let mut action = None;
    ui.input(|i| {
        for event in &i.events {
            let egui::Event::Key { key, pressed: true, .. } = event else {
                continue;
            };
            action = match (key, ctrl) {
                (Key::Z, true) => Some(Shortcut::Undo),
                (Key::Y, true) => Some(Shortcut::Redo),
                (Key::S, true) => Some(Shortcut::Save),
                (Key::N, true) => Some(Shortcut::New),
                (Key::O, true) => Some(Shortcut::Open),
                (Key::A, true) => Some(Shortcut::SelectAll),
                (Key::C, true) => Some(Shortcut::Copy),
                (Key::X, true) => Some(Shortcut::Cut),
                (Key::V, true) => Some(Shortcut::Paste),
                (Key::D, true) => Some(Shortcut::Duplicate),
                (Key::Delete | Key::Backspace, _) => Some(Shortcut::Delete),
                _ => None,
            };
            if action.is_some() {
                break;
            }
        }
    });
    match action {
        Some(Shortcut::Undo) => app.undo_action(),
        Some(Shortcut::Redo) => app.redo_action(),
        Some(Shortcut::Save) => app.save_current(),
        Some(Shortcut::New) => app.show_new_dialog = true,
        Some(Shortcut::Open) => app.begin_file_dialog(FileMode::Open),
        Some(Shortcut::SelectAll) => app.select_all(),
        Some(Shortcut::Copy) => app.copy_selected(),
        Some(Shortcut::Cut) => app.cut_selected(),
        Some(Shortcut::Paste) => app.paste(),
        Some(Shortcut::Duplicate) => app.duplicate_selected(),
        Some(Shortcut::Delete) => app.delete_selected(),
        None => return,
    }
    ui.ctx().request_repaint();
}

/// Capture the rendered window to `app.screenshot_path` and exit.
fn screenshot(ui: &mut Ui, app: &mut FireCraft) {
    let Some(path) = app.screenshot_path.clone() else {
        return;
    };
    if app.screenshot_saved {
        return;
    }
    app.screenshot_saved = true;
    let done = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = done.clone();
    let waiter = done.clone();
    ui.ctx().request_screenshot(move |image: std::sync::Arc<egui::ColorImage>| {
        let w = image.width();
        let h = image.height();
        let mut rgba = Vec::with_capacity(w * h * 4);
        for c in &image.pixels {
            rgba.push(c.r());
            rgba.push(c.g());
            rgba.push(c.b());
            rgba.push(c.a());
        }
        if let Some(img) = image::RgbaImage::from_raw(w as u32, h as u32, rgba) {
            if img.save(&path).is_ok() {
                eprintln!("Screenshot saved to {path}");
            }
        }
        flag.store(true, std::sync::atomic::Ordering::SeqCst);
    });
    ui.ctx().request_repaint();
    std::thread::spawn(move || {
        for _ in 0..150 {
            if waiter.load(std::sync::atomic::Ordering::SeqCst) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        std::process::exit(0);
    });
}
