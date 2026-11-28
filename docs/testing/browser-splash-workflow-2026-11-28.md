# Browser startup splash workflow — 2026-11-28

## Build and environment

- FireCraft 0.3.8 local WASM validation build; not a release or deployment.
- Base source commit: `166811a4f0314abf02e21227bf1feed15bc388e0`.
- Working source patch SHA-256: `3213dcbf7661be2c51e40128c013917c17161b17a2ac4252b50da40dec9260c8`.
- Fixed `firecraft_bg.wasm` SHA-256: `5d116029e3c2f7456ae381901a10b020aa265ac1748b9475c23fdd3c009b0deb`.
- Fixed `firecraft.js` SHA-256: `f786b385bbcd86d1568bd408cfd8cb36e802f586029af9b46d0cc2acd9e86d96`.
- Chromium 154.0.8022.0 headed on Fedora/Xvfb `:96`, viewport 1360×860, WebGL2/SwiftShader.

## Reproduction and cause

The unmodified WASM build's editor appeared without the brand overlay. Its screenshot, taken
about 1.45 seconds after canvas dimensions became available, had a uniform white center (one color
in the sampled 360×260 region): [pre-fix capture](evidence/splash-wasm-2026-11-28/pre-fix-no-logo.png).

`startup_splash` compared egui's input time to a 2.4-second duration. eframe's WebAssembly backend
sets that input time from `window.performance.now() / 1000`, whose time origin predates the first
FireCraft frame. Loading therefore consumed part or all of the splash interval before the app drew
its first frame. The splash now records the time of its first FireCraft frame and measures the fade
from that point.

## After-fix result

- The centered logo appears above the live editor and canvas, with transparent image pixels showing
  the canvas below it: [visible overlay](evidence/splash-wasm-2026-11-28/overlay-visible.png).
- The later capture shows the logo partway through fade-out:
  [mid-fade](evidence/splash-wasm-2026-11-28/overlay-mid-fade.png).
- At the next capture (about 2.6–2.7 seconds after the browser's canvas-ready marker; screenshot
  capture itself adds latency), the logo is gone and the normal editor remains:
  [editor after fade](evidence/splash-wasm-2026-11-28/editor-after-fade.png).
- Pressing **P** about 0.6 seconds after canvas initialization selected the Pen tool while the logo
  remained visible; the active element stayed the canvas. No browser/app console errors occurred:
  [keyboard during overlay](evidence/splash-wasm-2026-11-28/keyboard-while-visible.png).
- Clicking **File** during the overlay skipped the remaining splash and opened the File menu in that
  interaction; [click-through capture](evidence/splash-wasm-2026-11-28/file-menu-click-through.png)
  shows the menu open.
- Pre-fix and after-fix fixed captures used the same viewport, browser and blank-document state.

## Validation and limits

- Regression test `harness::tests::startup_splash_is_a_transparent_overlay_over_the_live_workspace`
  was run against a simulated egui clock starting at 9,000 seconds: it failed before the fix and
  passed after it. The pure 2.4-second fade curve and transparent artwork tests also passed.
- A real headed Chromium run loaded the release-mode WASM, showed the logo over the live editor,
  accepted the Pen shortcut during the visible overlay, observed the fade, and reached the unobscured
  editor with no JS errors. WebGL2 initialized at 1360×860.
- This verifies WASM in Chromium only. Native Windows/macOS/Linux startup, cold-cache loading on other
  browsers, reduced-motion preferences, and low-end hardware remain untested.
- The existing `wasm-bindgen` warning about a missing `target_features` custom section remains; it
  is unrelated to the splash fix.
