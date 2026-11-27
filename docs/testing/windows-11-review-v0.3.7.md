# Windows 11 review checklist — FireCraft v0.3.7

The v0.3.7 Windows release workflow builds and smoke-tests the native binary. This checklist covers interactive Windows behavior; passing CI does not count as Windows 11 acceptance.

## Package

- [Download `firecraft-v0.3.7-windows-x86_64.zip`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/download/v0.3.7/firecraft-v0.3.7-windows-x86_64.zip)
- Verify the ZIP against `SHA256SUMS.txt` attached to the `v0.3.7` release.
- Extract and run `firecraft.exe`; no installer is required.

## Review steps

- [ ] Record Windows 11 build, display scale, GPU, and SmartScreen behavior; extract and launch.
- [ ] Confirm the splash takes 2–3 seconds, stays transparent over the live workspace, and fades into the GUI without blocking editing.
- [ ] Create a document, draw/select/move/resize a shape, change tools, and confirm no stray outlines or point-connecting artifacts appear.
- [ ] Copy an external image, move the pointer over the canvas while a dock text field retains focus, and press Ctrl+V; the image should land on the canvas and the field should remain unchanged.
- [ ] Repeat with external text, then move the pointer outside the canvas and paste into a focused text field; each paste should reach only its intended target.
- [ ] Use the right-click canvas Paste action and confirm it still works; test move and layer reorder without bitmap pixels changing or duplicating.
- [ ] Save and reopen `.fw.png`; export PNG and SVG and verify dimensions/content in another viewer.
- [ ] Exercise raster eraser drag/undo, gradient direction editing, menus, keyboard focus/Tab mode, dialogs, panels, floating canvas, undo/redo, and zoom.
- [ ] Attach exact steps, expected/actual result, screenshots, and a sample document for each defect.

## Sign-off

- Reviewer / date:
- Result: pending
- Issues or evidence links:
