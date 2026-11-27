# Feature Surface

This is the detailed inventory. The root README is intentionally shorter.

## Workspace

- 10 CS6 menus with Alt mnemonics and arrow navigation.
- Main toolbar, multi-document tabs, Original/Preview/2-Up/4-Up.
- Collapsible, draggable, stackable, snapping panel groups; dual right dock and iconic mode.
- Full-screen canvas (`Tab` / `F4`), light/dark/system themes, workspace presets.
- Windows AppUserModelID and taskbar Jump List (recent documents, New Document, About).

## Canvas

- Page-clipped editing plus **god view** infinite canvas (see [Infinite Canvas](infinite-canvas.md)).
- Rulers (px/in/cm/mm), grid, guides, smart snapping, selection handles, marquee/lasso/wand.
- Live effects: drop/inner shadow, inner/outer glow, bevel, Gaussian blur, brightness/contrast.
- Slices, hotspots, page states, onion skin, animation playback, motion tween.

## Tools (36)

| Group | Tools |
|-------|-------|
| Select | Pointer (`V`), Subselection (`A`), Scale (`Q`), Crop (`C`), Marquee (`M`), Oval Marquee (`O`), Lasso (`L`), Magic Wand (`W`) |
| Bitmap | Brush (`B`), Pencil, featherable alpha Eraser (`E`, size/feather in Properties), Blur, Sharpen, Dodge, Burn, Rubber Stamp (`S`), Replace Color (`R`), Red Eye Removal (`Y`) |
| Vector | Line (`N`), Pen (`P`), Freeform (`F`), Rectangle (`U`), Rounded Rectangle, Ellipse, Polygon, Star, Text (`T`), Knife |
| Web | Rectangle Slice (`K`), Rectangle Hotspot (`J`) |
| Colors | Eyedropper (`I`), Paint Bucket (`G`), Gradient |
| View | Hand (`H`), Zoom (`Z`), Fit (`Ctrl+0`) |

The complete inventory is visible in the three-column rail; no tool is hidden behind an unlabeled
flyout.

## Panels

Optimize, History, Align, Path, Pages, States, Layers, Styles, Color Palette, Color Mixer, Swatches,
Info, Special Characters, Image Editing, Auto Shapes, Auto Shape Properties, Document Library,
Common Library, CSS Properties, URL, Behaviors, plus bottom Properties, Symbol Properties, and
Intelligence.

## File and edit operations

- Open/save/export PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, SVG, HTML5.
- Layered `.fw.png` round-trip with out-of-page objects.
- Find & Replace in text, names, fills, strokes, and any object color (see
  [Find & Replace](find-replace.md)).
- In-app file browser with editable address bar and file-type dropdown (see
  [File Browser](file-browser.md)).
- Undo/redo, grouping, symbols, styles, path booleans, gradients, clipboard images/text.

## Automation

- MCP JSON-RPC 2.0 stdio server.
- Headless CLI: `info`, `export`, `convert`, `batch`, `apply`, `list-commands`.
- Command palette (`Ctrl+K`) and saved commands.

## Verification status

Automated tests cover document/model behavior, color math, path booleans, layered PNG, clipboard
gating, MCP dispatch, styles, gradients, and editor workflows. Real Windows taskbar Jump List
behavior, app-switch flashing, and some manual drag/menu gestures require a Windows desktop and are
reported as unverified when not exercised.
