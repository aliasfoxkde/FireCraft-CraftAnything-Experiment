# Windows 11 review checklist — FireCraft v0.3.3

This checklist records native GUI behavior that a Windows CI build cannot prove. The package has passed hosted binary/CLI smoke and checksum checks, but has not been accepted on a Windows 11 desktop yet.

## Package

- [Download `firecraft-v0.3.3-windows-x86_64.zip`](https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/download/v0.3.3/firecraft-v0.3.3-windows-x86_64.zip)
- SHA-256: `2222474b3d79fd12040242321da219a76f140179fb6ecad7a8b2e910978d751a`
- Release source: `918ca8ddad5b850ae18f4101327a5e76fcf12966`
- Extract the ZIP, then run `firecraft.exe`. No installer is required for this review build.

## Review steps

- [ ] Extract and launch on a supported Windows 11 system; note OS build, display scale, GPU, and whether Windows SmartScreen shows a warning.
- [ ] Confirm the app opens without a blank window, crash, or clipped first-run dialog; dismiss onboarding and restart once.
- [ ] Create a document, draw and select a shape, move/resize it, and confirm the selection outline tracks the object.
- [ ] Paste a copied image with Ctrl+V, move it, change layer order, save as `.fw.png`, close, reopen, and confirm the pixels and ordering are preserved.
- [ ] Export PNG and SVG; open each exported file in another viewer and check dimensions/content.
- [ ] Verify core menus, tool switching (including Shift+Q), undo/redo, zoom, canvas focus, and pop-out behavior.
- [ ] Record any regression with exact steps, expected/actual result, and a screenshot or sample document where possible.

## Sign-off

- Reviewer / date:
- Result: pending
- Issues or evidence links:

