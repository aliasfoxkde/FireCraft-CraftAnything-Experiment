# Browser Pen path workflow receipt — 2026-11-28

## Build and environment

- FireCraft 0.3.8 local validation bundle; this is not a public release or deployment.
- Base source commit: `3ed49050ad1eecc2da54f0e6c1caae036d392dcd`.
- Working app-source patch SHA-256: `8010ba96bbb2145951affbde0e1f3050bd16835be22bf33cccd726e1917fb152`.
- Rust `1.100.0`, wasm-bindgen `0.2.130`, target `wasm32-unknown-unknown`.
- Chromium 154.0.8022.0, headed on Fedora with Xvfb display `:96`, SwiftShader WebGL, viewport 1360×860.
- Build output: `output/playwright/hill-climb-2026-11-28/current-main/`.
- `firecraft_bg.wasm` SHA-256: `5908e2eb7f01e3944054ca2877f0c56f0cba4fb33b5cf1ca017e01508ed3ef94`.
- `firecraft.js` SHA-256: `23153382d2046e94bd68444c894ec4b648522f486efb05b6c06b8868d3d9a76d`.
- `index.html` SHA-256: `0c0d78a034f625dbeba1f5385d3ebbb3aa5710f3351726979c8fe8106f599d1a`.
- Browser automation: `output/playwright/hill-climb-2026-11-28/path-workflow/path-roundtrip.mjs`.

## Scenario and result

The workflow drove the actual app canvas and toolbar with pointer and keyboard input:

1. Press **P**, click three anchors, then click the first anchor to close a filled triangle.
2. Press **A** and drag the first anchor to reshape it.
3. Press **Ctrl+Z** and **Ctrl+Y**. Undo restored the prior anchor position; redo restored the edit.
4. Save As `path-roundtrip.fw.png`, reopen the downloaded file, and inspect the editable Layers entry.
5. Export `export.png` and inspect both its signature and rendered pixels.

The canvas showed the filled orange triangle; the reopened layered document retained one editable
Path object. The exported PNG is 1920×1080, has the PNG signature `89504e470d0a1a0a`, and contains
the expected orange fill at document pixel `(1000, 500)` (`232, 163, 61, 255`). Chromium reported no
page or application console errors.

The run first exposed two real defects. Before the shortcut fix, browser Ctrl+Z selected the Zoom
tool and left the moved anchor in place even though the DOM key event carried `ctrlKey: true`. Before
the raster fix, the same closed path exported as an outline with no fill. The corresponding red
tests failed before the fixes and pass after them. The dispatcher now uses modifiers from each key
event. Raster export fills valid closed Paths and Polygon shapes using a scanline pass after checking
the contour with the same simple-polygon triangulation used by canvas rendering; open, degenerate,
and self-intersecting paths do not receive an invented fill.

## Evidence

- [Path before node edit](evidence/path-wasm-2026-11-28/path-before-edit.png)
- [Path after node edit](evidence/path-wasm-2026-11-28/path-after-edit.png)
- [Path after undo](evidence/path-wasm-2026-11-28/path-after-undo.png)
- [Path after redo](evidence/path-wasm-2026-11-28/path-after-redo.png)
- [Editable path after layered save/reopen](evidence/path-wasm-2026-11-28/path-after-reopen.png)
- [Raster export with visible fill](evidence/path-wasm-2026-11-28/export.png)

Screenshot SHA-256 values:

| Capture | SHA-256 |
| --- | --- |
| `path-before-edit.png` | `89dd0b31f8812ac395129e878e10f921e22ea53bd2baa23e462d1e7da5f16d2a` |
| `path-after-edit.png` | `7d76039b57086c2fbf188a5a23a80f636776cab696d0211f1258ca8943f47424` |
| `path-after-undo.png` | `7e2afc7d4353b06ebcc4cdc064519b2f0b57d0a9e3e89f497855fbe13c17172b` |
| `path-after-redo.png` | `b7dcd4e5743526630f7dccd8a1ce666f18e3ddd305dc7eead6a7afdf92ae5e0f` |
| `path-after-reopen.png` | `6c9f1583de53fafc6e47453556769e87991f5439321a6b38942599fc560041f4` |
| `export.png` | `c23ba905dea38160ecd4f828d9df42a161be7313f454b508b9ff19b123576f21` |

## Validation and limits

- Full local Rust tests: 221 unit passed, 1 existing timing test ignored, 12 CLI passed, 5 hygiene
  passed, and 10 MCP passed.
- Strict Clippy (`--all-targets --all-features -- -D warnings`) and `cargo fmt --check` passed.
- Release WASM build and exact-bundle Chromium pointer/keyboard/save/reopen/export workflow passed.
- This receipt covers one simple closed triangle in Chromium. Native Windows/macOS path editing,
  freeform curves/handles, compound Boolean paths with holes or disconnected contours, self-
  intersecting editing behavior, zoom/DPI variants, and broad shape/effect export parity remain
  unverified. The raster export change does not establish complete Fireworks CS6 path parity.
- The `wasm-bindgen` build warns that the `target_features` custom section is missing because the
  release profile strips it; this warning remains open in the release build pipeline.
