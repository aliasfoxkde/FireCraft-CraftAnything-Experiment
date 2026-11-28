# Browser compact footer and Tools flyout — 2026-11-28

## Build and environment

- FireCraft 0.3.8 release-mode WASM validation build; not a release or deployment.
- Base source commit: `e1b1761b10b8dd866fdf6912f197a0dfb29b8baf`.
- `app/src/layout/chrome.rs` working patch SHA-256: `2b9182fc873fdca870fe7eb061ec484ae9dde9a128349e0b57c01b7c2cf45992`.
- `firecraft_bg.wasm` SHA-256: `ab63ede618a128d65161c9d98039e46bd81f25c23f258a672fd006504333fc06`.
- `firecraft.js` SHA-256: `f786b385bbcd86d1568bd408cfd8cb36e802f586029af9b46d0cc2acd9e86d96`.
- Chromium 154.0.8022.0 headed on Fedora/Xvfb `:96`, WebGL2/SwiftShader.

## Reproduction

At 960×640, the existing canvas footer placed the document format and `1 / 1` disabled state transport beside the controls. The **Show All** label was not visible. Clicking where the control should be opened the resolution preset menu instead: [before](evidence/footer-wasm-2026-11-28/before-clipped-footer.png) and [wrong menu opened by the attempted Show All click](evidence/footer-wasm-2026-11-28/before-resolution-menu-opened.png).

A new real-frame regression clicked the expected Show All location at `(350, 483)` in a 960×640 workspace. It failed before the layout change because `infinite_canvas` remained false.

## Fix and browser verification

The footer switches to a compact layout below 700 px of available width. It shortens the document format label, narrows the resolution/zoom/DPI controls, omits state transport when the document has one state, and uses previous/next plus a state count for multi-state documents. The full state controls remain in the wide layout.

- The 960×640 capture now shows **Snap: On**, **Rulers**, **Grid**, and **Show All** without clipping: [compact footer](evidence/footer-wasm-2026-11-28/compact-footer-960.png).
- Clicking **Show All** turns infinite canvas on; the status bar reports the mode: [Show All on](evidence/footer-wasm-2026-11-28/show-all-on-960.png).
- Clicking **Rulers** hides the ruler markings: [Rulers off](evidence/footer-wasm-2026-11-28/rulers-off-960.png).
- Clicking **Grid** displays the document grid: [Grid on](evidence/footer-wasm-2026-11-28/grid-on-960.png).
- The 1360×860 wide layout retains the document-format label and full control widths: [wide footer](evidence/footer-wasm-2026-11-28/wide-footer-1360.png).
- At 960×640 the Pen group flyout shows its icons and short labels; selecting **Freeform Path** updates the active tool and status: [Pen flyout](evidence/footer-wasm-2026-11-28/pen-flyout-960.png), [Freeform selected](evidence/footer-wasm-2026-11-28/freeform-selected-960.png).
- Browser console/page errors: none. Canvas remained focused.

## Validation and limits

- `harness::tests::show_all_toggle_is_reachable_in_the_compact_canvas_footer` failed before the layout change and passed after it. The compact-breakpoint unit test also passes.
- Release-mode WASM rebuilt from the changed footer implementation and exercised in real headed Chromium at both viewport sizes.
- This covers the blank one-state document's compact footer, the visible view-toggle actions, and one Pen-group flyout. It does not prove all tool groups, multi-state transport behavior, desktop-native layout, or Adobe Fireworks CS6 parity.
