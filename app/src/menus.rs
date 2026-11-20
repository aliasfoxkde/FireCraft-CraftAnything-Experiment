//! The menu bar.
//!
//! Every item here is wired to a real action. Menus that were previously
//! placeholders that only wrote a line into the status bar are gone.

use crate::app::{FileMode, FireCraft};
use crate::constants;
use crate::theme::ThemeMode;

use eframe::egui::{self, RichText};

/// Draw the application menu bar.
pub fn menu_bar(ui: &mut egui::Ui, app: &mut FireCraft) {
    ui.horizontal(|ui| {
        ui.style_mut().visuals.widgets.inactive.fg_stroke =
            egui::Stroke::new(1.0, app.palette.text_dim);
        ui.menu_button("File", |ui| file_menu(ui, app));
        ui.menu_button("Edit", |ui| edit_menu(ui, app));
        ui.menu_button("View", |ui| view_menu(ui, app));
        ui.menu_button("Select", |ui| select_menu(ui, app));
        ui.menu_button("Modify", |ui| modify_menu(ui, app));
        ui.menu_button("Window", |ui| window_menu(ui, app));
        ui.separator();
        ui.label(RichText::new(format!("{:.0}%", app.doc.zoom * 100.0)).small());
    });
}

fn file_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    if ui.button("New…").clicked() {
        app.show_new_dialog = true;
        ui.close();
    }
    if ui.button("Open…").clicked() {
        app.begin_file_dialog(FileMode::Open);
        ui.close();
    }
    if ui.button("Save").clicked() {
        app.save_current();
        ui.close();
    }
    if ui.button("Save As…").clicked() {
        app.begin_file_dialog(FileMode::Save);
        ui.close();
    }
    if ui.button("Export…").clicked() {
        app.begin_file_dialog(FileMode::Export);
        ui.close();
    }
    ui.separator();
    if ui.button("Welcome Screen").clicked() {
        app.show_welcome = true;
        ui.close();
    }
    if ui.button("Close Document").clicked() {
        app.close_document();
        ui.close();
    }
}

fn edit_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    let can_undo = !app.undo.is_empty();
    let can_redo = !app.redo.is_empty();
    let has_selection = app.selected.is_some();

    if ui
        .add_enabled(can_undo, egui::Button::new("Undo"))
        .clicked()
    {
        app.undo_action();
        ui.close();
    }
    if ui
        .add_enabled(can_redo, egui::Button::new("Redo"))
        .clicked()
    {
        app.redo_action();
        ui.close();
    }
    ui.separator();
    if ui
        .add_enabled(has_selection, egui::Button::new("Cut"))
        .clicked()
    {
        app.cut_selected();
        ui.close();
    }
    if ui
        .add_enabled(has_selection, egui::Button::new("Copy"))
        .clicked()
    {
        app.copy_selected();
        ui.close();
    }
    if ui
        .add_enabled(app.clipboard.is_some(), egui::Button::new("Paste"))
        .clicked()
    {
        app.paste();
        ui.close();
    }
    if ui
        .add_enabled(has_selection, egui::Button::new("Duplicate"))
        .clicked()
    {
        app.duplicate_selected();
        ui.close();
    }
    if ui
        .add_enabled(has_selection, egui::Button::new("Delete"))
        .clicked()
    {
        app.delete_selected();
        ui.close();
    }
}

fn view_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    if ui.button("Zoom In").clicked() {
        app.zoom_by(constants::ZOOM_STEP);
        ui.close();
    }
    if ui.button("Zoom Out").clicked() {
        app.zoom_by(1.0 / constants::ZOOM_STEP);
        ui.close();
    }
    if ui.button("Fit in Window").clicked() {
        app.fit_in_window();
        ui.close();
    }
    ui.separator();
    for preset in constants::ZOOM_PRESETS {
        if ui
            .button(format!("{:.0}%", preset * 100.0))
            .clicked()
        {
            app.set_zoom(*preset);
            ui.close();
        }
    }
    ui.separator();
    ui.checkbox(&mut app.show_rulers, "Rulers");
    ui.checkbox(&mut app.show_grid, "Grid");
}

fn select_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    if ui.button("Select All").clicked() {
        app.select_all();
        ui.close();
    }
    if ui.button("Deselect").clicked() {
        app.selected = None;
        app.status = "Deselected".to_string();
        ui.close();
    }
}

fn modify_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    let has_selection = app.selected.is_some();
    if ui
        .add_enabled(has_selection, egui::Button::new("Flip Horizontal"))
        .clicked()
    {
        app.flip_horizontal();
        ui.close();
    }
    if ui
        .add_enabled(has_selection, egui::Button::new("Flip Vertical"))
        .clicked()
    {
        app.flip_vertical();
        ui.close();
    }
    if ui
        .add_enabled(has_selection, egui::Button::new("Rotate 90°"))
        .clicked()
    {
        app.rotate_90();
        ui.close();
    }
    ui.separator();
    if ui
        .add_enabled(has_selection, egui::Button::new("Bring to Front"))
        .clicked()
    {
        app.reorder_selected(true);
        ui.close();
    }
    if ui
        .add_enabled(has_selection, egui::Button::new("Send to Back"))
        .clicked()
    {
        app.reorder_selected(false);
        ui.close();
    }
}

fn window_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    ui.label("Theme");
    for (mode, label) in [
        (ThemeMode::Dark, "Dark"),
        (ThemeMode::Light, "Light"),
        (ThemeMode::System, "Follow system"),
    ] {
        if ui
            .selectable_label(app.theme_mode == mode, label)
            .clicked()
        {
            app.set_theme(mode);
        }
    }
    ui.separator();
    if ui.button("Reset Layout").clicked() {
        app.right_tab = 0;
        app.status = "Layout reset".to_string();
        ui.close();
    }
}
