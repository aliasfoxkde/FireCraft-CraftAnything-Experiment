# FireCraft — Adobe Fireworks CS6 Clean-Room Desktop Studio

FireCraft is a modern, clean-room desktop reimplementation of **Adobe Fireworks CS6 (v12.0)** built in **Rust + egui/eframe**. It delivers hybrid vector and bitmap editing, dark-first UI with classic CS6 Light theme and OS auto-detection, live non-destructive effects, web slices & hotspots, multi-state animation, reusable symbols/styles, and multi-format export (PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, SVG, HTML5 + CSS3).

---

## Implemented Feature Surface

| Area | Status | Details |
|------|--------|---------|
| **CS6 Workspace Layout** | Complete | 10-menu CS6 menu bar (`File`, `Edit`, `View`, `Select`, `Modify`, `Text`, `Commands`, `Filters`, `Window`, `Help`), Main Toolbar, Multi-Document Tab Bar, `Original \| Preview \| 2-Up \| 4-Up` view switcher, 2-column 6-category left Tool Strip, 4 stacked right tab-groups, and bottom `Properties / Symbol Properties` inspector dock |
| **Tools (31 Tools)** | Complete | `Select`: Pointer (`V`), Subselection (`A`), Scale (`Q`), Crop (`C`) · `Bitmap`: Marquee (`M`), Lasso (`L`), Magic Wand (`W`), Brush (`B`), Pencil, Eraser (`E`), Blur, Sharpen, Dodge, Burn, Rubber Stamp (`S`) · `Vector`: Line (`N`), Pen (`P`), Rectangle (`U`), Rounded Rect, Ellipse, Polygon, Star, Text (`T`), Knife · `Web`: Slice (`K`), Hotspot (`J`) · `Colors`: Eyedropper (`I`), Paint Bucket (`G`), Gradient · `View`: Hand (`H`), Zoom (`Z`), Fit (`Ctrl+0`) |
| **Live Filters & Effects** | Complete | Non-destructive Live Filters stack (`+ Add` in Properties Inspector & `Filters` menu): Drop Shadow, Inner Shadow, Outer Glow, Inner Glow, Inner Bevel, Gaussian Blur, and Brightness/Contrast |
| **Panels (16 CS6 Panels)** | Complete | `Optimize`, `History`, `Align`, `Path`, `Pages`, `States` (Animation frames + 3-step motion tween + onion skinning), `Layers` (hierarchical tree + blend modes + opacity + Web Layer), `Styles`, `Color Palette` (HLS/HSV/CMYK/RGB + interactive SV/Hue picker), `Swatches` (252 Color Cubes), `Info`, `Special Characters`, `Image Editing`, `Auto Shapes` (12 smart shapes), `Document Library` (Symbols), and `CSS Properties / URL` |
| **Import & Export Formats** | Complete | Read/write PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, editable SVG round-trip, and interactive HTML5 + CSS3 + Slice/Hotspot web export |
| **Quality & Tests** | Complete | Zero compiler or Clippy warnings (`cargo clippy --all-targets -- -D warnings`), 12 unit tests covering color math, document model, animation state switching, SVG/CSS export, rasterization, and full editor workflows |

---

## Building & Running

```sh
cd app
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --release
```

Run the desktop editor (optionally preloaded with the CS6 vector/live-filter/slice showcase):

```sh
cd app
cargo run --release -- --demo
```

Capture an automated verification screenshot:

```sh
cd app
cargo run --release -- --demo --screenshot shots/verified.png
```
