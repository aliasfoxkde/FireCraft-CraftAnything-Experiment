# Workflow verification catalog

This catalog is the index for repeatable user workflows. IDs remain stable when implementation,
tests, or documentation move. Each row separates behavior already proven at a code seam from the
interactive acceptance still needed; a passing unit or headless test never implies desktop/browser
sign-off.

## Evidence states

- **Code verified** — focused tests exercise the named model/app behavior.
- **Partially verified** — some steps have code-level proof; the complete user flow is still open.
- **Unverified** — no current end-to-end pass is recorded for the named workflow.
- **Planned** — the product behavior has not shipped; its specification/task is the only evidence.

When a workflow is run, copy the [evidence receipt form](../testing/workflow-evidence-template.md)
to a dated file under `docs/testing/` (or keep bulky traces as CI artifacts) and record
build/source identity, platform, commands or checklist revision, result, evidence paths/checksums,
and reviewer. Add screenshot paths only after capturing actual app output. Never mark a row passed
because a screenshot merely looks correct.

## Priority workflow catalog

| Stable ID | User outcome and surface | Existing proof | Canonical docs | Current state and next acceptance |
|---|---|---|---|---|
| `FC-WF-STARTUP-01` | Launch to editable workspace; native and WASM | `dialogs/mod.rs` splash timing/transparency tests; `harness.rs::startup_splash_is_a_transparent_overlay_over_the_live_workspace` | [Windows review v0.3.7](../testing/windows-11-review-v0.3.7.md), [evidence process](../DOCUMENTATION-AND-WORKFLOW-EVIDENCE.md) | **Partially verified.** Run timing, transparency, click-through, and fade on the tagged native/Pages builds; record OS/browser and trace. Task [37.9](../TASKS.md#phase-37). |
| `FC-WF-FOCUS-TAB-01` | Repeatedly toggle canvas mode with Tab and return to editing; native and WASM | `harness.rs::tab_can_toggle_canvas_mode_repeatedly_while_startup_splash_is_visible` and `tab_toggles_the_full_screen_canvas_and_shift_tab_the_right_dock` | [Keybindings](../tutorials/keybindings.md) | **Partially verified.** Perform a keyboard-only pass after startup, menus, text edits, and tab switches on desktop and browser. Task [44.7](../TASKS.md#phase-44). |
| `FC-WF-CLIPBOARD-NATIVE-01` | Paste internal objects and external text/image with Ctrl+V; native desktop | Ctrl+V harness routing, focused-field routing, and `app/edit.rs` clipboard-source tests | [Clipboard and workflow plan](../DOCUMENTATION-AND-WORKFLOW-EVIDENCE.md) | **Partially verified.** Native OS clipboard, browser-copy-image into desktop, field/canvas focus, and floating-canvas paste need real interaction evidence. Task [44.7](../TASKS.md#phase-44). |
| `FC-WF-CLIPBOARD-WEB-01` | Paste copied browser image/text onto the canvas; WASM | `app/edit.rs::paste_browser_image_payload_decodes_and_rejects_malformed_data`; current-build Playwright receipt below | [Clipboard and workflow plan](../DOCUMENTATION-AND-WORKFLOW-EVIDENCE.md), [Chromium clipboard receipt](../testing/browser-clipboard-workflow-2026-11-28.md) | **Partially verified (2026-11-28).** Chromium clipboard permission + real Ctrl+V event inserted and selected a 128×128 bitmap; `clipboardData` contained one file and the app reported paste success. External desktop copy, denial, text paste, field-focus routing, and large/malformed payloads remain unverified. Task [32.1](../TASKS.md#phase-32). |
| `FC-WF-FLOAT-CANVAS-01` | Float a document, edit it, save, and re-dock without losing the active document; native desktop | `harness.rs::floating_canvas_dispatches_child_viewport_keyboard_paste_to_its_document` and `closing_floated_document_does_not_overwrite_the_next_document` | [Architecture](../architecture.md), [Windows review v0.3.7](../testing/windows-11-review-v0.3.7.md) | **Partially verified.** Native viewport creation, pointer editing, drag/drop, save, focus, and re-dock need an interactive run. Task [44.7](../TASKS.md#phase-44). |
| `FC-WF-TOOLS-01` | Find, select, collapse/expand, and use every tool group/flyout; native and WASM | Current GUI state captures are static; tool behavior has individual code paths | [Canvas tools plan](../CANVAS-TOOLS-AND-EFFECTS-PLAN.md), [Windows review v0.3.7](../testing/windows-11-review-v0.3.7.md) | **Unverified end to end.** Check normal/compact widths, labels, scrolling, flyout icons/details, selection, and tool actions. Task [21.5](../TASKS.md#phase-21). |
| `FC-WF-PATH-01` | Draw, close, select, edit anchors, undo/redo, save/reopen, and export a path; native and WASM | `geometry.rs::{triangulates_concave_contour_and_keeps_notch_outside,triangulation_handles_reversed_winding_and_redundant_anchors,degenerate_and_self_intersecting_contours_are_not_filled}` and `canvas.rs::{concave_closed_path_solid_fill_leaves_the_notch_unpainted,concave_closed_path_gradient_leaves_the_notch_unpainted,concave_closed_path_hit_testing_ignores_the_empty_notch}` prove geometry/render/hit-test seams; `app/object.rs` covers path operations | [Canvas tools plan](../CANVAS-TOOLS-AND-EFFECTS-PLAN.md), [drawing tutorial](../tutorials/drawing-and-editing.md) | **Partially verified.** Exact-head GitHub gates pass, but complete pointer/keyboard gestures and rendered geometry through save/reopen/export remain open. See [PR #57](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/pull/57), task [48.4](../TASKS.md#phase-48), and the [workflow receipt form](../testing/workflow-evidence-template.md). |
| `FC-WF-FILE-DIALOG-01` | Open/import/save/export; format, filename, download, cancel, and viewport behavior; desktop and WASM | `formats.rs::browser_safe_bytes_preserve_layered_documents_and_decode_svg`; `app/files.rs::browser_open_and_import_use_supplied_bytes_without_a_disk_path`; `dialogs/file_browser.rs::file_dialog_actions_stay_visible_at_desktop_viewport_sizes` | [File browser guide](../features/file-browser.md), [browser workflow receipt](../testing/browser-file-workflow-2026-11-28.md) | **Partially verified (2026-11-28).** Headed Chromium opened an SVG, imported a PNG, saved `.fw.png`, reopened it with three objects preserved, and downloaded PNG at 1360×860; Open and Save As also passed at 1024×600. Native Fedora Xvfb Open via Go, Save As, and layered reopen passed at 800×500. Desktop overwrite/error paths, browser cancel and OS drag/drop, high-DPI, and native Windows/macOS review remain open. Task [23.6](../TASKS.md#phase-23). |
| `FC-WF-LAYER-REORDER-01` | Reorder image/shape layers, undo/redo, save/reopen, and retain original pixels | `app/arrange.rs::bitmap_textures_follow_images_when_hotkey_reorders_objects`; `canvas.rs::bitmap_rendering_keeps_each_image_texture_after_hotkey_reorder` | [Canvas tools plan](../CANVAS-TOOLS-AND-EFFECTS-PLAN.md) | **Partially verified.** Model/renderer identity regressions pass; repeat the user-reported visual sequence across drag and keyboard reorder on a real build. Task [44.7](../TASKS.md#phase-44). |
| `FC-WF-RASTER-ERASER-01` | Erase part of a bitmap with size/feather, undo, save, and export | Eraser sampling, alpha, undo, transform, and lock tests are recorded in task history | [Canvas tools plan](../CANVAS-TOOLS-AND-EFFECTS-PLAN.md) | **Partially verified.** Selection clipping, opacity/tip options, and canvas/export visual acceptance remain open. Task [37.2](../TASKS.md#phase-37). |
| `FC-WF-GRADIENT-FILTER-01` | Edit gradient axis/stops and live effects; compare canvas and exported output | Direction/axis and some filter render/export regression tests | [Canvas tools plan](../CANVAS-TOOLS-AND-EFFECTS-PLAN.md) | **Partially verified.** Multi-stop/type editing and full shape/filter/zoom parity need completion and visual review. Tasks [37.3](../TASKS.md#phase-37) and [37.4](../TASKS.md#phase-37). |
| `FC-WF-SAVE-RECOVERY-01` | Recover a periodic autosave after an interrupted session; native and WASM | No shipped autosave/recovery workflow | [Product roadmap](../PRODUCT_ROADMAP.md) | **Planned.** Define storage, retention, atomicity, recovery choice, conflict/failure handling, then implement with fault-injection proof. Tasks [23.1–23.2](../TASKS.md#phase-23). |
| `FC-WF-SAMPLE-EXPORT-01` | Create a sample design, save the editable source, export, and reopen it | Existing showcase geometry is a demo scene, not yet a tutorial sample project | [Tutorial index](../tutorials/README.md) | **Unverified.** Create a real, licensed sample document and record source/export checksums plus tutorial screenshots. Task [46.10](../TASKS.md#phase-46). |

## How to close a row

1. Write or update the scenario in `docs/testing/` with start state, exact actions, expected result,
   undo/recovery path, keyboard-only path, and target surface(s).
2. Link it to a focused model/app test. For user-visible interaction, add browser automation or a
   versioned manual checklist; a headless egui test is not a substitute for native viewport, OS
   clipboard, screen-reader, or platform behavior.
3. Run the exact review artifact and save its version, commit, checksum, platform/browser, viewport,
   display scale, theme, command/checklist revision, and reviewer.
4. If it fails, save the failing trace/screenshot first, file or update a focused task/issue, add a
   red regression at the narrowest reliable seam, fix it, then rerun the original scenario.
5. From a passing run, author or update the tutorial and feature guide. Use unaltered app captures,
   concise alt text, expected results, and links back to this stable ID.
6. Change status only when the evidence receipt exists and a reviewer checks that its build identity
   matches the app shown. Update [TASKS.md](../TASKS.md), docs links, and the next release receipt.

The task ledger remains the only active checkbox list. This catalog records stable scenario identity
and current proof; it does not imply the unverified workflows have passed.
