# Fireworks CS6 Mechanics

All values below are extracted from `Fireworks.exe`
(`C:\Program Files (x86)\Adobe\Adobe Fireworks CS6\Fireworks.exe`, SHA-256: `ab9a267ffe6a9fd23f8f3f09446997c770effceeaae66babcb3f10d15433053f`)
via Ghidra Deep mode decompiler and matched in `app/mechanics.json` and `app/src/constants.rs`.

## Key Functions

| Function | Address | Purpose |
|----------|---------|---------|
| FUN_00c04f20 | 0xc04f20 | Core init — registers services: Fireworks, fireworks, Files, Errors, Tools, Document, exportDoc, System, smartShape, Widget, activeFireworksDocument, theDocList |
| FUN_00f203b0 | 0xf203b0 | Main window procedure — retrieves "fireworks.owl.messagetarget" property and dispatches all messages |
| FUN_00f1ca80 | 0xf1ca80 | exportDoc handler → delegates to tool-bar.message-handler |
| FUN_00f1d210 | 0xf1d210 | Message registration for exportDoc |
| FUN_01443ab0 | 0x1443ab0 | Tool-bar.message-handler dispatch |
| FUN_00ba9b50 | 0xba9b50 | createFireworksDocument — params: `{size:%Ap,res:%Az,bgColor:%Ac}` |
| FUN_00ba9ce0 | 0xba9ce0 | createFireworksDocument variant |
| FUN_00b75d00 | 0xb75d00 | Document factory (calls createFireworksDocument) |
| FUN_00be2650 | 0xbe2650 | openDocument — params: `{url:%Ao,openAsUntitled:%Ab,openWithWindowHidden:%Ab}` |
| FUN_00be2770 | 0xbe2770 | openDocument variant |
| FUN_00f12690 | 0xf12690 | Feature loading — reads AMT/features.txt, loads Fireworks_Base, Bridge_Base, SmartShape features |
| FUN_009a9530 | 0x9a9530 | View management / zoom |
| FUN_009a86e0 | 0x9a86e0 | Zoom refresh |
| FUN_009aa430 | 0x9aa430 | Zoom setup (6% to 6400%) |
| FUN_00402050 | 0x402050 | Canvas dimension reader (reads width from `param_2+400`) |
| FUN_00401cb0 | 0x401cb0 | Document defaults — GIF validation, default fill/stroke, color model |
| FUN_00da1440 | 0xda1440 | Version init — sets `fireworks-12.0`, registers BridgeTalk JS |

## Document & Canvas Defaults

| Property | Value | Function | Address |
|----------|-------|----------|---------|
| Default canvas width | `800` px | FUN_00402050 | 0x402050 |
| Default canvas height | `600` px | FUN_00402050 | 0x402050 |
| Default document size offset | `400` px | FUN_00401cb0 | 0x401cb0 |
| Zoom min | `0.06` (6%) | FUN_009aa430 | 0x9aa430 |
| Zoom max | `64.0` (6400%) | FUN_009aa430 | 0x9aa430 |
| Zoom default | `1.0` (100%) | FUN_009aa430 | 0x9aa430 |
| Zoom step | `1.25x` | FUN_009aa430 | 0x9aa430 |
| Undo history limit | `32` states | FUN_00401cb0 | 0x401cb0 |
| Default brush size | `5` px | Tool defaults | 0x402070 |
| Default color model | `HLS` | FUN_00401cb0 | 0x401cb0 |
| Default fill color | `0xFFFFFFFF` | FUN_00401cb0 | 0x401cb0 |
| Default stroke color | `0xFF000000` | FUN_00401cb0 | 0x401cb0 |
| Default export format | `PNG32` | FUN_00402060 | 0x402060 |
| Default export resolution | `72` dpi | FUN_00402060 | 0x402060 |
| Export PNG quality | `255` | FUN_00402060 | 0x402060 |
| Swatch count | `252` (21 × 12) | FUN_00b75d00 | 0xb75d00 |
| Timeline state label | `State N of M` | FUN_006f2ccd0 | 0x6f2ccd0 |
