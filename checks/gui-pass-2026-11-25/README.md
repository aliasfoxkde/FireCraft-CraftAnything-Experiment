# GUI pass — 2026-11-25

Captured with `scripts/gui-shot.sh` (Xvfb + software GL, the app's own `--screenshot` path), so
the images are the exact client area at the requested size.

| File | State |
|------|-------|
| `before-1360.png` | before the pass: radial gradient disc on the page, slice URL painted over the CTA label, selection handles on the brand emblem, hover-scrolled inspector |
| `after-1360.png` | after: clipped radial gradient with an opaque outer stop, URL tab below the slice, no selection on load, properties scroller |
| `after-960.png` | after, minimum window: the showcase now fits (28% zoom) instead of keeping the 1360 px zoom and clipping the page |

## Defects fixed in this pass

1. Radial gradients painted unclipped circles outside the object and left the rect corners
   unpainted. Now clipped to the object, with an outer-stop rect fill so the shape is seamless.
2. Slice URL labels were painted inside the slice rect, covering the button label underneath. Now
   a tab below the slice, elided to the slice width.
3. `text_width` counted newline characters, so multi-line text had an oversized selection box and
   made card text look like it overflowed. Width now uses the longest line; height uses line count.
4. The showcase did not fit the window on first frame (`fit_in_window` ran before the canvas rect
   existed). Fitting is deferred to the first frame with a live rect, for any window size.
5. The demo opened with selection handles over the brand emblem. Now opens unselected.
6. The properties inspector clipped its second row under the 118 px dock. Now scrolls.
7. Left-rail hint text was clipped mid-sentence. Now wraps.
8. The Optimize panel's Matte control overlapped its own label. Now on its own row.
9. CTA label baseline sat in the upper third of its button. Now vertically centred.

## Still open (tracked in `docs/TASKS.md`)

- The right dock's first header strip renders as an empty bar (`»` + grip dots) above the tab row.
- The Color Mixer's last control is clipped when its panel group is short.
- The top-right workspace cluster has no spacing at 960 px.
- Toolbar's Bridge button is a text label (`Br`) while its neighbours are icons.
