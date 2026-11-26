# Optimize and Export

## The Optimize panel (F6)

The Optimize panel sets the export format for the current document: PNG, GIF,
JPEG, WebP and more. Switch the format and the panel shows that format's
controls (palette size for GIF, quality for JPEG).

## Image Preview

**Ctrl+Alt+X** opens the Image Preview dialog: a side-by-side before/after
with the projected file size, so you can tune quality before committing.

## Slices and hotspots

- **Rectangle Slice (K)** draws an export region on the canvas. Each slice
  can override the document format in the Optimize panel when selected.
- **Rectangle Hotspot (J)** draws a web link area — exported into HTML as an
  image map.

## Exporting

- **Ctrl+Shift+R** (File → Export) writes the document (or selected slice) in
  the optimized format.
- Export as **HTML** writes an image-map page including slices and hotspots.
- GIF exports of pages with multiple states write an animated GIF.

## Copying as CSS

Select an object and press **Ctrl+Alt+C** to copy CSS reflecting its
geometry, fill, stroke, and effects — handy for hand-off to web work.
