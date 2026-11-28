# Web canvas window workflow — 2026-11-28

## Build identity and environment

- FireCraft 0.3.8 local release-mode WASM build; this is a review artifact, not a release or deployment.
- Base source commit: `f82ddf3a35149db4de82a52757df8d3265405e36`.
- Working source patch SHA-256 (layout, chrome, and regression harness diff):
  `5746915725ac2fa1fffd7059a683bda6f3f6cb41fd4ef9f01e76f069126fb3ec`.
- Source file SHA-256: `layout.rs` `fee79059401be3ee082e9791b4c5fa0982764bdbf849d93ef52e12f927ba1959`,
  `layout/chrome.rs` `f0fad97be799ad6ef69c80d4cf286136e4ed8ccb814c30ba8c99d32f873cdd25`,
  `harness.rs` `c5a2a790c5c72a973661587c6a214a2feaad16f71f9e270ebe1246afba385af2`.
- WASM input SHA-256: `e28bfc831ae4b8c0b1940ac492bf24f9454f382d59b85f5cb7cfb9d9337e01e2`.
- Served `firecraft_bg.wasm` SHA-256: `dc2f3a82b37a21e5e4d84ffe382ebb32683a1b255dba6f6651da2252664b9307`.
- Chromium 154 headed on Fedora/Xvfb `:96`, 1360×860 CSS viewport, WebGL2/SwiftShader.

## Reproduction and fix

On the previous browser build, selecting **Float** left a placeholder in the main canvas and no
editable canvas appeared. The WebAssembly runner uses embedded viewports and cannot open a separate
OS window. The old UI treated the document as if an external viewport existed, so the user lost the
visible editing surface.

When `egui::Context::embed_viewports()` is true, FireCraft now renders each floated document in a
movable in-workspace window. The canvas is the normal editable canvas and shares the document with
the main tools and Properties/Layers panels. The main workspace explains where the canvas went;
**Dock to Main Workspace** and the title-bar close button return it to the document tabs. Native
external viewport behavior remains on the non-embedded backend.

## Headed browser interaction

The final served WASM artifact was opened in Chromium. Using actual pointer input through X11, I:

1. Opened the document in a movable canvas window.
2. Selected Rectangle and dragged on that window's canvas.
3. Confirmed the orange rectangle appeared in the canvas and the Layers panel listed it.
4. Closed the floating window with its title-bar **X**.
5. Confirmed the same rectangle remained selected and visible in the main workspace and Layers panel.

Screenshots from that run:

- [Canvas window open](evidence/popout-web-2026-11-28/canvas-window-open-final.png)
- [Rectangle created inside the window](evidence/popout-web-2026-11-28/canvas-edited-final.png)
- [Title-bar close returned the edited document](evidence/popout-web-2026-11-28/canvas-window-closed-final.png)
- Earlier same-artifact checks also captured [undo](evidence/popout-web-2026-11-28/canvas-undo.png),
  [redo](evidence/popout-web-2026-11-28/canvas-redo.png), [dragged window](evidence/popout-web-2026-11-28/canvas-window-moved.png),
  and [explicit dock](evidence/popout-web-2026-11-28/canvas-docked.png).

The browser reported zero JavaScript errors. It also reported a WASM external-instance warning and
software-renderer performance warnings (`getImageData` readback and WebGL `ReadPixels` stalls).
Those are recorded observations, not proof of user-visible stutter; hardware GPU performance still
needs measurement on representative systems.

## Regression coverage and remaining scope

- `harness::tests::embedded_floating_canvas_keeps_an_editable_canvas_in_the_workspace` verifies an
  actual shape drag, undo, and redo against the embedded floating canvas.
- `harness::tests::embedded_floating_canvas_under_pointer_becomes_the_active_document` verifies
  shortcuts target the document under the pointer when multiple documents are floated.
- Candidate suite: 225 unit/harness + 12 CLI + 5 hygiene + 10 MCP tests passed; strict Clippy and
  formatting passed. The isolated 500-object frame-time budget test passed.
- Coverage on the same worktree: 71.66% lines (29,109/36,674) and 75.13% functions (1,338/1,781);
  cargo llvm-cov reported no branch counters for this target.
- Explicit docking was also clicked in Chromium and returned the canvas to the main workspace.
- This establishes browser/WASM editing and return-to-workspace behavior. It does not establish
  native OS viewport editing, native save/reopen/drag between monitors, multiple floating windows
  through the full UI, or acceptance on Windows/macOS/Linux. Those remain open in the platform
  review checklist.
