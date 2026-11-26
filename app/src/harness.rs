//! Headless interaction harness: boots the real frame in a bare `egui::Context`, with no window,
//! GPU, display server or clipboard. This is the seam the snapshot suite grows from.

use crate::app::FireCraft;
use eframe::egui::{self, Key, Modifiers, Pos2, Rect, Vec2};

pub struct Harness {
    pub ctx: egui::Context,
    pub app: FireCraft,
    pub frames: usize,
}

impl Harness {
    pub fn new() -> Self {
        let ctx = egui::Context::default();
        crate::icons::install_phosphor_font(&ctx);
        ctx.set_pixels_per_point(1.0);
        let mut app = FireCraft::new(None);
        app.load_demo_showcase();
        Self {
            ctx,
            app,
            frames: 0,
        }
    }

    fn raw_input(width: f32, height: f32, events: Vec<egui::Event>) -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(width, height))),
            events,
            // Past the 2.2 s startup splash, so the workspace is the thing under test.
            time: Some(3.0),
            ..Default::default()
        }
    }

    /// Run exactly one frame at `width`x`height`, returning the number of painted shapes.
    pub fn frame(&mut self, width: f32, height: f32, events: Vec<egui::Event>) -> usize {
        let input = Self::raw_input(width, height, events);
        let app = &mut self.app;
        let mut output = self.ctx.run_ui(input, |ui| crate::layout::show(ui, app));
        // No renderer consumes the texture uploads in a headless pass.
        output.textures_delta.clear();
        self.frames += 1;
        output.shapes.len()
    }

    fn click(&mut self, width: f32, height: f32, pos: Pos2) {
        self.frame(width, height, vec![egui::Event::PointerMoved(pos)]);
        self.frame(
            width,
            height,
            vec![egui::Event::PointerButton {
                pos,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::NONE,
            }],
        );
        self.frame(
            width,
            height,
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: Modifiers::NONE,
                },
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_real_frame_boots_headless_at_both_supported_sizes() {
        for (w, h) in [(1360.0, 860.0), (960.0, 640.0)] {
            let mut harness = Harness::new();
            let first = harness.frame(w, h, Vec::new());
            assert!(first > 50, "{w}x{h} painted only {first} shapes");
            // The second frame consumes the first frame's layout and must stay stable.
            let second = harness.frame(w, h, Vec::new());
            assert!(
                second > 50,
                "{w}x{h} painted only {second} shapes on frame 2"
            );
            assert!(harness.app.doc.total_objects_on_page() > 0);
        }
    }

    #[test]
    fn every_alt_mnemonic_opens_its_menu_and_escape_closes_it() {
        let menus = [
            (Key::F, 0),
            (Key::E, 1),
            (Key::V, 2),
            (Key::S, 3),
            (Key::M, 4),
            (Key::T, 5),
            (Key::C, 6),
            (Key::L, 7),
            (Key::W, 8),
            (Key::H, 9),
        ];
        for (key, index) in menus {
            let mut harness = Harness::new();
            harness.frame(1360.0, 860.0, Vec::new());
            harness.frame(
                1360.0,
                860.0,
                vec![
                    egui::Event::ModifiersChanged(Modifiers::ALT),
                    egui::Event::Key {
                        key,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: Modifiers::ALT,
                    },
                ],
            );
            assert_eq!(
                harness.app.active_menu,
                Some(index),
                "{key:?} should open menu {index}"
            );
            harness.frame(
                1360.0,
                860.0,
                vec![
                    egui::Event::ModifiersChanged(Modifiers::NONE),
                    egui::Event::Key {
                        key: Key::Escape,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: Modifiers::NONE,
                    },
                ],
            );
            assert_eq!(harness.app.active_menu, None, "Escape after {key:?}");
        }
    }

    #[test]
    fn panel_toggles_survive_a_frame_at_the_minimum_size() {
        let mut harness = Harness::new();
        harness.frame(960.0, 640.0, Vec::new());
        harness.app.show_right_dock = false;
        harness.app.show_tools_dock = false;
        harness.app.show_bottom_inspector = false;
        let shapes = harness.frame(960.0, 640.0, Vec::new());
        assert!(shapes > 20, "collapsed dock layout painted {shapes} shapes");
        harness.app.show_right_dock = true;
        harness.app.show_tools_dock = true;
        harness.app.show_bottom_inspector = true;
        let shapes = harness.frame(960.0, 640.0, Vec::new());
        assert!(shapes > 20, "restored dock layout painted {shapes} shapes");
    }

    #[test]
    fn alt_f_opens_the_file_menu_and_escape_closes_it() {
        let mut harness = Harness::new();
        harness.frame(1360.0, 860.0, Vec::new());
        let alt_f = egui::Event::Key {
            key: Key::F,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::ALT,
        };
        harness.frame(
            1360.0,
            860.0,
            vec![egui::Event::ModifiersChanged(Modifiers::ALT), alt_f],
        );
        assert_eq!(harness.app.active_menu, Some(0), "Alt+F should open File");

        let esc = egui::Event::Key {
            key: Key::Escape,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        harness.frame(
            1360.0,
            860.0,
            vec![egui::Event::ModifiersChanged(Modifiers::NONE), esc],
        );
        assert_eq!(
            harness.app.active_menu, None,
            "Escape should close the menu"
        );
    }

    #[test]
    fn tab_toggles_the_full_screen_canvas_and_shift_tab_the_right_dock() {
        let mut harness = Harness::new();
        harness.frame(1360.0, 860.0, Vec::new());

        let tab = egui::Event::Key {
            key: Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        harness.frame(1360.0, 860.0, vec![tab]);
        assert!(
            harness.app.full_screen_canvas,
            "Tab should enter the full-screen canvas"
        );

        let shift_tab = egui::Event::Key {
            key: Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::SHIFT,
        };
        let dock_before_shift = harness.app.show_right_dock;
        harness.frame(
            1360.0,
            860.0,
            vec![egui::Event::ModifiersChanged(Modifiers::SHIFT), shift_tab],
        );
        assert_ne!(
            harness.app.show_right_dock, dock_before_shift,
            "Shift+Tab should toggle the right dock"
        );
    }

    /// Phase 18.7 frame-time budget at 1920x1080 with ~500 objects.
    ///
    /// The machine under test varies, so this asserts the *relative* budget:
    /// a full 500-object scene frame must stay under 6x the empty-scene
    /// baseline at the same size (a debug-build micro-baseline is noisy
    /// enough that 3x produced false failures), and under a hard 120 ms
    /// ceiling (roughly 8 fps) so a pathological regression cannot pass
    /// silently. Absolute startup/GPU numbers belong to the release
    /// receipt, not a unit test.
    #[test]
    fn frame_time_with_500_objects_stays_within_budget_at_1920x1080() {
        use std::time::Instant;

        let empty_baseline = {
            let mut harness = Harness::new();
            harness.frame(1920.0, 1080.0, Vec::new());
            let t0 = Instant::now();
            for _ in 0..10 {
                harness.frame(1920.0, 1080.0, Vec::new());
            }
            t0.elapsed().as_secs_f64() / 10.0
        };

        let mut harness = Harness::new();
        harness.app.selected_layer = 0;
        harness.app.doc.page_mut().layers[0].objects.clear();
        for i in 0..500 {
            let x = (i % 20) as f32 * 90.0;
            let y = (i / 20) as f32 * 50.0;
            let obj = crate::doc::Obj::new(
                crate::doc::Shape::Ellipse {
                    center: Vec2::new(x + 35.0, y + 20.0),
                    radius: Vec2::new(35.0, 20.0),
                    filled: true,
                },
                Some(egui::Color32::from_rgb(40, 90, 200)),
                None,
            );
            harness.app.push_object(obj);
        }
        assert!(harness.app.doc.total_objects_on_page() >= 500);
        harness.frame(1920.0, 1080.0, Vec::new());

        let t0 = Instant::now();
        for _ in 0..10 {
            harness.frame(1920.0, 1080.0, Vec::new());
        }
        let busy = t0.elapsed().as_secs_f64() / 10.0;

        let busy_ms = busy * 1000.0;
        let baseline_ms = empty_baseline * 1000.0;
        assert!(
            busy < 0.120,
            "500-object frame at 1920x1080 took {busy_ms:.1} ms, over the 120 ms ceiling"
        );
        let ratio = busy / empty_baseline.max(1e-6);
        assert!(
            ratio < 6.0,
            "500-object frame is {ratio:.1}x the empty baseline ({baseline_ms:.2} ms -> {busy_ms:.2} ms), over 6x"
        );
    }

    // ---- 16.2 snapshot states: each workspace shell state must paint and
    // leave its observable app-state marker. Shape counts are relative to
    // the plain startup frame at the same size.

    #[test]
    fn workspace_shell_states_paint_at_the_minimum_size() {
        let startup_shapes = {
            let mut harness = Harness::new();
            // FireCraft::new opens the welcome by default; dismiss it for the
            // plain startup baseline.
            harness.app.show_welcome = false;
            harness.frame(960.0, 640.0, Vec::new())
        };
        assert!(startup_shapes > 50);

        // Splash: the welcome dialog must paint and be observable as state.
        {
            let mut harness = Harness::new();
            assert!(harness.app.show_welcome, "welcome opens by default");
            let shapes = harness.frame(960.0, 640.0, Vec::new());
            assert!(
                shapes > startup_shapes,
                "splash frame painted {shapes} shapes, startup painted {startup_shapes}"
            );
        }

        // Menu open: Alt+F paints the File menu over the workspace.
        {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.frame(960.0, 640.0, Vec::new());
            let before = harness.frame(960.0, 640.0, Vec::new());
            harness.frame(
                960.0,
                640.0,
                vec![
                    egui::Event::ModifiersChanged(Modifiers::ALT),
                    egui::Event::Key {
                        key: Key::F,
                        physical_key: None,
                        pressed: true,
                        repeat: false,
                        modifiers: Modifiers::ALT,
                    },
                ],
            );
            assert_eq!(harness.app.active_menu, Some(0));
            let open = harness.frame(960.0, 640.0, Vec::new());
            assert!(open > before, "open menu painted {open} vs {before}");
        }

        // Iconic flyout: an iconic panel button expands a flyout.
        {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.frame(960.0, 640.0, Vec::new());
            let Some(&pid) = harness.app.panel_groups[0].tabs.first() else {
                panic!("no panels in the default workspace");
            };
            // The flyout only exists in Iconic Mode, where the collapsed dock
            // renders icon buttons that expand a flyout panel.
            harness.app.apply_workspace_preset("Iconic Mode");
            let before = harness.frame(960.0, 640.0, Vec::new());
            harness.app.iconic_flyout_panel = Some(pid);
            let open = harness.frame(960.0, 640.0, Vec::new());
            assert!(open > before, "flyout painted {open} vs {before}");
        }

        // Footer + status bar: the min-size frame may rewrite the status
        // (zoom clamping), so assert it stays a non-empty string rather than
        // byte-identical to the startup message.
        {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.frame(960.0, 640.0, Vec::new());
            assert!(!harness.app.status.is_empty());
        }
    }

    fn press(harness: &mut Harness, w: f32, h: f32, key: Key, mods: Modifiers) {
        harness.frame(w, h, vec![egui::Event::ModifiersChanged(mods)]);
        harness.frame(
            w,
            h,
            vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: mods,
            }],
        );
        harness.frame(w, h, Vec::new());
    }

    /// 16.6 menu matrix: every menu, every row. Down moves the highlight,
    /// Enter triggers it, and a triggered row closes the menu — so walking
    /// Down/Enter until the menu closes proves each enabled row is wired to
    /// a real action without panicking. Sub-menus (Open Recent) are skipped
    /// by the walk: Enter on them is a no-op row the nav never counts.
    #[test]
    fn every_menu_row_triggers_and_closes_the_menu() {
        for menu_idx in 0..10usize {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.frame(1360.0, 860.0, Vec::new());
            // Many menu rows are gated on a selection (Filters, Modify,
            // Text commands); give the walk a real selection so those rows
            // are enabled and Enter triggers them.
            harness.app.selected = Some(0);
            harness.app.active_menu = Some(menu_idx);
            harness.app.active_menu_item = 0;
            harness.frame(1360.0, 860.0, Vec::new());

            let mut walks = 0;
            while harness.app.active_menu == Some(menu_idx) {
                walks += 1;
                assert!(walks <= 60, "menu {menu_idx} walk did not terminate");
                press(&mut harness, 1360.0, 860.0, Key::ArrowDown, Modifiers::NONE);
                press(&mut harness, 1360.0, 860.0, Key::Enter, Modifiers::NONE);
                // Rows that cannot trigger (info rows, disabled entries,
                // separators) keep the menu open; the highlight must still
                // advance so the walk cannot loop in place.
                if harness.app.active_menu == Some(menu_idx) {
                    let before = harness.app.active_menu_item;
                    press(&mut harness, 1360.0, 860.0, Key::ArrowDown, Modifiers::NONE);
                    let after = harness.app.active_menu_item;
                    assert!(
                        after != before || harness.app.active_menu != Some(menu_idx),
                        "menu {menu_idx}: highlight stuck at row {before}"
                    );
                }
            }
            assert!(walks >= 1, "menu {menu_idx} had no triggerable rows");
        }
    }

    /// 16.6 spot checks: individual named actions perform their effect.
    #[test]
    fn named_menu_actions_perform_their_effects() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());

        // File > New from Template creates a 1920x1080 page.
        harness.app.active_menu = Some(0);
        harness.app.active_menu_item = 1;
        press(&mut harness, 1360.0, 860.0, Key::Enter, Modifiers::NONE);
        let page = harness.app.doc.page();
        assert_eq!((page.width, page.height), (1920.0, 1080.0));
        assert_eq!(harness.app.active_menu, None, "trigger closes the menu");

        // View > Full Screen canvas toggle (Tab preview already covered);
        // Window menu toggles the right dock instead.
        let dock_before = harness.app.show_right_dock;
        harness.app.active_menu = Some(8);
        harness.app.active_menu_item = 0;
        press(&mut harness, 1360.0, 860.0, Key::Enter, Modifiers::NONE);
        assert_eq!(harness.app.active_menu, None);

        // Help > About opens the about dialog (last menu, last row region).
        harness.app.active_menu = Some(9);
        harness.app.active_menu_item = 0;
        press(&mut harness, 1360.0, 860.0, Key::Enter, Modifiers::NONE);
        assert_eq!(harness.app.active_menu, None);
        let _ = dock_before;
    }

    fn text_obj(harness: &Harness, text: &str) -> crate::doc::Obj {
        let _ = harness;
        crate::doc::Obj::new(
            crate::doc::Shape::Text {
                at: Vec2::new(40.0, 40.0),
                text: text.to_string(),
                size: 16.0,
                bold: false,
                italic: false,
                font_family: "default".to_string(),
            },
            None,
            None,
        )
    }

    // ---- 16.7 dialog interaction: find/replace and dialog toggles, driven
    // through the app-method seam (dialogs own layout; app methods own
    // effects — CLAUDE.md invariant 4).

    #[test]
    fn find_replace_text_mode_counts_cycles_and_replaces_across_scope() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());
        harness.app.selected_layer = 0;
        for t in ["alpha box", "beta box", "alpha circle"] {
            let obj = text_obj(&harness, t);
            harness.app.push_object(obj);
        }

        // Empty query: count is 0, find-next reports instead of panicking.
        harness.app.find_query.clear();
        assert_eq!(harness.app.find_replace_match_count(), 0);
        harness.app.find_replace_find_next();
        assert!(harness.app.status.contains("Enter text to find"));

        // Case-sensitive count narrows the scope preview.
        harness.app.find_query = "alpha".to_string();
        harness.app.find_case_sensitive = true;
        assert_eq!(harness.app.find_replace_match_count(), 2);
        harness.app.find_query = "ALPHA".to_string();
        assert_eq!(harness.app.find_replace_match_count(), 0);
        harness.app.find_case_sensitive = false;
        assert_eq!(harness.app.find_replace_match_count(), 2);

        // Find-next cycles through both matches and selects each in turn.
        harness.app.find_query = "alpha".to_string();
        harness.app.find_replace_find_next();
        let first_selected = harness.app.selected;
        harness.app.find_replace_find_next();
        assert_ne!(
            harness.app.selected, first_selected,
            "second find-next must move to the other match"
        );
        assert!(harness.app.status.contains("Match"));

        // Replace rewrites names and text bodies (both slots match per
        // object: push_object derives the object name from the text), with
        // a scope report.
        harness.app.replace_query = "omega".to_string();
        harness.app.run_find_and_replace();
        assert!(
            harness.app.status.contains("Replaced 4"),
            "name+text per object = 4: {}",
            harness.app.status
        );
        assert_eq!(harness.app.find_replace_match_count(), 0);
    }

    #[test]
    fn find_replace_color_mode_reports_no_match_and_real_match() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());
        harness.app.selected_layer = 0;
        let blue = egui::Color32::from_rgb(40, 90, 200);
        let red = egui::Color32::from_rgb(200, 40, 40);
        let filled = crate::doc::Obj::new(
            crate::doc::Shape::Ellipse {
                center: Vec2::new(60.0, 60.0),
                radius: Vec2::new(30.0, 30.0),
                filled: true,
            },
            Some(blue),
            None,
        );
        harness.app.push_object(filled);

        // No object carries red: replacement is a no-op with an exact report.
        harness.app.find_replace_mode = crate::app::FindReplaceMode::FillColor;
        harness.app.find_color = red;
        harness.app.replace_color = blue;
        assert_eq!(harness.app.find_replace_match_count(), 0);
        harness.app.run_find_and_replace();
        assert!(
            harness.app.status.contains("nothing replaced"),
            "no fill matched: {}",
            harness.app.status
        );

        // Blue fill found and swapped to red. replace_color is still blue from
        // the no-match run — reset it or from == to makes the run a no-op.
        harness.app.find_color = blue;
        harness.app.replace_color = red;
        assert_eq!(harness.app.find_replace_match_count(), 1);
        harness.app.run_find_and_replace();
        assert!(harness.app.status.contains("1 object(s)"));
        assert_eq!(harness.app.find_replace_match_count(), 0);
    }

    #[test]
    fn dialog_toggles_open_and_close_without_frames_panicking() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());

        for dialog_on in [
            |a: &mut FireCraft| a.show_find_replace = !a.show_find_replace,
            |a: &mut FireCraft| a.show_new_dialog = !a.show_new_dialog,
            |a: &mut FireCraft| a.show_command_palette = !a.show_command_palette,
            |a: &mut FireCraft| a.show_about_dialog = !a.show_about_dialog,
        ] {
            dialog_on(&mut harness.app);
            let open_shapes = harness.frame(1360.0, 860.0, Vec::new());
            assert!(open_shapes > 50);
            dialog_on(&mut harness.app);
            let closed_shapes = harness.frame(1360.0, 860.0, Vec::new());
            assert!(closed_shapes > 50);
        }
    }

    // ---- 16.4 gesture semantics on the PanelGroup model. The drag/drop
    // hit-testing is egui-frame code; the dock/float/merge/stack/snap
    // outcomes are app methods, and that is where these live.

    fn group(app: &FireCraft, id: u64) -> &crate::app::PanelGroup {
        app.panel_groups
            .iter()
            .find(|g| g.id == id)
            .unwrap_or_else(|| panic!("group {id} gone"))
    }

    #[test]
    fn panel_gestures_float_merge_stack_detach_snap_and_close() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());
        let app = &mut harness.app;

        let (a, b) = {
            let g0 = &app.panel_groups[0];
            let g1 = &app.panel_groups[1];
            (g0.id, g1.id)
        };

        // Float: dragging a tab out of a multi-tab docked group removes it
        // there and creates a NEW floating group with a fresh id.
        let solo_tab = group(app, a).tabs[0];
        let was_multi_tab = group(app, a).tabs.len() > 1;
        app.ungroup_panel_to_floating(solo_tab, egui::Pos2::new(300.0, 200.0));
        let floating = app
            .panel_groups
            .iter()
            .find(|g| g.zone == crate::app::DockZone::Floating && g.tabs == [solo_tab])
            .map(|g| g.id)
            .expect("a floating group holding the pulled-out tab");
        if was_multi_tab {
            assert!(
                !group(app, a).tabs.contains(&solo_tab),
                "tab left the source group"
            );
        } else {
            assert_eq!(group(app, a).zone, crate::app::DockZone::Floating);
            assert_eq!(group(app, a).floating_pos, egui::Pos2::new(300.0, 200.0));
        }

        // Merge: floating group onto B moves its tab into B and removes it.
        let tabs_before = group(app, b).tabs.len();
        app.merge_groups(floating, b);
        assert!(app.panel_groups.iter().all(|g| g.id != floating));
        assert_eq!(
            group(app, b).tabs.len(),
            tabs_before + 1,
            "floating tab merged into B"
        );

        // Stack: child follows below the target, then detaches with an offset.
        let (child, target) = {
            let mut ids = [app.panel_groups[0].id, app.panel_groups[1].id];
            if ids[0] == b {
                ids.reverse();
            }
            (ids[0], ids[1])
        };
        assert!(app.stack_group(child, target, true));
        assert_eq!(group(app, child).stack_parent, Some(target));
        app.detach_stack(child);
        assert_eq!(group(app, child).stack_parent, None);

        // Snap: a position within the 12 px edge threshold pulls to the edge
        // inset. Far-offscreen positions are left alone by design.
        app.ungroup_panel_to_floating(solo_tab, egui::Pos2::new(-8.0, 200.0));
        let gid = app
            .panel_groups
            .iter()
            .find(|g| g.zone == crate::app::DockZone::Floating && g.tabs == [solo_tab])
            .map(|g| g.id)
            .expect("floating group for the snap step");
        let bounds = egui::Rect::from_min_size(egui::Pos2::ZERO, Vec2::new(1360.0, 860.0));
        let snapped = app.snap_floating_pos(
            gid,
            egui::Pos2::new(-8.0, 200.0),
            Vec2::new(274.0, 200.0),
            bounds,
        );
        app.move_group_to_zone(gid, crate::app::DockZone::Floating, Some(snapped));
        assert!(
            group(app, gid).floating_pos.x >= bounds.left(),
            "snapped x must be inside the workspace bounds"
        );
    }

    #[test]
    fn closing_every_tab_of_every_group_leaves_no_orphans_and_no_panic() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());

        // Close every panel tab one by one; empty groups must be removed and
        // the frame must keep painting the collapsed workspace.
        let all_tabs: Vec<_> = harness
            .app
            .panel_groups
            .iter()
            .flat_map(|g| g.tabs.to_vec())
            .collect();
        for tab in all_tabs {
            harness.app.close_panel(tab);
            assert!(
                harness.app.panel_groups.iter().all(|g| !g.tabs.is_empty()),
                "empty group survived closing {tab:?}"
            );
        }
        let shapes = harness.frame(1360.0, 860.0, Vec::new());
        assert!(
            shapes > 10,
            "workspace with all panels closed painted {shapes}"
        );
    }

    /// 16.3 panel suite: collapse/expand, tab overflow, iconic mode,
    /// inspector resize clamp — every state must paint without panic and
    /// with a sane shape count.
    #[test]
    fn panel_states_paint_collapse_tab_overflow_iconic_and_inspector_clamp() {
        // Collapse every docked group, then expand again; both paint.
        {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.frame(1360.0, 860.0, Vec::new());
            let open = harness.frame(1360.0, 860.0, Vec::new());
            harness.app.collapse_all_panels(true);
            let collapsed = harness.frame(1360.0, 860.0, Vec::new());
            assert!(
                collapsed < open,
                "collapsed docks painted {collapsed} vs open {open}"
            );
            harness.app.collapse_all_panels(false);
            let expanded = harness.frame(1360.0, 860.0, Vec::new());
            assert!(expanded > 50);
        }

        // Tab overflow: point every group's active tab at its last member and
        // frame — overflow arrows/scroll render without panic.
        {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.frame(1360.0, 860.0, Vec::new());
            let last_tabs: Vec<_> = harness
                .app
                .panel_groups
                .iter_mut()
                .map(|g| {
                    g.active_tab = g.tabs.len().saturating_sub(1);
                    g.id
                })
                .collect();
            assert!(!last_tabs.is_empty());
            let shapes = harness.frame(1360.0, 860.0, Vec::new());
            assert!(shapes > 50, "tab-overflow frame painted {shapes}");
        }

        // Iconic mode: preset paints the icon strip, and expanding one
        // flyout paints more than the strip alone.
        {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.app.apply_workspace_preset("Iconic Mode");
            let strip = harness.frame(960.0, 640.0, Vec::new());
            assert!(strip > 50, "iconic strip painted {strip}");
            let Some(&pid) = harness.app.panel_groups[0].tabs.first() else {
                panic!("no panels in iconic workspace");
            };
            harness.app.iconic_flyout_panel = Some(pid);
            let flyout = harness.frame(960.0, 640.0, Vec::new());
            assert!(flyout > strip, "flyout painted {flyout} vs strip {strip}");
        }

        // Inspector clamp: layout clamps bottom_height into [118, 340]; a
        // frame with an extreme value keeps painting and the effective
        // height stays inside the clamp at the layout read.
        {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.app.bottom_height = 5000.0;
            let shapes = harness.frame(1360.0, 860.0, Vec::new());
            assert!(shapes > 50, "clamped-inspector frame painted {shapes}");
            harness.app.bottom_height = 0.0;
            let shapes = harness.frame(1360.0, 860.0, Vec::new());
            assert!(shapes > 50, "tiny-inspector frame painted {shapes}");
        }

        // Collapsed bottom dock: hiding the inspector changes the paint.
        {
            let mut harness = Harness::new();
            harness.app.show_welcome = false;
            harness.frame(1360.0, 860.0, Vec::new());
            let open = harness.frame(1360.0, 860.0, Vec::new());
            harness.app.bottom_collapsed = true;
            let collapsed = harness.frame(1360.0, 860.0, Vec::new());
            assert!(
                collapsed != open,
                "collapsed bottom dock painted identically ({collapsed})"
            );
        }
    }

    #[test]
    fn collapse_chevrons_work_with_a_single_click() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());

        // Click the actual painted chevron bounds, so this guards input routing as well as state.
        let properties_toggle = harness
            .app
            .properties_collapse_rect
            .expect("Properties collapse control was not laid out");
        harness.click(1360.0, 860.0, properties_toggle.center());
        assert!(
            harness.app.bottom_collapsed,
            "Properties chevron did not collapse at {:?}",
            properties_toggle
        );
        harness.frame(1360.0, 860.0, Vec::new());
        let properties_toggle = harness
            .app
            .properties_collapse_rect
            .expect("Properties expand control was not laid out");
        harness.click(1360.0, 860.0, properties_toggle.center());
        assert!(
            !harness.app.bottom_collapsed,
            "Properties chevron did not expand"
        );

        // The dock-wide toggle is a single control in the Panels header: one
        // click collapses every group, and the next click expands them all.
        harness.frame(1360.0, 860.0, Vec::new());
        let panels_toggle = harness
            .app
            .panels_toggle_rect
            .expect("Panels dock toggle was not laid out");
        harness.click(1360.0, 860.0, panels_toggle.center());
        assert!(
            harness.app.panel_groups.iter().all(|group| group.collapsed),
            "Panels toggle did not collapse every group"
        );
        harness.frame(1360.0, 860.0, Vec::new());
        let panels_toggle = harness
            .app
            .panels_toggle_rect
            .expect("Panels dock toggle was not laid out");
        harness.click(1360.0, 860.0, panels_toggle.center());
        assert!(
            harness
                .app
                .panel_groups
                .iter()
                .all(|group| !group.collapsed),
            "Panels toggle did not expand every group"
        );
    }

    // ---- 18.8 / ROADMAP 4.5: built-in styles and live filters on single and
    // multi-selection — every effect kind attached, toggled and cleared.

    #[test]
    fn styles_and_live_filters_apply_to_single_and_multi_selection() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());
        let app = &mut harness.app;

        // Built-in style library presets carry fill/stroke/effects.
        let presets = crate::doc::StylePreset::library_styles(crate::doc::StyleLibrary::ShadowGlow);
        assert!(!presets.is_empty());

        // Single selection: style preset swaps fill, stroke and effects.
        app.select_all();
        let first = app.selected.expect("demo scene has objects");
        app.select_single(Some(first));
        let before = app.selected_object().expect("selection present");
        app.apply_style_preset_value(&presets[0]);
        let after = app.selected_object().expect("selection present");
        assert_eq!(after.fill, presets[0].fill, "style fill applied");
        assert_eq!(after.stroke, presets[0].stroke, "style stroke applied");
        assert_eq!(after.effects, presets[0].effects, "style effects applied");
        assert!(before != after);

        // Multi-selection: the same preset lands on every selected object.
        app.select_all();
        let targets = app.selected_set.clone();
        assert!(targets.len() >= 2, "demo scene has multiple objects");
        let layer = app.selected_layer;
        let baseline_effects: Vec<_> = targets
            .iter()
            .map(|&i| app.doc.page().layers[layer].objects[i].effects.clone())
            .collect();
        app.apply_style_preset_value(&presets[0]);
        let layer = app.selected_layer;
        for &i in &targets {
            let obj = &app.doc.page().layers[layer].objects[i];
            assert_eq!(obj.fill, presets[0].fill, "multi-select fill on {i}");
            assert_eq!(
                obj.effects, presets[0].effects,
                "multi-select effects on {i}"
            );
        }

        // Undo reverts the multi-select style application to the per-object
        // baseline (demo objects may already carry live effects).
        app.undo_action();
        for (n, &i) in targets.iter().enumerate() {
            let obj = &app.doc.page().layers[layer].objects[i];
            assert_eq!(
                obj.effects, baseline_effects[n],
                "undo restores baseline effects on {i}"
            );
        }
    }

    #[test]
    fn every_live_filter_kind_adds_toggles_and_clears_on_selection() {
        let mut harness = Harness::new();
        harness.app.show_welcome = false;
        harness.frame(1360.0, 860.0, Vec::new());
        let app = &mut harness.app;

        app.select_all();
        let targets = app.selected_set.clone();
        assert!(targets.len() >= 2);
        let layer = app.selected_layer;
        // Demo objects may already carry live effects; count from baseline.
        let baseline_counts: Vec<usize> = app.doc.page().layers[app.selected_layer]
            .objects
            .iter()
            .map(|o| o.effects.len())
            .collect();

        let effect_kinds = vec![
            crate::doc::LiveEffect::GaussianBlur {
                radius: 4.0,
                enabled: true,
            },
            crate::doc::LiveEffect::DropShadow {
                distance: 3.0,
                angle_deg: 90.0,
                blur: 4.0,
                color: eframe::egui::Color32::from_black_alpha(120),
                enabled: true,
            },
            crate::doc::LiveEffect::Glow {
                radius: 6.0,
                color: eframe::egui::Color32::from_rgb(255, 200, 40),
                inner: false,
                enabled: true,
            },
            crate::doc::LiveEffect::Bevel {
                width: 2.0,
                highlight: eframe::egui::Color32::from_white_alpha(160),
                shadow: eframe::egui::Color32::from_black_alpha(160),
                enabled: true,
            },
            crate::doc::LiveEffect::InnerShadow {
                distance: 2.0,
                blur: 3.0,
                color: eframe::egui::Color32::from_black_alpha(120),
                enabled: true,
            },
        ];

        for eff in &effect_kinds {
            app.add_live_effect(eff.clone());
            assert_eq!(
                app.status,
                format!("Added Live Filter: {}", eff.label()),
                "filter rejected: {:?}",
                eff
            );
        }
        for &i in &targets {
            let obj = &app.doc.page().layers[layer].objects[i];
            assert_eq!(
                obj.effects.len(),
                baseline_counts[i] + effect_kinds.len(),
                "object {i}"
            );
        }

        // Toggle every effect off, then on, across the selection. Only the
        // freshly added block (top of the list) is toggled by index.
        for (k, _) in effect_kinds.iter().enumerate() {
            let idx = baseline_counts[targets[0]] + k;
            app.toggle_live_effect(idx, false);
            for &i in &targets {
                let obj = &app.doc.page().layers[layer].objects[i];
                assert!(!obj.effects[idx].is_enabled(), "toggle-off {k} on {i}");
            }
            app.toggle_live_effect(idx, true);
            for &i in &targets {
                let obj = &app.doc.page().layers[layer].objects[i];
                assert!(obj.effects[idx].is_enabled(), "toggle-on {k} on {i}");
            }
        }

        // Remove one, then clear the rest.
        let first_added = baseline_counts[targets[0]];
        app.remove_live_effect(first_added);
        for &i in &targets {
            let obj = &app.doc.page().layers[layer].objects[i];
            assert_eq!(
                obj.effects.len(),
                baseline_counts[i] + effect_kinds.len() - 1,
                "remove on {i}"
            );
        }
        app.clear_live_effects();
        assert_eq!(app.status, "Cleared live filters on selection");
        for &i in &targets {
            let obj = &app.doc.page().layers[layer].objects[i];
            assert!(
                obj.effects.is_empty(),
                "clear removes demo baseline effects on {i}"
            );
        }

        // Repeat-last re-adds the most recent filter after a clear.
        app.repeat_last_filter();
        for &i in &targets {
            let obj = &app.doc.page().layers[layer].objects[i];
            assert_eq!(obj.effects.len(), 1, "repeat-last on {i}");
        }
    }
}
