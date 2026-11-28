# Canvas tools, effects, and rendering quality plan

**Status:** active defect pass plus staged architecture work. The task ledger in
[`TASKS.md`](TASKS.md) records completion only after proof. This plan distinguishes current code,
reference observations, fixes in progress, and future capabilities.

## Goal

Make each canvas tool predictable for its target, preserve editability and undo, render the same
result in the editor and exports, and provide Fireworks-like controls without implying that a
minimal parameter row is a complete raster/vector engine.

## Current baseline and findings

- `app/src/app.rs` owns pointer state transitions; `app/src/canvas.rs` converts viewport coordinates,
  hit-tests, draws objects and selection overlays; `app/src/doc.rs` stores vector and bitmap data;
  `app/src/formats.rs` has export rasterization. Tool options live in `panels/tools.rs` and
  `panels/properties.rs`.
- Startup presentation used to paint an opaque full-window scrim and a pretend progress bar, return
  early, and hold the editor UI for 2.2 seconds before opening a separate Welcome modal. The new
  design is a maximum 2.4-second alpha-fading transparent logo overlay over the live editor; the
  Welcome screen is opt-in. It does not report fake loading progress or block pointer input.
- The Eraser used to delete any active-layer object whose bounding box covered the pointer. It did
  not subtract image pixels. The active fix now writes feather-weighted alpha changes to a bitmap
  and preserves its object identity. Freehand tool samples also bypass guide/grid snapping; a
  regression test showed a dab near the page origin was silently snapped to (0, 0), away from the
  pointer location.
- Bitmap textures were cached by `(layer_index, object_index)`. Reordering changes those slots, so
  two images can render each other's cached texture. A first allocation-address fix was invalidated
  by the renderer's per-frame document clone. The corrected fix keys by pixel-content hash and
  dimensions, so it survives reorder and snapshot cloning while pixel edits invalidate the entry.
  Regression tests cover different colors, cloned pixels, changed pixels, and hotkey reorder.
- Concave Star fill was drawn as many separately-antialiased triangles from a center point. Their
  shared edges could expose background seams/spokes. It now uses one shared-edge triangle mesh.
  Selection rectangles/handles are now limited to selection/transform tools, avoiding an extra
  transform box while a shape tool is active.
- Closed `Path` and `Polygon` objects previously rendered their fill in the editor and SVG but were
  exported to PNG as outlines only. Raster export now fills valid simple contours by scanline after
  applying the shared path triangulation validation, and samples the object's gradient on that fill.
  A real Chromium create/edit/undo/redo/save/reopen/export run confirmed the orange fill survives in
  PNG. Wider shape/effect parity and native Windows/macOS acceptance remain open; see the
  [browser workflow receipt](testing/browser-path-workflow-2026-11-28.md).
- `GradientFill` currently contains two colors and vertical-only rendering. The separate
  `Shape::Gradient` contains two colors and a radial boolean but lacks arbitrary stops and a
  draggable direction handle. Task 37.3 specifies the schema/interaction work before changing
  serialized project data.
- Live Effects are non-destructive and rendered for supported types, but the old Properties UI
  exposed only effect enable checkboxes. Parameter/color controls and undoable updates are being
  added. Every effect must still be proven visually for each supported shape type and at export.
- FireCraft's tool shortcut/action map remains hardcoded in `layout.rs`. `Shift+Q` now toggles to
  the last distinct tool, retaining current grouped-tool cycling. A single customizable action
  registry is a later architecture task, not a prerequisite for this local fix.

## Reference-derived behavior contracts

The 2026-11-27 review in [`research/emberframe-bonbon-audit.md`](research/emberframe-bonbon-audit.md)
compares Emberframe and BonBon/Libreworks source revisions. Transferable, testable contracts:

1. A bitmap paint/erase action affects the bitmap under the pointer; vector-mode paint outside an
   image is an explicit and separately documented behavior.
2. Brush size, softness, opacity, tip, target mapping, selection clipping, undo grouping and texture
   invalidation form one stroke contract. A list of UI controls without a shared stroke engine is
   insufficient.
3. Image pixel editing uses explicit document↔bitmap coordinate transforms, including non-square,
   scaled and transformed image bounds.
4. Gradients need a type, ordered stops, editable vector handles, clear modifier behavior, no
   degenerate-vector crashes, and stable round-trip serialization.
5. Effects expose real parameters, can be toggled/reordered/removed, render non-destructively, and
   participate in exact undo/redo. Unsupported combinations must explain themselves.
6. Reorder, grouping, duplication, flattening, page/state/document changes and undo must never
   cross-wire cached render data. Cache identity follows bitmap content, not a mutable array slot
   or a temporary per-frame allocation.
7. Each tool has a visible active/hover/flyout state, selection feedback appropriate to the active
   tool, a keyboard alternative, and tests for both successful and rejected targets.

## Phased work plan

### Phase A — Stop data-loss and wrong-render defects

1. Finish previous-tool action and shortcut-map/help proof.
2. Finish bitmap texture identity fix; cover reorder-forward/back, send-front/back, undo/redo,
   layer/page/document switches, delete-then-add, duplicate, flatten, and dimension changes.
3. Finish Eraser alpha compositing; cover one-click dabs, interpolated drags at minimum brush size,
   feather extremes, scaled/non-square bitmaps, transparent pixels, locked/hidden objects/layers,
   selection constraints, empty targets, unsnapped freehand input, undo/redo and export.
4. Fix the Star triangle seams; capture before/after at low and high zoom and verify flat fill,
   gradient fill, stroke, selection overlay, and export rasterization.
5. Reproduce the intermittent object outline with a deterministic GUI sequence. Separate create
   preview, selected-object bounds, auto-shape handles, and stroke styling; keep only intentional
   affordances for the active tool.

**Exit:** focused regression tests pass; full unit/harness tests and clippy pass; no affected object
is silently deleted or rendered with another bitmap's pixels; exports agree with canvas pixels.

### Phase B — Make existing filters actually useful

1. Give every listed filter a compact Fireworks-style row: exact name, enabled toggle, summary,
   disclosure for editable parameters, color chip where applicable, remove/reorder controls.
2. Route parameter/toggle/reorder/remove edits through one undoable app command seam. Avoid pushing
   duplicate snapshots for unchanged values.
3. For each effect, declare supported object types and render path. Test visible deltas for shapes,
   text, paths, groups and bitmaps where claimed; test zoom scaling, clipping, opacity, blend, and
   export. Implement missing paths or label them unsupported.
4. Revisit effect algorithms (blur kernel/quality, shadow silhouette/offset/softness, inner effect
   clipping) using deterministic pixel/geometry tests and visual snapshots. Do not claim parity with
   Fireworks solely from matching names.

**Exit:** feature-specific renderer tests prove the effect changes pixels/paint primitives, undo
restores the prior value exactly, and no effect leaks outside its object's bounds unexpectedly.

### Phase C — Gradient model and interaction

1. Write the model decision: gradient kind, sorted normalized stops, interpolation space, normalized
   start/end handles, degenerate behavior, serde defaults and project-version policy.
2. Keep old two-stop files loading as an equivalent two-stop gradient. Add old/new fixtures before
   renderer changes.
3. Implement linear and radial first; add arbitrary stop insertion/deletion/repositioning, color
   editing, and drag handles on the selected shape. Provide Properties numeric/keyboard controls.
4. Implement one consistent command/undo seam. Use the same sampler in canvas rendering and export.
5. Add tests for endpoint and midpoint colors, stop ordering, invalid positions, zero-length vectors,
   radial center, selection bounds, affine transforms, save/load, and output parity. Add other
   Fireworks gradient types only as separate increments with their own acceptance cases.

**Exit:** stops, type, direction and colors survive save/load and transforms, can be changed without
leaving the canvas, and canvas/export sample equivalently.

### Phase D — Shared raster stroke architecture

1. Define an immutable `StrokeParams` snapshot and typed `StrokeSample` input (document point,
   pressure/tilt/time optional) separate from egui event handling.
2. Map document coordinates to source bitmap pixels with explicit inverse transforms. Reject or
   explain locked, hidden, unsupported-effect, or missing targets.
3. Convert input paths to deterministic coverage masks with brush size, feather/edge, antialias,
   tip, opacity, pressure, blend and selection clipping.
4. Composite from the stroke-start snapshot so dab overlap is stable and erasing/painting obey the
   same mask semantics. Record one undo transaction per gesture and a bounded dirty region.
5. Keep vector stroke creation a deliberate tool mode. Document behavior on and off bitmaps.
6. Benchmark large images and long strokes; include cancellation, frame coalescing and memory caps.

**Exit:** deterministic engine tests cover all mask/composite math; UI integration tests cover actual
gestures; frame, memory and undo snapshots stay within documented budgets.

### Phase E — Fireworks-inspired Properties and tool UX

1. Inventory every tool and option against checked-in CS6 screenshots and existing manuals. Record
   implemented/partial/missing/unsupported and exact proof.
2. Group Properties controls by Fill, Stroke, Geometry, Filters and Bitmap Options. Preserve
   keyboard focus and compact density; add advanced disclosures rather than hiding core settings.
3. Add operation-specific status/help, shortcuts in tooltips, target-sensitive cursors, and visible
   rejection reasons.
4. Review 960×640, 1360×860, high DPI, light/dark/system, pop-out canvas, keyboard-only and WASM.

**Exit:** no clipped labels or inaccessible options in the documented matrix; every visible control
changes a tested model property or carries an explicit unavailable reason.

### Phase F — Tablet and advanced brush capabilities

1. Define device/input support matrix and the no-tablet fallback.
2. Add pressure-size/opacity, tilt and stabilizer as engine parameters only after the shared engine
   is stable.
3. Add brush tip/preset resources and user preset import with license/provenance policy.
4. Validate with actual Windows/macOS/Linux tablets and touch devices; keep each platform claim tied
   to a reproducible receipt.

**Exit:** real-device tests plus deterministic synthetic sample tests; no support claims from
unexercised event fields.

## Progress and release discipline

Active items are [37.1–37.7 in `TASKS.md`](TASKS.md). Keep each independently reviewable and run
format, clippy, unit tests, interaction harness, and exact affected WASM/native build checks. Record
the commit SHA, evidence paths, and any platform gaps in `PROGRESS.md`. Do not release until Phase A
is green and user-facing filter/gradient feature claims match their tested implementation.
