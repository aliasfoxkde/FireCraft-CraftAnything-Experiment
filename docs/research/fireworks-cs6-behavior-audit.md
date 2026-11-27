# Fireworks CS6 behavior audit for FireCraft canvas tools

**Reviewed:** 2026-11-27 against Adobe's [Fireworks CS6 reference PDF](https://helpx.adobe.com/en/pdf/cs6/fireworks_reference.pdf),
the checked-in `screenshots/Fireworks-*.jpg` captures, and the current FireCraft app source. This is
a user-visible behavior comparison, not a comparison with Adobe's proprietary source code. Adobe's
original implementation source is not available in this repository. Feature names alone are not
evidence of equivalent behavior.

## Findings

| Workflow | Fireworks CS6 documented behavior | FireCraft implementation observed | Assessment and next work |
|---|---|---|---|
| Raster Eraser | Edits pixels in a bitmap/selection; round or square tip; Edge, Size and Opacity controls. | Previously removed whole objects by bounding-box hit. Active fix edits alpha on bitmap pixels with size and feather; always round and fully opaque erase. It currently targets the topmost eligible bitmap in the active layer and does not yet honor bitmap selection masks. | **Major functional gap; partial fix underway.** Add square/round, opacity and edge profile; test target-layer rules, selection masks and interpolation on scaled/non-square images. See 37.2. |
| Brush/Pencil | Drag paints pixels and creates bitmap objects on the current layer; Pencil has antialias/auto-erase/preserve-transparency options. | Brush/Pencil drags currently create vector `Shape::Path` objects; there is no unified raster stroke engine. | **Different editing semantics.** Implement the shared raster stroke engine only after target mapping, coverage, selection, compositing, undo and performance contracts are defined. |
| Paint Bucket | Flood-fills connected pixels within a tolerance; supports gradient fill of a pixel selection. | `Tool::Fill` sets the selected object's fill or creates a full-page rectangle; it is not a raster flood-fill algorithm. | **Not equivalent.** Add scanline/flood-fill coverage with tolerance, antialias and selection mask as a separate raster command with pixel fixtures. |
| Vector Pen | Clicks create straight segments; click-drag creates Bezier curve points; anchors/handles are editable with Subselection; open paths can be resumed/merged. | Clicks build straight `Shape::Path` points; there are no Bezier control handles. | **Partial, major feature gap.** Preserve original points when adding segment/handle types. Define anchor semantics, closure, continuation, join/split and keyboard behavior. Test geometry and serialization before UI. |
| Gradient fill | Multiple color and opacity stops, editable positions, multiple gradient types, drag-defined direction/length, on-canvas move/rotate/skew/width handles and keyboard-constrained rotation. | `GradientFill` has two color endpoints and draws vertically. `Shape::Gradient` has two colors and a radial bool; no stop list or on-canvas edit handles. | **Major feature gap.** Task 37.3; plan schema migration, handles, stop editor, type picker, sampler and canvas/export parity. |
| Live Filters | Non-destructive for vector, bitmap and text; settings editable after application; list can reorder, enable/disable, remove; combining order affects appearance. | Effect model stores multiple variants. Canvas paints drop shadow/outer glow and a rectangle-edge bevel; inner shadow, inner glow, blur and color adjustment do not have equivalent generic vector render paths. Properties previously had enable-only rows; new parameter controls are in progress. | **Partial and likely source of “does nothing” reports.** For each filter, test renderer and object types; add proper masks/silhouette clipping and undoable parameter/order/delete UI. Avoid claiming unimplemented filters. |
| Star/custom polygon fill | Filled custom shapes render without internal spokes; the visible outline is the configured stroke or active selection path. | Star fill was emitted as separate antialiased triangle wedges from center, which can show background seams. Selection transform box was painted even while a shape tool was active. | **Rendering defect fix underway.** Shared-edge mesh fill and selection-tool-only bounds are implemented; verify screenshots at multiple zooms and export. |
| Selection bounds | Pointer/selection tools show object selection; Fireworks also offers hiding selection paths to inspect web output. | Transform handles/bounds are drawn for selected objects regardless of active tool. | **Mode mismatch fixed in current pass** by limiting transform bounds to select/subselect/scale. Verify auto-shape handles remain independent and keyboard-only workflows still show enough state. |
| Bitmap cache / reorder | The same image remains the same image when objects are rearranged, reordered, or grouped. | Texture cache key was `(layer, object-index)`, so swapping two bitmap objects could retrieve the other bitmap's texture. Address keys also fail because the canvas renderer clones the document before drawing. | **Confirmed FireCraft defect.** Cache identity now follows pixel-content hash plus dimensions; tests cover reorder, cloned snapshots and pixel edits. Test undo, page changes, delete/reuse, flatten, duplicate and memory cleanup. Hashing every bitmap each frame remains a performance cost to measure. |
| Tool/keyboard options | Fireworks exposes Keyboard Shortcuts preferences, sets and custom/secondary bindings. | FireCraft has a hardcoded key match in `layout.rs`. | **Basic bindings only.** `Shift+Q` Previous Tool toggle added. Later registry work should cover customization, conflict checks, persistence, menu/tooltips/help and tests from one action source. |

## Evidence and implementation boundaries

The Adobe manual explicitly describes the Eraser's round/square, Edge, Size and Opacity options;
multi-color and opacity gradient stops; fill-handle movement/rotation/skew/width; drag-established
gradient direction; and editable/ordered/toggleable Live Filters. See its sections “Erase pixels in
a bitmap object or selection,” “Edit a gradient fill,” “Transform and distort fills,” “Draw a path
with curved segments,” and “Applying Live Filters.” FireCraft should use these as observable
acceptance scenarios, not copy Adobe source, images, or UI artwork.

The direct third-party source review is separate: [Emberframe and BonBon/Libreworks audit](emberframe-bonbon-audit.md).
It confirms an alternate open implementation of bitmap-target paint and a coverage-mask brush
engine, but those projects are not Adobe source. BonBon/Libreworks is GPL-3.0; its implementation
must not be copied into this MIT codebase.

## Verification plan

1. Keep one test table per tool: valid target, invalid target, active selection, locked/hidden state,
   zoom transform, undo/redo, keyboard path and export result.
2. Use small named RGBA fixtures with expected pixels for eraser, brush, bucket, filters and
   gradients. Cover transparent and semi-transparent input, boundary pixels, 1×/non-square images,
   scaled images and selection masks.
3. For vector geometry, use numeric invariants and round-trip fixtures for path anchors/handles,
   path closure, hit testing, boolean operations, transforms and self-intersections.
4. For rendering, use headless egui harness outputs plus before/after screenshots at 960×640,
   1360×860, low/high zoom, selected and deselected, shape tool and selection tool.
5. For serialization/export, round-trip current and legacy fixtures and compare canvas/export pixel
   samples at the same coordinates.
6. For performance, record image dimensions, stroke sample count, algorithm complexity, render time
   and memory before adding richer brush algorithms. Never use a “smooth” claim without measured
   interactive input-to-paint evidence.

## Status

Active code changes and tasks are tracked under [Phase 37](../TASKS.md). **The Eraser, filter controls,
texture cache, star fill and Previous Tool changes are not fully closed until formatting, clippy,
the complete test suite, GUI snapshots and relevant platform checks pass.** The richer brush,
Paint Bucket, Bezier Pen and full gradient/filter parity remain planned work, not completed features.
