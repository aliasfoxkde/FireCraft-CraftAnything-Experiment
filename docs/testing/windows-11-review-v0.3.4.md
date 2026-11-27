# Windows 11 review checklist — FireCraft v0.3.4

The hosted release workflow passed Windows binary build, CLI smoke, packaging, and checksum verification. This checklist covers native GUI behavior that Linux-hosted Windows CI cannot prove.

## Package

- [Download `firecraft-v0.3.4-windows-x86_64.zip`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/download/v0.3.4/firecraft-v0.3.4-windows-x86_64.zip)
- SHA-256: `5d613e835769dbe9c1116f35a7602c0b84ef450cff6c782cd2cf9600e5091fd5`
- Release source: `c73ea44e90933288fbe8f4c3554ba47d4b574d94`
- Extract the ZIP, then run `firecraft.exe`. No installer is required for this review build.
- A local copy is available at `output/release-v0.3.4/firecraft-v0.3.4-windows-x86_64.zip` in the build workspace.

## Review steps

- [ ] Extract and launch on Windows 11; note OS build, display scale, GPU, and whether SmartScreen shows a warning.
- [ ] Confirm the app opens without a blank window, crash, or clipped first-run dialog; dismiss onboarding and restart once.
- [ ] Create a document, draw/select/move/resize shapes, and confirm selection outlines track the object.
- [ ] Paste a copied image with Ctrl+V, move it, change layer order, save as `.fw.png`, close/reopen, and confirm pixels and ordering are preserved.
- [ ] Erase part of a raster image with multiple brush sizes/feather values; confirm the object remains and undo/redo restores the stroke.
- [ ] Export PNG and SVG; open each in another viewer and check dimensions/content.
- [ ] Verify menus, tool switching (including Shift+Q), undo/redo, zoom, canvas focus/Tab, pop-out canvas, and file dialogs.
- [ ] Record exact reproduction steps, expected/actual result, and a screenshot or sample document for every issue.

## Sign-off

- Reviewer / date:
- Result: pending
- Issues or evidence links:
