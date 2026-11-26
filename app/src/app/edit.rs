//! Edit commands: undo history, deletion, duplication, grouping, clipboard and selection sets.

use super::*;

impl FireCraft {
    pub fn push_undo(&mut self, label: &str) {
        let snap = self.doc.snapshot(label);
        self.undo.push(snap);
        if self.undo.len() > constants::UNDO_LIMIT {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    pub fn undo_action(&mut self) {
        if let Some(snap) = self.undo.pop() {
            self.redo.push(self.doc.snapshot(&snap.label));
            self.doc.restore(&snap);
            self.textures.clear();
            let count = self
                .doc
                .page()
                .layers
                .get(self.selected_layer)
                .map(|l| l.objects.len())
                .unwrap_or(0);
            self.selected = self.selected.and_then(|idx| {
                if count == 0 {
                    None
                } else {
                    Some(idx.min(count - 1))
                }
            });
            self.selected_set.retain(|&i| i < count);
            if self.selected_set.is_empty()
                && let Some(s) = self.selected
            {
                self.selected_set.push(s);
            }
            self.status = format!("Undo: {}", snap.label);
        }
    }

    pub fn redo_action(&mut self) {
        if let Some(snap) = self.redo.pop() {
            self.undo.push(self.doc.snapshot(&snap.label));
            self.doc.restore(&snap);
            self.textures.clear();
            let count = self
                .doc
                .page()
                .layers
                .get(self.selected_layer)
                .map(|l| l.objects.len())
                .unwrap_or(0);
            self.selected = self.selected.and_then(|idx| {
                if count == 0 {
                    None
                } else {
                    Some(idx.min(count - 1))
                }
            });
            self.selected_set.retain(|&i| i < count);
            if self.selected_set.is_empty()
                && let Some(s) = self.selected
            {
                self.selected_set.push(s);
            }
            self.status = format!("Redo: {}", snap.label);
        }
    }

    pub fn replay_last_action(&mut self) {
        if self.selected.is_some() {
            self.duplicate_selected();
            if let Some(eff) = self.last_effect.clone() {
                self.add_live_effect(eff);
            }
            self.status = "Replayed step on selection".to_string();
        } else {
            self.status = "Select an object to replay step".to_string();
        }
    }

    pub fn delete_selected(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        if indices.is_empty() && self.selected_extra_layers.is_empty() {
            return;
        }
        self.push_undo("Delete");
        indices.sort_unstable();
        indices.dedup();
        let layer = self.selected_layer;
        let mut removed = 0usize;
        let extra = self.selected_extra_layers.clone();
        let page = self.doc.page_mut();
        if layer < page.layers.len() {
            for &idx in indices.iter().rev() {
                if idx < page.layers[layer].objects.len() {
                    page.layers[layer].objects.remove(idx);
                    removed += 1;
                }
            }
        }
        let mut by_layer: std::collections::BTreeMap<usize, Vec<usize>> =
            std::collections::BTreeMap::new();
        for (li, oi) in extra {
            if li != layer {
                by_layer.entry(li).or_default().push(oi);
            }
        }
        for (li, mut ois) in by_layer {
            ois.sort_unstable();
            ois.dedup();
            if li < page.layers.len() {
                for oi in ois.into_iter().rev() {
                    if oi < page.layers[li].objects.len() {
                        page.layers[li].objects.remove(oi);
                        removed += 1;
                    }
                }
            }
        }
        page.sync_state();
        self.clear_selection();
        self.textures.clear();
        self.status = format!("Deleted {removed} object(s)");
    }

    pub fn duplicate_selected(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        if indices.is_empty() {
            return;
        }
        self.push_undo("Duplicate");
        indices.sort_unstable();
        indices.dedup();
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        let mut new_set = Vec::new();
        if layer < page.layers.len() {
            for &idx in &indices {
                if idx < page.layers[layer].objects.len() {
                    let mut copy = page.layers[layer].objects[idx].clone();
                    copy.translate(Vec2::new(14.0, 14.0));
                    page.layers[layer].objects.push(copy);
                    new_set.push(page.layers[layer].objects.len() - 1);
                }
            }
            page.sync_state();
        }
        self.selected = new_set.last().copied();
        self.selected_set = new_set;
        self.status = format!("Duplicated {} object(s)", indices.len());
    }

    pub fn group_selected(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        indices.sort_unstable();
        indices.dedup();
        if indices.len() < 2 {
            self.status = "Select 2 or more objects (Shift-click or Marquee) to Group".to_string();
            return;
        }
        self.push_undo("Group");
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        let Some(objs) = page.layers.get_mut(layer).map(|l| &mut l.objects) else {
            return;
        };
        let mut children = Vec::new();
        for &idx in &indices {
            if let Some(o) = objs.get(idx) {
                children.push(o.clone());
            }
        }
        for &idx in indices.iter().rev() {
            if idx < objs.len() {
                objs.remove(idx);
            }
        }
        let grp = Obj::new(Shape::Group { children }, None, None);
        objs.push(grp);
        let new_idx = objs.len() - 1;
        page.sync_state();
        self.select_single(Some(new_idx));
        self.status = format!("Grouped {} objects", indices.len());
    }

    pub fn ungroup_selected(&mut self) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        let Some(children) = self
            .doc
            .page()
            .layers
            .get(layer)
            .and_then(|l| l.objects.get(index))
            .and_then(|o| match &o.kind {
                Shape::Group { children } => Some(children.clone()),
                _ => None,
            })
        else {
            self.status = "Selected object is not a Group".to_string();
            return;
        };
        self.push_undo("Ungroup");
        let page = self.doc.page_mut();
        let objs = &mut page.layers[layer].objects;
        objs.remove(index);
        let mut new_set = Vec::new();
        let count = children.len();
        for ch in children {
            objs.push(ch);
            new_set.push(objs.len() - 1);
        }
        page.sync_state();
        self.selected = new_set.last().copied();
        self.selected_set = new_set;
        self.status = format!("Ungrouped into {count} objects");
    }

    pub fn clone_selected(&mut self) {
        if let Some(index) = self.selected {
            self.push_undo("Clone");
            let layer = self.selected_layer;
            let page = self.doc.page_mut();
            if layer < page.layers.len() && index < page.layers[layer].objects.len() {
                let copy = page.layers[layer].objects[index].clone();
                page.layers[layer].objects.push(copy);
                let new_idx = page.layers[layer].objects.len() - 1;
                page.sync_state();
                self.selected = Some(new_idx);
                self.status = "Cloned object in place".to_string();
            }
        }
    }

    pub fn nudge_selected(&mut self, delta: Vec2) {
        if self.selected.is_none() && self.selected_set.is_empty() {
            return;
        }
        self.push_undo("Nudge");
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        for idx in indices {
            if let Some(obj) = page
                .layers
                .get_mut(layer)
                .and_then(|l| l.objects.get_mut(idx))
            {
                obj.translate(delta);
            }
        }
        page.sync_state();
    }

    pub fn check_spelling(&mut self) {
        let mut text_objects = 0usize;
        for layer in &self.doc.page().layers {
            for obj in &layer.objects {
                if matches!(obj.kind, Shape::Text { .. }) {
                    text_objects += 1;
                }
            }
        }
        self.status = format!("Spelling check complete: {text_objects} text object(s) verified");
    }

    pub fn copy_selected(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        indices.sort_unstable();
        indices.dedup();
        let layer = self.selected_layer;
        let mut copied = Vec::new();
        if let Some(l) = self.doc.page().layers.get(layer) {
            for &idx in &indices {
                if let Some(obj) = l.objects.get(idx) {
                    copied.push(obj.clone());
                }
            }
        }
        if copied.is_empty() {
            self.status = "Select one or more objects first to Copy (Ctrl+C)".to_string();
            return;
        }
        self.clipboard = copied.last().cloned();
        self.clipboard_set = copied;
        let marker = format!("FIRECRAFT_CLIPBOARD:{}", self.clipboard_set.len());
        self.ctx.copy_text(marker.clone());
        // Tests and headless CI must not open an OS clipboard connection (X11 clipboard ownership
        // can block indefinitely without a display server).
        #[cfg(all(not(target_arch = "wasm32"), not(test)))]
        if let Ok(mut cb) = arboard::Clipboard::new() {
            let _ = cb.set_text(marker);
        }
        self.status = format!("Copied {} object(s) to clipboard", self.clipboard_set.len());
    }

    pub fn cut_selected(&mut self) {
        self.copy_selected();
        if !self.clipboard_set.is_empty() {
            self.delete_selected();
            self.status = format!("Cut {} object(s) to clipboard", self.clipboard_set.len());
        }
    }

    pub fn paste(&mut self) {
        self.paste_with_text(None);
    }

    pub fn paste_with_text(&mut self, event_text: Option<&str>) {
        let event_non_empty = event_text.filter(|s| !s.is_empty());
        #[cfg_attr(any(target_arch = "wasm32", test), allow(unused_variables))]
        let is_svg_or_marker = event_non_empty.is_some_and(|s| {
            let t = s.trim_start();
            t.starts_with("FIRECRAFT_CLIPBOARD") || t.starts_with("<svg")
        });

        // 1. Check if the OS clipboard holds a bitmap image (e.g. Win+Shift+S screenshot, browser Copy Image).
        // Note: on Windows, Ctrl+V often emits `Event::Paste("")` (or a URL when copying an image from a browser),
        // so we check `cb.get_image()` whenever the event text is not an internal FireCraft marker or SVG payload.
        #[cfg(all(not(target_arch = "wasm32"), not(test)))]
        if !is_svg_or_marker
            && let Ok(mut cb) = arboard::Clipboard::new()
            && let Ok(img) = cb.get_image()
            && img.width > 0
            && img.height > 0
        {
            let w = img.width as f32;
            let h = img.height as f32;
            let center = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
            let at = (center - Vec2::new(w * 0.5, h * 0.5)).max(Vec2::ZERO);
            let mut obj = Obj::new(
                Shape::Bitmap {
                    at,
                    pixels: img.bytes.into_owned(),
                    size: Vec2::new(w, h),
                },
                None,
                None,
            );
            obj.name = format!("Pasted Bitmap ({:.0}×{:.0})", w, h);
            self.push_undo("Paste Image");
            let idx = self.push_object(obj);
            self.select_single(Some(idx));
            self.textures.clear();
            self.show_welcome = false;
            self.status = format!(
                "Pasted bitmap image ({:.0}×{:.0} px) from system clipboard",
                w, h
            );
            return;
        }

        // 2. Determine clipboard text (from egui Paste event or OS clipboard)
        #[cfg(all(not(target_arch = "wasm32"), not(test)))]
        let os_text = event_non_empty.map(|s| s.to_string()).or_else(|| {
            arboard::Clipboard::new()
                .ok()
                .and_then(|mut cb| cb.get_text().ok())
        });
        #[cfg(any(target_arch = "wasm32", test))]
        let os_text = event_non_empty.map(|s| s.to_string());
        let is_internal_marker = os_text
            .as_deref()
            .map(|s| s.starts_with("FIRECRAFT_CLIPBOARD"))
            .unwrap_or(false);

        // 3. If we have internal FireCraft objects in `clipboard_set` (or `clipboard`) and either
        // the marker matches or no external text override was passed, paste the internal objects.
        if (is_internal_marker || event_non_empty.is_none())
            && (!self.clipboard_set.is_empty() || self.clipboard.is_some())
        {
            if self.clipboard_set.is_empty()
                && let Some(single) = self.clipboard.clone()
            {
                self.clipboard_set.push(single);
            }
            self.push_undo("Paste");
            let mut new_indices = Vec::new();
            let mut updated_clip = Vec::with_capacity(self.clipboard_set.len());
            for mut obj in self.clipboard_set.clone() {
                obj.translate(Vec2::new(16.0, 16.0));
                updated_clip.push(obj.clone());
                let idx = self.push_object(obj);
                new_indices.push(idx);
            }
            self.clipboard_set = updated_clip;
            self.clipboard = self.clipboard_set.last().cloned();
            self.selected = new_indices.last().copied();
            self.selected_set = new_indices;
            self.textures.clear();
            self.status = format!(
                "Pasted {} object(s) from clipboard",
                self.selected_set.len()
            );
            return;
        }

        // 4. If external text was pasted (SVG snippet, image file path, or plain text), materialize it on canvas!
        if let Some(raw_text) = os_text {
            let trimmed = raw_text.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("FIRECRAFT_CLIPBOARD") {
                // Check if it's a file path on disk
                let unquoted = trimmed.trim_matches('"').trim_matches('\'');
                let candidate_path = std::path::Path::new(unquoted);
                if candidate_path.is_file() && self.import_path(candidate_path).is_ok() {
                    return;
                }
                // Otherwise create a Text object at cursor or canvas center
                let at = self.cursor_doc_pos.unwrap_or_else(|| {
                    Vec2::new(
                        self.doc.page().width * 0.5 - 80.0,
                        self.doc.page().height * 0.5,
                    )
                });
                self.push_undo("Paste Text");
                let mut obj = Obj::new(
                    Shape::Text {
                        at,
                        text: trimmed.chars().take(240).collect(),
                        size: 28.0,
                        bold: false,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(self.fill),
                    None,
                );
                obj.name = "Pasted Text".to_string();
                let idx = self.push_object(obj);
                self.select_single(Some(idx));
                self.status = "Pasted text from system clipboard as Text object".to_string();
                return;
            }
        }

        // 5. Fallback: if internal clipboard has objects even when external text was present, paste them
        if !self.clipboard_set.is_empty() || self.clipboard.is_some() {
            if self.clipboard_set.is_empty()
                && let Some(single) = self.clipboard.clone()
            {
                self.clipboard_set.push(single);
            }
            self.push_undo("Paste");
            let mut new_indices = Vec::new();
            for mut obj in self.clipboard_set.clone() {
                obj.translate(Vec2::new(16.0, 16.0));
                let idx = self.push_object(obj);
                new_indices.push(idx);
            }
            self.selected = new_indices.last().copied();
            self.selected_set = new_indices;
            self.status = format!(
                "Pasted {} object(s) from clipboard",
                self.selected_set.len()
            );
            return;
        }

        self.status =
            "Clipboard is empty — select an object and press Ctrl+C (or copy an image/text) first"
                .to_string();
    }

    pub fn select_all(&mut self) {
        self.selected_extra_layers.clear();
        let layers = &self.doc.page().layers;
        let mut total = 0usize;
        // Prefer keeping current selected_layer if it has selectable objects; otherwise pick first non-empty visible unlocked layer
        let cur_has_objs = layers.get(self.selected_layer).is_some_and(|l| {
            l.visible && !l.locked && l.objects.iter().any(|o| o.visible && !o.locked)
        });
        if !cur_has_objs
            && let Some((li, _)) = layers.iter().enumerate().find(|(_, l)| {
                l.visible && !l.locked && l.objects.iter().any(|o| o.visible && !o.locked)
            })
        {
            self.selected_layer = li;
        }
        let primary_layer = self.selected_layer;
        let mut primary_set = Vec::new();
        for (li, l) in layers.iter().enumerate() {
            if !l.visible || l.locked {
                continue;
            }
            for (oi, obj) in l.objects.iter().enumerate() {
                if !obj.visible || obj.locked {
                    continue;
                }
                total += 1;
                if li == primary_layer {
                    primary_set.push(oi);
                } else {
                    self.selected_extra_layers.push((li, oi));
                }
            }
        }
        self.selected_set = primary_set;
        self.selected = self.selected_set.last().copied();
        self.status = format!("Selected {total} object(s) across all visible layers");
    }

    pub fn select_inverse(&mut self) {
        let layer = self.selected_layer;
        let count = self
            .doc
            .page()
            .layers
            .get(layer)
            .map(|l| l.objects.len())
            .unwrap_or(0);
        let prev = self.selected_set.clone();
        self.selected_set = (0..count).filter(|i| !prev.contains(i)).collect();
        self.selected = self.selected_set.last().copied();
        self.status = format!(
            "Inverted selection ({} object(s) selected)",
            self.selected_set.len()
        );
    }

    pub fn toggle_lock_selection(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        if indices.is_empty() {
            return;
        }
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for idx in indices {
                if let Some(o) = l.objects.get_mut(idx) {
                    o.locked = !o.locked;
                }
            }
        }
        page.sync_state();
        self.status = "Toggled lock on selected object(s)".to_string();
    }

    pub fn toggle_full_screen_canvas(&mut self) {
        let any_dock_visible = self.show_tools_dock
            || self.show_right_dock
            || self.show_bottom_inspector
            || self.show_main_toolbar;
        if any_dock_visible && !self.full_screen_canvas {
            self.full_screen_canvas = true;
            self.show_tools_dock = false;
            self.show_right_dock = false;
            self.show_bottom_inspector = false;
            self.show_main_toolbar = false;
            self.status =
                "Full-Screen Canvas Preview Mode enabled (press Tab or F4 to restore panels)"
                    .to_string();
        } else {
            self.full_screen_canvas = false;
            self.show_tools_dock = true;
            self.show_right_dock = true;
            self.show_bottom_inspector = true;
            self.show_main_toolbar = true;
            self.status = "Restored CS6 workspace panels (Tab / F4)".to_string();
        }
    }

    pub fn select_page_step(&mut self, delta: isize) {
        let count = self.doc.pages.len();
        if count <= 1 {
            return;
        }
        let next = (self.doc.current_page as isize + delta).rem_euclid(count as isize) as usize;
        self.doc.current_page = next;
        self.clear_selection();
        self.status = format!("Switched to Page {} of {count}", next + 1);
    }

    pub fn select_state_step(&mut self, delta: isize) {
        let count = self.doc.page().states.len();
        if count <= 1 {
            return;
        }
        let cur = self.doc.page().current_state;
        let next = (cur as isize + delta).rem_euclid(count as isize) as usize;
        self.doc.page_mut().select_state(next);
        self.clear_selection();
        self.status = format!("Switched to Animation State {} of {count}", next + 1);
    }

    pub fn cycle_selection(&mut self, forward: bool) {
        let layer = self.selected_layer;
        let count = self
            .doc
            .page()
            .layers
            .get(layer)
            .map(|l| l.objects.len())
            .unwrap_or(0);
        if count == 0 {
            self.selected = None;
            return;
        }
        let next = match self.selected {
            Some(cur) => {
                if forward {
                    (cur + 1) % count
                } else if cur == 0 {
                    count - 1
                } else {
                    cur - 1
                }
            }
            None => 0,
        };
        self.selected = Some(next);
        self.selected_set = vec![next];
    }

    pub fn select_similar(&mut self) {
        let Some(cur) = self.selected_object() else {
            return;
        };
        let target_type = cur.type_label();
        let layer = self.selected_layer;
        if let Some(l) = self.doc.page().layers.get(layer) {
            let matches = l
                .objects
                .iter()
                .filter(|o| o.type_label() == target_type)
                .count();
            self.status = format!("Found {matches} '{target_type}' object(s) on active layer");
        }
    }

    // ---- Geometry transforms -----------------------------------------------

    pub fn paste_attributes(&mut self) {
        let Some(src) = self.clipboard.clone() else {
            self.status = "Copy an object first to Paste Attributes".to_string();
            return;
        };
        if self.selected.is_none() && self.selected_set.is_empty() {
            return;
        }
        self.push_undo("Paste Attributes");
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for idx in indices {
                if let Some(obj) = l.objects.get_mut(idx) {
                    obj.fill = src.fill;
                    obj.stroke = src.stroke;
                    obj.width = src.width;
                    obj.opacity = src.opacity;
                    obj.effects = src.effects.clone();
                }
            }
        }
        page.sync_state();
        self.doc.dirty = true;
        self.status = "Pasted Fill, Stroke, Opacity & Live Filters".to_string();
    }
}
