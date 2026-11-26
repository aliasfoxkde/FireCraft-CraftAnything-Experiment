# FireCraft Keybindings

Shortcut dispatch is gated on text-field focus: when you are typing in a text
box, these bindings are suppressed. `Esc` always closes dialogs and menus
first. The authoritative table is `app/src/layout.rs` (`shortcut_table`) and is
tested in `app/src/harness.rs`.

## Tools

| Key | Tool |
|-----|------|
| V | Pointer / Select |
| A | Subselection |
| Q | Scale / Transform |
| C | Crop |
| M | Marquee (again for Oval Marquee) |
| L | Lasso |
| W | Magic Wand |
| B | Brush / Pencil |
| E | Eraser |
| S | Rubber Stamp |
| I | Eyedropper |
| G | Paint Bucket / Gradient |
| N | Line |
| P | Pen |
| F | Freeform Path |
| U | Rectangle (Auto Shapes group: Rounded, Ellipse, Polygon, Star) |
| T | Text |
| K | Rectangle Slice |
| J | Rectangle Hotspot |
| H | Hand (pan) |
| Z | Zoom |

Tool groups with a corner triangle open a flyout: click the triangle or
right-click the button.

## File and Edit

| Shortcut | Command |
|----------|---------|
| Ctrl+N | New document |
| Ctrl+O | Open |
| Ctrl+S / Ctrl+Shift+S | Save / Save As |
| Ctrl+W | Close tab |
| Ctrl+Q | Quit |
| Ctrl+R / Ctrl+Shift+R | Import / Export |
| Ctrl+Z / Ctrl+Y | Undo / Redo (Ctrl+Shift+Z also redoes) |
| Ctrl+C / Ctrl+X / Ctrl+V | Copy / Cut / Paste |
| Ctrl+Alt+C | Copy as CSS |
| Ctrl+D / Ctrl+Shift+D | Duplicate / Clone |
| Ctrl+F | Find & Replace |
| Ctrl+K or Ctrl+P | Command palette |
| Ctrl+G / Ctrl+Shift+G | Group / Ungroup |
| Ctrl+A / Ctrl+Shift+A or Esc | Select all / Deselect |
| Ctrl+Shift+I | Select inverse |

## View and Magnification

| Shortcut | Command |
|----------|---------|
| Tab / F4 | Full-screen canvas (hide panels) |
| Shift+Tab | Toggle right dock |
| F11 | Window fullscreen |
| Ctrl+0 or Insert | Fit in window |
| Ctrl+1 / Ctrl+2 / Ctrl+5 | Zoom 100% / 200% / 50% |
| Ctrl+= or Ctrl+- (or bare + / -) | Zoom in / out |
| Ctrl+R / Ctrl+G | Rulers / Grid |
| Ctrl+; | Guides |
| Ctrl+Shift+F | Infinite canvas (god view) |
| [ / ] | Brush size down / up |
| Home / End | First / last animation state |
| PageUp / PageDown | Previous / next page |

## Panel focus

| Key | Panel |
|-----|-------|
| F2 | Layers (Shift+F2 States, Ctrl+F2 Tools dock) |
| F3 (Ctrl+) | Properties dock |
| F5 | Pages |
| F6 | Optimize (Shift+F10 History) |
| F7 | Document Library (Shift+F7 CSS/URL) |
| F9 | Swatches (Shift+F9 Color Mixer) |
| F11 (Shift+) | Styles |
| F12 (Shift+Alt+) | Info |

## Canvas and selection

| Shortcut | Command |
|----------|---------|
| Ctrl+Up / Ctrl+Down | Bring to front / send to back (with Shift) / nudge z-order |
| Ctrl+Left / Ctrl+Right | Cycle selection backward / forward |
| Ctrl+J / Ctrl+Shift+J | Join / split paths |
| Ctrl+E | Flatten selection to bitmap |
| Ctrl+L | Lock/unlock selection |
| Ctrl+B / Ctrl+I | Bold / italic (text) |
| Ctrl+Alt+X | Image preview (Optimize panel) |
| Ctrl+Alt+F | Fit canvas to contents |
| X | Swap fill & stroke |
| D | Default colors |

## Mouse

| Gesture | Action |
|---------|--------|
| Ctrl+wheel | Zoom |
| Shift+scroll | Pan horizontally |
| Scroll | Pan vertically |
| Alt+drag | Hand-pan |
