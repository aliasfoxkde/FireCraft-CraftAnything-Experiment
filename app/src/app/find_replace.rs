//! Find & Replace: text/object-name search and exact-color replacement across every page.
//!
//! This module owns the matching semantics and replacement side effects so canvas, palette, and
//! dialog code can call one interface instead of reimplementing color walks.

use super::*;

impl FireCraft {
    /// Number of objects that currently match the active Find/Replace mode. This powers the live
    /// preview in the dialog, so users can see scope (all pages, hidden layers included) before
    /// committing a change.
    pub fn find_replace_match_count(&self) -> usize {
        if self.find_replace_mode == FindReplaceMode::Text && self.find_query.is_empty() {
            return 0;
        }
        self.doc
            .pages
            .iter()
            .flat_map(|p| p.layers.iter())
            .flat_map(|l| l.objects.iter())
            .filter(|o| self.object_matches_find(o))
            .count()
    }

    fn object_matches_find(&self, obj: &Obj) -> bool {
        match self.find_replace_mode {
            FindReplaceMode::Text => {
                object_matches_text(obj, &self.find_query, self.find_case_sensitive)
            }
            FindReplaceMode::FillColor => object_has_fill_color(obj, self.find_color),
            FindReplaceMode::StrokeColor => obj.stroke.is_some_and(|c| c == self.find_color),
            FindReplaceMode::AnyColor => object_has_any_color(obj, self.find_color),
        }
    }

    /// Selects the next matching object, cycling through pages. Color matches jump to the object
    /// itself so the user can inspect context before replacing.
    pub fn find_replace_find_next(&mut self) {
        if self.find_replace_mode == FindReplaceMode::Text && self.find_query.is_empty() {
            self.status = "Enter text to find first".to_string();
            return;
        }
        let mut matches: Vec<(usize, usize, usize)> = Vec::new();
        for (pi, page) in self.doc.pages.iter().enumerate() {
            for (li, layer) in page.layers.iter().enumerate() {
                for (oi, obj) in layer.objects.iter().enumerate() {
                    if self.object_matches_find(obj) {
                        matches.push((pi, li, oi));
                    }
                }
            }
        }
        if matches.is_empty() {
            self.status = "No matches found".to_string();
            return;
        }
        let current = self
            .selected
            .map(|oi| (self.doc.current_page, self.selected_layer, oi));
        let pos = current
            .and_then(|cur| matches.iter().position(|m| *m == cur))
            .map(|p| (p + 1) % matches.len())
            .unwrap_or(0);
        let (pi, li, oi) = matches[pos];
        if self.doc.current_page != pi {
            self.doc.current_page = pi;
            self.textures.clear();
        }
        self.selected_layer = li;
        self.select_single(Some(oi));
        let page_note = if self.doc.pages.len() > 1 {
            format!(" · page {}/{}", pi + 1, self.doc.pages.len())
        } else {
            String::new()
        };
        self.status = format!("Match {}/{}{page_note} selected", pos + 1, matches.len());
    }

    pub fn run_find_and_replace(&mut self) {
        match self.find_replace_mode {
            FindReplaceMode::Text => self.run_text_find_and_replace(),
            mode => self.run_color_find_and_replace(mode),
        }
    }

    fn run_text_find_and_replace(&mut self) {
        if self.find_query.is_empty() {
            self.status = "Enter text to replace first".to_string();
            return;
        }
        self.push_undo("Find and Replace");
        let from = self.find_query.clone();
        let to = self.replace_query.clone();
        let case_sensitive = self.find_case_sensitive;
        let mut count = 0usize;
        for page in &mut self.doc.pages {
            for layer in &mut page.layers {
                for obj in &mut layer.objects {
                    count += replace_text_in(&mut obj.name, &from, &to, case_sensitive);
                    if let Shape::Text { text, .. } = &mut obj.kind {
                        count += replace_text_in(text, &from, &to, case_sensitive);
                    }
                }
            }
            page.sync_state();
        }
        self.doc.dirty = true;
        self.status = format!(
            "Replaced {count} text/name occurrence(s) of '{from}' across {} page(s)",
            self.doc.pages.len()
        );
    }

    fn run_color_find_and_replace(&mut self, mode: FindReplaceMode) {
        self.push_undo("Replace Color");
        let from = self.find_color;
        let to = self.replace_color;
        let mut objects = 0usize;
        for page in &mut self.doc.pages {
            for layer in &mut page.layers {
                for obj in &mut layer.objects {
                    if replace_colors_in_obj(obj, mode, from, to) > 0 {
                        objects += 1;
                    }
                }
            }
            page.sync_state();
        }
        self.doc.dirty = true;
        self.status = if objects == 0 {
            format!(
                "No {} matched {} — nothing replaced",
                mode.label().to_lowercase(),
                colors::hex_of(from)
            )
        } else {
            format!(
                "Replaced {} with {} in {objects} object(s) across {} page(s)",
                colors::hex_of(from),
                colors::hex_of(to),
                self.doc.pages.len()
            )
        };
    }

    /// Number of hidden objects on the current page, used by the god-view legend.
    pub fn hidden_object_count(&self) -> usize {
        let page = self.doc.page();
        page.layers
            .iter()
            .map(|l| {
                let layer_hidden = usize::from(!l.visible) * l.objects.len();
                layer_hidden + l.objects.iter().filter(|o| !o.visible).count()
            })
            .sum()
    }
}

fn object_matches_text(obj: &Obj, needle: &str, case_sensitive: bool) -> bool {
    if needle.is_empty() {
        return false;
    }
    let contains = |haystack: &str| {
        if case_sensitive {
            haystack.contains(needle)
        } else {
            haystack.to_lowercase().contains(&needle.to_lowercase())
        }
    };
    contains(&obj.name) || matches!(&obj.kind, Shape::Text { text, .. } if contains(text))
}

fn replace_text_in(value: &mut String, from: &str, to: &str, case_sensitive: bool) -> usize {
    if from.is_empty() {
        return 0;
    }
    let occurrences = if case_sensitive {
        value.matches(from).count()
    } else {
        value.to_lowercase().matches(&from.to_lowercase()).count()
    };
    if occurrences > 0 {
        *value = if case_sensitive {
            value.replace(from, to)
        } else {
            // Case-insensitive replacement is intentionally simple and deterministic: the query
            // is matched without regard to case and the replacement keeps the typed casing.
            let mut result = String::with_capacity(value.len());
            let lower = value.to_lowercase();
            let needle = from.to_lowercase();
            let mut cursor = 0usize;
            while let Some(rel) = lower[cursor..].find(&needle) {
                let start = cursor + rel;
                result.push_str(&value[cursor..start]);
                result.push_str(to);
                cursor = start + needle.len();
            }
            result.push_str(&value[cursor..]);
            result
        };
    }
    occurrences
}

fn object_has_fill_color(obj: &Obj, color: Color32) -> bool {
    obj.fill.is_some_and(|c| c == color)
        || obj
            .gradient
            .is_some_and(|g| g.start == color || g.end == color)
}

fn effect_has_color(effect: &LiveEffect, color: Color32) -> bool {
    match effect {
        LiveEffect::DropShadow { color: c, .. }
        | LiveEffect::InnerShadow { color: c, .. }
        | LiveEffect::Glow { color: c, .. } => *c == color,
        LiveEffect::Bevel {
            highlight, shadow, ..
        } => *highlight == color || *shadow == color,
        LiveEffect::GaussianBlur { .. } | LiveEffect::ColorAdjust { .. } => false,
    }
}

fn object_has_any_color(obj: &Obj, color: Color32) -> bool {
    obj.fill.is_some_and(|c| c == color)
        || obj.stroke.is_some_and(|c| c == color)
        || obj
            .gradient
            .is_some_and(|g| g.start == color || g.end == color)
        || obj.effects.iter().any(|e| effect_has_color(e, color))
}

fn replace_color_slot(slot: &mut Option<Color32>, from: Color32, to: Color32) -> usize {
    if slot.is_some_and(|c| c == from) {
        *slot = Some(to);
        1
    } else {
        0
    }
}

fn replace_effect_colors(
    effects: &mut [LiveEffect],
    from: Color32,
    to: Color32,
    fill_only: bool,
    stroke_only: bool,
) -> usize {
    let mut count = 0usize;
    for effect in effects {
        match effect {
            LiveEffect::DropShadow { color, .. }
            | LiveEffect::InnerShadow { color, .. }
            | LiveEffect::Glow { color, .. } => {
                if *color == from && !stroke_only {
                    *color = to;
                    count += 1;
                }
            }
            LiveEffect::Bevel {
                highlight, shadow, ..
            } => {
                if *highlight == from && !stroke_only {
                    *highlight = to;
                    count += 1;
                }
                if *shadow == from && !fill_only {
                    *shadow = to;
                    count += 1;
                }
            }
            LiveEffect::GaussianBlur { .. } | LiveEffect::ColorAdjust { .. } => {}
        }
    }
    count
}

fn replace_colors_in_obj(
    obj: &mut Obj,
    mode: FindReplaceMode,
    from: Color32,
    to: Color32,
) -> usize {
    if from == to {
        return 0;
    }
    let mut count = 0usize;
    match mode {
        FindReplaceMode::Text => {}
        FindReplaceMode::FillColor => {
            count += replace_color_slot(&mut obj.fill, from, to);
            if let Some(gradient) = &mut obj.gradient {
                if gradient.start == from {
                    gradient.start = to;
                    count += 1;
                }
                if gradient.end == from {
                    gradient.end = to;
                    count += 1;
                }
            }
        }
        FindReplaceMode::StrokeColor => {
            count += replace_color_slot(&mut obj.stroke, from, to);
        }
        FindReplaceMode::AnyColor => {
            count += replace_color_slot(&mut obj.fill, from, to);
            count += replace_color_slot(&mut obj.stroke, from, to);
            if let Some(gradient) = &mut obj.gradient {
                if gradient.start == from {
                    gradient.start = to;
                    count += 1;
                }
                if gradient.end == from {
                    gradient.end = to;
                    count += 1;
                }
            }
            count += replace_effect_colors(&mut obj.effects, from, to, false, false);
        }
    }
    count
}
