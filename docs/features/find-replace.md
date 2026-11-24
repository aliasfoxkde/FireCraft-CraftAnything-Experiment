# Find & Replace

`Edit > Find and Replace` (`Ctrl+F`) supports text/name replacement and Fireworks-style color
replacement.

## Modes

| Mode | Matches | Replaces |
|------|---------|----------|
| **Text & object names** | Object names and text-object content, with optional case matching | All occurrences in names and text, across every page |
| **Fill colors** | Flat fill and gradient start/end stops | Fill and both gradient stops |
| **Stroke colors** | Object stroke color | Stroke |
| **Any object color** | Fill, stroke, gradient stops, and live-effect colors | All of the above, including drop shadow, inner shadow, glow, and bevel colors |

Layers may be hidden; Find/Replace still sees their objects. This is intentional for template
cleanup.

## Dialog behavior

- A live count shows matching objects across pages.
- **Find Next** cycles through matching objects, switches pages when needed, and selects the object
  so context is visible before replacing.
- **Replace All** is one undoable action.
- Color modes provide a color picker, hex readout, and “Use fill / Use stroke” shortcuts.
- The window is movable and resizable; it is not centered by an immovable anchor.

## Replacement semantics

- Matching is exact for colors (alpha included).
- `Any object color` walks gradient endpoints and live-effect color fields.
- Text replacement preserves the rest of the name/text; case-insensitive mode replaces matches
  without reconstructing the whole string.
- A replace action marks the document dirty.

## Implementation

- `app/find_replace.rs` owns match semantics, Find Next selection, and replacement effects.
- `dialogs/find_replace.rs` owns only the dialog surface and calls the app methods.
- `FindReplaceMode` and dialog state live in `app.rs`.

## Tests to keep

- Text replacement across multiple pages, including hidden layers.
- Color replacement of fill, gradient start/end, stroke, and effect colors.
- No-op replacement when source and destination colors match.
- Find Next cycles rather than selecting the same first object forever.
