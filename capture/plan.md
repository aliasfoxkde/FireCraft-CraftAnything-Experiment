# Fireworks CS6 Recovery Plan — Revised

## Task
Recover Adobe Fireworks CS6's real default values from Fireworks.exe and rebuild them in FireCraft.

## Constants to find (with expected hex values)
1. **Canvas default size**: 900x700 → 0x384, 0x2BC
2. **Zoom levels**: 10, 25, 33, 50, 66, 75, 100, 150, 200, 300, 400
3. **Undo limit**: 32
4. **Swatch colors**: 0x000000, 0xFFFFFF, 0xFF0000, 0x00FF00, 0x0000FF, etc.
5. **Export defaults**: JPEG quality (60, 75, 90?), PNG settings
6. **Tool defaults**: Brush sizes, fill colors (0xFFFFFFFF)

## Search Strategy
1. Search for hex immediates 0x384, 0x2BC in code sections
2. Search for "900@" in string sections (might be "900@700@" format)
3. Search for zoom percentages and their fractional representations
4. Search for undo limit value 32
5. Search for swatch palette color constants
6. Search for export quality values
7. Use Ghidra for cross-references and decompiler analysis

## Values to record in MECHANICS.md
- Canvas: width, height
- Zoom: list of levels
- Undo: max stack depth
- Swatch: ordered palette
- Export: JPEG quality, PNG compression, etc.
- Tool defaults: brush size, fill/stroke colors
