//! Arrange commands: align, distribute, space, match size, flip, rotate, scale and z-order.

use super::*;

impl FireCraft {
    pub fn align_selected_to_canvas(&mut self, mode: &str) {
        if self.selected_set.is_empty() && self.selected.is_none() {
            return;
        }
        let indices: Vec<usize> = if self.selected_set.is_empty() {
            self.selected.into_iter().collect()
        } else {
            self.selected_set.clone()
        };
        let (cw, ch) = (self.doc.page().width, self.doc.page().height);
        let layer = self.selected_layer;
        let use_canvas = self.align_to_canvas || indices.len() <= 1;
        let (ref_lo, ref_hi) = if use_canvas {
            (Vec2::ZERO, Vec2::new(cw, ch))
        } else {
            let mut u_lo = Vec2::splat(f32::MAX);
            let mut u_hi = Vec2::splat(f32::MIN);
            if let Some(l) = self.doc.page().layers.get(layer) {
                for &idx in &indices {
                    if let Some((lo, hi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                        u_lo = u_lo.min(lo);
                        u_hi = u_hi.max(hi);
                    }
                }
            }
            if u_lo.x > u_hi.x {
                (Vec2::ZERO, Vec2::new(cw, ch))
            } else {
                (u_lo, u_hi)
            }
        };
        let ref_size = ref_hi - ref_lo;
        self.push_undo("Align");
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for &idx in &indices {
                if let Some(obj) = l.objects.get_mut(idx)
                    && let Some((lo, hi)) = obj.bounds()
                {
                    let size = hi - lo;
                    let new_lo = match mode {
                        "left" => Vec2::new(ref_lo.x, lo.y),
                        "center_x" => Vec2::new(ref_lo.x + (ref_size.x - size.x) * 0.5, lo.y),
                        "right" => Vec2::new(ref_hi.x - size.x, lo.y),
                        "top" => Vec2::new(lo.x, ref_lo.y),
                        "center_y" => Vec2::new(lo.x, ref_lo.y + (ref_size.y - size.y) * 0.5),
                        "bottom" => Vec2::new(lo.x, ref_hi.y - size.y),
                        _ => lo,
                    };
                    obj.set_bounds(new_lo, new_lo + size);
                }
            }
        }
        page.sync_state();
        self.status = format!(
            "Aligned {} object(s) ({mode}, {})",
            indices.len(),
            if use_canvas {
                "Relative to Canvas"
            } else {
                "Relative to Object"
            }
        );
    }

    pub fn distribute_selected(&mut self, mode: &str) {
        let indices: Vec<usize> = if self.selected_set.is_empty() {
            self.selected.into_iter().collect()
        } else {
            self.selected_set.clone()
        };
        if indices.is_empty() {
            return;
        }
        if indices.len() == 1 {
            // Single selection falls back to canvas alignment for the corresponding axis
            let fallback = match mode {
                "top" => "top",
                "center_v" => "center_y",
                "bottom" => "bottom",
                "left" => "left",
                "center_h" => "center_x",
                "right" => "right",
                _ => "center_x",
            };
            self.align_selected_to_canvas(fallback);
            return;
        }
        let layer = self.selected_layer;
        let mut items: Vec<(usize, Vec2, Vec2)> = Vec::new();
        if let Some(l) = self.doc.page().layers.get(layer) {
            for &idx in &indices {
                if let Some((lo, hi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                    items.push((idx, lo, hi));
                }
            }
        }
        if items.len() < 2 {
            return;
        }
        let vertical = matches!(mode, "top" | "center_v" | "bottom");
        items.sort_by(|a, b| {
            let ka = if vertical { a.1.y } else { a.1.x };
            let kb = if vertical { b.1.y } else { b.1.x };
            ka.partial_cmp(&kb).unwrap_or(std::cmp::Ordering::Equal)
        });
        let (cw, ch) = (self.doc.page().width, self.doc.page().height);
        let first_val = match mode {
            "top" => {
                if self.align_to_canvas {
                    0.0
                } else {
                    items.first().map(|x| x.1.y).unwrap_or(0.0)
                }
            }
            "center_v" => {
                if self.align_to_canvas {
                    items.first().map(|x| (x.2.y - x.1.y) * 0.5).unwrap_or(0.0)
                } else {
                    items.first().map(|x| (x.1.y + x.2.y) * 0.5).unwrap_or(0.0)
                }
            }
            "bottom" => {
                if self.align_to_canvas {
                    items.first().map(|x| x.2.y - x.1.y).unwrap_or(0.0)
                } else {
                    items.first().map(|x| x.2.y).unwrap_or(0.0)
                }
            }
            "left" => {
                if self.align_to_canvas {
                    0.0
                } else {
                    items.first().map(|x| x.1.x).unwrap_or(0.0)
                }
            }
            "center_h" => {
                if self.align_to_canvas {
                    items.first().map(|x| (x.2.x - x.1.x) * 0.5).unwrap_or(0.0)
                } else {
                    items.first().map(|x| (x.1.x + x.2.x) * 0.5).unwrap_or(0.0)
                }
            }
            _ => {
                if self.align_to_canvas {
                    items.first().map(|x| x.2.x - x.1.x).unwrap_or(0.0)
                } else {
                    items.first().map(|x| x.2.x).unwrap_or(0.0)
                }
            }
        };
        let last_val = match mode {
            "top" => {
                if self.align_to_canvas {
                    ch - items.last().map(|x| x.2.y - x.1.y).unwrap_or(0.0)
                } else {
                    items.last().map(|x| x.1.y).unwrap_or(ch)
                }
            }
            "center_v" => {
                if self.align_to_canvas {
                    ch - items.last().map(|x| (x.2.y - x.1.y) * 0.5).unwrap_or(0.0)
                } else {
                    items.last().map(|x| (x.1.y + x.2.y) * 0.5).unwrap_or(ch)
                }
            }
            "bottom" => {
                if self.align_to_canvas {
                    ch
                } else {
                    items.last().map(|x| x.2.y).unwrap_or(ch)
                }
            }
            "left" => {
                if self.align_to_canvas {
                    cw - items.last().map(|x| x.2.x - x.1.x).unwrap_or(0.0)
                } else {
                    items.last().map(|x| x.1.x).unwrap_or(cw)
                }
            }
            "center_h" => {
                if self.align_to_canvas {
                    cw - items.last().map(|x| (x.2.x - x.1.x) * 0.5).unwrap_or(0.0)
                } else {
                    items.last().map(|x| (x.1.x + x.2.x) * 0.5).unwrap_or(cw)
                }
            }
            _ => {
                if self.align_to_canvas {
                    cw
                } else {
                    items.last().map(|x| x.2.x).unwrap_or(cw)
                }
            }
        };
        self.push_undo("Distribute");
        let n = (items.len() - 1).max(1) as f32;
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for (step, (idx, lo, hi)) in items.into_iter().enumerate() {
                let target = first_val + (last_val - first_val) * (step as f32 / n);
                let size = hi - lo;
                let new_lo = match mode {
                    "top" => Vec2::new(lo.x, target),
                    "center_v" => Vec2::new(lo.x, target - size.y * 0.5),
                    "bottom" => Vec2::new(lo.x, target - size.y),
                    "left" => Vec2::new(target, lo.y),
                    "center_h" => Vec2::new(target - size.x * 0.5, lo.y),
                    _ => Vec2::new(target - size.x, lo.y),
                };
                if let Some(obj) = l.objects.get_mut(idx) {
                    obj.set_bounds(new_lo, new_lo + size);
                }
            }
        }
        page.sync_state();
        self.status = format!("Distributed selection ({mode})");
    }

    pub fn match_canvas_dimension(&mut self, match_w: bool, match_h: bool) {
        let indices: Vec<usize> = if self.selected_set.is_empty() {
            self.selected.into_iter().collect()
        } else {
            self.selected_set.clone()
        };
        if indices.is_empty() {
            return;
        }
        let layer = self.selected_layer;
        let (cw, ch) = (self.doc.page().width, self.doc.page().height);
        let use_canvas = self.align_to_canvas || indices.len() == 1;
        let (target_w, target_h) = if use_canvas {
            (cw, ch)
        } else {
            let mut max_w = 1.0f32;
            let mut max_h = 1.0f32;
            if let Some(l) = self.doc.page().layers.get(layer) {
                for &idx in &indices {
                    if let Some((lo, hi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                        max_w = max_w.max(hi.x - lo.x);
                        max_h = max_h.max(hi.y - lo.y);
                    }
                }
            }
            (max_w, max_h)
        };
        self.push_undo("Match Size");
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for &idx in &indices {
                if let Some(obj) = l.objects.get_mut(idx)
                    && let Some((lo, hi)) = obj.bounds()
                {
                    let new_lo = Vec2::new(
                        if match_w && use_canvas { 0.0 } else { lo.x },
                        if match_h && use_canvas { 0.0 } else { lo.y },
                    );
                    let new_hi = Vec2::new(
                        if match_w { new_lo.x + target_w } else { hi.x },
                        if match_h { new_lo.y + target_h } else { hi.y },
                    );
                    obj.set_bounds(new_lo, new_hi);
                }
            }
        }
        page.sync_state();
        self.status = format!(
            "Matched size ({})",
            if use_canvas {
                "Relative to Canvas"
            } else {
                "Relative to Largest Object"
            }
        );
    }

    pub fn space_selected(&mut self, horizontal: bool) {
        let indices: Vec<usize> = if self.selected_set.is_empty() {
            self.selected.into_iter().collect()
        } else {
            self.selected_set.clone()
        };
        if indices.len() < 2 {
            self.distribute_selected(if horizontal { "center_h" } else { "center_v" });
            return;
        }
        let fixed_gap: Option<f32> = self.space_mode.trim_end_matches(" px").parse::<f32>().ok();
        let layer = self.selected_layer;
        let mut items: Vec<(usize, Vec2, Vec2)> = Vec::new();
        if let Some(l) = self.doc.page().layers.get(layer) {
            for &idx in &indices {
                if let Some((lo, hi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                    items.push((idx, lo, hi));
                }
            }
        }
        if items.len() < 2 {
            return;
        }
        items.sort_by(|a, b| {
            let ka = if horizontal { a.1.x } else { a.1.y };
            let kb = if horizontal { b.1.x } else { b.1.y };
            ka.partial_cmp(&kb).unwrap_or(std::cmp::Ordering::Equal)
        });
        let gap = if let Some(g) = fixed_gap {
            g
        } else {
            let total_span = if horizontal {
                items.last().map(|x| x.2.x).unwrap_or(0.0)
                    - items.first().map(|x| x.1.x).unwrap_or(0.0)
            } else {
                items.last().map(|x| x.2.y).unwrap_or(0.0)
                    - items.first().map(|x| x.1.y).unwrap_or(0.0)
            };
            let sum_sizes: f32 = items
                .iter()
                .map(|(_, lo, hi)| if horizontal { hi.x - lo.x } else { hi.y - lo.y })
                .sum();
            ((total_span - sum_sizes) / (items.len() - 1) as f32).max(0.0)
        };
        self.push_undo("Space Objects");
        let mut cursor = if horizontal {
            items[0].1.x
        } else {
            items[0].1.y
        };
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for (idx, lo, hi) in items {
                let size = hi - lo;
                let new_lo = if horizontal {
                    Vec2::new(cursor, lo.y)
                } else {
                    Vec2::new(lo.x, cursor)
                };
                if let Some(obj) = l.objects.get_mut(idx) {
                    obj.set_bounds(new_lo, new_lo + size);
                }
                cursor += if horizontal { size.x } else { size.y } + gap;
            }
        }
        page.sync_state();
        self.status = format!(
            "Spaced objects {} ({})",
            if horizontal {
                "horizontally"
            } else {
                "vertically"
            },
            self.space_mode
        );
    }

    pub fn reorder_selected_step(&mut self, forward: bool) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        let len = self
            .doc
            .page()
            .layers
            .get(layer)
            .map(|l| l.objects.len())
            .unwrap_or(0);
        if len < 2 {
            return;
        }
        let target = if forward {
            (index + 1).min(len - 1)
        } else {
            index.saturating_sub(1)
        };
        if target == index {
            return;
        }
        self.push_undo(if forward {
            "Bring Forward"
        } else {
            "Send Backward"
        });
        let page = self.doc.page_mut();
        page.layers[layer].objects.swap(index, target);
        page.sync_state();
        self.select_single(Some(target));
        self.status = if forward {
            "Brought selection forward one level".to_string()
        } else {
            "Sent selection backward one level".to_string()
        };
    }

    pub fn rotate_ccw_90(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        let Some((lo, hi)) = obj.bounds() else {
            return;
        };
        self.push_undo("Rotate 90° CCW");
        let center = (lo + hi) * 0.5;
        let half = Vec2::new((hi.y - lo.y) * 0.5, (hi.x - lo.x) * 0.5);
        self.set_selected_bounds(center - half, center + half);
        self.status = "Rotated selection 90° counter-clockwise".to_string();
    }

    pub fn scale_selected(&mut self, factor: f32) {
        if let Some((lo, hi)) = self.selected_object().and_then(|o| o.bounds()) {
            self.push_undo("Scale");
            let center = (lo + hi) * 0.5;
            let half = (hi - lo) * 0.5 * factor;
            self.set_selected_bounds(center - half, center + half);
            self.status = format!("Scaled selection by {:.0}%", factor * 100.0);
        }
    }

    pub fn flip_horizontal(&mut self) {
        self.mirror_selected(true);
    }

    pub fn flip_vertical(&mut self) {
        self.mirror_selected(false);
    }

    fn mirror_obj_around(obj: &mut Obj, axis: f32, horizontal: bool) {
        let reflect_pt = |p: &mut Vec2| {
            if horizontal {
                p.x = axis - p.x;
            } else {
                p.y = axis - p.y;
            }
        };
        match &mut obj.kind {
            Shape::Rect { min, max, .. } | Shape::Slice { min, max, .. } => {
                reflect_pt(min);
                reflect_pt(max);
                let nlo = min.min(*max);
                let nhi = min.max(*max);
                *min = nlo;
                *max = nhi;
            }
            Shape::Gradient {
                min, max, from, to, ..
            } => {
                reflect_pt(min);
                reflect_pt(max);
                let nlo = min.min(*max);
                let nhi = min.max(*max);
                *min = nlo;
                *max = nhi;
                std::mem::swap(from, to);
            }
            Shape::Ellipse { center, .. } => reflect_pt(center),
            Shape::Polygon {
                center, angle_rad, ..
            } => {
                reflect_pt(center);
                *angle_rad = if horizontal {
                    -*angle_rad
                } else {
                    std::f32::consts::PI - *angle_rad
                };
            }
            Shape::Line { a, b } => {
                reflect_pt(a);
                reflect_pt(b);
            }
            Shape::Path { points, .. } => {
                for p in points.iter_mut() {
                    reflect_pt(p);
                }
            }
            Shape::Bitmap { at, size, pixels } => {
                if horizontal {
                    at.x = axis - (at.x + size.x);
                } else {
                    at.y = axis - (at.y + size.y);
                }
                let w = size.x.max(1.0) as usize;
                let h = size.y.max(1.0) as usize;
                if pixels.len() >= w * h * 4 {
                    if horizontal {
                        for row in 0..h {
                            for col in 0..(w / 2) {
                                let a = (row * w + col) * 4;
                                let b = (row * w + (w - 1 - col)) * 4;
                                for c in 0..4 {
                                    pixels.swap(a + c, b + c);
                                }
                            }
                        }
                    } else {
                        for row in 0..(h / 2) {
                            for col in 0..w {
                                let a = (row * w + col) * 4;
                                let b = ((h - 1 - row) * w + col) * 4;
                                for c in 0..4 {
                                    pixels.swap(a + c, b + c);
                                }
                            }
                        }
                    }
                }
            }
            Shape::Text { .. } | Shape::SymbolInstance { .. } => {
                if let Some((lo, hi)) = obj.bounds() {
                    let size = hi - lo;
                    let new_lo = Vec2::new(
                        if horizontal {
                            axis - (lo.x + size.x)
                        } else {
                            lo.x
                        },
                        if horizontal {
                            lo.y
                        } else {
                            axis - (lo.y + size.y)
                        },
                    );
                    obj.translate(new_lo - lo);
                }
            }
            Shape::Group { children } => {
                for ch in children.iter_mut() {
                    Self::mirror_obj_around(ch, axis, horizontal);
                }
            }
        }
    }

    fn mirror_selected(&mut self, horizontal: bool) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        indices.sort_unstable();
        indices.dedup();
        if indices.is_empty() {
            return;
        }
        let layer = self.selected_layer;
        let mut lo = Vec2::splat(f32::MAX);
        let mut hi = Vec2::splat(f32::MIN);
        let mut any = false;
        if let Some(l) = self.doc.page().layers.get(layer) {
            for &idx in &indices {
                if let Some((olo, ohi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                    lo = lo.min(olo);
                    hi = hi.max(ohi);
                    any = true;
                }
            }
        }
        if !any {
            return;
        }
        self.push_undo("Flip");
        let axis = if horizontal { lo.x + hi.x } else { lo.y + hi.y };
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for idx in indices {
                if let Some(obj) = l.objects.get_mut(idx) {
                    Self::mirror_obj_around(obj, axis, horizontal);
                }
            }
        }
        page.sync_state();
        self.textures.clear();
        self.doc.dirty = true;
        self.status = if horizontal {
            "Flipped horizontally".to_string()
        } else {
            "Flipped vertically".to_string()
        };
    }

    pub fn rotate_90(&mut self) {
        let Some((lo, hi)) = self.selected_object().and_then(|o| o.bounds()) else {
            return;
        };
        let centre = (lo + hi) / 2.0;
        self.push_undo("Rotate 90°");
        let rotate = |p: &mut Vec2| {
            let d = *p - centre;
            *p = centre + Vec2::new(d.y, -d.x);
        };
        self.with_selected_mut(|obj| match &mut obj.kind {
            Shape::Rect { min, max, .. }
            | Shape::Gradient { min, max, .. }
            | Shape::Slice { min, max, .. } => {
                rotate(min);
                rotate(max);
            }
            Shape::Ellipse { center, radius, .. } => {
                rotate(center);
                std::mem::swap(&mut radius.x, &mut radius.y);
            }
            Shape::Polygon { angle_rad, .. } => {
                *angle_rad += std::f32::consts::FRAC_PI_2;
            }
            Shape::Line { a, b } => {
                rotate(a);
                rotate(b);
            }
            Shape::Path { points, .. } => {
                for p in points.iter_mut() {
                    rotate(p);
                }
            }
            Shape::Text { at, .. }
            | Shape::Bitmap { at, .. }
            | Shape::SymbolInstance { at, .. } => rotate(at),
            Shape::Group { children } => {
                for ch in children.iter_mut() {
                    if let Some((clo, chi)) = ch.bounds() {
                        let mut a = clo;
                        let mut b = chi;
                        rotate(&mut a);
                        rotate(&mut b);
                        ch.set_bounds(a.min(b), a.max(b));
                    }
                }
            }
        });
        self.textures.clear();
        self.status = "Rotated 90°".to_string();
    }

    pub fn reorder_selected(&mut self, to_front: bool) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        self.push_undo("Reorder");
        let page = self.doc.page_mut();
        let Some(objects) = page.layers.get_mut(layer).map(|l| &mut l.objects) else {
            return;
        };
        if index >= objects.len() {
            return;
        }
        let obj = objects.remove(index);
        let new_index = if to_front {
            objects.push(obj);
            objects.len() - 1
        } else {
            objects.insert(0, obj);
            0
        };
        page.sync_state();
        self.selected = Some(new_index);
        self.status = if to_front {
            "Brought to front".to_string()
        } else {
            "Sent to back".to_string()
        };
    }

    // ---- View & Theme ------------------------------------------------------
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(app: &mut FireCraft, lo: Vec2, hi: Vec2) -> usize {
        app.push_object(Obj::new(
            Shape::Rect {
                min: lo,
                max: hi,
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::WHITE),
            None,
        ))
    }

    fn bounds(app: &FireCraft, index: usize) -> (Vec2, Vec2) {
        app.doc.page().layers[app.selected_layer].objects[index]
            .bounds()
            .expect("test rectangle has bounds")
    }

    fn close(actual: f32, expected: f32) {
        assert!((actual - expected).abs() < 0.001, "{actual} != {expected}");
    }

    #[test]
    fn align_multiple_objects_uses_selection_bounds_and_undo_restores_positions() {
        let mut app = FireCraft::new(None);
        let first = rect(&mut app, Vec2::new(20.0, 30.0), Vec2::new(60.0, 50.0));
        let second = rect(&mut app, Vec2::new(120.0, 90.0), Vec2::new(180.0, 120.0));
        app.selected_set = vec![first, second];
        app.align_to_canvas = false;

        app.align_selected_to_canvas("right");

        close(bounds(&app, first).0.x, 140.0);
        close(bounds(&app, second).0.x, 120.0);
        assert_eq!(
            app.status,
            "Aligned 2 object(s) (right, Relative to Object)"
        );
        app.undo_action();
        close(bounds(&app, first).0.x, 20.0);
        close(bounds(&app, second).0.x, 120.0);
    }

    #[test]
    fn single_object_alignment_is_relative_to_canvas_and_empty_selection_is_noop() {
        let mut app = FireCraft::new(None);
        app.align_selected_to_canvas("left");
        assert!(app.undo.is_empty());
        let index = rect(&mut app, Vec2::new(25.0, 30.0), Vec2::new(75.0, 55.0));
        app.select_single(Some(index));

        app.align_selected_to_canvas("center_x");

        close(bounds(&app, index).0.x, (app.doc.page().width - 50.0) / 2.0);
        assert!(app.status.contains("Relative to Canvas"));
    }

    #[test]
    fn distribute_sorts_by_axis_and_single_selection_falls_back_to_alignment() {
        let mut app = FireCraft::new(None);
        let a = rect(&mut app, Vec2::new(200.0, 10.0), Vec2::new(220.0, 30.0));
        let b = rect(&mut app, Vec2::new(20.0, 10.0), Vec2::new(40.0, 30.0));
        let c = rect(&mut app, Vec2::new(100.0, 10.0), Vec2::new(120.0, 30.0));
        app.selected_set = vec![a, b, c];
        app.align_to_canvas = false;

        app.distribute_selected("center_h");

        close(bounds(&app, b).0.x, 20.0);
        close(bounds(&app, c).0.x, 110.0);
        close(bounds(&app, a).0.x, 200.0);
        app.select_single(Some(c));
        app.distribute_selected("right");
        close(bounds(&app, c).0.x, app.doc.page().width - 20.0);
    }

    #[test]
    fn match_size_uses_largest_selected_object_and_preserves_unmatched_dimension() {
        let mut app = FireCraft::new(None);
        let a = rect(&mut app, Vec2::new(10.0, 15.0), Vec2::new(30.0, 45.0));
        let b = rect(&mut app, Vec2::new(80.0, 60.0), Vec2::new(130.0, 75.0));
        app.selected_set = vec![a, b];
        app.align_to_canvas = false;

        app.match_canvas_dimension(true, false);

        let (lo, hi) = bounds(&app, a);
        close(hi.x - lo.x, 50.0);
        close(hi.y - lo.y, 30.0);
        assert!(app.status.contains("Largest Object"));
    }

    #[test]
    fn fixed_spacing_repositions_objects_in_order_without_changing_sizes() {
        let mut app = FireCraft::new(None);
        let a = rect(&mut app, Vec2::new(100.0, 20.0), Vec2::new(120.0, 40.0));
        let b = rect(&mut app, Vec2::new(10.0, 30.0), Vec2::new(20.0, 40.0));
        app.selected_set = vec![a, b];
        app.space_mode = "15 px".to_string();

        app.space_selected(true);

        close(bounds(&app, b).0.x, 10.0);
        close(bounds(&app, a).0.x, 35.0);
        close(bounds(&app, a).1.x - bounds(&app, a).0.x, 20.0);
        assert!(app.status.contains("horizontally"));
    }

    #[test]
    fn reorder_commands_respect_stack_boundaries_and_update_selection() {
        let mut app = FireCraft::new(None);
        let a = rect(&mut app, Vec2::ZERO, Vec2::splat(10.0));
        let b = rect(&mut app, Vec2::splat(20.0), Vec2::splat(30.0));
        app.select_single(Some(a));
        app.reorder_selected_step(false);
        assert!(app.undo.is_empty());
        app.reorder_selected_step(true);
        assert_eq!(app.selected, Some(1));
        assert_eq!(
            app.doc.page().layers[app.selected_layer].objects[1].name,
            "Rectangle"
        );
        app.undo_action();
        assert_eq!(app.doc.page().layers[app.selected_layer].objects.len(), 2);
        app.select_single(Some(b));
        app.reorder_selected(true);
        assert_eq!(app.selected, Some(1));
        app.reorder_selected(false);
        assert_eq!(app.selected, Some(0));
    }

    #[test]
    fn scale_rotate_and_flip_keep_expected_geometry_and_report_actions() {
        let mut app = FireCraft::new(None);
        let index = rect(&mut app, Vec2::new(10.0, 20.0), Vec2::new(30.0, 60.0));
        app.select_single(Some(index));
        app.scale_selected(2.0);
        let (lo, hi) = bounds(&app, index);
        close(hi.x - lo.x, 40.0);
        close(hi.y - lo.y, 80.0);
        app.rotate_ccw_90();
        let (lo, hi) = bounds(&app, index);
        close(hi.x - lo.x, 80.0);
        close(hi.y - lo.y, 40.0);
        app.flip_horizontal();
        assert_eq!(app.status, "Flipped horizontally");
        app.rotate_90();
        assert_eq!(app.status, "Rotated 90°");
    }

    #[test]
    fn flipping_bitmap_mirrors_pixel_rows_and_columns() {
        let mut app = FireCraft::new(None);
        let index = app.push_object(Obj::new(
            Shape::Bitmap {
                at: Vec2::new(4.0, 8.0),
                size: Vec2::new(2.0, 2.0),
                pixels: vec![1, 0, 0, 255, 2, 0, 0, 255, 3, 0, 0, 255, 4, 0, 0, 255],
            },
            None,
            None,
        ));
        app.select_single(Some(index));

        app.flip_horizontal();

        let Shape::Bitmap { pixels, .. } =
            &app.doc.page().layers[app.selected_layer].objects[index].kind
        else {
            panic!("bitmap shape should remain a bitmap");
        };
        assert_eq!(&pixels[..4], &[2, 0, 0, 255]);
        assert_eq!(&pixels[8..12], &[4, 0, 0, 255]);
        app.flip_vertical();
        let Shape::Bitmap { pixels, .. } =
            &app.doc.page().layers[app.selected_layer].objects[index].kind
        else {
            panic!("bitmap shape should remain a bitmap");
        };
        assert_eq!(&pixels[..4], &[4, 0, 0, 255]);
    }
}
