# Windows 11 review checklist — FireCraft v0.3.5

The hosted release workflow passed the Windows x86_64 build, CLI smoke, package assembly, and checksum verification. These steps cover native GUI behavior that Linux-hosted Windows CI cannot prove.

## Package

- [Download `firecraft-v0.3.5-windows-x86_64.zip`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/download/v0.3.5/firecraft-v0.3.5-windows-x86_64.zip)
- SHA-256: `4cfdad8cc68133fa448d2cd424846a25612447bc25e0b509fe4167b300ed0868`
- Release source: `bec0b65f3d05f66d299568f0ea80227e049daee4`
- Extract the ZIP, then run `firecraft.exe`. No installer is required for this review build.
- Local build workspace copy: `output/release-v0.3.5/firecraft-v0.3.5-windows-x86_64.zip`.

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
