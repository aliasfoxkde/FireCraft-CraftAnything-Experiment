//! The 10-menu Adobe Fireworks CS6 menu bar:
//! File, Edit, View, Select, Modify, Text, Commands, Filters, Window, Help.
//! Every item dispatches to a real editor action.

use crate::app::{CanvasViewMode, FileMode, FireCraft, Tool};
use crate::constants;
use crate::doc::LiveEffect;
use crate::theme::ThemeMode;

use eframe::egui::{self, Color32, RichText};

/// Draw the full 10-menu Fireworks CS6 application menu bar plus header controls.
pub fn menu_bar(ui: &mut egui::Ui, app: &mut FireCraft) {
    ui.horizontal(|ui| {
        ui.label(
            RichText::new("Fw")
                .strong()
                .color(app.palette.accent)
                .size(14.0),
        );
        ui.separator();
        ui.menu_button("File", |ui| file_menu(ui, app));
        ui.menu_button("Edit", |ui| edit_menu(ui, app));
        ui.menu_button("View", |ui| view_menu(ui, app));
        ui.menu_button("Select", |ui| select_menu(ui, app));
        ui.menu_button("Modify", |ui| modify_menu(ui, app));
        ui.menu_button("Text", |ui| text_menu(ui, app));
        ui.menu_button("Commands", |ui| commands_menu(ui, app));
        ui.menu_button("Filters", |ui| filters_menu(ui, app));
        ui.menu_button("Window", |ui| window_menu(ui, app));
        ui.menu_button("Help", |ui| help_menu(ui, app));

        ui.separator();
        if ui
            .small_button("Br")
            .on_hover_text("Browse in Bridge / Open File Browser")
            .clicked()
        {
            app.begin_file_dialog(FileMode::Open);
        }
        if ui
            .selectable_label(app.tool == Tool::Hand, "Hand")
            .on_hover_text("Hand Tool (H)")
            .clicked()
        {
            app.tool = Tool::Hand;
        }
        if ui
            .selectable_label(app.tool == Tool::Zoom, "Zoom")
            .on_hover_text("Zoom Tool (Z)")
            .clicked()
        {
            app.tool = Tool::Zoom;
        }

        egui::ComboBox::from_id_salt("header_zoom_combo")
            .width(68.0)
            .selected_text(format!("{:.0}%", app.doc.zoom * 100.0))
            .show_ui(ui, |ui| {
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
                .width(115.0)
                .selected_text(if app.show_right_dock {
                    "Expanded Mode"
                } else {
                    "Compact Mode"
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(app.show_right_dock, "Expanded Mode")
                        .clicked()
                    {
                        app.show_right_dock = true;
                        app.show_bottom_inspector = true;
                    }
                    if ui
                        .selectable_label(!app.show_right_dock, "Compact Mode")
                        .clicked()
                    {
                        app.show_right_dock = false;
                    }
                });

            egui::ComboBox::from_id_salt("header_theme_combo")
                .width(105.0)
                .selected_text(app.theme_mode.short_label())
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
}

fn file_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    if ui.button("New…                     Ctrl+N").clicked() {
        app.show_new_dialog = true;
        ui.close();
    }
    ui.menu_button("New from Template", |ui| {
        for (name, w, h) in [
            ("Web Hero Banner (1200×630)", 1200.0, 630.0),
            ("Full HD UI Layout (1920×1080)", 1920.0, 1080.0),
            ("Mobile App Screen (390×844)", 390.0, 844.0),
            ("App Icon Grid (512×512)", 512.0, 512.0),
        ] {
            if ui.button(name).clicked() {
                app.create_template_document(name, w, h);
                ui.close();
            }
        }
    });
    if ui.button("Open…                    Ctrl+O").clicked() {
        app.begin_file_dialog(FileMode::Open);
        ui.close();
    }
    ui.menu_button("Open Recent", |ui| {
        if app.recent_files.is_empty() {
            ui.label(RichText::new("(No recent files)").small());
        } else {
            for path in app.recent_files.clone() {
                let label = path.to_string_lossy().to_string();
                if ui.button(label).clicked() {
                    let _ = app.open_path(&path);
                    ui.close();
                }
            }
        }
    });
    ui.separator();
    if ui.button("Close                    Ctrl+W").clicked() {
        app.close_active_tab();
        ui.close();
    }
    if ui.button("Save                     Ctrl+S").clicked() {
        app.save_current();
        ui.close();
    }
    if ui.button("Save As…           Ctrl+Shift+S").clicked() {
        app.begin_file_dialog(FileMode::Save);
        ui.close();
    }
    ui.separator();
    if ui.button("Import…                  Ctrl+R").clicked() {
        app.begin_file_dialog(FileMode::Import);
        ui.close();
    }
    if ui.button("Export…            Ctrl+Shift+R").clicked() {
        app.begin_file_dialog(FileMode::Export);
        ui.close();
    }
    if ui.button("Image Preview…     Ctrl+Shift+X").clicked() {
        app.view_mode = CanvasViewMode::TwoUp;
        app.status = "2-Up Image Preview enabled".to_string();
        ui.close();
    }
    if ui.button("Export HTML5 + CSS…").clicked() {
        app.file_dialog_name = "index.html".to_string();
        app.begin_file_dialog(FileMode::Export);
        app.file_dialog_name = "index.html".to_string();
        ui.close();
    }
    ui.separator();
    if ui.button("Canvas Size…").clicked() {
        app.open_canvas_size_dialog();
        ui.close();
    }
    if ui.button("Welcome Screen").clicked() {
        app.show_welcome = true;
        ui.close();
    }
}

fn edit_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    let can_undo = !app.undo.is_empty();
    let can_redo = !app.redo.is_empty();
    let has_selection = app.selected.is_some();

    if ui
        .add_enabled(
            can_undo,
            egui::Button::new("Undo                     Ctrl+Z"),
        )
        .clicked()
    {
        app.undo_action();
        ui.close();
    }
    if ui
        .add_enabled(
            can_redo,
            egui::Button::new("Redo                     Ctrl+Y"),
        )
        .clicked()
    {
        app.redo_action();
        ui.close();
    }
    ui.separator();
    ui.menu_button("Insert", |ui| {
        if ui.button("New Layer").clicked() {
            app.add_layer();
            ui.close();
        }
        if ui.button("New Page").clicked() {
            app.add_page();
            ui.close();
        }
        if ui.button("New Animation State").clicked() {
            app.add_state();
            ui.close();
        }
        if ui.button("New Symbol from Selection…").clicked() {
            app.convert_selection_to_symbol();
            ui.close();
        }
        if ui.button("Rectangular Slice").clicked() {
            app.insert_slice(false);
            ui.close();
        }
        if ui.button("Hotspot").clicked() {
            app.insert_slice(true);
            ui.close();
        }
    });
    if ui.button("Find and Replace…        Ctrl+F").clicked() {
        app.show_find_replace = true;
        ui.close();
    }
    ui.separator();
    if ui
        .add_enabled(
            has_selection,
            egui::Button::new("Cut                      Ctrl+X"),
        )
        .clicked()
    {
        app.cut_selected();
        ui.close();
    }
    if ui
        .add_enabled(
            has_selection,
            egui::Button::new("Copy                     Ctrl+C"),
        )
        .clicked()
    {
        app.copy_selected();
        ui.close();
    }
    if ui
        .add_enabled(has_selection, egui::Button::new("Copy CSS Properties"))
        .clicked()
    {
        app.copy_selected_css();
        ui.close();
    }
    if ui
        .add_enabled(
            app.clipboard.is_some(),
            egui::Button::new("Paste                    Ctrl+V"),
        )
        .clicked()
    {
        app.paste();
        ui.close();
    }
    if ui
        .add_enabled(
            has_selection,
            egui::Button::new("Duplicate                Ctrl+D"),
        )
        .clicked()
    {
        app.duplicate_selected();
        ui.close();
    }
    if ui
        .add_enabled(
            has_selection,
            egui::Button::new("Clone              Ctrl+Shift+D"),
        )
        .clicked()
    {
        app.clone_selected();
        ui.close();
    }
    if ui
        .add_enabled(
            has_selection,
            egui::Button::new("Delete                   Del"),
        )
        .clicked()
    {
        app.delete_selected();
        ui.close();
    }
}

fn view_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    if ui.button("Zoom In                  Ctrl++").clicked() {
        app.zoom_by(constants::ZOOM_STEP);
        ui.close();
    }
    if ui.button("Zoom Out                 Ctrl+-").clicked() {
        app.zoom_by(1.0 / constants::ZOOM_STEP);
        ui.close();
    }
    ui.menu_button("Magnification", |ui| {
        for &preset in constants::ZOOM_PRESETS {
            if ui
                .selectable_label(
                    (app.doc.zoom - preset).abs() < 0.01,
                    format!("{:.0}%", preset * 100.0),
                )
                .clicked()
            {
                app.set_zoom(preset);
                ui.close();
            }
        }
    });
    if ui.button("Fit in Window            Ctrl+0").clicked() {
        app.fit_in_window();
        ui.close();
    }
    ui.separator();
    for (mode, label) in [
        (CanvasViewMode::Original, "Original View"),
        (CanvasViewMode::Preview, "Preview"),
        (CanvasViewMode::TwoUp, "2-Up Split View"),
        (CanvasViewMode::FourUp, "4-Up Compare View"),
    ] {
        if ui.selectable_label(app.view_mode == mode, label).clicked() {
            app.view_mode = mode;
            ui.close();
        }
    }
    ui.separator();
    ui.checkbox(&mut app.show_rulers, "Rulers                   Ctrl+Alt+R");
    ui.checkbox(&mut app.show_grid, "Grid                     Ctrl+Alt+G");
    ui.checkbox(&mut app.snap_to_grid, "Snap to Grid");
    ui.checkbox(&mut app.show_guides, "Guides                   Ctrl+;");
    ui.checkbox(&mut app.show_slices, "Web Slices & Hotspots");
    ui.checkbox(&mut app.onion_skin, "Onion Skinning (Animation)");
}

fn select_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    if ui.button("Select All               Ctrl+A").clicked() {
        app.select_all();
        ui.close();
    }
    if ui.button("Deselect                 Ctrl+D").clicked() {
        app.selected = None;
        app.status = "Deselected".to_string();
        ui.close();
    }
    ui.separator();
    if ui.button("Select Next Object       Tab").clicked() {
        app.cycle_selection(true);
        ui.close();
    }
    if ui.button("Select Previous Object   Shift+Tab").clicked() {
        app.cycle_selection(false);
        ui.close();
    }
    ui.separator();
    if ui.button("Select Similar Type").clicked() {
        app.select_similar();
        ui.close();
    }
}

fn modify_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    let has_selection = app.selected.is_some();
    ui.menu_button("Canvas", |ui| {
        if ui.button("Canvas Size…").clicked() {
            app.open_canvas_size_dialog();
            ui.close();
        }
        if ui.button("Fit Canvas to Objects").clicked() {
            app.fit_canvas_to_contents();
            ui.close();
        }
        if ui.button("Rotate Canvas 90° CW").clicked() {
            app.rotate_canvas_90();
            ui.close();
        }
        if ui.button("Toggle Transparent Canvas").clicked() {
            let page = app.doc.page_mut();
            page.transparent_bg = !page.transparent_bg;
            ui.close();
        }
    });
    ui.menu_button("Animation", |ui| {
        if ui.button("Add Animation State").clicked() {
            app.add_state();
            ui.close();
        }
        if ui.button("Duplicate State").clicked() {
            app.duplicate_state();
            ui.close();
        }
        if ui.button("Animate Selection (3-Step Tween)").clicked() {
            app.tween_selected_across_states();
            ui.close();
        }
    });
    ui.menu_button("Symbol", |ui| {
        if ui
            .add_enabled(has_selection, egui::Button::new("Convert to Symbol…   F8"))
            .clicked()
        {
            app.convert_selection_to_symbol();
            ui.close();
        }
    });
    ui.separator();
    ui.menu_button("Transform", |ui| {
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
            .add_enabled(has_selection, egui::Button::new("Rotate 90° CW"))
            .clicked()
        {
            app.rotate_90();
            ui.close();
        }
        if ui
            .add_enabled(has_selection, egui::Button::new("Scale 125%"))
            .clicked()
        {
            app.scale_selected(1.25);
            ui.close();
        }
        if ui
            .add_enabled(has_selection, egui::Button::new("Scale 80%"))
            .clicked()
        {
            app.scale_selected(0.80);
            ui.close();
        }
    });
    ui.menu_button("Arrange", |ui| {
        if ui
            .add_enabled(has_selection, egui::Button::new("Bring to Front"))
            .clicked()
        {
            app.reorder_selected(true);
            ui.close();
        }
        if ui
            .add_enabled(has_selection, egui::Button::new("Bring Forward"))
            .clicked()
        {
            app.step_order_selected(1);
            ui.close();
        }
        if ui
            .add_enabled(has_selection, egui::Button::new("Send Backward"))
            .clicked()
        {
            app.step_order_selected(-1);
            ui.close();
        }
        if ui
            .add_enabled(has_selection, egui::Button::new("Send to Back"))
            .clicked()
        {
            app.reorder_selected(false);
            ui.close();
        }
    });
    ui.menu_button("Align", |ui| {
        for (mode, label) in [
            ("left", "Align Left Edge"),
            ("center_x", "Align Horizontal Center"),
            ("right", "Align Right Edge"),
            ("top", "Align Top Edge"),
            ("center_y", "Align Vertical Center"),
            ("bottom", "Align Bottom Edge"),
        ] {
            if ui
                .add_enabled(has_selection, egui::Button::new(label))
                .clicked()
            {
                app.align_selected_to_canvas(mode);
                ui.close();
            }
        }
    });
    ui.menu_button("Combine Paths", |ui| {
        for (op, label) in [
            ("union", "Union"),
            ("intersect", "Intersect"),
            ("punch", "Punch"),
            ("crop", "Crop"),
        ] {
            if ui
                .add_enabled(has_selection, egui::Button::new(label))
                .clicked()
            {
                app.combine_paths_op(op);
                ui.close();
            }
        }
    });
    ui.menu_button("Alter Path", |ui| {
        if ui
            .add_enabled(has_selection, egui::Button::new("Simplify Path"))
            .clicked()
        {
            app.simplify_selected_path();
            ui.close();
        }
        if ui
            .add_enabled(has_selection, egui::Button::new("Expand Stroke (+4 px)"))
            .clicked()
        {
            app.expand_selected_bounds(4.0);
            ui.close();
        }
        if ui
            .add_enabled(has_selection, egui::Button::new("Inset Path (-4 px)"))
            .clicked()
        {
            app.expand_selected_bounds(-4.0);
            ui.close();
        }
    });
    ui.separator();
    let multi_sel = app.selected_set.len() >= 2;
    let is_group = app
        .selected_object()
        .is_some_and(|o| matches!(o.kind, crate::doc::Shape::Group { .. }));
    if ui
        .add_enabled(
            multi_sel,
            egui::Button::new("Group                    Ctrl+G"),
        )
        .clicked()
    {
        app.group_selected();
        ui.close();
    }
    if ui
        .add_enabled(
            is_group,
            egui::Button::new("Ungroup            Ctrl+Shift+G"),
        )
        .clicked()
    {
        app.ungroup_selected();
        ui.close();
    }
    if ui
        .add_enabled(
            has_selection,
            egui::Button::new("Flatten Selection to Bitmap"),
        )
        .clicked()
    {
        app.flatten_selected_to_bitmap();
        ui.close();
    }
}

fn text_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    ui.menu_button("Font", |ui| {
        for fam in ["Sans-Serif", "Serif", "Monospace", "Display", "Geometric"] {
            if ui.button(fam).clicked() {
                app.set_selected_font_family(fam);
                ui.close();
            }
        }
    });
    ui.menu_button("Size", |ui| {
        for sz in [10.0, 12.0, 14.0, 18.0, 24.0, 32.0, 48.0, 64.0, 96.0] {
            if ui.button(format!("{sz:.0} pt")).clicked() {
                app.set_selected_text_size(sz);
                ui.close();
            }
        }
    });
    ui.menu_button("Style", |ui| {
        if ui.button("Toggle Bold              Ctrl+B").clicked() {
            app.toggle_selected_text_bold();
            ui.close();
        }
        if ui.button("Toggle Italic            Ctrl+I").clicked() {
            app.toggle_selected_text_italic();
            ui.close();
        }
    });
    ui.separator();
    if ui.button("Convert Text to Path").clicked() {
        app.convert_selected_to_path();
        ui.close();
    }
    if ui.button("Check Spelling…").clicked() {
        app.check_spelling();
        ui.close();
    }
}

fn commands_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    if ui.button("Command Palette…         Ctrl+K").clicked() {
        app.show_command_palette = true;
        ui.close();
    }
    if ui.button("MCP & Headless CLI Console…").clicked() {
        app.show_mcp_dialog = true;
        ui.close();
    }
    if ui.button("Manage Saved Commands…").clicked() {
        app.show_commands_dialog = true;
        ui.close();
    }
    if ui.button("Run Script / Replay History Steps").clicked() {
        app.replay_last_action();
        ui.close();
    }
    ui.separator();
    ui.menu_button("Creative", |ui| {
        if ui.button("Add Picture Frame").clicked() {
            app.command_add_picture_frame();
            ui.close();
        }
        if ui.button("Add Drop Shadow Card").clicked() {
            app.command_add_shadow_card();
            ui.close();
        }
        if ui.button("Convert to Grayscale").clicked() {
            app.apply_color_command("grayscale");
            ui.close();
        }
        if ui.button("Convert to Sepia Tone").clicked() {
            app.apply_color_command("sepia");
            ui.close();
        }
    });
    ui.menu_button("Document", |ui| {
        if ui.button("Center All Objects on Canvas").clicked() {
            app.align_selected_to_canvas("center_x");
            app.align_selected_to_canvas("center_y");
            ui.close();
        }
        if ui.button("Distribute Guides 3×3 Grid").clicked() {
            app.command_add_thirds_guides();
            ui.close();
        }
    });
    ui.menu_button("Web", |ui| {
        if ui.button("Create Slice from Selection").clicked() {
            app.create_slice_from_selection();
            ui.close();
        }
        if ui.button("Export CSS Rules to Status").clicked() {
            app.copy_selected_css();
            ui.close();
        }
    });
}

fn filters_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    let has_selection = app.selected.is_some();
    if ui
        .add_enabled(
            has_selection,
            egui::Button::new("Repeat Last Filter       Ctrl+Alt+Shift+X"),
        )
        .clicked()
    {
        app.repeat_last_filter();
        ui.close();
    }
    ui.separator();
    ui.menu_button("Shadow and Glow", |ui| {
        if ui.button("Drop Shadow").clicked() {
            app.add_live_effect(LiveEffect::DropShadow {
                distance: 5.0,
                angle_deg: 45.0,
                blur: 5.0,
                color: Color32::from_black_alpha(150),
                enabled: true,
            });
            ui.close();
        }
        if ui.button("Inner Shadow").clicked() {
            app.add_live_effect(LiveEffect::InnerShadow {
                distance: 3.0,
                blur: 4.0,
                color: Color32::from_black_alpha(140),
                enabled: true,
            });
            ui.close();
        }
        if ui.button("Outer Glow").clicked() {
            app.add_live_effect(LiveEffect::Glow {
                radius: 6.0,
                color: Color32::from_rgba_unmultiplied(255, 190, 40, 180),
                inner: false,
                enabled: true,
            });
            ui.close();
        }
        if ui.button("Inner Glow").clicked() {
            app.add_live_effect(LiveEffect::Glow {
                radius: 4.0,
                color: Color32::from_rgba_unmultiplied(255, 255, 255, 160),
                inner: true,
                enabled: true,
            });
            ui.close();
        }
    });
    ui.menu_button("Bevel and Emboss", |ui| {
        if ui.button("Inner Bevel").clicked() {
            app.add_live_effect(LiveEffect::Bevel {
                width: 2.0,
                highlight: Color32::from_white_alpha(190),
                shadow: Color32::from_black_alpha(160),
                enabled: true,
            });
            ui.close();
        }
    });
    ui.menu_button("Adjust Color", |ui| {
        if ui.button("Brightness / Contrast (Live)").clicked() {
            app.add_live_effect(LiveEffect::ColorAdjust {
                brightness: 10.0,
                contrast: 15.0,
                saturation: 0.0,
                enabled: true,
            });
            app.apply_color_command("brighten");
            ui.close();
        }
        if ui.button("Invert Colors").clicked() {
            app.apply_color_command("invert");
            ui.close();
        }
        if ui.button("Grayscale").clicked() {
            app.apply_color_command("grayscale");
            ui.close();
        }
        if ui.button("Sepia").clicked() {
            app.apply_color_command("sepia");
            ui.close();
        }
    });
    ui.menu_button("Blur", |ui| {
        if ui.button("Box Blur (Bitmap)").clicked() {
            app.run_bitmap_filter(Tool::Blur);
            ui.close();
        }
        if ui.button("Gaussian Blur (Live)").clicked() {
            app.add_live_effect(LiveEffect::GaussianBlur {
                radius: 4.0,
                enabled: true,
            });
            ui.close();
        }
    });
    ui.menu_button("Sharpen", |ui| {
        if ui.button("Unsharp Mask (Bitmap)").clicked() {
            app.run_bitmap_filter(Tool::Sharpen);
            ui.close();
        }
    });
    ui.separator();
    if ui
        .add_enabled(has_selection, egui::Button::new("Clear All Live Filters"))
        .clicked()
    {
        app.clear_live_effects();
        ui.close();
    }
}

fn window_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    ui.checkbox(&mut app.show_tools_dock, "Tools                    Ctrl+F2");
    ui.checkbox(
        &mut app.show_bottom_inspector,
        "Properties               Ctrl+F3",
    );
    ui.checkbox(&mut app.show_right_dock, "Right Panels             F4");
    ui.separator();
    for (label, group, tab) in [
        ("Optimize                 F6", 0usize, 0usize),
        ("History                  Shift+F10", 0, 1),
        ("Align", 0, 2),
        ("Path", 0, 3),
        ("Pages                    F5", 1, 0),
        ("States                   Shift+F2", 1, 1),
        ("Layers                   F2", 1, 2),
        ("Styles                   Ctrl+F11", 2, 0),
        ("Color Palette            Shift+F9", 2, 1),
        ("Swatches                 Ctrl+F9", 2, 2),
        ("Info                     Alt+Shift+F12", 2, 3),
        ("Special Characters", 3, 0),
        ("Image Editing", 3, 1),
        ("Auto Shapes", 3, 2),
        ("Document Library         F11", 3, 3),
        ("CSS Properties / URL", 3, 4),
    ] {
        if ui.button(label).clicked() {
            app.show_right_dock = true;
            match group {
                0 => app.top_right_tab = tab,
                1 => app.layers_tab = tab,
                2 => app.middle_right_tab = tab,
                _ => app.library_tab = tab,
            }
            ui.close();
        }
    }
    ui.separator();
    ui.label(
        RichText::new("Appearance Theme")
            .small()
            .color(app.palette.text_dim),
    );
    for mode in [ThemeMode::Dark, ThemeMode::Light, ThemeMode::System] {
        if ui
            .selectable_label(app.theme_mode == mode, mode.label())
            .clicked()
        {
            app.set_theme(mode);
            ui.close();
        }
    }
    ui.separator();
    if ui.button("Reset Workspace Layout").clicked() {
        app.show_tools_dock = true;
        app.show_right_dock = true;
        app.show_bottom_inspector = true;
        app.top_right_tab = 0;
        app.layers_tab = 2;
        app.middle_right_tab = 1;
        app.library_tab = 2;
        app.status = "Workspace layout reset to CS6 Expanded Mode".to_string();
        ui.close();
    }
}

fn help_menu(ui: &mut egui::Ui, app: &mut FireCraft) {
    if ui.button("FireCraft User Guide &Shortcuts…").clicked() {
        app.show_about_dialog = true;
        ui.close();
    }
    if ui.button("Welcome Screen…").clicked() {
        app.show_welcome = true;
        ui.close();
    }
    ui.separator();
    ui.hyperlink_to(
        "FireCraft Wiki (GitHub)",
        "https://github.com/aliasfoxkde/FireCraft/wiki",
    );
    ui.hyperlink_to(
        "FireCraft Repository & Docs",
        "https://github.com/aliasfoxkde/FireCraft/blob/main/README.md",
    );
    ui.separator();
    if ui.button("Load Demo Vector & Web Showcase").clicked() {
        app.load_demo_showcase();
        ui.close();
    }
    if ui.button("About FireCraft…").clicked() {
        app.show_about_dialog = true;
        ui.close();
    }
}
