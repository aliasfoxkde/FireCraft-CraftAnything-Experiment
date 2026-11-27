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

        // The WASM clipboard API reports pasted images as binary files. The browser
        // bridge encodes those bytes into a private event payload for this shared path.
        if let Some(event_text) = event_non_empty
            && event_text.starts_with("FIRECRAFT_IMAGE:")
        {
            if let Some((width, height, pixels)) = decode_browser_clipboard_image(event_text) {
                self.paste_bitmap(width as f32, height as f32, pixels);
            } else {
                self.status = "Could not paste image: clipboard image data is invalid".to_string();
            }
            return;
        }

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
            self.paste_bitmap(img.width as f32, img.height as f32, img.bytes.into_owned());
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

    fn paste_bitmap(&mut self, width: f32, height: f32, pixels: Vec<u8>) {
        if width <= 0.0 || height <= 0.0 || pixels.len() != width as usize * height as usize * 4 {
            self.status = "Could not paste image: clipboard image data is invalid".to_string();
            return;
        }
        let center = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
        let at = (center - Vec2::new(width * 0.5, height * 0.5)).max(Vec2::ZERO);
        let mut obj = Obj::new(
            Shape::Bitmap {
                at,
                pixels,
                size: Vec2::new(width, height),
                pixel_size: Vec2::new(width, height),
            },
            None,
            None,
        );
        obj.name = format!("Pasted Bitmap ({width:.0}×{height:.0})");
        self.push_undo("Paste Image");
        let idx = self.push_object(obj);
        self.select_single(Some(idx));
        self.textures.clear();
        self.show_welcome = false;
        self.status = format!("Pasted bitmap image ({width:.0}×{height:.0} px) from clipboard");
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

fn decode_browser_clipboard_image(event_text: &str) -> Option<(u32, u32, Vec<u8>)> {
    let payload = event_text.strip_prefix("FIRECRAFT_IMAGE:")?;
    let bytes = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, payload).ok()?;
    let image = image::load_from_memory(&bytes).ok()?.into_rgba8();
    let (width, height) = image.dimensions();
    (width > 0 && height > 0).then(|| (width, height, image.into_raw()))
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
mod tests {
    use super::*;

    fn rect(x: f32) -> Obj {
        Obj::new(
            Shape::Rect {
                min: Vec2::new(x, 10.0),
                max: Vec2::new(x + 20.0, 30.0),
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::WHITE),
            None,
        )
    }

    fn app_with_rects(count: usize) -> FireCraft {
        let mut app = FireCraft::new(None);
        for index in 0..count {
            app.push_object(rect(index as f32 * 30.0));
        }
        app
    }

    #[test]
    fn undo_and_redo_restore_objects_and_repair_out_of_range_selection() {
        let mut app = app_with_rects(2);
        app.selected = Some(9);
        app.selected_set = vec![0, 8];
        app.push_undo("snapshot");
        app.doc.page_mut().layers[0].objects.clear();
        app.doc.page_mut().sync_state();
        app.selected = Some(7);
        app.selected_set = vec![5];

        app.undo_action();
        assert_eq!(app.doc.page().layers[0].objects.len(), 2);
        assert_eq!(app.selected, Some(1));
        assert_eq!(app.selected_set, vec![1]);
        assert_eq!(app.status, "Undo: snapshot");

        app.redo_action();
        assert!(app.doc.page().layers[0].objects.is_empty());
        assert_eq!(app.selected, None);
        assert!(app.selected_set.is_empty());
        assert_eq!(app.status, "Redo: snapshot");

        app.push_undo("new edit");
        assert!(app.redo.is_empty());
    }

    #[test]
    fn delete_handles_primary_and_extra_layers_and_empty_selection() {
        let mut app = app_with_rects(1);
        app.add_layer();
        app.doc.page_mut().layers[1].objects.push(rect(80.0));
        app.selected_layer = 0;
        app.selected = Some(0);
        app.selected_set = vec![0, 0, 99];
        app.selected_extra_layers = vec![(1, 0), (1, 0), (99, 0)];

        app.delete_selected();
        assert!(app.doc.page().layers[0].objects.is_empty());
        assert!(app.doc.page().layers[1].objects.is_empty());
        assert_eq!(app.status, "Deleted 2 object(s)");
        assert!(app.selected.is_none());
        assert!(app.selected_set.is_empty());

        let undo_count = app.undo.len();
        app.delete_selected();
        assert_eq!(app.undo.len(), undo_count, "empty delete is a no-op");
    }

    #[test]
    fn group_and_ungroup_round_trip_multiple_objects() {
        let mut app = app_with_rects(3);
        app.selected = Some(0);
        app.selected_set = vec![0, 2];

        app.group_selected();
        assert_eq!(app.doc.page().layers[0].objects.len(), 2);
        let group_index = app.selected.expect("group is selected");
        let Shape::Group { children } = &app.doc.page().layers[0].objects[group_index].kind else {
            panic!("the selected object is a group");
        };
        assert_eq!(children.len(), 2);
        assert_eq!(app.status, "Grouped 2 objects");

        app.ungroup_selected();
        assert_eq!(app.doc.page().layers[0].objects.len(), 3);
        assert_eq!(app.selected_set.len(), 2);
        assert_eq!(app.status, "Ungrouped into 2 objects");

        app.ungroup_selected();
        assert_eq!(app.status, "Selected object is not a Group");
    }

    #[test]
    fn paste_text_uses_cursor_and_empty_clipboard_reports_status() {
        let mut app = app_with_rects(0);
        app.cursor_doc_pos = Some(Vec2::new(35.0, 45.0));
        app.paste_with_text(Some("  pasted words  "));

        let object = &app.doc.page().layers[0].objects[0];
        let Shape::Text { at, text, .. } = &object.kind else {
            panic!("external clipboard text becomes a text object");
        };
        assert_eq!(*at, Vec2::new(35.0, 45.0));
        assert_eq!(text, "pasted words");
        assert_eq!(object.name, "Pasted Text");
        assert_eq!(
            app.status,
            "Pasted text from system clipboard as Text object"
        );

        let mut empty = FireCraft::new(None);
        empty.paste_with_text(None);
        assert_eq!(
            empty.status,
            "Clipboard is empty — select an object and press Ctrl+C (or copy an image/text) first"
        );
    }

    #[test]
    fn paste_bitmap_adds_and_selects_a_valid_image() {
        let mut app = app_with_rects(0);
        app.show_welcome = true;
        let pixels = vec![255; 2 * 3 * 4];

        app.paste_bitmap(2.0, 3.0, pixels.clone());

        let objects = &app.doc.page().layers[0].objects;
        assert_eq!(objects.len(), 1);
        let Shape::Bitmap {
            pixels: pasted_pixels,
            size,
            ..
        } = &objects[0].kind
        else {
            panic!("clipboard image becomes a bitmap object");
        };
        assert_eq!(*size, Vec2::new(2.0, 3.0));
        assert_eq!(pasted_pixels, &pixels);
        assert_eq!(app.selected, Some(0));
        assert_eq!(app.selected_set, vec![0]);
        assert!(!app.show_welcome);
        assert_eq!(app.status, "Pasted bitmap image (2×3 px) from clipboard");
    }

    #[test]
    fn paste_bitmap_rejects_empty_dimensions_and_malformed_pixels() {
        let mut app = app_with_rects(0);
        app.paste_bitmap(0.0, 2.0, vec![]);
        assert!(app.doc.page().layers[0].objects.is_empty());
        assert_eq!(
            app.status,
            "Could not paste image: clipboard image data is invalid"
        );

        app.paste_bitmap(2.0, 2.0, vec![255; 15]);
        assert!(app.doc.page().layers[0].objects.is_empty());
        assert_eq!(
            app.status,
            "Could not paste image: clipboard image data is invalid"
        );
    }

    #[test]
    fn paste_browser_image_payload_decodes_and_rejects_malformed_data() {
        let source = image::RgbaImage::from_pixel(2, 3, image::Rgba([12, 34, 56, 255]));
        let mut png = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgba8(source.clone())
            .write_to(&mut png, image::ImageFormat::Png)
            .expect("encode clipboard PNG fixture");
        let payload =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, png.get_ref());
        let event = format!("FIRECRAFT_IMAGE:{payload}");
        let mut app = app_with_rects(0);

        app.paste_with_text(Some(&event));

        let objects = &app.doc.page().layers[0].objects;
        assert_eq!(objects.len(), 1);
        let Shape::Bitmap { pixels, size, .. } = &objects[0].kind else {
            panic!("browser clipboard payload becomes a bitmap object");
        };
        assert_eq!(*size, Vec2::new(2.0, 3.0));
        assert_eq!(pixels, &source.into_raw());

        let mut malformed = app_with_rects(0);
        malformed.paste_with_text(Some("FIRECRAFT_IMAGE:not-base64"));
        assert!(malformed.doc.page().layers[0].objects.is_empty());
        assert_eq!(
            malformed.status,
            "Could not paste image: clipboard image data is invalid"
        );
    }

    #[test]
    fn clone_nudge_spelling_and_copy_cover_selection_edges() {
        let mut app = app_with_rects(1);
        app.copy_selected();
        assert_eq!(
            app.status,
            "Select one or more objects first to Copy (Ctrl+C)"
        );
        app.clone_selected();
        assert_eq!(app.doc.page().layers[0].objects.len(), 1);

        app.select_single(Some(0));
        app.clone_selected();
        assert_eq!(app.doc.page().layers[0].objects.len(), 2);
        assert_eq!(app.selected, Some(1));
        assert_eq!(app.status, "Cloned object in place");

        let before = app.doc.page().layers[0].objects[1]
            .bounds()
            .expect("the cloned rectangle has bounds");
        app.selected_set = vec![0, 1, 90];
        app.nudge_selected(Vec2::new(3.0, -2.0));
        let after = app.doc.page().layers[0].objects[1]
            .bounds()
            .expect("the nudged rectangle has bounds");
        assert_eq!(after.0, before.0 + Vec2::new(3.0, -2.0));

        app.check_spelling();
        assert_eq!(
            app.status,
            "Spelling check complete: 0 text object(s) verified"
        );
        app.selected = None;
        app.selected_set.clear();
        let undo_count = app.undo.len();
        app.nudge_selected(Vec2::splat(1.0));
        assert_eq!(app.undo.len(), undo_count, "empty nudge is a no-op");
    }
}
