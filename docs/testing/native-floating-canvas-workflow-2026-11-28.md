# Native floating canvas workflow — 2026-11-28

## Build and environment

- FireCraft 0.3.8 local debug build from the current working source; not a release binary.
- Base source commit: `e06b4adde8faee9cc726aa808be8b2d2ccc3c52a`.
- Working source patch SHA-256 (canvas hit routing, floating-document restore, and regression test):
  `aa0f889645b0ed9f65d89d197ff700f596bd4e7da3837a3ae970a91ab3ecc675`.
- Binary `app/target/debug/firecraft` SHA-256:
  `85096e8f0462e06f33d97fb44ec1b708ecd1e25459c93b7c46189bfb27be63c4`.
- Linux x86_64, Xvfb `:98` at 1360×900, Openbox, software GL; main window 960×640 and floating
  native window 860×620. Input was delivered by X11 mouse events.

## Failure and cause

In the real native window, the Float command opened an OS window, but rectangle gestures did not
persist. The real-frame regression showed two coupled issues:

1. On the child viewport press frame, egui delivered the pointer inside the canvas bounds but
   reported `Response::hovered()` false. The canvas therefore did not begin the drag.
2. With a single floated document, the native viewport loop synchronized the child edit into
   `open_docs`, then restored a stale `saved_doc` to `app.doc`. The next frame copied that stale
   document over the just-edited tab.

The canvas now accepts a primary press inside its own bounds when the child viewport delivered it,
and continues routing the active drag. At the end of the viewport loop, the active `app.doc` is
restored from the synchronized `open_docs` slot, with the saved document as the empty-list fallback.

## Native interaction result

1. Launched the current native demo build and clicked the document's Float control.
2. Selected Rectangle in the child window and dragged on the visible demo page.
3. Confirmed the orange rectangle appeared, the Layers row showed `Layer 1 (22)` (from 21), and
   Properties showed the selected rectangle.
4. Closed the floating OS window using its title-bar close control.
5. Confirmed the rectangle and updated layer count remained in the main window.

Evidence: [edited native floating window](evidence/popout-native-2026-11-28/canvas-edited-in-native-window.png)
and [main canvas after close](evidence/popout-native-2026-11-28/canvas-edited-after-close.png).

## Automated regression and limits

- `harness::tests::floating_canvas_pointer_edit_updates_its_document` first failed with the real
  one-document, active-floating setup (`22` objects instead of `23`). It passes after the fix and
  runs an additional parent frame to verify the edit survives the next synchronization cycle.
- `harness::tests::floating_canvas_dispatches_child_viewport_keyboard_paste_to_its_document` keeps
  native child keyboard routing isolated from a separate docked document.
- Final local validation on this source tree: 226 unit/harness tests passed (one existing isolated
  timing test was ignored in the full suite), 12 CLI, 5 hygiene, and 10 MCP tests passed; strict
  Clippy and rustfmt passed. The isolated 500-object frame-time test passed. WASM target check,
  documentation link/claim check (83 Markdown files), module-size gate, and macOS plist test passed.
- `cargo llvm-cov --locked --summary-only`: 71.74% line, 75.20% function, and 73.66% region
  coverage; branch counters are unavailable for this target. This is repository-wide coverage, not
  a claim that each feature or workflow is 99% covered.
- The Xvfb run proves the Linux native child-window create/edit/close path. It does not cover
  save/reopen, drag/drop between monitors, multiple native windows, high-DPI displays, or Windows
  and macOS window managers. Those remain open; see [the WASM workflow receipt](browser-popout-workflow-2026-11-28.md)
  for the separately verified embedded browser behavior.
