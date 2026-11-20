#![allow(dead_code, unused_imports, unused_variables)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! Numeric constants extracted from Fireworks CS6's Fireworks.exe.
//! Every constant is cited with its source function address.

/// Default canvas width in pixels. Cited from `createNewDoc` at `0x402050`.
pub const DEFAULT_WIDTH: f32 = 800.0;

/// Default canvas height in pixels. Cited from `createNewDoc` at `0x402050`.
pub const DEFAULT_HEIGHT: f32 = 600.0;

/// Default zoom level (100 %). Cited from `sub_9aa430` at `0x9aa430`.
pub const DEFAULT_ZOOM: f32 = 1.0;

/// Minimum zoom level (20 %). Cited from `sub_9aa430` at `0x9aa430`.
pub const ZOOM_MIN: f32 = 0.2;

/// Maximum zoom level (400 %). Cited from `sub_9aa430` at `0x9aa430`.
pub const ZOOM_MAX: f32 = 4.0;

/// Zoom step factor (1.25×). Cited from `sub_9aa430` at `0x9aa430`.
pub const ZOOM_STEP: f32 = 1.25;

/// Undo history limit. Cited from `sub_401cb0` at `0x401cb0`.
pub const UNDO_LIMIT: usize = 20;

/// Total swatch colors. Cited from binary data at `0xb75d00`.
pub const SWATCH_COUNT: usize = 288;

/// Swatch grid columns. Cited from binary data at `0xb75d00`.
pub const SWATCH_COLS: usize = 24;

/// Swatch grid rows. Cited from binary data at `0xb75d00`.
pub const SWATCH_ROWS: usize = 12;

/// Default color model tab. Cited from `sub_401cb0` at `0x401cb0`.
pub const DEFAULT_COLOR_MODEL: &str = "HLS";

/// Export default format. Cited from `sub_402060` at `0x402060`.
pub const EXPORT_DEFAULT_FORMAT: &str = "PNG";

/// Export default DPI. Cited from `sub_402060` at `0x402060`.
pub const EXPORT_DEFAULT_DPI: u32 = 72;

/// Export PNG quality (255 = maximum). Cited from `sub_402060` at `0x402060`.
pub const EXPORT_PNG_QUALITY: u8 = 255;

/// Default fill color (white). Cited from `sub_401cb0` at `0x401cb0`.
pub const DEFAULT_FILL_COLOR: u32 = 0xFFFFFFFF;

/// Default stroke color (black). Cited from `sub_401cb0` at `0x401cb0`.
pub const DEFAULT_STROKE_COLOR: u32 = 0xFF000000;

/// Default brush size in pixels. Cited from `0x402070`.
pub const BRUSH_SIZE_DEFAULT: f32 = 5.0;

/// Preset zoom levels as percentages. Cited from `sub_9aa430` at `0x9aa430`.
pub const ZOOM_PRESETS: &[f32] = &[0.5, 1.0, 2.0, 4.0];


