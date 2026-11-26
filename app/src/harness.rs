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
}
