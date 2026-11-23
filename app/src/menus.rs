//! The 10-menu Adobe Fireworks CS6 menu bar:
//! File, Edit, View, Select, Modify, Text, Commands, Filters, Window, Help.
//!
//! Features full Windows/CS6 keyboard navigation:
//! - `Alt+F` (File), `Alt+E` (Edit), `Alt+V` (View), `Alt+S` (Select), `Alt+M` (Modify),
//!   `Alt+T` (Text), `Alt+C` (Commands), `Alt+L`/`Alt+I` (Filters), `Alt+W` (Window), `Alt+H` (Help)
//! - `Left` / `Right` arrow keys cycle across the 10 top-level menus while open
//! - `Up` / `Down` arrow keys navigate through the active menu's items
//! - `Enter` / `Space` executes the highlighted menu item; `Escape` closes the menu.

use crate::app::{CanvasViewMode, FileMode, FireCraft, Tool};
use crate::constants;
use crate::doc::LiveEffect;
use crate::theme::ThemeMode;

use eframe::egui::{
    self, Align2, Color32, FontId, Key, Order, Pos2, Rect, Sense, Stroke, Ui, Vec2, pos2,
};

pub const MENU_TITLES: [(&str, usize); 10] = [
    ("File", 0),     // Alt+F (underline 'F')
    ("Edit", 0),     // Alt+E (underline 'E')
    ("View", 0),     // Alt+V (underline 'V')
    ("Select", 0),   // Alt+S (underline 'S')
    ("Modify", 0),   // Alt+M (underline 'M')
    ("Text", 0),     // Alt+T (underline 'T')
    ("Commands", 0), // Alt+C (underline 'C')
    ("Filters", 2),  // Alt+L / Alt+I (underline 'l')
    ("Window", 0),   // Alt+W (underline 'W')
    ("Help", 0),     // Alt+H (underline 'H')
];

struct MenuNav {
    cursor: usize,
    active_idx: usize,
    activate_key: bool,
    closed: bool,
}

impl MenuNav {
    fn row(
        &mut self,
        ui: &mut Ui,
        palette: crate::theme::Palette,
        enabled: bool,
        label: &str,
        shortcut: &str,
    ) -> bool {
        let idx = if enabled {
            let i = self.cursor;
            self.cursor += 1;
            Some(i)
        } else {
            None
        };
        let is_selected = idx.is_some_and(|i| i == self.active_idx);
        let width = ui.available_width().max(248.0);
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 20.0), Sense::click());
        if let Some(i) = idx
            && resp.hovered()
            && ui.input(|inp| inp.pointer.delta().length_sq() > 0.01)
        {
            self.active_idx = i;
        }
        let highlighted = idx.is_some_and(|i| i == self.active_idx);
        let painter = ui.painter();
        if highlighted && enabled {
            painter.rect_filled(rect, 0.0, palette.selection.gamma_multiply(0.38));
            painter.rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, palette.selection),
                egui::StrokeKind::Middle,
            );
        } else if highlighted {
            painter.rect_filled(rect, 0.0, palette.well);
        }
        let fg = if !enabled {
            palette.text_dim.gamma_multiply(0.55)
        } else {
            palette.text
        };
        painter.text(
            pos2(rect.left() + 20.0, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            FontId::proportional(11.0),
            fg,
        );
        if !shortcut.is_empty() {
            painter.text(
                pos2(rect.right() - 8.0, rect.center().y),
                Align2::RIGHT_CENTER,
                shortcut,
                FontId::proportional(10.5),
                if enabled { palette.text_dim } else { fg },
            );
        }
        if is_selected && self.activate_key {
            ui.scroll_to_rect(rect, None);
        }
        let triggered = enabled && (resp.clicked() || (is_selected && self.activate_key));
        if triggered {
            self.closed = true;
        }
        triggered
    }

    fn radio_row(
        &mut self,
        ui: &mut Ui,
        palette: crate::theme::Palette,
        selected: bool,
        label: &str,
        shortcut: &str,
    ) -> bool {
        let mut val = selected;
        self.check_row(ui, palette, &mut val, label, shortcut)
    }

    fn check_row(
        &mut self,
        ui: &mut Ui,
        palette: crate::theme::Palette,
        checked: &mut bool,
        label: &str,
        shortcut: &str,
    ) -> bool {
        let cur = *checked;
        let idx = self.cursor;
        self.cursor += 1;
        let is_selected = idx == self.active_idx;
        let width = ui.available_width().max(248.0);
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, 20.0), Sense::click());
        if resp.hovered() && ui.input(|i| i.pointer.delta().length_sq() > 0.01) {
            self.active_idx = idx;
        }
        let highlighted = idx == self.active_idx;
        let painter = ui.painter();
        if highlighted {
            painter.rect_filled(rect, 0.0, palette.selection.gamma_multiply(0.38));
            painter.rect_stroke(
                rect,
                0.0,
                Stroke::new(1.0, palette.selection),
                egui::StrokeKind::Middle,
            );
        }
        if cur {
            painter.text(
                pos2(rect.left() + 7.0, rect.center().y),
                Align2::LEFT_CENTER,
                "✓",
                FontId::proportional(11.0),
                palette.accent,
            );
        }
        painter.text(
            pos2(rect.left() + 20.0, rect.center().y),
            Align2::LEFT_CENTER,
            label,
            FontId::proportional(11.0),
            palette.text,
        );
        if !shortcut.is_empty() {
            painter.text(
                pos2(rect.right() - 8.0, rect.center().y),
                Align2::RIGHT_CENTER,
                shortcut,
                FontId::proportional(10.5),
                palette.text_dim,
            );
        }
        let triggered = resp.clicked() || (is_selected && self.activate_key);
        if triggered {
            *checked = !*checked;
            self.closed = true;
        }
        triggered
    }
}

/// Draw the full 10-menu Fireworks CS6 application menu bar plus header controls.
pub fn menu_bar(ui: &mut egui::Ui, app: &mut FireCraft) {
    let p = app.palette;
    let alt_held = ui.input(|i| i.modifiers.alt);
    let mut menu_anchors: [Pos2; 10] = [Pos2::ZERO; 10];
    let mut menu_rects: [Rect; 10] = [Rect::NOTHING; 10];

    // Keyboard navigation when a top-level menu is open
    let mut activate_key = false;
    let mut nav_delta: isize = 0;
    if let Some(open_idx) = app.active_menu {
        let mut next_menu = Some(open_idx);
        ui.input(|i| {
            for ev in &i.events {
                if let egui::Event::Key {
                    key, pressed: true, ..
                } = ev
                {
                    match key {
                        Key::Escape => {
                            next_menu = None;
                        }
                        Key::ArrowRight => {
                            next_menu = Some((open_idx + 1) % MENU_TITLES.len());
                            app.active_menu_item = 0;
                        }
                        Key::ArrowLeft => {
                            next_menu =
                                Some((open_idx + MENU_TITLES.len() - 1) % MENU_TITLES.len());
                            app.active_menu_item = 0;
                        }
                        Key::ArrowDown => {
                            nav_delta = 1;
                        }
                        Key::ArrowUp => {
                            nav_delta = -1;
                        }
                        Key::Enter | Key::Space => {
                            activate_key = true;
                        }
                        _ => {}
                    }
                }
            }
        });
        app.active_menu = next_menu;
    }

    ui.horizontal(|ui| {
        // 10 Top-Level CS6 Menus with mnemonic underlines and keyboard/mouse focus
        ui.spacing_mut().item_spacing = Vec2::new(2.0, 0.0);
        for (i, (title, mnemonic_pos)) in MENU_TITLES.iter().enumerate() {
            let is_open = app.active_menu == Some(i);
            let w = 14.0 + title.len() as f32 * 6.4;
            let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 20.0), Sense::click());
            menu_anchors[i] = rect.left_bottom();
            menu_rects[i] = rect;
            let painter = ui.painter();
            if is_open {
                painter.rect_filled(rect, 0.0, p.selection.gamma_multiply(0.38));
                painter.rect_stroke(
                    rect,
                    0.0,
                    Stroke::new(1.0, p.selection),
                    egui::StrokeKind::Middle,
                );
            } else if resp.hovered() {
                painter.rect_filled(rect, 0.0, p.well);
            }
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                *title,
                FontId::proportional(11.5),
                p.text,
            );
            // Underline mnemonic letter when Alt is held or a menu is open
            if alt_held || app.active_menu.is_some() {
                let text_w = title.len() as f32 * 6.0;
                let start_x = rect.center().x - text_w * 0.5 + (*mnemonic_pos as f32) * 6.0;
                let y = rect.bottom() - 3.0;
                painter.line_segment(
                    [pos2(start_x, y), pos2(start_x + 6.0, y)],
                    Stroke::new(1.0, p.accent),
                );
            }
            if resp.clicked() {
                if is_open {
                    app.active_menu = None;
                } else {
                    app.active_menu = Some(i);
                    app.active_menu_item = 0;
                }
            } else if resp.hovered() && app.active_menu.is_some() && !is_open {
                app.active_menu = Some(i);
                app.active_menu_item = 0;
            }
        }

        ui.separator();
        if ui
            .small_button("Br")
            .on_hover_text("Browse in Bridge / Open File Browser (Ctrl+Alt+O)")
            .clicked()
        {
            app.begin_file_dialog(FileMode::Open);
        }
        if crate::canvas::tool_button(
            ui,
            crate::icons::Icon::Hand,
            app.tool == Tool::Hand,
            "Hand Tool (H)",
        )
        .clicked()
        {
            app.tool = Tool::Hand;
        }
        if crate::canvas::tool_button(
            ui,
            crate::icons::Icon::Zoom,
            app.tool == Tool::Zoom,
            "Zoom Tool (Z)",
        )
        .clicked()
        {
            app.tool = Tool::Zoom;
        }

        egui::ComboBox::from_id_salt("header_zoom_combo")
            .width(68.0)
            .selected_text(format!("{:.0}%", app.doc.zoom * 100.0))
            .show_ui(ui, |ui| {
                ui.label(
                    egui::RichText::new("Zoom Level")
                        .small()
                        .color(app.palette.text_dim),
                );
                ui.separator();
                for &preset in constants::ZOOM_PRESETS {
                    let lbl = format!("{:.0}%", preset * 100.0);
                    if ui
                        .selectable_label((app.doc.zoom - preset).abs() < 0.01, lbl)
                        .clicked()
                    {
                        app.set_zoom(preset);
                    }
                }
            });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::ComboBox::from_id_salt("workspace_mode_combo")
                .width(122.0)
                .selected_text(if app.full_screen_canvas {
                    "Full Screen (Tab)"
                } else if app.show_right_dock && app.show_bottom_inspector {
                    "Expanded Mode"
                } else if !app.show_right_dock && app.show_bottom_inspector {
                    "Iconic Mode"
                } else {
                    "Netbook Mode"
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(
                            !app.full_screen_canvas
                                && app.show_right_dock
                                && app.show_bottom_inspector,
                            "Expanded Mode",
                        )
                        .clicked()
                    {
                        app.full_screen_canvas = false;
                        app.show_main_toolbar = true;
                        app.show_tools_dock = true;
                        app.show_right_dock = true;
                        app.show_bottom_inspector = true;
                        app.apply_workspace_preset("Expanded Mode");
                    }
                    if ui
                        .selectable_label(
                            app.workspace_preset == "CS6 Dual-Column",
                            "CS6 Dual-Column",
                        )
                        .clicked()
                    {
                        app.apply_workspace_preset("CS6 Dual-Column");
                    }
                    if ui
                        .selectable_label(
                            app.workspace_preset == "All Panels Floating",
                            "All Panels Floating",
                        )
                        .clicked()
                    {
                        app.apply_workspace_preset("All Panels Floating");
                    }
                    if ui
                        .selectable_label(
                            !app.full_screen_canvas
                                && !app.show_right_dock
                                && app.show_bottom_inspector,
                            "Iconic Mode",
                        )
                        .clicked()
                    {
                        app.full_screen_canvas = false;
                        app.show_main_toolbar = true;
                        app.show_tools_dock = true;
                        app.show_right_dock = false;
                        app.show_bottom_inspector = true;
                        app.status = "Workspace set to Iconic Mode".to_string();
                    }
                    if ui
                        .selectable_label(false, "Iconic Mode With Panel Names")
                        .clicked()
                    {
                        app.full_screen_canvas = false;
                        app.show_main_toolbar = true;
                        app.show_tools_dock = true;
                        app.show_right_dock = true;
                        app.show_bottom_inspector = true;
                        app.status = "Workspace set to Iconic Mode With Panel Names".to_string();
                    }
                    if ui
                        .selectable_label(
                            !app.full_screen_canvas
                                && !app.show_right_dock
                                && !app.show_bottom_inspector,
                            "Netbook Mode",
                        )
                        .clicked()
                    {
                        app.full_screen_canvas = false;
                        app.show_main_toolbar = true;
                        app.show_tools_dock = true;
                        app.show_right_dock = false;
                        app.show_bottom_inspector = false;
                        app.status = "Workspace set to Netbook Mode".to_string();
                    }
                    if ui
                        .selectable_label(app.full_screen_canvas, "Full Screen Canvas (Tab / F4)")
                        .clicked()
                    {
                        app.toggle_full_screen_canvas();
                    }
                    ui.separator();
                    if ui.button("Save Current").clicked() {
                        app.status = "Saved current workspace layout".to_string();
                        ui.close();
                    }
                });

            egui::ComboBox::from_id_salt("header_theme_combo")
                .width(128.0)
                .selected_text(app.theme_mode.label())
                .show_ui(ui, |ui| {
                    for mode in [ThemeMode::Dark, ThemeMode::Light, ThemeMode::System] {
                        if ui
                            .selectable_label(app.theme_mode == mode, mode.label())
                            .clicked()
                        {
                            app.set_theme(mode);
                        }
                    }
                });

            if ui
                .small_button(if app.window_fullscreen { "🗗" } else { "⛶" })
                .on_hover_text("Toggle App Fullscreen (F11)")
                .clicked()
            {
                app.toggle_window_fullscreen();
            }

            if ui
                .small_button("MCP / CLI")
                .on_hover_text("Open Model Context Protocol (MCP) JSON-RPC 2.0 & CLI Console")
                .clicked()
            {
                app.show_mcp_dialog = true;
            }
            if ui
                .small_button("Cmd (Ctrl+K)")
                .on_hover_text("Open Searchable Command Palette (Ctrl+K / Ctrl+P)")
                .clicked()
            {
                app.show_command_palette = !app.show_command_palette;
            }
        });
    });

    // Render the active dropdown menu popup with full keyboard + mouse support
    if let Some(menu_idx) = app.active_menu {
        let anchor = menu_anchors[menu_idx];
        let mut nav = MenuNav {
            cursor: 0,
            active_idx: app.active_menu_item,
            activate_key,
            closed: false,
        };
        let area_resp = egui::Area::new(ui.make_persistent_id(("cs6_top_menu_popup", menu_idx)))
            .order(Order::Foreground)
            .fixed_pos(pos2(anchor.x, anchor.y + 2.0))
            .show(&app.ctx.clone(), |ui| {
                egui::Frame::new()
                    .fill(p.panel)
                    .stroke(Stroke::new(1.0, p.stroke))
                    .inner_margin(egui::Margin::symmetric(3, 4))
                    .show(ui, |ui| {
                        ui.set_min_width(252.0);
                        ui.set_max_width(268.0);
                        ui.spacing_mut().item_spacing = Vec2::new(0.0, 1.0);
                        match menu_idx {
                            0 => file_menu(ui, app, &mut nav),
                            1 => edit_menu(ui, app, &mut nav),
                            2 => view_menu(ui, app, &mut nav),
                            3 => select_menu(ui, app, &mut nav),
                            4 => modify_menu(ui, app, &mut nav),
                            5 => text_menu(ui, app, &mut nav),
                            6 => commands_menu(ui, app, &mut nav),
                            7 => filters_menu(ui, app, &mut nav),
                            8 => window_menu(ui, app, &mut nav),
                            _ => help_menu(ui, app, &mut nav),
                        }
                    });
            });

        let total = nav.cursor.max(1);
        if nav_delta != 0 {
            app.active_menu_item =
                (nav.active_idx as isize + nav_delta).rem_euclid(total as isize) as usize;
        } else {
            app.active_menu_item = nav.active_idx.min(total - 1);
        }

        if nav.closed {
            app.active_menu = None;
        } else if ui.input(|i| i.pointer.primary_pressed())
            && let Some(pos) = ui.input(|i| i.pointer.interact_pos())
        {
            let on_bar = menu_rects.iter().any(|r| r.contains(pos));
            let on_popup = area_resp.response.rect.contains(pos);
            if !on_bar && !on_popup {
                app.active_menu = None;
            }
        }
    }
}

fn file_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    if nav.row(ui, p, true, "New…", "Ctrl+N") {
        app.show_new_dialog = true;
    }
    if nav.row(ui, p, true, "New from Template (1080p Showcase)…", "") {
        app.create_template_document("Full_HD_1080p_Showcase", 1920.0, 1080.0);
    }
    if nav.row(ui, p, true, "Open…", "Ctrl+O") {
        app.begin_file_dialog(FileMode::Open);
    }
    if nav.row(ui, p, true, "Browse in Bridge…", "Ctrl+Alt+O") {
        app.begin_file_dialog(FileMode::Open);
    }
    ui.separator();
    if nav.row(ui, p, true, "Close", "Ctrl+W") {
        app.close_active_tab();
    }
    if nav.row(ui, p, app.open_docs.len() > 1, "Close All", "") {
        app.close_document();
    }
    ui.separator();
    if nav.row(ui, p, true, "Save", "Ctrl+S") {
        app.save_current();
    }
    if nav.row(ui, p, true, "Save As…", "Ctrl+Shift+S") {
        app.begin_file_dialog(FileMode::Save);
    }
    if nav.row(ui, p, true, "Save All", "") {
        app.save_current();
    }
    if nav.row(ui, p, app.current_path.is_some(), "Revert", "")
        && let Some(path) = app.current_path.clone()
    {
        let _ = app.open_path(&path);
    }
    ui.separator();
    if nav.row(ui, p, true, "Import…", "Ctrl+R") {
        app.begin_file_dialog(FileMode::Import);
    }
    if nav.row(ui, p, true, "Export…", "Ctrl+Shift+R") {
        app.begin_file_dialog(FileMode::Export);
    }
    if nav.row(ui, p, true, "Image Preview (2-Up)…", "Ctrl+Shift+X") {
        app.view_mode = CanvasViewMode::TwoUp;
        app.status = "2-Up Image Preview enabled".to_string();
    }
    if nav.row(ui, p, true, "Export HTML5 + CSS…", "") {
        app.file_dialog_name = "index.html".to_string();
        app.begin_file_dialog(FileMode::Export);
        app.file_dialog_name = "index.html".to_string();
    }
    ui.separator();
    if nav.row(ui, p, true, "Batch Process / MCP Console…", "") {
        app.show_mcp_dialog = true;
    }
    if nav.row(ui, p, true, "Canvas Size / Page Setup…", "") {
        app.open_canvas_size_dialog();
    }
    if nav.row(ui, p, true, "Welcome Screen…", "") {
        app.show_welcome = true;
    }
    ui.separator();
    if nav.row(ui, p, true, "Exit", "Ctrl+Q") {
        std::process::exit(0);
    }
}

fn edit_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    let can_undo = !app.undo.is_empty();
    let can_redo = !app.redo.is_empty();
    let has_sel = app.selected.is_some() || !app.selected_set.is_empty();

    if nav.row(ui, p, can_undo, "Undo", "Ctrl+Z") {
        app.undo_action();
    }
    if nav.row(ui, p, can_redo, "Redo", "Ctrl+Y") {
        app.redo_action();
    }
    ui.separator();
    if nav.row(ui, p, true, "Insert New Layer", "") {
        app.add_layer();
    }
    if nav.row(ui, p, true, "Insert New Page", "") {
        app.add_page();
    }
    if nav.row(ui, p, true, "Insert New Animation State", "") {
        app.add_state();
    }
    if nav.row(ui, p, has_sel, "Insert New Symbol…", "F8") {
        app.convert_selection_to_symbol();
    }
    if nav.row(ui, p, true, "Insert Rectangular Slice", "") {
        app.insert_slice(false);
    }
    if nav.row(ui, p, true, "Insert Hotspot", "") {
        app.insert_slice(true);
    }
    if nav.row(ui, p, true, "Find and Replace…", "Ctrl+F") {
        app.show_find_replace = true;
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Cut", "Ctrl+X") {
        app.cut_selected();
    }
    if nav.row(ui, p, has_sel, "Copy", "Ctrl+C") {
        app.copy_selected();
    }
    if nav.row(ui, p, has_sel, "Copy HTML / CSS Code…", "Ctrl+Alt+C") {
        app.copy_selected_css();
    }
    if nav.row(ui, p, true, "Paste", "Ctrl+V") {
        app.paste();
    }
    if nav.row(ui, p, has_sel, "Clear", "Backspace") {
        app.delete_selected();
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Duplicate", "Ctrl+Alt+D") {
        app.duplicate_selected();
    }
    if nav.row(ui, p, has_sel, "Clone", "Ctrl+Shift+D") {
        app.clone_selected();
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Crop Document to Selection", "")
        && let Some((lo, hi)) = app.selected_object().and_then(|o| o.bounds())
    {
        app.crop_canvas_to_rect(lo, hi);
    }
    if nav.row(ui, p, true, "Keyboard Shortcuts…", "F1") {
        app.show_about_dialog = true;
    }
}

fn view_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    if nav.row(ui, p, true, "Zoom In", "Ctrl++") {
        app.zoom_by(constants::ZOOM_STEP);
    }
    if nav.row(ui, p, true, "Zoom Out", "Ctrl+-") {
        app.zoom_by(1.0 / constants::ZOOM_STEP);
    }
    if nav.row(ui, p, true, "Magnification 100%", "Ctrl+1") {
        app.set_zoom(1.0);
    }
    if nav.row(ui, p, true, "Magnification 50%", "Ctrl+5") {
        app.set_zoom(0.5);
    }
    if nav.row(ui, p, true, "Magnification 200%", "Ctrl+2") {
        app.set_zoom(2.0);
    }
    if nav.row(ui, p, true, "Fit All in Window", "Ctrl+0") {
        app.fit_in_window();
    }
    if nav.row(ui, p, true, "Full Screen / Hide Panels", "Tab / F4") {
        app.toggle_full_screen_canvas();
    }
    ui.separator();
    for (mode, label) in [
        (CanvasViewMode::Original, "Original View"),
        (CanvasViewMode::Preview, "Preview View"),
        (CanvasViewMode::TwoUp, "2-Up Split View"),
        (CanvasViewMode::FourUp, "4-Up Compare View"),
    ] {
        let mut active = app.view_mode == mode;
        if nav.check_row(ui, p, &mut active, label, "") {
            app.view_mode = mode;
        }
    }
    ui.separator();
    let mut rulers = app.show_rulers;
    if nav.check_row(ui, p, &mut rulers, "Rulers", "Ctrl+Alt+R") {
        app.show_rulers = rulers;
    }
    for ru in crate::app::RulerUnit::ALL {
        if nav.radio_row(
            ui,
            p,
            app.ruler_unit == ru,
            &format!("  Unit: {}", ru.label()),
            "",
        ) {
            app.ruler_unit = ru;
            app.show_rulers = true;
            app.status = format!("Ruler units: {}", ru.label());
        }
    }
    let mut grid = app.show_grid;
    if nav.check_row(ui, p, &mut grid, "Grid", "Ctrl+Alt+G") {
        app.show_grid = grid;
    }
    let mut snap = app.snap_to_grid;
    if nav.check_row(ui, p, &mut snap, "Snap to Grid", "Ctrl+Alt+Shift+G") {
        app.snap_to_grid = snap;
    }
    let mut guides = app.show_guides;
    if nav.check_row(ui, p, &mut guides, "Guides", "Ctrl+;") {
        app.show_guides = guides;
    }
    let mut slices = app.show_slices;
    if nav.check_row(ui, p, &mut slices, "Slice Guides & Overlay", "") {
        app.show_slices = slices;
    }
    let mut onion = app.onion_skin;
    if nav.check_row(ui, p, &mut onion, "Onion Skinning", "") {
        app.onion_skin = onion;
    }
}

fn select_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    let has_sel = app.selected.is_some() || !app.selected_set.is_empty();
    if nav.row(ui, p, true, "Select All", "Ctrl+A") {
        app.select_all();
    }
    if nav.row(ui, p, has_sel, "Deselect", "Ctrl+D") {
        app.clear_selection();
        app.status = "Deselected".to_string();
    }
    if nav.row(ui, p, true, "Superselect / Next Object", "Ctrl+Right") {
        app.cycle_selection(true);
    }
    if nav.row(ui, p, true, "Subselect / Previous Object", "Ctrl+Left") {
        app.cycle_selection(false);
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Select Similar", "") {
        app.select_similar();
    }
    if nav.row(ui, p, true, "Select Inverse", "Ctrl+Shift+I") {
        app.select_inverse();
    }
    if nav.row(ui, p, has_sel, "Feather Edge…", "Ctrl+Alt+Shift+D") {
        app.add_live_effect(LiveEffect::GaussianBlur {
            radius: 3.0,
            enabled: true,
        });
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Expand Marquee (+4 px)…", "") {
        app.expand_selected_bounds(4.0);
    }
    if nav.row(ui, p, has_sel, "Contract Marquee (-4 px)…", "") {
        app.expand_selected_bounds(-4.0);
    }
    if nav.row(ui, p, has_sel, "Convert Marquee to Path", "") {
        app.convert_selected_to_path();
    }
}

fn modify_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    let has_sel = app.selected.is_some() || !app.selected_set.is_empty();
    let multi_sel = app.selected_set.len() >= 2;
    let is_group = app
        .selected_object()
        .is_some_and(|o| matches!(o.kind, crate::doc::Shape::Group { .. }));

    if nav.row(ui, p, true, "Canvas Size…", "") {
        app.open_canvas_size_dialog();
    }
    if nav.row(ui, p, true, "Fit Canvas", "Ctrl+Alt+F") {
        app.fit_canvas_to_contents();
    }
    if nav.row(ui, p, true, "Rotate Canvas 90° CW", "") {
        app.rotate_canvas_90();
    }
    if nav.row(
        ui,
        p,
        has_sel,
        "Animate Selection (3-Step Tween)…",
        "Alt+Shift+F8",
    ) {
        app.tween_selected_across_states();
    }
    if nav.row(ui, p, has_sel, "Convert to Symbol…", "F8") {
        app.convert_selection_to_symbol();
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Lock / Unlock Selection", "Ctrl+Alt+L") {
        app.toggle_lock_selection();
    }
    if nav.row(ui, p, has_sel, "Flatten Selection", "Ctrl+Alt+Shift+Z") {
        app.flatten_selected_to_bitmap();
    }
    if nav.row(ui, p, has_sel, "Merge Down", "Ctrl+E") {
        app.flatten_selected_to_bitmap();
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Bring to Front", "Ctrl+Shift+Up") {
        app.reorder_selected(true);
    }
    if nav.row(ui, p, has_sel, "Bring Forward", "Ctrl+Up") {
        app.reorder_selected_step(true);
    }
    if nav.row(ui, p, has_sel, "Send Backward", "Ctrl+Down") {
        app.reorder_selected_step(false);
    }
    if nav.row(ui, p, has_sel, "Send to Back", "Ctrl+Shift+Down") {
        app.reorder_selected(false);
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Rotate 90° CW", "Ctrl+9") {
        app.rotate_90();
    }
    if nav.row(ui, p, has_sel, "Rotate 90° CCW", "Ctrl+7") {
        app.rotate_ccw_90();
    }
    if nav.row(ui, p, has_sel, "Flip Horizontal", "") {
        app.flip_horizontal();
    }
    if nav.row(ui, p, has_sel, "Flip Vertical", "") {
        app.flip_vertical();
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Join Paths", "Ctrl+J") {
        app.combine_paths_op("join");
    }
    if nav.row(ui, p, has_sel, "Split Paths", "Ctrl+Shift+J") {
        app.combine_paths_op("split");
    }
    if nav.row(ui, p, has_sel, "Union Paths", "") {
        app.combine_paths_op("union");
    }
    if nav.row(ui, p, has_sel, "Punch Paths", "") {
        app.combine_paths_op("punch");
    }
    if nav.row(ui, p, has_sel, "Snap To Pixel", "") {
        app.snap_selected_to_pixel();
    }
    ui.separator();
    if nav.row(ui, p, multi_sel, "Group", "Ctrl+G") {
        app.group_selected();
    }
    if nav.row(ui, p, is_group, "Ungroup", "Ctrl+Shift+G") {
        app.ungroup_selected();
    }
}

fn text_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    for fam in ["Sans-Serif", "Serif", "Monospace", "Display", "Geometric"] {
        if nav.row(ui, p, true, &format!("Font: {fam}"), "") {
            app.set_selected_font_family(fam);
        }
    }
    ui.separator();
    for sz in [12.0, 18.0, 24.0, 36.0, 48.0, 72.0] {
        if nav.row(ui, p, true, &format!("Size: {sz:.0} pt"), "") {
            app.set_selected_text_size(sz);
        }
    }
    ui.separator();
    if nav.row(ui, p, true, "Bold", "Ctrl+B") {
        app.toggle_selected_text_bold();
    }
    if nav.row(ui, p, true, "Italic", "Ctrl+I") {
        app.toggle_selected_text_italic();
    }
    ui.separator();
    if nav.row(ui, p, true, "Convert Text to Path", "Ctrl+Shift+P") {
        app.convert_selected_to_path();
    }
    if nav.row(ui, p, true, "Check Spelling…", "Shift+F7") {
        app.check_spelling();
    }
}

fn commands_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    if nav.row(ui, p, true, "Command Palette…", "Ctrl+K") {
        app.show_command_palette = true;
    }
    if nav.row(ui, p, true, "MCP & Headless CLI Console…", "") {
        app.show_mcp_dialog = true;
    }
    if nav.row(ui, p, true, "Manage Saved Commands…", "") {
        app.show_commands_dialog = true;
    }
    if nav.row(ui, p, true, "Run Script / Replay History Steps", "") {
        app.replay_last_action();
    }
    ui.separator();
    if nav.row(ui, p, true, "Creative ▸ Add Picture Frame", "") {
        app.command_add_picture_frame();
    }
    if nav.row(ui, p, true, "Creative ▸ Add Drop Shadow Card", "") {
        app.command_add_shadow_card();
    }
    if nav.row(ui, p, true, "Creative ▸ Convert to Grayscale", "") {
        app.apply_color_command("grayscale");
    }
    if nav.row(ui, p, true, "Creative ▸ Convert to Sepia Tone", "") {
        app.apply_color_command("sepia");
    }
    ui.separator();
    if nav.row(ui, p, true, "Document ▸ Center Selection on Canvas", "") {
        app.align_selected_to_canvas("center_x");
        app.align_selected_to_canvas("center_y");
    }
    if nav.row(ui, p, true, "Document ▸ Distribute 3×3 Guides", "") {
        app.command_add_thirds_guides();
    }
    if nav.row(ui, p, true, "Web ▸ Create Slice from Selection", "") {
        app.create_slice_from_selection();
    }
    if nav.row(ui, p, true, "Web ▸ Copy CSS3 Rules to Clipboard", "") {
        app.copy_selected_css();
    }
}

fn filters_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    let has_sel = app.selected.is_some() || !app.selected_set.is_empty();
    if nav.row(ui, p, has_sel, "Repeat Filter", "Ctrl+Alt+Shift+X") {
        app.repeat_last_filter();
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Adjust Color ▸ Brightness / Contrast", "") {
        app.add_live_effect(LiveEffect::ColorAdjust {
            brightness: 10.0,
            contrast: 15.0,
            saturation: 0.0,
            enabled: true,
        });
        app.apply_color_command("brighten");
    }
    if nav.row(ui, p, has_sel, "Adjust Color ▸ Invert", "Ctrl+Alt+Shift+I") {
        app.apply_color_command("invert");
    }
    if nav.row(ui, p, has_sel, "Adjust Color ▸ Grayscale", "") {
        app.apply_color_command("grayscale");
    }
    if nav.row(ui, p, has_sel, "Adjust Color ▸ Sepia Tone", "") {
        app.apply_color_command("sepia");
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Shadow and Glow ▸ Drop Shadow", "") {
        app.add_live_effect(LiveEffect::DropShadow {
            distance: 5.0,
            angle_deg: 45.0,
            blur: 5.0,
            color: Color32::from_black_alpha(150),
            enabled: true,
        });
    }
    if nav.row(ui, p, has_sel, "Shadow and Glow ▸ Inner Shadow", "") {
        app.add_live_effect(LiveEffect::InnerShadow {
            distance: 3.0,
            blur: 4.0,
            color: Color32::from_black_alpha(140),
            enabled: true,
        });
    }
    if nav.row(ui, p, has_sel, "Shadow and Glow ▸ Outer Glow", "") {
        app.add_live_effect(LiveEffect::Glow {
            radius: 6.0,
            color: Color32::from_rgba_unmultiplied(255, 190, 40, 180),
            inner: false,
            enabled: true,
        });
    }
    if nav.row(ui, p, has_sel, "Bevel and Emboss ▸ Inner Bevel", "") {
        app.add_live_effect(LiveEffect::Bevel {
            width: 2.0,
            highlight: Color32::from_white_alpha(190),
            shadow: Color32::from_black_alpha(160),
            enabled: true,
        });
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Blur ▸ Gaussian Blur…", "") {
        app.add_live_effect(LiveEffect::GaussianBlur {
            radius: 4.0,
            enabled: true,
        });
    }
    if nav.row(ui, p, has_sel, "Blur ▸ Box Blur (Bitmap)", "") {
        app.run_bitmap_filter(Tool::Blur);
    }
    if nav.row(ui, p, has_sel, "Sharpen ▸ Unsharp Mask (Bitmap)", "") {
        app.run_bitmap_filter(Tool::Sharpen);
    }
    ui.separator();
    if nav.row(ui, p, has_sel, "Clear All Live Filters", "") {
        app.clear_live_effects();
    }
}

fn window_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    let mut hide_panels = app.full_screen_canvas;
    if nav.check_row(ui, p, &mut hide_panels, "Hide Panels", "F4 / Tab") {
        app.toggle_full_screen_canvas();
    }
    let mut main_tb = app.show_main_toolbar;
    if nav.check_row(ui, p, &mut main_tb, "Main Toolbar", "") {
        app.show_main_toolbar = main_tb;
    }
    let mut tools = app.show_tools_dock;
    if nav.check_row(ui, p, &mut tools, "Tools", "Ctrl+F2") {
        app.show_tools_dock = tools;
    }
    let mut props = app.show_bottom_inspector;
    if nav.check_row(ui, p, &mut props, "Properties", "Ctrl+F3") {
        app.show_bottom_inspector = props;
    }
    ui.separator();
    for pid in crate::app::PanelId::ALL {
        let mut active = app.show_right_dock && app.is_panel_open(pid);
        if nav.check_row(ui, p, &mut active, pid.title(), pid.shortcut()) {
            app.full_screen_canvas = false;
            app.show_right_dock = true;
            app.focus_or_open_panel(pid);
        }
    }
    ui.separator();
    for mode in [ThemeMode::Dark, ThemeMode::Light, ThemeMode::System] {
        if nav.radio_row(ui, p, app.theme_mode == mode, mode.label(), "") {
            app.set_theme(mode);
        }
    }
    ui.separator();
    if nav.row(ui, p, true, "Reset Workspace Layout", "") {
        app.full_screen_canvas = false;
        app.show_main_toolbar = true;
        app.show_tools_dock = true;
        app.tools_collapsed = false;
        app.show_right_dock = true;
        app.show_bottom_inspector = true;
        app.bottom_collapsed = false;
        app.apply_workspace_preset("Expanded Mode");
        app.status = "Workspace layout reset to CS6 Expanded Mode".to_string();
    }
}

fn help_menu(ui: &mut Ui, app: &mut FireCraft, nav: &mut MenuNav) {
    let p = app.palette;
    if nav.row(ui, p, true, "FireCraft Help & Keyboard Shortcuts…", "F1") {
        app.show_about_dialog = true;
    }
    if nav.row(ui, p, true, "Welcome Screen…", "") {
        app.show_welcome = true;
    }
    if nav.row(ui, p, true, "FireCraft Exchange (MCP / CLI)…", "") {
        app.show_mcp_dialog = true;
    }
    if nav.row(ui, p, true, "Load 1080p Demo Showcase", "") {
        app.load_demo_showcase();
    }
    ui.separator();
    if nav.row(ui, p, true, "About FireCraft CS6…", "") {
        app.show_about_dialog = true;
    }
}
