# Browser clipboard workflow receipt — 2026-11-28

## Build and environment

- FireCraft 0.3.8 local validation WASM candidate; not deployed or released.
- App source patch identity: `0bc6ffca2355c4a744dc77bb1c71741da31061936e7df01e27e0d2e275809e31`
  relative to base commit `83268df5ff5a009792e8b20b7a20b0ff84b09c79`.
- WASM SHA-256: `0e45df0253a6885f5bab79ee0b4c5039623b7954216c05fc45b0da1bfcf87f18`;
  JavaScript glue SHA-256: `a550193af6dd25d7985a3c427de70443d96bf6091194c6e4e7e27d2aa8c8390`.
- Chromium 154.0.8022.0 under Xvfb and SwiftShader, viewport 1360×860, clipboard-read/write
  permissions granted to the local test origin.
- Scenario script: `output/playwright/hill-climb-2026-11-28/clipboard-current-build.mjs`.

## Scenario and observed result

1. Started the served WASM app and waited for its editor canvas.
2. Wrote the repository's actual `app/favicon.png` (14,007 bytes, `image/png`) to the browser's
   clipboard through `navigator.clipboard.write`.
3. Clicked the editor canvas and pressed Ctrl+V.
4. Chromium delivered one clipboard file (`clipboardData.types` was `Files`, with one file). The
   editor displayed a 128×128 FireCraft bitmap at the canvas center, selection handles surrounded
   it, Layers showed `Pasted Bitmap (128×128) [Bitmap]`, and the status read
   `Pasted bitmap image (128×128 px) from clipboard`.
5. The app emitted no page errors or application console errors. An ImageMagick pixel comparison of
   the 120×120 canvas-center crop found 4,108 changed RGB pixels between before/after captures.

The first test assertion incorrectly expected Chromium to put `image/png` in
`clipboardData.types`; this engine reports `Files` and exposes the file separately. The assertion
was corrected to check the real browser event representation and one-file payload. The observed
canvas/layer/status result confirms the image was inserted, not merely that Ctrl+V was sent.

## Evidence

- [Canvas before paste](evidence/clipboard-wasm-2026-11-28/clipboard-current-before.png)
- [Selected bitmap after paste](evidence/clipboard-wasm-2026-11-28/clipboard-current-after.png)

## Limits

- This proves the browser Clipboard API → real browser paste event → editor insertion path with
  clipboard permission granted. It does not prove copying from an arbitrary external desktop app
  into the browser, permission-denied behavior, text paste, OS clipboard behavior in native builds,
  focus isolation inside every text field, or large/malformed clipboard payload handling.
- Full remaining scope stays open in task [32.1](../TASKS.md#phase-32) and workflow
  `FC-WF-CLIPBOARD-WEB-01`.
