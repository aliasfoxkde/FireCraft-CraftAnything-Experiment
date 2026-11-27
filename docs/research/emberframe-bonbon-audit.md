# Emberframe and BonBon/Libreworks audit

**Reviewed:** 2026-11-27. This is a source-and-documentation review for clean-room product
planning, not an endorsement, compatibility claim, or exhaustive third-party security audit.

## Source identity and boundaries

| Project | Reviewed revision | License reported by repository | Implementation |
|---|---|---|---|
| [Emberframe](https://github.com/gregstein/emberframe) | `64dcd718dd0308e6d771f55e3d96783574a5731c` | MIT | Rust + GPUI; its README describes an early 0.1 product and lists substantial limitations. |
| [BonBon / Libreworks](https://github.com/librebon/BonBon) | `8e11e2e0526e07abc23452ffc945c3e2716835f7` | GPL-3.0 | C++17 + Qt 6; README and source tree expose a broad editor and paint-engine architecture. |

The repositories were cloned read-only to `/tmp` for inspection. No source, assets, or test
fixtures were copied into FireCraft. BonBon's GPL-3.0 code is not a drop-in source for this MIT
project. Emberframe's MIT notice is permissive, but this audit uses its behavior and architecture
as reference; any future code reuse still needs a file-level attribution and dependency review.

## Useful behavior and architecture

### Emberframe

The user guide makes a valuable distinction: Brush, Pencil, and Eraser edit the bitmap beneath the
pointer; Brush and Pencil draw vector strokes when the pointer is off a bitmap. It also documents
pixel selections constraining paint, fill, and filters; copy/paste of selected pixels as a new
bitmap; Alt-click source selection for the clone tool; Enter-to-finish and first-node-to-close for
the Pen; and explicit separation between saving an editable native document and exporting a flat
image. These interaction contracts are useful because they describe what each tool does at its
target and what happens outside the target.

Its documented limitations are equally instructive: the 0.1 Pen is straight-segment only, masks and
live effects in Styles are absent, bitmap paste/move behavior is incomplete, and only flat images
are read from layered Fireworks PNGs. A tool inventory must not imply those deeper behaviors are
finished.

### BonBon / Libreworks

The source separates a brush parameter model from the stroke engine. Its documented brush state
includes size, edge softness, tip shape/category, texture, texture amount, opacity, blend mode,
color, erasing, preserve-transparency, pressure response, tilt, and calligraphic nib geometry. The
stroke API models sampled path segments, damage bounds, deferred rendering, snapshots, and
recomposition from a base image plus stroke coverage. Its comments explain a key quality property:
stroke opacity is applied to the whole coverage mask, so overlapping dabs do not accidentally
darken a normal brush stroke. This is a strong architecture direction for FireCraft's future brush
engine, beyond simply adding more sliders.

Its pixel-edit module formalizes document-coordinate ↔ image-pixel transforms and distinguishes
effects that can be baked losslessly from effects that must block an edit. Its fill module exposes
gradient stops, gradient kind, presets, handle-to-document transforms, vector updates from a drag,
fill-value sampling, and shared handle logic for gradients and patterns. README behavior additionally
describes multi-axis gradient handles and a stable modifier contract (constrain angle vs scale
symmetrically); this is a helpful design reference for a future FireCraft gradient editor.

The README describes configurable keyboard sets with one live binding source, explicit inheritance
vs unbinding, conflict reassignment, current shortcuts in tooltips, and keyboard search/tree UI.
Other useful contracts include paste-at-pointer as a preference, per-document brush/tool state,
drag-and-drop image placement, and keeping tool-setting changes out of document dirty state.

These are upstream claims, not independently verified feature acceptance. The source tree contains
dedicated `BrushEngine`, `PixelEdit`, `Fills`, `Stroke`, `SelectionMask`, `Effects`, `Render`,
`Tablet`, and test modules, so the architecture claims have inspectable implementation owners.

## Direct comparison to FireCraft at the reviewed revision

| Area | FireCraft state found | Recommendation / task |
|---|---|---|
| Bitmap Eraser | `app/src/app.rs::erase_at` used to remove every object whose bounding box contained the pointer. It did not edit bitmap alpha. | Replaced with bitmap alpha painting in Phase 37.2. Validate scaled/non-square images, small brush continuous strokes, locked layers, undo/redo, and browser/native behavior before closing. |
| Brush model | One global brush size; Eraser now has one global feather ratio. Stroke and image operations do not yet share a full target-aware coverage engine. | Plan a typed, deterministic stroke engine with pixel-space mapping, coverage masks, selection clipping, opacity, antialiasing, tip presets, pressure/tilt, and bounded dirty-region updates. Preserve simple mouse defaults. |
| Gradient editor | `GradientFill` stores only start/end colors; `Shape::Gradient` stores a rectangle, two endpoint colors, and radial bool. Linear rendering is vertical, without editable direction handles or a multi-stop model. | Task 37.3: versioned/defaulted multi-stop/type/handle model, on-canvas start/end handles, keyboard alternatives, interpolation tests, old-document fixtures, render and export parity. Keep the current two-stop appearance for old files. |
| Shape filters | Effects are stored non-destructively and some are painted, but Properties only offered enabled checkboxes; there were no usable per-effect parameter controls. | Task 37.4 adds editable parameters and color, undo, rendering evidence at zoom levels, and a broader CS6-style inspector review. Verify each listed effect on the shape kinds it claims to support. |
| Keyboard map | Tool bindings are hardcoded in `layout.rs`; grouped tools cycle on repeated keypress. | Added `Shift+Q` Previous Tool in 37.1. Long-term, move bindings to a typed action registry only when menus, help, command palette, customization, conflict detection, and tests can all share that source. |
| Pixel selection/masks | Marquee/Lasso/Wand selection and raster editing exist, but the app does not yet offer one shared pixel coverage mask used by all paint/fill/filter operations. | Plan a selection-mask contract before adding more bitmap tools; tests should cover add/subtract/intersect, transforms, clipping, persistence policy, and exact undo. |

## Ordered implementation plan

1. **Stabilize reported behaviors:** finish tests for alpha erasing, segment interpolation, scale mapping,
   lock/visibility checks, selection clipping, undo and texture invalidation. Complete the current
   Previous Tool test and keyboard map/help documentation.
2. **Make existing controls truthful:** add parameter editing and undo for each existing live filter;
   add renderer tests proving visible effect deltas for bitmap and vector targets. Explicitly mark
   unsupported effect/shape combinations in UI instead of silently no-oping.
3. **Design the gradient document extension:** decide enum and stop representation, normalized
   document-space handles, degenerate-vector rules, color interpolation space, default migration,
   export behavior, and versioned fixtures. Then implement vector/radial type, arbitrary stops,
   on-canvas handles, keyboard editing, previews, and undoable edits.
4. **Introduce a shared raster edit seam:** immutable stroke parameters + sampled input → target-aware
   coverage mask → mask/selection/alpha compositing → one undo transaction and dirty rectangle.
   Keep vector strokes as a separate explicit mode or document the chosen target behavior.
5. **Extend input and presets:** add brush tip presets, opacity/blend, pressure/tilt and stabilizer
   only with deterministic mouse fallback, per-device tests, and actual tablet receipts. Do not claim
   tablet support from event fields alone.
6. **Polish Properties to the CS6 workflow:** organize fill/stroke/effects as compact sections with
   concise defaults and expandable controls; test 960×640, large screens, keyboard navigation,
   high-DPI and all effect states. Use repository screenshots for reference; draw original UI assets.
7. **Release only after proof:** run format, clippy, unit and harness tests; WASM/native checks;
   real browser clipboard/eraser checks and the relevant platform GUI matrix; update the task board,
   progress receipt and changelog with exact SHA and unverified limitations.

## References

- [Emberframe README](https://github.com/gregstein/emberframe) and [user guide](https://github.com/gregstein/emberframe/blob/main/docs/guide.md)
- [Emberframe bitmap operations](https://github.com/gregstein/emberframe/blob/main/src/bitmap.rs)
- [BonBon / Libreworks README](https://github.com/librebon/BonBon)
- [BonBon brush engine](https://github.com/librebon/BonBon/blob/main/src/core/BrushEngine.h), [pixel editing](https://github.com/librebon/BonBon/blob/main/src/core/PixelEdit.h), [fills and gradient handles](https://github.com/librebon/BonBon/blob/main/src/core/Fills.h)
- Existing FireCraft task tracker: [TASKS.md](../TASKS.md)
