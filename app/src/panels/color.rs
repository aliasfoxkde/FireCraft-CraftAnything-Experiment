//! Color Mixer panel body.

use super::bodies::slider_row;
use super::*;

pub fn color_panel(ui: &mut Ui, app: &mut FireCraft) {
    let p = app.palette;
    ui.spacing_mut().item_spacing = Vec2::new(3.0, 2.0);

    // Top CS6 segmented sub-tabs: Selector | Mixers | Blender
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(1.0, 1.0);
        for (i, label) in ["Selector", "Mixers", "Blender"].iter().enumerate() {
            if ui
                .selectable_label(app.color_subtab == i, RichText::new(*label).small())
                .clicked()
            {
                app.color_subtab = i;
            }
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button("⇄")
                .on_hover_text("Swap Fill & Stroke Colors (X)")
                .clicked()
            {
                std::mem::swap(&mut app.fill, &mut app.stroke);
            }
            if ui
                .small_button("⊘")
                .on_hover_text("Set No Fill on Selection")
                .clicked()
            {
                app.set_selected_fill(None);
            }
        });
    });

    // Stroke & Fill icon + swatch + hex rows matching CS6 Color Palette
    ui.horizontal(|ui| {
        // Stroke box
        let (pen_r, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::hover());
        icons::draw(ui.painter(), pen_r, icons::Icon::Pencil, p.text_dim, 1.0);
        let (st_rect, st_resp) = ui.allocate_exact_size(Vec2::new(18.0, 14.0), Sense::click());
        ui.painter().rect_filled(st_rect, 0.0, app.stroke);
        ui.painter().rect_stroke(
            st_rect,
            0.0,
            Stroke::new(1.0, p.stroke),
            egui::StrokeKind::Middle,
        );
        if st_resp
            .on_hover_text("Stroke Color (click to swap with Fill)")
            .clicked()
        {
            std::mem::swap(&mut app.fill, &mut app.stroke);
        }
        let mut hex2 = colors::hex_of(app.stroke);
        if ui
            .add(
                TextEdit::singleline(&mut hex2)
                    .desired_width(68.0)
                    .font(FontId::monospace(10.5))
                    .text_color(p.text),
            )
            .changed()
            && let Some(c) = colors::from_hex(&hex2)
        {
            app.stroke = c;
            app.set_selected_stroke(Some(c));
        }

        ui.add_space(4.0);

        // Fill box
        let (fill_ic_r, _) = ui.allocate_exact_size(Vec2::new(14.0, 14.0), Sense::hover());
        icons::draw(ui.painter(), fill_ic_r, icons::Icon::Fill, p.text_dim, 1.0);
        let (fl_rect, _) = ui.allocate_exact_size(Vec2::new(18.0, 14.0), Sense::click());
        ui.painter().rect_filled(fl_rect, 0.0, app.fill);
        ui.painter().rect_stroke(
            fl_rect,
            0.0,
            Stroke::new(1.0, p.accent),
            egui::StrokeKind::Middle,
        );
        let mut hex = colors::hex_of(app.fill);
        if ui
            .add(
                TextEdit::singleline(&mut hex)
                    .desired_width(68.0)
                    .font(FontId::monospace(10.5))
                    .text_color(p.text),
            )
            .changed()
            && let Some(c) = colors::from_hex(&hex)
        {
            app.fill = c;
            app.set_selected_fill(Some(c));
        }
    });

    if app.color_subtab == 2 {
        // Blender tab: live 12-step interpolation ramp between Stroke and Fill
        ui.label(
            RichText::new("Blend Ramp (Left-click: Fill, Right-click: Stroke):")
                .small()
                .color(p.text_dim),
        );
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(2.0, 2.0);
            for step in 0..12 {
                let t = step as f32 / 11.0;
                let r = (app.stroke.r() as f32 * (1.0 - t) + app.fill.r() as f32 * t) as u8;
                let g = (app.stroke.g() as f32 * (1.0 - t) + app.fill.g() as f32 * t) as u8;
                let b = (app.stroke.b() as f32 * (1.0 - t) + app.fill.b() as f32 * t) as u8;
                let c = Color32::from_rgb(r, g, b);
                let (cell, resp) = ui.allocate_exact_size(Vec2::new(20.0, 52.0), Sense::click());
                ui.painter().rect_filled(cell, 0.0, c);
                ui.painter().rect_stroke(
                    cell,
                    0.0,
                    Stroke::new(1.0, p.stroke),
                    egui::StrokeKind::Middle,
                );
                if resp.clicked() {
                    app.fill = c;
                    app.set_selected_fill(Some(c));
                }
                if resp.secondary_clicked() {
                    app.stroke = c;
                    app.set_selected_stroke(Some(c));
                }
                resp.on_hover_text(colors::hex_of(c));
            }
        });
        return;
    }

    let spaces = ["HLS", "HSV", "CMYK", "RGB"];
    ui.horizontal(|ui| {
        for (i, name) in spaces.iter().enumerate() {
            let selected = app.color_tab == i;
            if ui
                .selectable_label(selected, RichText::new(*name).small())
                .clicked()
            {
                app.color_tab = i;
            }
        }
    });

    ui.horizontal(|ui| {
        let mut hsv = colors::rgb_to_hsv(app.fill);
        let (square, sq_resp) =
            ui.allocate_exact_size(Vec2::new(92.0, 68.0), Sense::click_and_drag());
        let painter = ui.painter();
        let steps_x = 24;
        let steps_y = 18;
        for iy in 0..steps_y {
            for ix in 0..steps_x {
                let s = ix as f32 / (steps_x - 1) as f32;
                let v = 1.0 - iy as f32 / (steps_y - 1) as f32;
                let c = colors::hsv_to_rgb(colors::Hsv { h: hsv.h, s, v });
                let cell = Rect::from_min_size(
                    pos2(
                        square.left() + square.width() * ix as f32 / steps_x as f32,
                        square.top() + square.height() * iy as f32 / steps_y as f32,
                    ),
                    Vec2::new(
                        square.width() / steps_x as f32 + 0.8,
                        square.height() / steps_y as f32 + 0.8,
                    ),
                );
                painter.rect_filled(cell, 0.0, c);
            }
        }
        painter.rect_stroke(
            square,
            0.0,
            Stroke::new(1.0, p.stroke),
            egui::StrokeKind::Middle,
        );
        // Active SV crosshair ring
        let cur_pt = pos2(
            square.left() + hsv.s.clamp(0.0, 1.0) * square.width(),
            square.top() + (1.0 - hsv.v.clamp(0.0, 1.0)) * square.height(),
        );
        painter.circle_stroke(cur_pt, 3.5, Stroke::new(1.2, Color32::WHITE));
        painter.circle_stroke(cur_pt, 4.5, Stroke::new(1.0, Color32::BLACK));

        if (sq_resp.clicked() || sq_resp.dragged())
            && let Some(pos) = sq_resp.interact_pointer_pos()
        {
            hsv.s = ((pos.x - square.left()) / square.width()).clamp(0.0, 1.0);
            hsv.v = (1.0 - (pos.y - square.top()) / square.height()).clamp(0.0, 1.0);
            app.fill = colors::hsv_to_rgb(hsv);
            app.set_selected_fill(Some(app.fill));
        }

        let (bar, bar_resp) =
            ui.allocate_exact_size(Vec2::new(14.0, 68.0), Sense::click_and_drag());
        let painter = ui.painter();
        for iy in 0..32 {
            let h = iy as f32 / 32.0 * 360.0;
            let c = colors::hsv_to_rgb(colors::Hsv { h, s: 1.0, v: 1.0 });
            let cell = Rect::from_min_size(
                pos2(bar.left(), bar.top() + bar.height() * iy as f32 / 32.0),
                Vec2::new(bar.width(), bar.height() / 32.0 + 0.8),
            );
            painter.rect_filled(cell, 0.0, c);
        }
        painter.rect_stroke(
            bar,
            0.0,
            Stroke::new(1.0, p.stroke),
            egui::StrokeKind::Middle,
        );
        // Hue indicator tick
        let hy = bar.top() + (hsv.h / 360.0).clamp(0.0, 1.0) * bar.height();
        painter.line_segment(
            [pos2(bar.left(), hy), pos2(bar.right(), hy)],
            Stroke::new(1.5, Color32::WHITE),
        );
        if (bar_resp.clicked() || bar_resp.dragged())
            && let Some(pos) = bar_resp.interact_pointer_pos()
        {
            hsv.h = (((pos.y - bar.top()) / bar.height()).clamp(0.0, 1.0)) * 360.0;
            app.fill = colors::hsv_to_rgb(hsv);
            app.set_selected_fill(Some(app.fill));
        }

        let mut changed = false;
        ui.vertical(|ui| match app.color_tab {
            0 => {
                let mut hls = colors::rgb_to_hls(app.fill);
                let mut l_pct = hls.l * 100.0;
                let mut s_pct = hls.s * 100.0;
                changed |= slider_row(ui, "H", &mut hls.h, 0.0, 360.0, "°");
                changed |= slider_row(ui, "L", &mut l_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "S", &mut s_pct, 0.0, 100.0, "%");
                if changed {
                    hls.l = l_pct / 100.0;
                    hls.s = s_pct / 100.0;
                    app.fill = colors::hls_to_rgb(hls);
                    app.set_selected_fill(Some(app.fill));
                }
            }
            1 => {
                let mut hsv = colors::rgb_to_hsv(app.fill);
                let mut s_pct = hsv.s * 100.0;
                let mut v_pct = hsv.v * 100.0;
                changed |= slider_row(ui, "H", &mut hsv.h, 0.0, 360.0, "°");
                changed |= slider_row(ui, "S", &mut s_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "V", &mut v_pct, 0.0, 100.0, "%");
                if changed {
                    hsv.s = s_pct / 100.0;
                    hsv.v = v_pct / 100.0;
                    app.fill = colors::hsv_to_rgb(hsv);
                    app.set_selected_fill(Some(app.fill));
                }
            }
            2 => {
                let mut cmyk = colors::rgb_to_cmyk(app.fill);
                let mut c_pct = cmyk.c * 100.0;
                let mut m_pct = cmyk.m * 100.0;
                let mut y_pct = cmyk.y * 100.0;
                let mut k_pct = cmyk.k * 100.0;
                changed |= slider_row(ui, "C", &mut c_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "M", &mut m_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "Y", &mut y_pct, 0.0, 100.0, "%");
                changed |= slider_row(ui, "K", &mut k_pct, 0.0, 100.0, "%");
                if changed {
                    cmyk.c = c_pct / 100.0;
                    cmyk.m = m_pct / 100.0;
                    cmyk.y = y_pct / 100.0;
                    cmyk.k = k_pct / 100.0;
                    app.fill = colors::cmyk_to_rgb(cmyk);
                    app.set_selected_fill(Some(app.fill));
                }
            }
            _ => {
                let mut r = app.fill.r() as f32;
                let mut g = app.fill.g() as f32;
                let mut b = app.fill.b() as f32;
                changed |= slider_row(ui, "R", &mut r, 0.0, 255.0, "");
                changed |= slider_row(ui, "G", &mut g, 0.0, 255.0, "");
                changed |= slider_row(ui, "B", &mut b, 0.0, 255.0, "");
                if changed {
                    app.fill = Color32::from_rgb(r as u8, g as u8, b as u8);
                    app.set_selected_fill(Some(app.fill));
                }
            }
        });
    });
}
