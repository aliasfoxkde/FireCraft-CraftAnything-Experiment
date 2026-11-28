# Web canvas window workflow — 2026-11-28

## Build identity and environment

- FireCraft 0.3.8 local release-mode WASM build; this is a review artifact, not a release or deployment.
- Base source commit: `e06b4adde8faee9cc726aa808be8b2d2ccc3c52a`.
- Working source patch SHA-256 (canvas, layout, and regression harness diff):
  `aa0f889645b0ed9f65d89d197ff700f596bd4e7da3837a3ae970a91ab3ecc675`.
- Source file SHA-256: `canvas.rs` `2e03d2f6c2000437f2ea02ae7677deac97796e6128bb24154e88bfdecb19f37c`,
  `layout.rs` `3abe5aa51ecde7949ee4527e32dafb3b4122715ab66fe6b99a0b3b4a9eeb458f`,
  `harness.rs` `9a01396722f675333d7edb56233408adba07e6350d1d3927eb3d1959fcf8b5cf`.
- WASM input SHA-256: `788682cafff9a6d0315fa413bb24b7169f8ab4082b88f194f0d1a210e92c41b5`.
- Served `firecraft_bg.wasm` SHA-256: `b3d571fe8f1bb56799762aa10ceb25005ee4816ff23917eac4a590739e5a35c0`;
  `firecraft.js` SHA-256: `fb14afbad574179ac4c9721c92f4a70f5d22bc1035314ee9cf8ecc98a31c51db`.
- Chromium 154 headed on the Linux Xvfb sandbox (`:96`), 1360×860 CSS viewport, WebGL2/SwiftShader.

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

Screenshots from that final build:

- [Canvas window open](evidence/popout-web-2026-11-28/canvas-window-open-after-native-fix.png)
- [Rectangle created inside the window](evidence/popout-web-2026-11-28/canvas-edited-after-native-fix.png)
- [Ctrl+Z removed the rectangle](evidence/popout-web-2026-11-28/canvas-undo-after-native-fix.png)
- [Ctrl+Y restored the rectangle](evidence/popout-web-2026-11-28/canvas-redo-after-native-fix.png)
- [Title-bar close returned the edited document](evidence/popout-web-2026-11-28/canvas-closed-after-native-fix.png)
- Earlier same-artifact checks also captured [window drag](evidence/popout-web-2026-11-28/canvas-window-moved.png)
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
- Candidate suite: 226 unit/harness passed, 1 existing timing test ignored, 12 CLI, 5 hygiene, and
  10 MCP tests passed; strict Clippy and formatting passed. The isolated 500-object frame-time
  budget test passed.
- Coverage on the final working tree: 71.74% line, 75.20% function, and 73.66% region coverage;
  cargo llvm-cov reported no branch counters for this target. The Linux native regression is included.
- Explicit docking was also clicked in Chromium and returned the canvas to the main workspace.
- The Linux native child-window create/edit/close path is separately verified in the
  [native receipt](native-floating-canvas-workflow-2026-11-28.md). Native save/reopen/drag between
  monitors, multiple floating windows through the full UI, and Windows/macOS acceptance remain open.
