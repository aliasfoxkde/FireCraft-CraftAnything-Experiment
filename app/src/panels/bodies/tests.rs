#[cfg(test)]
mod body_render_tests {
    use super::*;

    fn render_body(app: &mut FireCraft, mut body: impl FnMut(&mut Ui, &mut FireCraft)) -> usize {
        let ctx = egui::Context::default();
        app.ctx = ctx.clone();
        let mut output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    Vec2::new(900.0, 700.0),
                )),
                time: Some(3.0),
                ..Default::default()
            },
            |ui| body(ui, app),
        );
        let shape_count = output.shapes.len();
        output.textures_delta.clear();
        shape_count
    }

    fn assert_panel_renders(app: &mut FireCraft, body: impl FnMut(&mut Ui, &mut FireCraft)) {
        let shapes = render_body(app, body);
        assert!(shapes > 0, "panel body should paint at least one shape");
    }

    #[test]
    fn primary_panel_bodies_render_empty_and_populated_states() {
        let mut app = FireCraft::new(None);
        assert_panel_renders(&mut app, optimize_panel);
        assert_panel_renders(&mut app, history_panel);
        assert_panel_renders(&mut app, align_panel);
        assert_panel_renders(&mut app, paths_panel);
        assert_panel_renders(&mut app, swatches_panel);
        assert_panel_renders(&mut app, info_panel);

        app.load_demo_showcase();
        app.select_all();
        app.undo.push(app.doc.snapshot("Render history entry"));
        app.redo.push(app.doc.snapshot("Render redo entry"));
        assert_panel_renders(&mut app, history_panel);
        assert_panel_renders(&mut app, align_panel);
        assert_panel_renders(&mut app, info_panel);
        assert_panel_renders(&mut app, paths_panel);
    }

    #[test]
    fn styles_and_intelligence_render_each_available_state() {
        let mut app = FireCraft::new(None);
        for library in crate::doc::StyleLibrary::ALL {
            app.style_library = library;
            assert_panel_renders(&mut app, styles_panel);
        }

        app.load_demo_showcase();
        app.select_all();
        app.style_library = crate::doc::StyleLibrary::CurrentDocument;
        app.selected_style_idx = Some(0);
        assert_panel_renders(&mut app, styles_panel);

        app.show_ai_settings = false;
        app.ai_response.clear();
        assert_panel_renders(&mut app, intelligence_panel);
        app.show_ai_settings = true;
        app.ai_response = "A real panel response state".to_string();
        assert_panel_renders(&mut app, intelligence_panel);
    }

    #[test]
    fn library_tabs_render_empty_and_populated_content() {
        let mut app = FireCraft::new(None);
        for tab in 0..5 {
            app.library_tab = tab;
            assert_panel_renders(&mut app, library_panel);
        }

        app.load_demo_showcase();
        app.select_all();
        app.doc.symbols.push(crate::doc::SymbolDef {
            id: 0,
            name: "Panel symbol".to_string(),
            kind: "Graphic".to_string(),
            objects: Vec::new(),
        });
        for tab in 1..5 {
            app.library_tab = tab;
            assert_panel_renders(&mut app, library_panel);
        }
    }

    #[test]
    fn tab_adapters_select_their_document_and_library_pages() {
        let mut app = FireCraft::new(None);
        for (tab, render) in [pages_tab, states_tab, layers_tab].into_iter().enumerate() {
            assert_panel_renders(&mut app, render);
            assert_eq!(app.layers_tab, tab);
        }
        for (tab, render) in [
            special_chars_tab,
            image_editing_tab,
            auto_shapes_tab,
            document_library_tab,
            css_properties_tab,
        ]
        .into_iter()
        .enumerate()
        {
            assert_panel_renders(&mut app, render);
            assert_eq!(app.library_tab, tab);
        }
    }
}
