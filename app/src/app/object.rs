//! Object commands: paths and booleans, symbols, styles, auto-shapes, slices, filters and flatten.

use super::*;

impl FireCraft {
    pub fn duplicate_state(&mut self) {
        let page = self.doc.page_mut();
        page.sync_state();
        let cur = page.current_state.min(page.states.len() - 1);
        let mut copy = page.states[cur].clone();
        copy.name = format!("State {}", page.states.len() + 1);
        page.states.push(copy);
        let last = page.states.len() - 1;
        page.select_state(last);
        self.status = "Duplicated current animation state".to_string();
    }

    pub fn tween_selected_across_states(&mut self) {
        let Some(obj) = self.selected_object() else {
            self.status = "Select an object to tween across 3 animation states".to_string();
            return;
        };
        let page = self.doc.page_mut();
        page.sync_state();
        for step in 1..=3 {
            let mut next_st = page.states[page.current_state].clone();
            next_st.name = format!("State {}", page.states.len() + 1);
            let mut moved = obj.clone();
            moved.translate(Vec2::new(step as f32 * 36.0, step as f32 * 14.0));
            moved.opacity = (1.0 - step as f32 * 0.18).max(0.25);
            if let Some(first_layer) = next_st.layers.first_mut() {
                first_layer.objects.push(moved);
            }
            page.states.push(next_st);
        }
        self.status = "Generated 3 motion-tween animation states".to_string();
    }

    pub fn convert_selection_to_symbol(&mut self) {
        let Some(obj) = self.selected_object() else {
            self.status = "Select an object to convert to a Symbol".to_string();
            return;
        };
        self.push_undo("Convert to Symbol");
        let id = self.doc.symbols.len();
        let name = format!("{} Symbol", obj.name);
        let (at, _) = obj
            .bounds()
            .unwrap_or((Vec2::new(100.0, 100.0), Vec2::new(164.0, 164.0)));
        self.doc.symbols.push(SymbolDef {
            id,
            name: name.clone(),
            kind: "Graphic".to_string(),
            objects: vec![obj],
        });
        let inst = Obj::new(
            Shape::SymbolInstance {
                symbol_id: id,
                at,
                scale: Vec2::splat(1.0),
            },
            Some(self.fill),
            Some(self.stroke),
        );
        let idx = self.push_object(inst);
        self.select_single(Some(idx));
        self.status = format!("Converted to symbol: {name}");
    }

    pub fn create_default_button_symbol(&mut self) {
        if !self.doc.symbols.is_empty() {
            return;
        }
        let btn = Obj::new(
            Shape::Rect {
                min: Vec2::ZERO,
                max: Vec2::new(120.0, 40.0),
                corner_radius: 8.0,
                filled: true,
            },
            Some(Color32::from_rgb(232, 163, 61)),
            Some(Color32::from_rgb(30, 30, 36)),
        );
        self.doc.symbols.push(SymbolDef {
            id: 0,
            name: "Primary CTA Button".to_string(),
            kind: "Button".to_string(),
            objects: vec![btn],
        });
    }

    pub fn place_symbol_instance(&mut self, symbol_id: usize) {
        self.push_undo("Place Symbol");
        let center = Vec2::new(
            self.doc.page().width * 0.5 - 32.0,
            self.doc.page().height * 0.5 - 32.0,
        );
        let inst = Obj::new(
            Shape::SymbolInstance {
                symbol_id,
                at: center,
                scale: Vec2::splat(1.2),
            },
            Some(self.fill),
            Some(self.stroke),
        );
        let idx = self.push_object(inst);
        self.select_single(Some(idx));
        self.status = format!("Placed Symbol #{} instance", symbol_id + 1);
    }

    pub fn apply_style_preset_value(&mut self, preset: &StylePreset) {
        if self.selected.is_none() && self.selected_set.is_empty() {
            self.fill = preset.fill.unwrap_or(self.fill);
            self.stroke = preset.stroke.unwrap_or(self.stroke);
            self.status = format!("Loaded style colors: {}", preset.name);
            return;
        }
        self.push_undo("Apply Style");
        let name = preset.name.clone();
        let preset_clone = preset.clone();
        self.with_selection_mut(|obj| {
            obj.fill = preset_clone.fill;
            obj.stroke = preset_clone.stroke;
            obj.width = preset_clone.stroke_width;
            obj.effects = preset_clone.effects.clone();
            obj.gradient = match (preset_clone.fill, preset_clone.gradient_end) {
                (Some(start), Some(end)) => Some(GradientFill { start, end }),
                _ => None,
            };
        });
        self.status = format!("Applied style: {name}");
    }

    pub fn delete_selected_style(&mut self) {
        if self.style_library != crate::doc::StyleLibrary::CurrentDocument {
            return;
        }
        if let Some(idx) = self.selected_style_idx
            && idx < self.doc.styles.len()
        {
            let removed = self.doc.styles.remove(idx);
            self.selected_style_idx = None;
            self.doc.dirty = true;
            self.status = format!("Deleted style: {}", removed.name);
        }
    }

    pub fn save_selection_as_style(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        let name = format!("Style {}", self.doc.styles.len() + 1);
        self.doc.styles.push(StylePreset {
            name: name.clone(),
            fill: obj.fill,
            gradient_end: obj.gradient.map(|g| g.end),
            stroke: obj.stroke,
            stroke_width: obj.width,
            effects: obj.effects.clone(),
        });
        self.style_library = crate::doc::StyleLibrary::CurrentDocument;
        self.selected_style_idx = Some(self.doc.styles.len() - 1);
        self.status = format!("Saved new style preset: {name}");
    }

    pub fn insert_auto_shape(&mut self, kind: AutoShapeKind) {
        self.push_undo("Insert Auto Shape");
        let c = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
        let shape = match kind {
            AutoShapeKind::RoundedRect => Shape::Rect {
                min: c - Vec2::new(70.0, 45.0),
                max: c + Vec2::new(70.0, 45.0),
                corner_radius: 16.0,
                filled: true,
            },
            AutoShapeKind::ChamferRect => Shape::Rect {
                min: c - Vec2::new(70.0, 45.0),
                max: c + Vec2::new(70.0, 45.0),
                corner_radius: 8.0,
                filled: true,
            },
            AutoShapeKind::Star => Shape::Polygon {
                center: c,
                radius: 64.0,
                sides: 5,
                star_inner: 0.45,
                angle_rad: 0.0,
            },
            AutoShapeKind::Polygon => Shape::Polygon {
                center: c,
                radius: 60.0,
                sides: 6,
                star_inner: 1.0,
                angle_rad: 0.0,
            },
            AutoShapeKind::Gear => Shape::Polygon {
                center: c,
                radius: 62.0,
                sides: 12,
                star_inner: 0.78,
                angle_rad: 0.0,
            },
            AutoShapeKind::Arrow => Shape::Path {
                points: vec![
                    c + Vec2::new(-70.0, -18.0),
                    c + Vec2::new(15.0, -18.0),
                    c + Vec2::new(15.0, -40.0),
                    c + Vec2::new(70.0, 0.0),
                    c + Vec2::new(15.0, 40.0),
                    c + Vec2::new(15.0, 18.0),
                    c + Vec2::new(-70.0, 18.0),
                ],
                closed: true,
            },
            AutoShapeKind::Callout => Shape::Path {
                points: vec![
                    c + Vec2::new(-70.0, -45.0),
                    c + Vec2::new(70.0, -45.0),
                    c + Vec2::new(70.0, 25.0),
                    c + Vec2::new(-10.0, 25.0),
                    c + Vec2::new(-35.0, 55.0),
                    c + Vec2::new(-30.0, 25.0),
                    c + Vec2::new(-70.0, 25.0),
                ],
                closed: true,
            },
            AutoShapeKind::Cross => Shape::Path {
                points: vec![
                    c + Vec2::new(-20.0, -60.0),
                    c + Vec2::new(20.0, -60.0),
                    c + Vec2::new(20.0, -20.0),
                    c + Vec2::new(60.0, -20.0),
                    c + Vec2::new(60.0, 20.0),
                    c + Vec2::new(20.0, 20.0),
                    c + Vec2::new(20.0, 60.0),
                    c + Vec2::new(-20.0, 60.0),
                    c + Vec2::new(-20.0, 20.0),
                    c + Vec2::new(-60.0, 20.0),
                    c + Vec2::new(-60.0, -20.0),
                    c + Vec2::new(-20.0, -20.0),
                ],
                closed: true,
            },
            AutoShapeKind::LShape => Shape::Path {
                points: vec![
                    c + Vec2::new(-55.0, -55.0),
                    c + Vec2::new(-15.0, -55.0),
                    c + Vec2::new(-15.0, 15.0),
                    c + Vec2::new(55.0, 15.0),
                    c + Vec2::new(55.0, 55.0),
                    c + Vec2::new(-55.0, 55.0),
                ],
                closed: true,
            },
            AutoShapeKind::Spiral => {
                let mut pts = Vec::new();
                for i in 0..36 {
                    let t = i as f32 * 0.35;
                    let r = 4.0 + i as f32 * 1.7;
                    pts.push(c + Vec2::new(r * t.cos(), r * t.sin()));
                }
                Shape::Path {
                    points: pts,
                    closed: false,
                }
            }
            AutoShapeKind::Pie | AutoShapeKind::Donut => Shape::Ellipse {
                center: c,
                radius: Vec2::splat(56.0),
                filled: true,
            },
        };
        let mut obj = Obj::new(shape, Some(self.fill), Some(self.stroke));
        obj.name = kind.label().to_string();
        obj.width = 2.0;
        let idx = self.push_object(obj);
        self.select_single(Some(idx));
        self.status = format!("Inserted Auto Shape: {}", kind.label());
    }

    pub fn insert_special_glyph(&mut self, glyph: &str) {
        if let Some(obj) = self.selected_object()
            && let Shape::Text { mut text, .. } = obj.kind
        {
            text.push_str(glyph);
            self.set_selected_text(text);
            self.status = format!("Appended '{glyph}' to selected text");
            return;
        }
        self.push_undo("Insert Character");
        let c = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
        let obj = Obj::new(
            Shape::Text {
                at: c,
                text: glyph.to_string(),
                size: 36.0,
                bold: false,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(self.fill),
            None,
        );
        let idx = self.push_object(obj);
        self.select_single(Some(idx));
        self.status = format!("Inserted special character '{glyph}'");
    }

    pub fn insert_slice(&mut self, is_hotspot: bool) {
        self.push_undo(if is_hotspot {
            "Add Hotspot"
        } else {
            "Add Slice"
        });
        let (lo, hi) = self
            .selected_object()
            .and_then(|o| o.bounds())
            .unwrap_or_else(|| {
                let c = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
                (c - Vec2::new(80.0, 30.0), c + Vec2::new(80.0, 30.0))
            });
        let obj = Obj::new(
            Shape::Slice {
                min: lo,
                max: hi,
                url: "https://github.com/aliasfoxkde/FireCraft".to_string(),
                alt: "FireCraft Slice".to_string(),
                target: "_self".to_string(),
                format: "PNG32".to_string(),
                is_hotspot,
            },
            None,
            None,
        );
        let idx = self.push_object(obj);
        self.select_single(Some(idx));
        self.show_slices = true;
        self.status = if is_hotspot {
            "Web Hotspot created on Web Layer".to_string()
        } else {
            "Web Slice created on Web Layer".to_string()
        };
    }

    pub fn create_slice_from_selection(&mut self) {
        self.insert_slice(false);
    }

    // ---- Path, Alignment, Color & Document Commands ------------------------

    fn obj_to_polygon_points(obj: &Obj) -> Vec<Vec2> {
        match &obj.kind {
            Shape::Rect { min, max, .. } => {
                let lo = min.min(*max);
                let hi = min.max(*max);
                vec![lo, Vec2::new(hi.x, lo.y), hi, Vec2::new(lo.x, hi.y)]
            }
            Shape::Ellipse { center, radius, .. } => (0..24)
                .map(|i| {
                    let t = i as f32 / 24.0 * std::f32::consts::TAU;
                    *center + Vec2::new(radius.x * t.cos(), radius.y * t.sin())
                })
                .collect(),
            Shape::Polygon {
                center,
                radius,
                sides,
                star_inner,
                angle_rad,
            } => Obj::polygon_vertices(*center, *radius, *sides, *star_inner, *angle_rad),
            Shape::Path { points, .. } => points.clone(),
            Shape::Line { a, b } => vec![*a, *b],
            _ => obj
                .bounds()
                .map(|(lo, hi)| vec![lo, Vec2::new(hi.x, lo.y), hi, Vec2::new(lo.x, hi.y)])
                .unwrap_or_default(),
        }
    }

    pub fn combine_paths_op(&mut self, op: &str) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        indices.sort_unstable();
        indices.dedup();
        if indices.is_empty() {
            self.status = "Select 1 or 2 objects first".to_string();
            return;
        }
        if op == "split" {
            self.split_selected_path();
            return;
        }
        self.push_undo("Combine Paths");
        if indices.len() == 1 {
            self.convert_selected_to_path();
            if op == "join" {
                self.with_selected_mut(|obj| {
                    if let Shape::Path { closed, .. } = &mut obj.kind {
                        *closed = true;
                    }
                });
                self.status = "Closed path endpoints (Join)".to_string();
            } else {
                self.status = format!("Select 2+ objects to apply {op}");
            }
            return;
        }
        let layer = self.selected_layer;
        let first_idx = indices[0];
        let page = self.doc.page_mut();
        let Some(objs) = page.layers.get_mut(layer).map(|l| &mut l.objects) else {
            return;
        };
        if *indices.last().unwrap_or(&0) >= objs.len() {
            return;
        }

        let mut failure: Option<String> = None;
        if op == "join" {
            // Join chains contours at their nearest endpoints instead of closing every path and
            // importing unrelated subpaths into one object.
            let mut joined: Vec<Vec2> = Vec::new();
            for &idx in &indices {
                let mut pts = Self::obj_to_polygon_points(&objs[idx]);
                if pts.len() < 2 {
                    continue;
                }
                if joined.is_empty() {
                    joined = pts;
                    continue;
                }
                let tail = *joined.last().unwrap_or(&Vec2::ZERO);
                let d_start = (tail - pts[0]).length();
                let d_end = (tail - *pts.last().unwrap()).length();
                if d_end < d_start {
                    pts.reverse();
                }
                let first = pts[0];
                if (tail - first).length() < 0.01 {
                    pts.remove(0);
                }
                joined.extend(pts);
            }
            if joined.len() < 2 {
                failure = Some("Join needs at least one path with two points".to_string());
            } else {
                objs[first_idx].kind = Shape::Path {
                    points: joined,
                    closed: false,
                };
                for &other_idx in indices[1..].iter().rev() {
                    objs.remove(other_idx);
                }
            }
        } else {
            let mut acc_pts = Self::obj_to_polygon_points(&objs[first_idx]);
            for &other_idx in &indices[1..] {
                let other_pts = Self::obj_to_polygon_points(&objs[other_idx]);
                match polygon_boolean(&acc_pts, &other_pts, op) {
                    Some(result) if result.len() >= 3 => acc_pts = result,
                    Some(_) => {
                        failure = Some(format!("{op} produced an empty path"));
                        break;
                    }
                    None => {
                        failure = Some(format!("{op} needs two closed, non-degenerate paths"));
                        break;
                    }
                }
            }
            if failure.is_none() {
                objs[first_idx].kind = Shape::Path {
                    points: acc_pts,
                    closed: true,
                };
                for &other_idx in indices[1..].iter().rev() {
                    objs.remove(other_idx);
                }
            }
        }
        page.sync_state();
        if let Some(err) = failure {
            // The undo snapshot was pushed before validation; discard the empty snapshot so a
            // failed boolean operation cannot be undone into a duplicate state.
            self.undo.pop();
            self.status = format!("Combine Paths failed: {err}");
            return;
        }
        self.select_single(Some(first_idx));
        self.status = format!("Applied Combine Paths: {op}");
    }

    pub fn convert_selected_to_path(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        let mut override_fill: Option<Option<Color32>> = None;
        let new_kind = match &obj.kind {
            Shape::Rect {
                min,
                max,
                corner_radius,
                ..
            } => {
                let lo = min.min(*max);
                let hi = min.max(*max);
                let cr = (*corner_radius)
                    .min((hi.x - lo.x) * 0.5)
                    .min((hi.y - lo.y) * 0.5);
                let points = if cr > 0.5 {
                    let mut pts = Vec::with_capacity(16);
                    let corners = [
                        (
                            Vec2::new(lo.x + cr, lo.y + cr),
                            std::f32::consts::PI,
                            std::f32::consts::PI * 1.5,
                        ),
                        (
                            Vec2::new(hi.x - cr, lo.y + cr),
                            std::f32::consts::PI * 1.5,
                            std::f32::consts::TAU,
                        ),
                        (
                            Vec2::new(hi.x - cr, hi.y - cr),
                            0.0,
                            std::f32::consts::FRAC_PI_2,
                        ),
                        (
                            Vec2::new(lo.x + cr, hi.y - cr),
                            std::f32::consts::FRAC_PI_2,
                            std::f32::consts::PI,
                        ),
                    ];
                    for (c, a0, a1) in corners {
                        for step in 0..=3 {
                            let a = a0 + (a1 - a0) * (step as f32 / 3.0);
                            pts.push(c + Vec2::new(cr * a.cos(), cr * a.sin()));
                        }
                    }
                    pts
                } else {
                    vec![lo, Vec2::new(hi.x, lo.y), hi, Vec2::new(lo.x, hi.y)]
                };
                Shape::Path {
                    points,
                    closed: true,
                }
            }
            Shape::Ellipse { center, radius, .. } => {
                let pts = (0..16)
                    .map(|i| {
                        let t = i as f32 / 16.0 * std::f32::consts::TAU;
                        *center + Vec2::new(radius.x * t.cos(), radius.y * t.sin())
                    })
                    .collect();
                Shape::Path {
                    points: pts,
                    closed: true,
                }
            }
            Shape::Polygon {
                center,
                radius,
                sides,
                star_inner,
                angle_rad,
            } => Shape::Path {
                points: Obj::polygon_vertices(*center, *radius, *sides, *star_inner, *angle_rad),
                closed: true,
            },
            Shape::Line { a, b } => Shape::Path {
                points: vec![*a, *b],
                closed: false,
            },
            Shape::Gradient { min, max, from, .. } => {
                let lo = min.min(*max);
                let hi = min.max(*max);
                override_fill = Some(Some(*from));
                Shape::Path {
                    points: vec![lo, Vec2::new(hi.x, lo.y), hi, Vec2::new(lo.x, hi.y)],
                    closed: true,
                }
            }
            Shape::Text { .. } => {
                self.status = "Text outlines are not available; the text object was left unchanged"
                    .to_string();
                return;
            }
            Shape::Path { .. } => {
                self.status = "Selection is already an editable Vector Path".to_string();
                return;
            }
            _ => return,
        };
        self.inline_text_edit_obj = None;
        self.push_undo("Convert to Path");
        self.with_selected_mut(|o| {
            o.kind = new_kind;
            if let Some(f) = override_fill {
                o.fill = f;
            }
        });
        self.set_tool(Tool::Subselect);
        self.status = "Converted selection to editable Vector Path (Subselect nodes)".to_string();
    }

    pub fn simplify_selected_path(&mut self) {
        const TOLERANCE: f32 = 0.75;
        let Some(obj) = self.selected_object() else {
            self.status = "Select a path to simplify".to_string();
            return;
        };
        let Shape::Path { points, closed } = &obj.kind else {
            self.status = "Convert the selection to a path before simplifying".to_string();
            return;
        };
        let simplified = simplify_polyline(points, *closed, TOLERANCE);
        if simplified == *points {
            self.status = "Path is already within the simplify tolerance".to_string();
            return;
        }
        self.push_undo("Simplify Path");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind {
                *points = simplified;
            }
        });
        self.selected_path_nodes.clear();
        self.status = format!("Simplified path within {TOLERANCE:.2}px tolerance");
    }

    pub fn split_selected_path(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        if let Shape::Path { points, .. } = &obj.kind
            && points.len() >= 4
        {
            self.push_undo("Knife Split Path");
            let mid = points.len() / 2;
            let first_half = points[..=mid].to_vec();
            let second_half = points[mid..].to_vec();
            self.with_selected_mut(|o| {
                o.kind = Shape::Path {
                    points: first_half,
                    closed: false,
                };
            });
            let mut second_obj = obj.clone();
            second_obj.kind = Shape::Path {
                points: second_half,
                closed: false,
            };
            second_obj.name = "Split Path".to_string();
            self.push_object(second_obj);
            self.status = "Split path into two contiguous contours".to_string();
        }
    }

    pub fn toggle_selected_path_closed(&mut self) {
        let Some(obj) = self.selected_object() else {
            self.status = "Select a path first".to_string();
            return;
        };
        let Shape::Path { points, closed } = &obj.kind else {
            self.status = "Convert the selection to a path first".to_string();
            return;
        };
        if !closed && points.len() < 3 {
            self.status = "A closed path needs at least three anchors".to_string();
            return;
        }
        self.push_undo("Toggle Path Closure");
        self.with_selected_mut(|obj| {
            if let Shape::Path { closed, .. } = &mut obj.kind {
                *closed = !*closed;
            }
        });
        self.status = if *closed {
            "Opened path contour".to_string()
        } else {
            "Closed path contour".to_string()
        };
    }

    pub fn expand_selected_bounds(&mut self, delta: f32) {
        if let Some((lo, hi)) = self.selected_object().and_then(|o| o.bounds()) {
            self.push_undo("Alter Path Bounds");
            self.set_selected_bounds(lo - Vec2::splat(delta), hi + Vec2::splat(delta));
            self.status = if delta >= 0.0 {
                format!("Expanded path bounds by +{delta:.0}px")
            } else {
                format!("Inset path bounds by {delta:.0}px")
            };
        }
    }

    pub fn reverse_selected_path(&mut self) {
        if !self
            .selected_object()
            .is_some_and(|obj| matches!(obj.kind, Shape::Path { .. }))
        {
            self.convert_selected_to_path();
        }
        if !self
            .selected_object()
            .is_some_and(|obj| matches!(obj.kind, Shape::Path { .. }))
        {
            return;
        }
        self.push_undo("Reverse Path");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind {
                points.reverse();
            }
        });
        self.status = "Reversed path contour direction".to_string();
    }

    pub fn add_points_to_selected_path(&mut self) {
        self.convert_selected_to_path();
        self.push_undo("Add Path Points");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, closed } = &mut obj.kind
                && points.len() >= 2
                && points.len() < 128
            {
                let mut next = Vec::with_capacity(points.len() * 2);
                for i in 0..points.len() {
                    let a = points[i];
                    next.push(a);
                    if i + 1 < points.len() {
                        next.push((a + points[i + 1]) * 0.5);
                    } else if *closed {
                        next.push((a + points[0]) * 0.5);
                    }
                }
                *points = next;
            }
        });
        self.status = "Subdivided path segments with new anchor points".to_string();
    }

    pub fn delete_point_from_selected_path(&mut self) {
        let Some(obj) = self.selected_object() else {
            self.status = "Select a path and anchor point first".to_string();
            return;
        };
        let Shape::Path { points, .. } = &obj.kind else {
            self.status = "Convert the selection to a path first".to_string();
            return;
        };
        let mut selected = self.selected_path_nodes.clone();
        selected.retain(|&index| index < points.len());
        selected.sort_unstable();
        selected.dedup();
        if selected.is_empty() {
            self.status = "Click an anchor with Subselection, then choose Delete Point".to_string();
            return;
        }
        let min_points = 3;
        if points.len().saturating_sub(selected.len()) < min_points {
            self.status = format!("A path needs at least {min_points} anchors");
            return;
        }
        self.push_undo("Delete Path Point");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind {
                for index in selected.iter().rev() {
                    points.remove(*index);
                }
            }
        });
        self.selected_path_nodes.clear();
        self.status = format!("Deleted {} selected anchor(s)", selected.len());
    }

    pub fn round_selected_path_points(&mut self, delta_radius: f32) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        if let Shape::Rect { corner_radius, .. } = obj.kind {
            let next = (corner_radius + delta_radius).clamp(0.0, 120.0);
            self.set_selected_corner_radius(next);
            self.status = format!("Adjusted corner roundness to {next:.0}px");
        } else {
            self.status =
                "Freeform corner rounding is unavailable; the path was left unchanged".to_string();
        }
    }

    pub fn align_selected_path_points(&mut self, horizontal: bool) {
        let Some(obj) = self.selected_object() else {
            self.status = "Select two or more path anchors to align".to_string();
            return;
        };
        let Shape::Path { points, .. } = &obj.kind else {
            self.status = "Convert the selection to a path first".to_string();
            return;
        };
        let mut selected = self.selected_path_nodes.clone();
        selected.retain(|&index| index < points.len());
        selected.sort_unstable();
        selected.dedup();
        if selected.len() < 2 {
            self.status = "Shift-click at least two path anchors to align them".to_string();
            return;
        }
        let avg = selected
            .iter()
            .map(|&index| {
                if horizontal {
                    points[index].y
                } else {
                    points[index].x
                }
            })
            .sum::<f32>()
            / selected.len() as f32;
        self.push_undo("Align Path Points");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind {
                for &index in &selected {
                    if horizontal {
                        points[index].y = avg;
                    } else {
                        points[index].x = avg;
                    }
                }
            }
        });
        self.status = if horizontal {
            "Aligned path points horizontally".to_string()
        } else {
            "Aligned path points vertically".to_string()
        };
    }

    pub fn snap_selected_to_pixel(&mut self) {
        self.push_undo("Snap to Pixel");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind {
                for p in points {
                    p.x = p.x.round();
                    p.y = p.y.round();
                }
            } else if let Some((lo, hi)) = obj.bounds() {
                obj.set_bounds(
                    Vec2::new(lo.x.round(), lo.y.round()),
                    Vec2::new(hi.x.round(), hi.y.round()),
                );
            }
        });
        self.status = "Snapped selection coordinates to pixel grid".to_string();
    }

    pub fn convert_stroke_to_fill(&mut self) {
        self.push_undo("Convert Stroke to Fill");
        let default_stroke = self.stroke;
        self.with_selected_mut(|obj| {
            let sc = obj.stroke.unwrap_or(default_stroke);
            obj.fill = Some(sc);
            obj.stroke = None;
        });
        self.status = "Converted stroke outline to filled shape".to_string();
    }

    pub fn apply_color_command(&mut self, cmd: &str) {
        if self.selected.is_none() {
            self.status = "Select an object first".to_string();
            return;
        }
        self.push_undo("Adjust Color");
        let xform = |c: Color32| -> Color32 {
            let (r, g, b, a) = (c.r() as f32, c.g() as f32, c.b() as f32, c.a());
            match cmd {
                "brighten" => Color32::from_rgba_unmultiplied(
                    (r * 1.18).min(255.0) as u8,
                    (g * 1.18).min(255.0) as u8,
                    (b * 1.18).min(255.0) as u8,
                    a,
                ),
                "darken" => Color32::from_rgba_unmultiplied(
                    (r * 0.82) as u8,
                    (g * 0.82) as u8,
                    (b * 0.82) as u8,
                    a,
                ),
                "contrast" => {
                    let f = |v: f32| ((v - 128.0) * 1.25 + 128.0).clamp(0.0, 255.0) as u8;
                    Color32::from_rgba_unmultiplied(f(r), f(g), f(b), a)
                }
                "invert" => {
                    Color32::from_rgba_unmultiplied(255 - c.r(), 255 - c.g(), 255 - c.b(), a)
                }
                "grayscale" => {
                    let y = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
                    Color32::from_rgba_unmultiplied(y, y, y, a)
                }
                "sepia" => {
                    let tr = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
                    let tg = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
                    let tb = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;
                    Color32::from_rgba_unmultiplied(tr, tg, tb, a)
                }
                _ => c,
            }
        };
        self.with_selected_mut(|obj| {
            obj.fill = obj.fill.map(xform);
            obj.stroke = obj.stroke.map(xform);
            if let Shape::Bitmap { pixels, .. } = &mut obj.kind {
                for px in pixels.as_chunks_mut::<4>().0 {
                    let c = xform(Color32::from_rgba_unmultiplied(px[0], px[1], px[2], px[3]));
                    px[0] = c.r();
                    px[1] = c.g();
                    px[2] = c.b();
                }
            }
        });
        self.textures.clear();
        self.status = format!("Applied color filter: {cmd}");
    }

    pub fn flatten_selected_to_bitmap(&mut self) {
        let mut indices = if !self.selected_set.is_empty() {
            self.selected_set.clone()
        } else if let Some(idx) = self.selected {
            vec![idx]
        } else {
            return;
        };
        indices.sort_unstable();
        let layer = self.selected_layer;
        let objs: Vec<Obj> = {
            let Some(l) = self.doc.page().layers.get(layer) else {
                return;
            };
            indices
                .iter()
                .filter_map(|&i| l.objects.get(i).cloned())
                .collect()
        };
        if objs.is_empty() {
            return;
        }
        let mut lo = Vec2::splat(f32::INFINITY);
        let mut hi = Vec2::splat(f32::NEG_INFINITY);
        for o in &objs {
            if let Some((a, b)) = o.bounds() {
                lo = lo.min(a);
                hi = hi.max(b);
            }
        }
        if !lo.x.is_finite() || !hi.x.is_finite() {
            return;
        }
        self.push_undo("Flatten Selection");
        let img = crate::formats::rasterize_selection(&objs, lo, hi);
        let (w, h) = (img.width() as f32, img.height() as f32);
        let first_idx = indices[0];
        if let Some(l) = self.doc.page_mut().layers.get_mut(layer) {
            for &i in indices.iter().rev() {
                if i < l.objects.len() {
                    l.objects.remove(i);
                }
            }
            let mut flat = Obj::new(
                Shape::Bitmap {
                    at: lo,
                    pixels: img.into_raw(),
                    size: Vec2::new(w, h),
                    pixel_size: Vec2::new(w, h),
                },
                None,
                None,
            );
            flat.name = format!("Flattened Bitmap ({:.0}×{:.0})", w, h);
            let insert_at = first_idx.min(l.objects.len());
            l.objects.insert(insert_at, flat);
            self.select_single(Some(insert_at));
        }
        self.doc.page_mut().sync_state();
        self.textures.clear();
        self.status = format!(
            "Flattened {} object(s) to Bitmap ({:.0}×{:.0})",
            objs.len(),
            w,
            h
        );
    }

    pub fn copy_selected_css(&mut self) {
        if let Some(obj) = self.selected_object() {
            let css = crate::formats::generate_css_for_object(&obj);
            self.ctx.copy_text(css);
            self.library_tab = 4; // Focus CSS tab
            self.status = "Copied CSS3 rules to clipboard".to_string();
        }
    }

    pub fn execute_saved_command(&mut self, cmd: &str) {
        match cmd {
            "Add Drop Shadow Card" => self.command_add_shadow_card(),
            "Add Picture Frame" => self.command_add_picture_frame(),
            "Distribute 3×3 Guides" => self.command_add_thirds_guides(),
            "Convert Selection to Grayscale" => self.apply_color_command("grayscale"),
            "Create Slice from Selection" => self.create_slice_from_selection(),
            _ => self.replay_last_action(),
        }
    }

    pub fn command_add_picture_frame(&mut self) {
        self.push_undo("Add Picture Frame");
        let (w, h) = (self.doc.page().width, self.doc.page().height);
        let mut frame = Obj::new(
            Shape::Rect {
                min: Vec2::new(16.0, 16.0),
                max: Vec2::new(w - 16.0, h - 16.0),
                corner_radius: 6.0,
                filled: false,
            },
            None,
            Some(self.stroke),
        );
        frame.name = "Picture Frame".to_string();
        frame.width = 8.0;
        let idx = self.push_object(frame);
        self.select_single(Some(idx));
        self.status = "Added Picture Frame".to_string();
    }

    pub fn command_add_shadow_card(&mut self) {
        self.push_undo("Add Shadow Card");
        let c = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
        let mut card = Obj::new(
            Shape::Rect {
                min: c - Vec2::new(120.0, 75.0),
                max: c + Vec2::new(120.0, 75.0),
                corner_radius: 12.0,
                filled: true,
            },
            Some(self.fill),
            Some(self.stroke),
        );
        card.name = "UI Card".to_string();
        card.effects.push(LiveEffect::DropShadow {
            distance: 6.0,
            angle_deg: 45.0,
            blur: 8.0,
            color: Color32::from_black_alpha(150),
            enabled: true,
        });
        let idx = self.push_object(card);
        self.select_single(Some(idx));
        self.status = "Added UI Card with Drop Shadow".to_string();
    }

    pub fn command_add_thirds_guides(&mut self) {
        let page = self.doc.page_mut();
        let (w, h) = (page.width, page.height);
        page.guides_v = vec![w / 3.0, w * 2.0 / 3.0];
        page.guides_h = vec![h / 3.0, h * 2.0 / 3.0];
        self.show_guides = true;
        self.status = "Added 3×3 rule-of-thirds guides".to_string();
    }
}

/// Simplify a linear contour while keeping every retained anchor within the given
/// perpendicular-distance tolerance of the original polyline. Closed contours are split
/// at a farthest pair and simplified as two open chains so the seam remains stable.
fn simplify_polyline(points: &[Vec2], closed: bool, tolerance: f32) -> Vec<Vec2> {
    if points.len() <= if closed { 3 } else { 2 } {
        return points.to_vec();
    }
    if !closed {
        return simplify_open_polyline(points, tolerance);
    }

    let start = points[0];
    let split = points
        .iter()
        .enumerate()
        .skip(1)
        .max_by(|(_, a), (_, b)| {
            (**a - start)
                .length_sq()
                .total_cmp(&(**b - start).length_sq())
        })
        .map(|(index, _)| index)
        .unwrap_or(1);
    if split == 0 || split == points.len() - 1 {
        return points.to_vec();
    }

    let mut first = simplify_open_polyline(&points[..=split], tolerance);
    let mut second_chain = points[split..].to_vec();
    second_chain.push(start);
    let second = simplify_open_polyline(&second_chain, tolerance);
    first.extend(
        second
            .into_iter()
            .skip(1)
            .take_while(|point| *point != start),
    );
    if first.len() < 3 {
        points.to_vec()
    } else {
        first
    }
}

fn simplify_open_polyline(points: &[Vec2], tolerance: f32) -> Vec<Vec2> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    let mut keep = vec![false; points.len()];
    keep[0] = true;
    keep[points.len() - 1] = true;
    let mut spans = vec![(0, points.len() - 1)];
    while let Some((first, last)) = spans.pop() {
        if last <= first + 1 {
            continue;
        }
        let a = points[first];
        let b = points[last];
        let direction = b - a;
        let length_sq = direction.length_sq();
        let mut farthest = (0usize, 0.0f32);
        for (index, point) in points.iter().enumerate().take(last).skip(first + 1) {
            let distance = if length_sq <= f32::EPSILON {
                (*point - a).length()
            } else {
                let t = ((*point - a).dot(direction) / length_sq).clamp(0.0, 1.0);
                (*point - (a + direction * t)).length()
            };
            if distance > farthest.1 {
                farthest = (index, distance);
            }
        }
        if farthest.1 > tolerance {
            keep[farthest.0] = true;
            spans.push((first, farthest.0));
            spans.push((farthest.0, last));
        }
    }
    points
        .iter()
        .zip(keep)
        .filter_map(|(point, keep)| keep.then_some(*point))
        .collect()
}

/// Polygon boolean operations backed by `geo`. `join` is handled separately because it is an
/// endpoint operation, not an area operation.
pub(super) fn polygon_boolean(a: &[Vec2], b: &[Vec2], op: &str) -> Option<Vec<Vec2>> {
    use geo::{BooleanOps, LineString, MultiPolygon, Polygon};
    if a.len() < 3 || b.len() < 3 {
        return None;
    }
    let poly = |pts: &[Vec2]| {
        let mut coords: Vec<(f64, f64)> = pts.iter().map(|p| (p.x as f64, p.y as f64)).collect();
        if coords.first() != coords.last() {
            coords.push(coords[0]);
        }
        Polygon::new(LineString::from(coords), Vec::new())
    };
    let pa = poly(a);
    let pb = poly(b);
    let result: MultiPolygon<f64> = match op {
        "intersect" | "crop" => pa.intersection(&pb),
        "punch" => pa.difference(&pb),
        _ => pa.union(&pb),
    };
    // A Path object holds one closed contour. A result with several disconnected regions, or
    // with holes, cannot be represented without silently discarding geometry, so reject it and
    // let the caller report the limitation instead of dropping part of the result.
    let mut contours: Vec<Vec<Vec2>> = Vec::new();
    for polygon in result.0 {
        if !polygon.interiors().is_empty() {
            return None;
        }
        let mut pts: Vec<Vec2> = polygon
            .exterior()
            .0
            .iter()
            .map(|c| Vec2::new(c.x as f32, c.y as f32))
            .collect();
        if pts.len() > 1 && (pts[0] - *pts.last().unwrap_or(&pts[0])).length_sq() <= 1e-10 {
            pts.pop();
        }
        if pts.len() >= 3 && polygon_area(&pts).abs() >= 0.5 {
            contours.push(pts);
        }
    }
    match contours.len() {
        1 => contours.pop(),
        _ => None,
    }
}

pub(super) fn polygon_area(pts: &[Vec2]) -> f64 {
    let mut area = 0.0_f64;
    for i in 0..pts.len() {
        let a = pts[i];
        let b = pts[(i + 1) % pts.len()];
        area += (a.x as f64) * (b.y as f64) - (b.x as f64) * (a.y as f64);
    }
    area * 0.5
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
            Some(Color32::from_rgb(60, 120, 180)),
            Some(Color32::BLACK),
        ))
    }

    #[test]
    fn convert_ellipse_line_and_gradient_to_editable_contours() {
        let mut app = FireCraft::new(None);
        app.push_object(Obj::new(
            Shape::Ellipse {
                center: Vec2::new(40.0, 50.0),
                radius: Vec2::new(20.0, 10.0),
                filled: true,
            },
            Some(Color32::RED),
            None,
        ));
        app.select_single(Some(0));
        app.convert_selected_to_path();
        assert!(matches!(
            &app.selected_object().unwrap().kind,
            Shape::Path { points, closed: true } if points.len() == 16
        ));

        app.push_object(Obj::new(
            Shape::Line {
                a: Vec2::new(3.0, 7.0),
                b: Vec2::new(13.0, 17.0),
            },
            None,
            Some(Color32::BLACK),
        ));
        app.select_single(Some(1));
        app.convert_selected_to_path();
        assert!(matches!(
            &app.selected_object().unwrap().kind,
            Shape::Path { points, closed: false } if points == &[Vec2::new(3.0, 7.0), Vec2::new(13.0, 17.0)]
        ));

        let from = Color32::from_rgb(12, 34, 56);
        app.push_object(Obj::new(
            Shape::Gradient {
                min: Vec2::new(5.0, 9.0),
                max: Vec2::new(25.0, 39.0),
                from,
                to: Color32::WHITE,
                radial: false,
            },
            None,
            None,
        ));
        app.select_single(Some(2));
        app.convert_selected_to_path();
        let converted = app.selected_object().unwrap();
        assert_eq!(converted.fill, Some(from));
        assert!(matches!(converted.kind, Shape::Path { closed: true, .. }));
    }

    #[test]
    fn path_edits_preserve_endpoints_and_snap_fractional_nodes() {
        let mut app = FireCraft::new(None);
        let points = vec![
            Vec2::new(0.2, 0.3),
            Vec2::new(5.1, 0.1),
            Vec2::new(10.2, 0.4),
            Vec2::new(10.1, 5.2),
            Vec2::new(10.3, 10.2),
            Vec2::new(5.2, 10.1),
            Vec2::new(0.1, 10.3),
            Vec2::new(0.3, 5.1),
        ];
        app.push_object(Obj::new(
            Shape::Path {
                points: points.clone(),
                closed: true,
            },
            None,
            None,
        ));
        app.select_single(Some(0));

        app.simplify_selected_path();
        let Shape::Path {
            points: simplified,
            closed,
        } = &app.selected_object().unwrap().kind
        else {
            panic!("simplifying a path must keep it a path");
        };
        assert!(*closed);
        assert_eq!(simplified.len(), 4);
        assert_eq!(simplified.first(), points.first());
        assert!((polygon_area(simplified) - polygon_area(&points)).abs() < 5.0);

        app.add_points_to_selected_path();
        assert!(matches!(
            &app.selected_object().unwrap().kind,
            Shape::Path { points, closed: true } if points.len() == 8
        ));
        app.selected_path_nodes = vec![1];
        let deleted_anchor = match &app.selected_object().unwrap().kind {
            Shape::Path { points, .. } => points[1],
            _ => unreachable!(),
        };
        app.delete_point_from_selected_path();
        assert!(matches!(
            &app.selected_object().unwrap().kind,
            Shape::Path { points, closed: true } if points.len() == 7 && !points.contains(&deleted_anchor)
        ));
        app.snap_selected_to_pixel();
        let Shape::Path { points, .. } = &app.selected_object().unwrap().kind else {
            panic!("snapping must keep the path type");
        };
        assert!(
            points
                .iter()
                .all(|point| point.x.fract() == 0.0 && point.y.fract() == 0.0)
        );
    }

    #[test]
    fn delete_point_keeps_minimum_triangle_and_join_closes_single_selection() {
        let mut app = FireCraft::new(None);
        app.push_object(Obj::new(
            Shape::Path {
                points: vec![Vec2::ZERO, Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0)],
                closed: false,
            },
            None,
            None,
        ));
        app.select_single(Some(0));
        app.selected_path_nodes = vec![0];
        app.delete_point_from_selected_path();
        assert!(matches!(
            &app.selected_object().unwrap().kind,
            Shape::Path { points, closed: false } if points.len() == 3
        ));
        app.combine_paths_op("join");
        assert!(matches!(
            &app.selected_object().unwrap().kind,
            Shape::Path { closed: true, .. }
        ));
        assert_eq!(app.status, "Closed path endpoints (Join)");
    }

    #[test]
    fn align_path_nodes_moves_all_selected_anchors_and_preserves_others() {
        let mut app = FireCraft::new(None);
        app.push_object(Obj::new(
            Shape::Path {
                points: vec![
                    Vec2::new(0.0, 0.0),
                    Vec2::new(10.0, 10.0),
                    Vec2::new(20.0, 30.0),
                ],
                closed: false,
            },
            None,
            Some(Color32::BLACK),
        ));
        app.select_single(Some(0));
        app.selected_path_nodes = vec![0, 1];
        app.align_selected_path_points(true);
        let Shape::Path { points, .. } = &app.selected_object().unwrap().kind else {
            panic!("alignment must preserve path type");
        };
        assert_eq!(points[0].y, 5.0);
        assert_eq!(points[1].y, 5.0);
        assert_eq!(points[2], Vec2::new(20.0, 30.0));
    }

    #[test]
    fn unsupported_freeform_rounding_leaves_geometry_unchanged() {
        let mut app = FireCraft::new(None);
        let points = vec![Vec2::ZERO, Vec2::new(10.0, 0.0), Vec2::new(10.0, 10.0)];
        app.push_object(Obj::new(
            Shape::Path {
                points: points.clone(),
                closed: true,
            },
            None,
            Some(Color32::BLACK),
        ));
        app.select_single(Some(0));
        app.round_selected_path_points(6.0);
        assert!(matches!(
            &app.selected_object().unwrap().kind,
            Shape::Path { points: current, closed: true } if current == &points
        ));
        assert!(app.status.contains("left unchanged"));
    }

    #[test]
    fn bitmap_color_filters_transform_channels_and_preserve_alpha() {
        let mut app = FireCraft::new(None);
        let alpha = 91;
        let index = app.push_object(Obj::new(
            Shape::Bitmap {
                at: Vec2::ZERO,
                size: Vec2::new(1.0, 1.0),
                pixels: vec![10, 100, 200, alpha],
                pixel_size: Vec2::new(1.0, 1.0),
            },
            None,
            None,
        ));
        app.select_single(Some(index));

        for command in [
            "brighten",
            "darken",
            "contrast",
            "invert",
            "grayscale",
            "sepia",
        ] {
            app.apply_color_command(command);
            let Shape::Bitmap { pixels, .. } = &app.selected_object().unwrap().kind else {
                panic!("color filters must keep bitmap data");
            };
            assert_eq!(pixels[3], alpha, "{command} changed pixel alpha");
            assert_eq!(app.status, format!("Applied color filter: {command}"));
        }
    }

    #[test]
    fn symbol_conversion_and_placement_keep_definition_and_instance_linked() {
        let mut app = FireCraft::new(None);
        rect(&mut app, Vec2::new(12.0, 20.0), Vec2::new(52.0, 60.0));
        app.select_single(Some(0));
        let symbol_id = app.doc.symbols.len();
        app.convert_selection_to_symbol();

        assert_eq!(app.doc.symbols.len(), symbol_id + 1);
        assert_eq!(app.doc.symbols[symbol_id].id, symbol_id);
        assert_eq!(app.doc.symbols[symbol_id].objects.len(), 1);
        assert_eq!(app.doc.symbols[symbol_id].name, "Rectangle Symbol");
        assert!(matches!(
            app.selected_object().unwrap().kind,
            Shape::SymbolInstance { symbol_id: id, .. } if id == symbol_id
        ));

        app.place_symbol_instance(symbol_id);
        assert!(matches!(
            app.selected_object().unwrap().kind,
            Shape::SymbolInstance { symbol_id: id, .. } if id == symbol_id
        ));
        assert!(
            app.status
                .contains(&format!("Placed Symbol #{}", symbol_id + 1))
        );
    }

    #[test]
    fn tween_without_selection_is_a_noop_and_selected_tween_adds_three_states() {
        let mut app = FireCraft::new(None);
        app.tween_selected_across_states();
        assert_eq!(app.doc.page().states.len(), 1);
        assert!(app.status.contains("Select an object"));

        let index = rect(&mut app, Vec2::new(10.0, 20.0), Vec2::new(30.0, 40.0));
        app.select_single(Some(index));
        app.tween_selected_across_states();
        let page = app.doc.page();
        assert_eq!(page.states.len(), 4);
        for (step, state) in page.states.iter().skip(1).enumerate() {
            let obj = &state.layers[0].objects[1];
            assert!((obj.bounds().unwrap().0.x - (10.0 + (step + 1) as f32 * 36.0)).abs() < 0.001);
            assert!((obj.opacity - (1.0 - (step + 1) as f32 * 0.18)).abs() < 0.001);
        }
    }
}
