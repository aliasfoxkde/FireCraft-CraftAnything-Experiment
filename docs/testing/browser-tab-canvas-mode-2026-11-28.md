# Browser Tab canvas-mode receipt — 2026-11-28

## Build and environment

- FireCraft 0.3.8 local validation WASM candidate; not deployed or released.
- App source patch identity: `0bc6ffca2355c4a744dc77bb1c71741da31061936e7df01e27e0d2e275809e31`
  relative to base commit `83268df5ff5a009792e8b20b7a20b0ff84b09c79`.
- WASM SHA-256: `0e45df0253a6885f5bab79ee0b4c5039623b7954216c05fc45b0da1bfcf87f18`.
- Chromium 154.0.8022.0 under Xvfb/SwiftShader, viewport 1360×860.
- Scenario script: `output/playwright/hill-climb-2026-11-28/tab-focus-current-build.mjs`.

## Scenario and observed result

1. Loaded the app and waited for its canvas. No mouse/pointer click was made before or during the
   keyboard sequence; `document.activeElement` was already the app canvas.
2. Sent four consecutive Tab keypresses, waiting for a rendered frame after each. The canvas stayed
   the active DOM element and the page reported no JavaScript or console errors.
3. The first Tab entered Full Screen Canvas mode. The second restored the standard workspace. The
   third and fourth repeated those transitions. ImageMagick pixel comparison confirmed the first
   Full Screen Canvas capture and third capture were identical (0 changed pixels), and the normal
   workspace captures after the second and fourth presses were identical (0 changed pixels). The
   initial normal-to-fullscreen comparison changed 609,914 pixels.

This is a current Chromium/WASM pass for repeated Tab toggling without pointer refocus. It does not
establish native Windows/macOS keyboard behavior or focus routing after typing in a text field,
opening menus, changing documents, or using a pop-out canvas.

## Evidence

- [Normal workspace before keypresses](evidence/tab-canvas-wasm-2026-11-28/canvas-normal.png)
- [Full Screen Canvas after first Tab](evidence/tab-canvas-wasm-2026-11-28/canvas-fullscreen.png)
- [Normal workspace restored after four Tabs](evidence/tab-canvas-wasm-2026-11-28/canvas-restored-after-four-tabs.png)
