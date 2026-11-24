# Infinite Canvas / God View

## What it is

The **∞ Canvas** toggle switches the canvas from a page-clipped editing view to an
editing-only **god view**:

- the page background extends across the entire viewport;
- objects outside page bounds are rendered and remain selectable;
- hidden layers and hidden objects are shown as low-opacity ghosts;
- the page edge becomes a dotted outline labeled “PAGE BOUNDS · flat exports crop here”;
- a legend reports how many hidden elements are being ghosted.

This is a visualization/editing mode, not a change to the document size or export behavior.

## Toggle

- Canvas footer: **∞ Canvas**
- `Ctrl+Shift+F`
- Command palette: “Infinite Canvas” / god-view entries

## Invariants

1. `infinite_canvas` affects rendering only. It must not be serialized into the document.
2. Flat raster, SVG, and HTML exports remain page-bounded. Layered `.fw.png` retains off-page
   objects because it embeds the document, not just the rendered page.
3. Preview mode (`Preview`, `2-Up`, `4-Up`) is export-faithful and does not use god view.
4. Hidden objects are ghosted, never unhidden. Selecting/editing still requires a visible,
   unlocked layer/object through normal hit testing.
5. Panning in god view must not snap back to center just because the page fits in the viewport.

## Rendering path

`canvas::render_viewport` chooses the draw clip:

- normal: `page_rect ∩ viewport_rect`
- god view: entire viewport

`canvas::draw_page` receives `god_view` and includes hidden layers/objects at reduced alpha.
The canvas fill, checkerboard, page outline, and legend are drawn before overlays such as rulers,
selection handles, guides, and the context menu.

## Verification

- Toggle on/off and confirm the background fills the viewport without changing the document.
- Place an object outside the page; confirm it renders in god view and disappears in Preview.
- Hide a layer; confirm it ghosts in god view and disappears in Preview/flat export.
- Pan far from the page and confirm the view does not recenter.
