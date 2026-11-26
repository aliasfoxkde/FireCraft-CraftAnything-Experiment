//! The application state, tool & pointer interaction, live effects,
//! multi-document tab controller, and Fireworks CS6 editing commands.

use crate::canvas::{self, TextureCache};
use crate::colors;
use crate::constants;
use crate::demo;
use crate::doc::{
    AnimationState, AutoShapeKind, Document, GradientFill, Layer, LiveEffect, Obj, Page, Shape,
    Snapshot, StylePreset, SymbolDef,
};
use crate::theme::{self, Palette, ThemeMode};

use image::GenericImageView;

use eframe::egui::{self, Color32, Context, TextureHandle, Vec2};

use std::path::PathBuf;

mod arrange;
mod edit;
mod files;
mod find_replace;
mod object;

pub use files::FileIntent;
#[cfg(test)]
pub use files::FileOutcome;

/// What the file dialog is being used for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileMode {
    Open,
    Import,
    Save,
    Export,
}

/// Find & Replace can operate on text/object names or on the color channels Fireworks retrofit
/// tools depend on (flat fills, gradient stops, strokes, and live-effect colors).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FindReplaceMode {
    Text,
    FillColor,
    StrokeColor,
    AnyColor,
}

impl FindReplaceMode {
    pub const ALL: [FindReplaceMode; 4] = [
        FindReplaceMode::Text,
        FindReplaceMode::FillColor,
        FindReplaceMode::StrokeColor,
        FindReplaceMode::AnyColor,
    ];

    pub fn label(self) -> &'static str {
        match self {
            FindReplaceMode::Text => "Text & object names",
            FindReplaceMode::FillColor => "Fill colors",
            FindReplaceMode::StrokeColor => "Stroke colors",
            FindReplaceMode::AnyColor => "Any object color",
        }
    }
}

/// Canvas view mode (CS6 top document bar: Original | Preview | 2-Up | 4-Up).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CanvasViewMode {
    Original,
    Preview,
    TwoUp,
    FourUp,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tool {
    Select,
    Subselect,
    Scale,
    Marquee,
    OvalMarquee,
    Lasso,
    Wand,
    Crop,
    Hand,
    Zoom,
    Fit,
    Brush,
    Pencil,
    Eraser,
    Blur,
    Sharpen,
    Dodge,
    Burn,
    Stamp,
    ReplaceColor,
    RedEye,
    Line,
    Rectangle,
    RoundedRect,
    Ellipse,
    Polygon,
    Star,
    Path,
    Freeform,
    Knife,
    Text,
    Slice,
    Hotspot,
    Picker,
    Fill,
    Gradient,
}

impl Tool {
    pub fn name(self) -> &'static str {
        match self {
            Tool::Select => "Pointer",
            Tool::Subselect => "Subselection",
            Tool::Scale => "Scale / Transform",
            Tool::Marquee => "Marquee Select",
            Tool::OvalMarquee => "Oval Marquee",
            Tool::Lasso => "Lasso Select",
            Tool::Wand => "Magic Wand",
            Tool::Crop => "Crop",
            Tool::Hand => "Hand",
            Tool::Zoom => "Zoom",
            Tool::Fit => "Fit in Window",
            Tool::Brush => "Brush",
            Tool::Pencil => "Pencil",
            Tool::Eraser => "Eraser",
            Tool::Blur => "Blur",
            Tool::Sharpen => "Sharpen",
            Tool::Dodge => "Dodge",
            Tool::Burn => "Burn",
            Tool::Stamp => "Rubber Stamp",
            Tool::ReplaceColor => "Replace Color",
            Tool::RedEye => "Red Eye Removal",
            Tool::Line => "Line",
            Tool::Rectangle => "Rectangle",
            Tool::RoundedRect => "Rounded Rectangle",
            Tool::Ellipse => "Ellipse",
            Tool::Polygon => "Polygon",
            Tool::Star => "Star",
            Tool::Path => "Pen",
            Tool::Freeform => "Freeform Path",
            Tool::Knife => "Knife",
            Tool::Text => "Text",
            Tool::Slice => "Slice",
            Tool::Hotspot => "Hotspot",
            Tool::Picker => "Eyedropper",
            Tool::Fill => "Paint Bucket",
            Tool::Gradient => "Gradient",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PanelId {
    Optimize,
    History,
    Align,
    Path,
    Pages,
    States,
    Layers,
    Styles,
    ColorPalette,
    ColorMixer,
    Swatches,
    Info,
    SpecialChars,
    ImageEditing,
    AutoShapes,
    AutoShapeProps,
    DocumentLibrary,
    CommonLibrary,
    CssProperties,
    Url,
    Behaviors,
}

#[allow(dead_code)]
impl PanelId {
    pub const ALL: [PanelId; 21] = [
        PanelId::Optimize,
        PanelId::History,
        PanelId::Align,
        PanelId::Path,
        PanelId::Pages,
        PanelId::States,
        PanelId::Layers,
        PanelId::Styles,
        PanelId::ColorPalette,
        PanelId::ColorMixer,
        PanelId::Swatches,
        PanelId::Info,
        PanelId::SpecialChars,
        PanelId::ImageEditing,
        PanelId::AutoShapes,
        PanelId::AutoShapeProps,
        PanelId::DocumentLibrary,
        PanelId::CommonLibrary,
        PanelId::CssProperties,
        PanelId::Url,
        PanelId::Behaviors,
    ];

    pub fn title(self) -> &'static str {
        match self {
            PanelId::Optimize => "Optimize",
            PanelId::History => "History",
            PanelId::Align => "Align",
            PanelId::Path => "Path",
            PanelId::Pages => "Pages",
            PanelId::States => "States",
            PanelId::Layers => "Layers",
            PanelId::Styles => "Styles",
            PanelId::ColorPalette => "Color Palette",
            PanelId::ColorMixer => "Color Mixer",
            PanelId::Swatches => "Swatches",
            PanelId::Info => "Info",
            PanelId::SpecialChars => "Special Characters",
            PanelId::ImageEditing => "Image Editing",
            PanelId::AutoShapes => "Auto Shapes",
            PanelId::AutoShapeProps => "Auto Shape Properties",
            PanelId::DocumentLibrary => "Document Library",
            PanelId::CommonLibrary => "Common Library",
            PanelId::CssProperties => "CSS Properties",
            PanelId::Url => "URL",
            PanelId::Behaviors => "Behaviors",
        }
    }

    pub fn short_tab(self) -> &'static str {
        match self {
            PanelId::ColorPalette => "Palette",
            PanelId::ColorMixer => "Mixer",
            PanelId::SpecialChars => "Chars",
            PanelId::ImageEditing => "Image",
            PanelId::AutoShapes => "Shapes",
            PanelId::AutoShapeProps => "Shape Props",
            PanelId::DocumentLibrary => "Doc Lib",
            PanelId::CommonLibrary => "Common",
            PanelId::CssProperties => "CSS",
            PanelId::Behaviors => "Behaviors",
            other => other.title(),
        }
    }

    pub fn shortcut(self) -> &'static str {
        match self {
            PanelId::Optimize => "F6",
            PanelId::Layers => "F2",
            PanelId::CommonLibrary => "F7",
            PanelId::CssProperties => "Ctrl+F7",
            PanelId::Pages => "F5",
            PanelId::States => "Shift+F2",
            PanelId::History => "Shift+F10",
            PanelId::Styles => "Ctrl+F11",
            PanelId::DocumentLibrary => "F11",
            PanelId::Url => "Alt+Shift+F10",
            PanelId::ColorMixer => "Shift+F9",
            PanelId::Swatches => "Ctrl+F9",
            PanelId::Info => "Alt+Shift+F12",
            PanelId::Behaviors => "Shift+F3",
            _ => "",
        }
    }

    pub fn default_height(self) -> f32 {
        crate::panels::default_panel_height(self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RulerUnit {
    Pixels,
    Inches,
    Centimeters,
    Millimeters,
}

impl RulerUnit {
    pub const ALL: [RulerUnit; 4] = [
        RulerUnit::Pixels,
        RulerUnit::Inches,
        RulerUnit::Centimeters,
        RulerUnit::Millimeters,
    ];

    pub fn label(self) -> &'static str {
        match self {
            RulerUnit::Pixels => "Pixels (px)",
            RulerUnit::Inches => "Inches (Imperial)",
            RulerUnit::Centimeters => "Centimeters (Metric)",
            RulerUnit::Millimeters => "Millimeters (Metric)",
        }
    }

    pub fn short_code(self) -> &'static str {
        match self {
            RulerUnit::Pixels => "px",
            RulerUnit::Inches => "in",
            RulerUnit::Centimeters => "cm",
            RulerUnit::Millimeters => "mm",
        }
    }

    pub fn px_per_unit(self, dpi: f32) -> f32 {
        let d = dpi.max(72.0);
        match self {
            RulerUnit::Pixels => 1.0,
            RulerUnit::Inches => d,
            RulerUnit::Centimeters => d / 2.54,
            RulerUnit::Millimeters => d / 25.4,
        }
    }

    pub fn subdivisions(self) -> usize {
        match self {
            RulerUnit::Pixels => 5,
            RulerUnit::Inches => 8,       // 1/8", 1/4", 1/2"
            RulerUnit::Centimeters => 10, // 1mm, 5mm
            RulerUnit::Millimeters => 5,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DockZone {
    RightOuter,
    RightInner,
    LeftInner,
    Floating,
}

#[derive(Clone, Debug)]
pub struct PanelGroup {
    pub id: u64,
    pub tabs: Vec<PanelId>,
    pub active_tab: usize,
    pub zone: DockZone,
    pub collapsed: bool,
    pub body_height: f32,
    pub floating_pos: egui::Pos2,
    pub floating_width: f32,
    /// When set, this floating group is vertically stacked under the referenced group and
    /// follows it when that group moves.
    pub stack_parent: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct WebBehavior {
    pub event: String,
    pub action: String,
    pub info: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DragKind {
    Marquee,
    Create,
    Move,
    MoveNode(usize),
    AdjustCornerRadius,
    AdjustStarInner,
    Resize(usize),
    Pan,
    Paint,
    Lasso,
}

pub struct Drag {
    pub kind: DragKind,
    pub start: Vec2,
    pub current: Vec2,
    pub orig_bounds: Option<(Vec2, Vec2)>,
    pub path: Vec<Vec2>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PaperSize {
    #[default]
    Letter,
    A4,
    Legal,
    Tabloid,
}

impl PaperSize {
    pub fn label(self) -> &'static str {
        match self {
            PaperSize::Letter => "Letter (8.5 × 11 in)",
            PaperSize::A4 => "A4 (210 × 297 mm)",
            PaperSize::Legal => "Legal (8.5 × 14 in)",
            PaperSize::Tabloid => "Tabloid (11 × 17 in)",
        }
    }

    pub fn css_name(self) -> &'static str {
        match self {
            PaperSize::Letter => "letter",
            PaperSize::A4 => "A4",
            PaperSize::Legal => "legal",
            PaperSize::Tabloid => "tabloid",
        }
    }
}

#[derive(Clone, Debug)]
pub struct PageSetupConfig {
    pub paper: PaperSize,
    pub landscape: bool,
    pub margin_in: f32,
    pub fit_to_page: bool,
    pub scale_pct: f32,
    pub print_guides_slices: bool,
}

impl Default for PageSetupConfig {
    fn default() -> Self {
        Self {
            paper: PaperSize::Letter,
            landscape: false,
            margin_in: 0.5,
            fit_to_page: true,
            scale_pct: 100.0,
            print_guides_slices: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ToolsDockMode {
    #[default]
    Left,
    Right,
    Floating,
}

pub struct FireCraft {
    pub ctx: Context,
    /// Document I/O boundary; defaults to `FormatFileService` (real disk I/O).
    pub file_io: Box<dyn files::FileService>,
    pub doc: Document,
    pub open_docs: Vec<Document>,
    pub active_doc: usize,
    /// Most-recently-used document indices, newest first. The tab strip uses this to keep the
    /// newest documents visible and fold the rest into the overflow menu.
    pub doc_tab_history: Vec<usize>,
    pub theme_mode: ThemeMode,
    pub palette: Palette,
    pub view_mode: CanvasViewMode,
    pub tool: Tool,
    pub fill: Color32,
    pub stroke: Color32,
    pub swatches: Vec<Color32>,
    pub selected: Option<usize>,
    pub selected_set: Vec<usize>,
    pub selected_extra_layers: Vec<(usize, usize)>,
    pub selected_layer: usize,
    pub context_menu_pos: Option<egui::Pos2>,
    pub show_command_palette: bool,
    pub palette_filter: String,
    pub show_mcp_dialog: bool,
    pub mcp_input: String,
    pub mcp_output: String,
    pub drag: Option<Drag>,
    pub undo: Vec<Snapshot>,
    pub redo: Vec<Snapshot>,
    pub status: String,
    pub top_right_tab: usize,
    pub layers_tab: usize,
    pub middle_right_tab: usize,
    pub library_tab: usize,
    pub bottom_tab: usize,
    pub color_tab: usize,
    pub color_subtab: usize,
    pub active_menu: Option<usize>,
    pub active_menu_item: usize,
    pub show_main_toolbar: bool,
    pub show_tools_dock: bool,
    pub tools_collapsed: bool,
    pub tools_zone: ToolsDockMode,
    pub tools_floating_pos: egui::Pos2,
    pub show_right_dock: bool,
    pub right_dock_collapsed: bool,
    pub right_dock_zone: ToolsDockMode,
    pub right_dock_floating_pos: egui::Pos2,
    pub iconic_flyout_panel: Option<PanelId>,
    pub floating_doc_tabs: Vec<usize>,
    pub show_bottom_inspector: bool,
    pub bottom_collapsed: bool,
    pub bottom_height: f32,
    pub full_screen_canvas: bool,
    pub window_fullscreen: bool,
    pub panel_groups: Vec<PanelGroup>,
    pub next_panel_group_id: u64,
    pub dragging_panel: Option<PanelId>,
    pub dragging_group: Option<u64>,
    pub panel_title_rects: Vec<(u64, egui::Rect)>,
    /// Open CS6 tool-group flyout: flattened group index and screen anchor.
    pub tool_flyout: Option<(usize, egui::Pos2)>,
    /// Nodes placed by the Pen tool before the path is committed.
    pub pen_nodes: Vec<Vec2>,
    /// Ask the canvas to take keyboard focus after a tab/view activation.
    pub focus_canvas: bool,
    pub workspace_preset: &'static str,
    pub anim_playing: bool,
    pub anim_last_tick: f64,
    pub behaviors: Vec<WebBehavior>,
    pub new_url_input: String,
    #[allow(dead_code)]
    pub rename_page_idx: Option<usize>,
    #[allow(dead_code)]
    pub rename_page_buf: String,
    #[allow(dead_code)]
    pub inline_text_edit_obj: Option<usize>,
    pub smart_guides_lines: Vec<(Vec2, Vec2)>,
    pub textures: TextureCache,
    pub logo_tex: Option<egui::TextureHandle>,
    pub frames: u32,
    pub screenshot_path: Option<String>,
    pub screenshot_saved: bool,
    pub show_rulers: bool,
    pub ruler_unit: RulerUnit,
    pub show_grid: bool,
    pub snap_to_grid: bool,
    pub snap_to_guides: bool,
    pub snap_to_objects: bool,
    pub show_guides: bool,
    pub show_slices: bool,
    pub infinite_canvas: bool,
    pub onion_skin: bool,
    pub align_to_canvas: bool,
    pub space_mode: String,
    pub alt_hand_pan: bool,
    pub canvas_size: Vec2,
    /// Set when a view should be fitted once the canvas rect is known. Fitting at load time
    /// used a stale/zero canvas size, so a demo opened at 960 px kept the 1360 px zoom.
    pub pending_fit: bool,
    pub cursor_doc_pos: Option<Vec2>,
    pub brush_size: f32,
    pub grid_size: f32,
    pub polygon_sides: u32,
    pub star_ratio: f32,
    pub current_path: Option<PathBuf>,
    pub recent_files: Vec<PathBuf>,
    pub show_new_dialog: bool,
    pub show_canvas_size_dialog: bool,
    pub show_html_setup_dialog: bool,
    pub show_page_setup_dialog: bool,
    pub html_setup: crate::formats::HtmlExportOptions,
    pub page_setup: PageSetupConfig,
    pub style_library: crate::doc::StyleLibrary,
    pub selected_style_idx: Option<usize>,
    pub show_find_replace: bool,
    pub show_commands_dialog: bool,
    pub show_about_dialog: bool,
    pub show_update_dialog: bool,
    pub pending_close_tab: Option<usize>,
    /// Remaining tabs in a multi-tab close, already ordered highest index first so an earlier
    /// close cannot invalidate the next target.
    pub pending_close_tabs: Vec<usize>,
    /// A tab close waiting on the Save As browser. Cleared if the browser is cancelled so an
    /// unsaved document is never discarded by accident.
    pub pending_close_after_save: Option<usize>,
    /// The quit flow is waiting for one or more Save As choices before it can close the app.
    pub pending_quit_after_save: bool,
    pub show_quit_confirm: bool,
    /// The user explicitly discarded unsaved work. The next OS close request must be honored
    /// instead of being cancelled by the unsaved-work interceptor.
    pub allow_close: bool,
    pub update_status: String,
    pub update_latest_version: Option<String>,
    pub update_release_url: Option<String>,
    pub find_query: String,
    pub replace_query: String,
    pub find_replace_mode: FindReplaceMode,
    pub find_color: Color32,
    pub replace_color: Color32,
    pub find_case_sensitive: bool,
    pub saved_commands: Vec<String>,
    pub new_command_name: String,
    pub new_doc_width: f32,
    pub new_doc_height: f32,
    pub new_doc_dpi: u32,
    pub new_doc_bg: [f32; 3],
    pub new_doc_transparent: bool,
    pub show_file_dialog: bool,
    pub file_mode: FileMode,
    pub file_dialog_dir: PathBuf,
    pub file_dialog_entries: Vec<PathBuf>,
    pub file_dialog_name: String,
    pub file_dialog_address: String,
    pub file_dialog_filter: String,
    pub file_dialog_type_filter: usize,
    pub clipboard: Option<Obj>,
    pub clipboard_set: Vec<Obj>,
    pub last_effect: Option<LiveEffect>,
    pub ai_base_url: String,
    pub ai_model: String,
    pub ai_key: String,
    pub ai_prompt: String,
    pub ai_response: String,
    pub ai_status: String,
    pub ai_rx: Option<std::sync::mpsc::Receiver<Result<String, String>>>,
    pub show_ai_settings: bool,
    pub show_welcome: bool,
    pub dont_show_welcome_again: bool,
}

impl FireCraft {
    pub fn new(screenshot_path: Option<String>) -> Self {
        let show_welcome_default = screenshot_path.is_none();
        let theme_mode = ThemeMode::System;
        let palette = theme::resolve(theme_mode);
        let mut initial_doc = Document::new(constants::DEFAULT_WIDTH, constants::DEFAULT_HEIGHT);
        initial_doc.zoom = 0.50; // Fit 1920x1080 comfortably in the initial viewport
        let mut app = Self {
            ctx: Context::default(),
            file_io: Box::new(files::FormatFileService),
            doc: initial_doc.clone(),
            open_docs: vec![initial_doc],
            active_doc: 0,
            doc_tab_history: vec![0],
            theme_mode,
            palette,
            view_mode: CanvasViewMode::Original,
            tool: Tool::Select,
            fill: Color32::from_rgb(232, 163, 61),
            stroke: Color32::from_rgb(24, 24, 28),
            swatches: colors::swatch_grid(),
            selected: None,
            selected_set: Vec::new(),
            selected_extra_layers: Vec::new(),
            selected_layer: 0,
            context_menu_pos: None,
            show_command_palette: false,
            palette_filter: String::new(),
            show_mcp_dialog: false,
            mcp_input: r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#.to_string(),
            mcp_output: String::new(),
            drag: None,
            undo: Vec::new(),
            redo: Vec::new(),
            status: "Ready — FireCraft Studio (Adobe Fireworks CS6 Feature Parity)".to_string(),
            top_right_tab: 0,
            layers_tab: 2,
            middle_right_tab: 1,
            library_tab: 2,
            bottom_tab: 0,
            color_tab: 0,
            color_subtab: 0,
            active_menu: None,
            active_menu_item: 0,
            show_main_toolbar: true,
            show_tools_dock: true,
            tools_collapsed: false,
            tools_zone: ToolsDockMode::Left,
            tools_floating_pos: egui::pos2(12.0, 96.0),
            show_right_dock: true,
            right_dock_collapsed: false,
            right_dock_zone: ToolsDockMode::Right,
            right_dock_floating_pos: egui::pos2(860.0, 96.0),
            iconic_flyout_panel: None,
            floating_doc_tabs: Vec::new(),
            show_bottom_inspector: true,
            bottom_collapsed: false,
            bottom_height: 142.0,
            full_screen_canvas: false,
            window_fullscreen: false,
            // Expanded Mode is CS6's default workspace: one right dock with five
            // stacked tab-groups, the full main toolbar, and the Properties inspector.
            panel_groups: Self::default_panel_groups(),
            next_panel_group_id: 100,
            dragging_panel: None,
            dragging_group: None,
            panel_title_rects: Vec::new(),
            tool_flyout: None,
            pen_nodes: Vec::new(),
            focus_canvas: false,
            workspace_preset: "Expanded Mode",
            anim_playing: false,
            anim_last_tick: 0.0,
            behaviors: vec![
                WebBehavior {
                    event: "onMouseOver".to_string(),
                    action: "Simple Rollover".to_string(),
                    info: "Swap to State 2".to_string(),
                },
                WebBehavior {
                    event: "onClick".to_string(),
                    action: "Set Nav Bar Image".to_string(),
                    info: "Target: _blank".to_string(),
                },
            ],
            new_url_input: "https://github.com/aliasfoxkde/FireCraft".to_string(),
            rename_page_idx: None,
            rename_page_buf: String::new(),
            inline_text_edit_obj: None,
            smart_guides_lines: Vec::new(),
            textures: TextureCache::new(),
            logo_tex: None,
            frames: 0,
            screenshot_path,
            screenshot_saved: false,
            show_rulers: true,
            ruler_unit: RulerUnit::Pixels,
            show_grid: false,
            snap_to_grid: false,
            snap_to_guides: true,
            snap_to_objects: true,
            show_guides: true,
            show_slices: true,
            pending_fit: false,
            infinite_canvas: false,
            onion_skin: false,
            align_to_canvas: false,
            space_mode: "Evenly".to_string(),
            alt_hand_pan: false,
            canvas_size: Vec2::new(980.0, 660.0),
            cursor_doc_pos: None,
            brush_size: constants::BRUSH_SIZE_DEFAULT,
            grid_size: 20.0,
            polygon_sides: 5,
            star_ratio: 0.48,
            current_path: None,
            recent_files: Vec::new(),
            show_new_dialog: false,
            show_canvas_size_dialog: false,
            show_html_setup_dialog: false,
            show_page_setup_dialog: false,
            html_setup: crate::formats::HtmlExportOptions::default(),
            page_setup: PageSetupConfig::default(),
            style_library: crate::doc::StyleLibrary::DarkChrome,
            selected_style_idx: None,
            show_find_replace: false,
            show_commands_dialog: false,
            show_about_dialog: false,
            show_update_dialog: false,
            pending_close_tab: None,
            pending_close_tabs: Vec::new(),
            pending_close_after_save: None,
            pending_quit_after_save: false,
            show_quit_confirm: false,
            allow_close: false,
            update_status: format!("Current version: v{}", env!("CARGO_PKG_VERSION")),
            update_latest_version: None,
            update_release_url: Some(
                "https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/latest"
                    .to_string(),
            ),
            find_query: String::new(),
            replace_query: String::new(),
            find_replace_mode: FindReplaceMode::Text,
            find_color: Color32::from_rgb(232, 163, 61),
            replace_color: Color32::from_rgb(56, 152, 255),
            find_case_sensitive: true,
            saved_commands: vec![
                "Add Drop Shadow Card".to_string(),
                "Add Picture Frame".to_string(),
                "Distribute 3×3 Guides".to_string(),
                "Convert Selection to Grayscale".to_string(),
                "Create Slice from Selection".to_string(),
            ],
            new_command_name: String::new(),
            new_doc_width: constants::DEFAULT_WIDTH,
            new_doc_height: constants::DEFAULT_HEIGHT,
            new_doc_dpi: constants::EXPORT_DEFAULT_DPI,
            new_doc_bg: [1.0, 1.0, 1.0],
            new_doc_transparent: false,
            show_file_dialog: false,
            file_mode: FileMode::Open,
            file_dialog_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            file_dialog_entries: Vec::new(),
            file_dialog_name: String::new(),
            file_dialog_address: String::new(),
            file_dialog_filter: String::new(),
            file_dialog_type_filter: 0,
            clipboard: None,
            clipboard_set: Vec::new(),
            last_effect: None,
            ai_base_url: std::env::var("FIRECRAFT_AI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string()),
            ai_model: std::env::var("FIRECRAFT_AI_MODEL").unwrap_or_default(),
            ai_key: String::new(),
            ai_prompt: String::new(),
            ai_response: String::new(),
            ai_status: "Configure an OpenAI-compatible endpoint to enable AI assistance."
                .to_string(),
            ai_rx: None,
            show_ai_settings: true,
            show_welcome: show_welcome_default,
            dont_show_welcome_again: false,
        };
        app.create_default_button_symbol();
        app
    }

    pub fn default_panel_groups() -> Vec<PanelGroup> {
        vec![
            PanelGroup {
                id: 1,
                tabs: vec![
                    PanelId::Optimize,
                    PanelId::History,
                    PanelId::Align,
                    PanelId::Path,
                ],
                active_tab: 0,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 182.0,
                floating_pos: egui::pos2(740.0, 62.0),
                floating_width: 286.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 2,
                tabs: vec![
                    PanelId::Pages,
                    PanelId::States,
                    PanelId::Layers,
                    PanelId::Behaviors,
                ],
                active_tab: 2,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 140.0,
                floating_pos: egui::pos2(740.0, 270.0),
                floating_width: 286.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 3,
                tabs: vec![
                    PanelId::Styles,
                    PanelId::ColorPalette,
                    PanelId::ColorMixer,
                    PanelId::Swatches,
                ],
                active_tab: 1,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 176.0,
                floating_pos: egui::pos2(740.0, 440.0),
                floating_width: 286.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 4,
                tabs: vec![
                    PanelId::Info,
                    PanelId::SpecialChars,
                    PanelId::ImageEditing,
                    PanelId::AutoShapes,
                    PanelId::AutoShapeProps,
                ],
                active_tab: 3,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 104.0,
                floating_pos: egui::pos2(740.0, 600.0),
                floating_width: 286.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 5,
                tabs: vec![
                    PanelId::DocumentLibrary,
                    PanelId::CommonLibrary,
                    PanelId::CssProperties,
                    PanelId::Url,
                ],
                active_tab: 0,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 96.0,
                floating_pos: egui::pos2(740.0, 720.0),
                floating_width: 286.0,
                stack_parent: None,
            },
        ]
    }

    pub fn cs6_dual_column_groups() -> Vec<PanelGroup> {
        // Exact dual-column right dock from screenshots/Fireworks_Layout_Design.jpg
        vec![
            // Inner Right Column
            PanelGroup {
                id: 11,
                tabs: vec![PanelId::Swatches],
                active_tab: 0,
                zone: DockZone::RightInner,
                collapsed: false,
                body_height: 142.0,
                floating_pos: egui::pos2(680.0, 62.0),
                floating_width: 268.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 12,
                tabs: vec![PanelId::ColorPalette, PanelId::ColorMixer, PanelId::Info],
                active_tab: 0,
                zone: DockZone::RightInner,
                collapsed: false,
                body_height: 175.0,
                floating_pos: egui::pos2(680.0, 232.0),
                floating_width: 268.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 13,
                tabs: vec![PanelId::Align],
                active_tab: 0,
                zone: DockZone::RightInner,
                collapsed: false,
                body_height: 158.0,
                floating_pos: egui::pos2(680.0, 434.0),
                floating_width: 268.0,
                stack_parent: None,
            },
            // Outer Right Column
            PanelGroup {
                id: 14,
                tabs: vec![PanelId::Optimize, PanelId::History, PanelId::Path],
                active_tab: 2,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 186.0,
                floating_pos: egui::pos2(960.0, 62.0),
                floating_width: 286.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 15,
                tabs: vec![PanelId::Pages, PanelId::States, PanelId::Layers],
                active_tab: 2,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 140.0,
                floating_pos: egui::pos2(960.0, 274.0),
                floating_width: 286.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 16,
                tabs: vec![PanelId::Styles],
                active_tab: 0,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 98.0,
                floating_pos: egui::pos2(960.0, 440.0),
                floating_width: 286.0,
                stack_parent: None,
            },
            PanelGroup {
                id: 17,
                tabs: vec![
                    PanelId::SpecialChars,
                    PanelId::ImageEditing,
                    PanelId::AutoShapes,
                    PanelId::AutoShapeProps,
                    PanelId::DocumentLibrary,
                    PanelId::CommonLibrary,
                    PanelId::Url,
                    PanelId::Behaviors,
                    PanelId::CssProperties,
                ],
                active_tab: 2,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 112.0,
                floating_pos: egui::pos2(960.0, 564.0),
                floating_width: 286.0,
                stack_parent: None,
            },
        ]
    }

    pub fn cs6_all_panels_floating_groups() -> Vec<PanelGroup> {
        // Exact multi-column snapped floating panels from screenshots/Fireworks-All_Window_Panels.jpg
        let mut groups = Self::cs6_dual_column_groups();
        let floating_specs: &[(u64, PanelId, f32, f32, f32)] = &[
            (21, PanelId::ColorMixer, 290.0, 68.0, 128.0),
            (22, PanelId::Info, 290.0, 224.0, 96.0),
            (23, PanelId::Url, 290.0, 348.0, 110.0),
            (24, PanelId::Behaviors, 290.0, 486.0, 110.0),
            (25, PanelId::ImageEditing, 536.0, 68.0, 152.0),
            (26, PanelId::SpecialChars, 536.0, 248.0, 134.0),
            (27, PanelId::CommonLibrary, 536.0, 410.0, 118.0),
            (28, PanelId::AutoShapeProps, 74.0, 540.0, 108.0),
        ];
        for &(gid, pid, x, y, h) in floating_specs {
            for g in &mut groups {
                g.tabs.retain(|&t| t != pid);
                if g.active_tab >= g.tabs.len() && !g.tabs.is_empty() {
                    g.active_tab = g.tabs.len() - 1;
                }
            }
            groups.retain(|g| !g.tabs.is_empty());
            groups.push(PanelGroup {
                id: gid,
                tabs: vec![pid],
                active_tab: 0,
                zone: DockZone::Floating,
                collapsed: false,
                body_height: h,
                floating_pos: egui::pos2(x, y),
                floating_width: 236.0,
                stack_parent: None,
            });
        }
        groups
    }

    pub fn apply_workspace_preset(&mut self, preset: &'static str) {
        self.workspace_preset = preset;
        match preset {
            "Expanded Mode" => {
                self.full_screen_canvas = false;
                self.show_main_toolbar = true;
                self.show_tools_dock = true;
                self.tools_collapsed = false;
                self.show_right_dock = true;
                self.right_dock_collapsed = false;
                self.right_dock_zone = ToolsDockMode::Right;
                self.iconic_flyout_panel = None;
                self.show_bottom_inspector = true;
                self.bottom_collapsed = false;
                self.panel_groups = Self::default_panel_groups();
                self.status = "Workspace set to CS6 Expanded Mode (Single Right Dock)".to_string();
            }
            "CS6 Dual-Column" => {
                self.full_screen_canvas = false;
                self.show_main_toolbar = true;
                self.show_tools_dock = true;
                self.tools_collapsed = false;
                self.show_right_dock = true;
                self.right_dock_collapsed = false;
                self.right_dock_zone = ToolsDockMode::Right;
                self.iconic_flyout_panel = None;
                self.show_bottom_inspector = true;
                self.bottom_collapsed = false;
                self.panel_groups = Self::cs6_dual_column_groups();
                self.status =
                    "Workspace set to CS6 Dual-Column Dock (Swatches/Color/Align + Optimize/Path/Layers)"
                        .to_string();
            }
            "All Panels Floating" => {
                self.full_screen_canvas = false;
                self.show_main_toolbar = true;
                self.show_tools_dock = true;
                self.show_right_dock = true;
                self.right_dock_collapsed = false;
                self.iconic_flyout_panel = None;
                self.show_bottom_inspector = true;
                self.show_welcome = false;
                self.panel_groups = Self::cs6_all_panels_floating_groups();
                self.status =
                    "Opened all CS6 Window Panels (drag headers to snap, drag tabs to group, or click 📌 to dock)"
                        .to_string();
            }
            "Iconic Mode" => {
                self.full_screen_canvas = false;
                self.show_main_toolbar = true;
                self.show_tools_dock = true;
                self.tools_collapsed = false;
                self.tools_zone = ToolsDockMode::Left;
                self.show_right_dock = true;
                self.right_dock_collapsed = true;
                self.right_dock_zone = ToolsDockMode::Right;
                self.iconic_flyout_panel = None;
                self.show_bottom_inspector = true;
                self.bottom_collapsed = false;
                self.panel_groups = Self::default_panel_groups();
                self.status =
                    "Workspace set to Iconic Mode (click any dock icon to open flyout, or << to expand)"
                        .to_string();
            }
            "Iconic Mode With Panel Names" => {
                self.full_screen_canvas = false;
                self.show_main_toolbar = true;
                self.show_tools_dock = true;
                self.tools_collapsed = false;
                self.tools_zone = ToolsDockMode::Left;
                self.show_right_dock = true;
                self.right_dock_collapsed = true;
                self.right_dock_zone = ToolsDockMode::Right;
                self.iconic_flyout_panel = None;
                self.show_bottom_inspector = true;
                self.bottom_collapsed = false;
                self.panel_groups = Self::default_panel_groups();
                self.status =
                    "Workspace set to Iconic Mode With Panel Names (click any row or << to expand)"
                        .to_string();
            }
            "Netbook Mode" => {
                self.full_screen_canvas = false;
                self.show_main_toolbar = true;
                self.show_tools_dock = true;
                self.tools_collapsed = true;
                self.show_right_dock = false;
                self.iconic_flyout_panel = None;
                self.show_bottom_inspector = false;
                self.status = "Workspace set to Netbook Mode".to_string();
            }
            "Full Screen" => {
                self.full_screen_canvas = false;
                self.toggle_full_screen_canvas();
            }
            _ => {}
        }
    }

    #[allow(dead_code)]
    pub fn is_panel_open(&self, id: PanelId) -> bool {
        self.panel_groups.iter().any(|g| g.tabs.contains(&id))
    }

    #[allow(dead_code)]
    pub fn sync_legacy_tabs_to_groups(&mut self) {
        // If legacy test/CLI fields mutated top_right_tab / layers_tab / middle_right_tab / library_tab,
        // ensure the corresponding PanelGroup reflects that active tab.
        let g1 = [
            PanelId::Optimize,
            PanelId::History,
            PanelId::Align,
            PanelId::Path,
        ];
        if let Some(&pid) = g1.get(self.top_right_tab) {
            for g in &mut self.panel_groups {
                if g.id == 1
                    && let Some(pos) = g.tabs.iter().position(|&t| t == pid)
                {
                    g.active_tab = pos;
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn focus_or_open_panel(&mut self, id: PanelId) {
        self.full_screen_canvas = false;
        self.show_right_dock = true;
        for g in &mut self.panel_groups {
            if let Some(pos) = g.tabs.iter().position(|&t| t == id) {
                g.active_tab = pos;
                g.collapsed = false;
                self.status = format!("Focused {} panel", id.title());
                return;
            }
        }
        // Panel wasn't in any open group; add it as a new docked group or into the closest group
        let gid = self.next_panel_group_id;
        self.next_panel_group_id += 1;
        self.panel_groups.push(PanelGroup {
            id: gid,
            tabs: vec![id],
            active_tab: 0,
            zone: DockZone::RightOuter,
            collapsed: false,
            body_height: id.default_height(),
            floating_pos: egui::pos2(420.0, 96.0),
            floating_width: 272.0,
            stack_parent: None,
        });
        self.status = format!("Opened {} panel", id.title());
    }

    #[allow(dead_code)]
    pub fn toggle_panel_visibility(&mut self, id: PanelId) {
        if self.is_panel_open(id) {
            // If it's already the active tab of an expanded group, close it; otherwise focus it
            let already_active = self
                .panel_groups
                .iter()
                .any(|g| !g.collapsed && g.tabs.get(g.active_tab).copied() == Some(id));
            if already_active {
                self.close_panel(id);
            } else {
                self.focus_or_open_panel(id);
            }
        } else {
            self.focus_or_open_panel(id);
        }
    }

    pub fn close_panel(&mut self, id: PanelId) {
        for g in &mut self.panel_groups {
            if let Some(pos) = g.tabs.iter().position(|&t| t == id) {
                g.tabs.remove(pos);
                if g.active_tab >= g.tabs.len() && !g.tabs.is_empty() {
                    g.active_tab = g.tabs.len() - 1;
                }
            }
        }
        self.panel_groups.retain(|g| !g.tabs.is_empty());
        self.status = format!(
            "Closed {} panel (reopen anytime via Window menu)",
            id.title()
        );
    }

    pub fn ungroup_panel_to_floating(&mut self, id: PanelId, pos: egui::Pos2) {
        for g in &mut self.panel_groups {
            if g.tabs.len() == 1 && g.tabs[0] == id {
                g.zone = DockZone::Floating;
                g.floating_pos = pos;
                g.collapsed = false;
                self.status = format!(
                    "Undocked {} to Floating Window (drag edges to snap, or click 📌 to dock)",
                    id.title()
                );
                return;
            }
            if let Some(idx) = g.tabs.iter().position(|&t| t == id) {
                g.tabs.remove(idx);
                if g.active_tab >= g.tabs.len() && !g.tabs.is_empty() {
                    g.active_tab = g.tabs.len() - 1;
                }
            }
        }
        self.panel_groups.retain(|g| !g.tabs.is_empty());
        let gid = self.next_panel_group_id;
        self.next_panel_group_id += 1;
        self.panel_groups.push(PanelGroup {
            id: gid,
            tabs: vec![id],
            active_tab: 0,
            zone: DockZone::Floating,
            collapsed: false,
            body_height: id.default_height(),
            floating_pos: pos,
            floating_width: 274.0,
            stack_parent: None,
        });
        self.status = format!(
            "Split {} into a Floating Window (drag onto any panel's tab bar to re-group)",
            id.title()
        );
    }

    pub fn ungroup_panel_below(&mut self, id: PanelId) {
        let mut target_zone = DockZone::RightOuter;
        let mut insert_after = None;
        for (gi, g) in self.panel_groups.iter_mut().enumerate() {
            if g.tabs.len() > 1
                && let Some(ti) = g.tabs.iter().position(|&t| t == id)
            {
                g.tabs.remove(ti);
                if g.active_tab >= g.tabs.len() {
                    g.active_tab = g.tabs.len() - 1;
                }
                target_zone = g.zone;
                insert_after = Some(gi + 1);
                break;
            }
        }
        if let Some(ins) = insert_after {
            let gid = self.next_panel_group_id;
            self.next_panel_group_id += 1;
            let new_group = PanelGroup {
                id: gid,
                tabs: vec![id],
                active_tab: 0,
                zone: target_zone,
                collapsed: false,
                body_height: id.default_height(),
                floating_pos: egui::pos2(440.0, 120.0),
                floating_width: 274.0,
                stack_parent: None,
            };
            if ins <= self.panel_groups.len() {
                self.panel_groups.insert(ins, new_group);
            } else {
                self.panel_groups.push(new_group);
            }
            self.status = format!("Ungrouped {} into its own panel section", id.title());
        }
    }

    pub fn group_panel_into(&mut self, panel: PanelId, target_group_id: u64) {
        if self
            .panel_groups
            .iter()
            .any(|g| g.id == target_group_id && g.tabs.len() == 1 && g.tabs[0] == panel)
        {
            return;
        }
        // Remove `panel` from its current group
        for g in &mut self.panel_groups {
            if g.id != target_group_id
                && let Some(pos) = g.tabs.iter().position(|&t| t == panel)
            {
                g.tabs.remove(pos);
                if g.active_tab >= g.tabs.len() && !g.tabs.is_empty() {
                    g.active_tab = g.tabs.len() - 1;
                }
            }
        }
        self.panel_groups.retain(|g| !g.tabs.is_empty());
        // Add `panel` to `target_group_id`
        if let Some(target) = self
            .panel_groups
            .iter_mut()
            .find(|g| g.id == target_group_id)
        {
            if let Some(existing) = target.tabs.iter().position(|&t| t == panel) {
                target.active_tab = existing;
            } else {
                target.tabs.push(panel);
                target.active_tab = target.tabs.len() - 1;
            }
            target.collapsed = false;
            self.status = format!("Grouped {} into tab group", panel.title());
        }
    }

    pub fn merge_groups(&mut self, src_id: u64, dst_id: u64) {
        if src_id == dst_id {
            return;
        }
        let Some(src_idx) = self.panel_groups.iter().position(|g| g.id == src_id) else {
            return;
        };
        let src_tabs = self.panel_groups[src_idx].tabs.clone();
        self.panel_groups.remove(src_idx);
        for g in &mut self.panel_groups {
            if g.stack_parent == Some(src_id) {
                g.stack_parent = None;
            }
        }
        if let Some(dst) = self.panel_groups.iter_mut().find(|g| g.id == dst_id) {
            for t in src_tabs {
                if !dst.tabs.contains(&t) {
                    dst.tabs.push(t);
                }
            }
            dst.active_tab = dst.tabs.len().saturating_sub(1);
            dst.collapsed = false;
            self.status = "Merged panel groups".to_string();
        }
    }

    pub fn move_group_to_zone(
        &mut self,
        group_id: u64,
        zone: DockZone,
        floating_pos: Option<egui::Pos2>,
    ) {
        if let Some(g) = self.panel_groups.iter_mut().find(|g| g.id == group_id) {
            g.zone = zone;
            g.stack_parent = None;
            if let Some(pos) = floating_pos {
                g.floating_pos = pos;
            }
            let title = g
                .tabs
                .get(g.active_tab)
                .copied()
                .map(|t| t.title())
                .unwrap_or("Panel");
            self.status = match zone {
                DockZone::RightOuter => format!("Pinned {title} to Primary Right Dock"),
                DockZone::RightInner => format!("Pinned {title} to Secondary Inner-Right Dock"),
                DockZone::LeftInner => format!("Pinned {title} to Left Dock"),
                DockZone::Floating => format!("Undocked {title} to Floating Window"),
            };
        }
    }

    pub fn collapse_all_panels(&mut self, collapsed: bool) {
        for g in &mut self.panel_groups {
            g.collapsed = collapsed;
        }
        self.status = if collapsed {
            "Collapsed all panel groups (click any header arrow ▸ to expand)".to_string()
        } else {
            "Expanded all panel groups".to_string()
        };
    }

    /// Magnetically snaps a floating panel window (`group_id`) within 12px of other floating windows
    /// or the canvas/dock edges (`screenshots/Fireworks-All_Window_Panels.jpg`).
    pub fn panel_group_height(group: &PanelGroup) -> f32 {
        crate::panels::geometry::group_height(group.collapsed, group.body_height)
    }

    /// Stacks `child_id` directly above or below `target_id`. Stacked windows follow their root
    /// when it moves and are rendered as one visual column.
    pub fn stack_group(&mut self, child_id: u64, target_id: u64, below: bool) -> bool {
        if child_id == target_id {
            return false;
        }
        let Some((target_parent, target_pos, target_width, target_h)) = self
            .panel_groups
            .iter()
            .find(|g| g.id == target_id)
            .map(|g| {
                (
                    g.stack_parent,
                    g.floating_pos,
                    g.floating_width,
                    Self::panel_group_height(g),
                )
            })
        else {
            return false;
        };
        if below || target_parent.is_some() {
            if let Some(child) = self.panel_groups.iter_mut().find(|g| g.id == child_id) {
                child.stack_parent = Some(target_id);
                child.zone = DockZone::Floating;
                child.floating_pos =
                    crate::panels::geometry::stacked_below(crate::panels::geometry::Slot {
                        pos: target_pos,
                        size: Vec2::new(target_width, target_h),
                    });
                child.floating_width = target_width;
            }
            self.status = "Stacked panel below target window".to_string();
        } else {
            // Place the dragged group above the target and make the target its child. Only safe
            // when the target is a stack root; otherwise fall back to stacking below.
            let child_h = self
                .panel_groups
                .iter()
                .find(|g| g.id == child_id)
                .map(Self::panel_group_height)
                .unwrap_or(160.0);
            if let Some(child) = self.panel_groups.iter_mut().find(|g| g.id == child_id) {
                child.stack_parent = None;
                child.zone = DockZone::Floating;
                child.floating_pos = target_pos;
                child.floating_width = target_width;
            }
            if let Some(target) = self.panel_groups.iter_mut().find(|g| g.id == target_id) {
                target.stack_parent = Some(child_id);
                target.floating_pos =
                    crate::panels::geometry::shifted_for_window_above(target_pos, child_h);
                target.floating_width = target_width;
            }
            self.status = "Stacked panel above target window".to_string();
        }
        true
    }

    pub fn detach_stack(&mut self, group_id: u64) {
        if let Some(g) = self.panel_groups.iter_mut().find(|g| g.id == group_id)
            && g.stack_parent.take().is_some()
        {
            g.floating_pos += Vec2::new(18.0, 18.0);
            g.zone = DockZone::Floating;
            self.status = "Detached panel from its window stack".to_string();
        }
    }

    pub fn snap_floating_pos(
        &self,
        group_id: u64,
        pos: egui::Pos2,
        size: Vec2,
        bounds: egui::Rect,
    ) -> egui::Pos2 {
        let others: Vec<crate::panels::geometry::Slot> = self
            .panel_groups
            .iter()
            .filter(|other| {
                other.id != group_id
                    && other.zone == DockZone::Floating
                    && other.stack_parent.is_none()
            })
            .map(|other| crate::panels::geometry::Slot {
                pos: other.floating_pos,
                size: Vec2::new(
                    other.floating_width,
                    if other.collapsed {
                        26.0
                    } else {
                        other.body_height + 30.0
                    },
                ),
            })
            .collect();
        crate::panels::geometry::snap_pos(pos, size, bounds, &others)
    }

    pub fn selected_object(&self) -> Option<Obj> {
        let idx = self.selected?;
        self.doc
            .page()
            .layers
            .get(self.selected_layer)
            .and_then(|l| l.objects.get(idx))
            .cloned()
    }

    pub fn is_selected(&self, idx: usize) -> bool {
        self.selected == Some(idx) || self.selected_set.contains(&idx)
    }

    pub fn select_single(&mut self, idx: Option<usize>) {
        self.selected = idx;
        self.selected_set = idx.into_iter().collect();
        self.selected_extra_layers.clear();
    }

    pub fn toggle_select(&mut self, idx: usize) {
        self.selected_extra_layers.clear();
        if let Some(pos) = self.selected_set.iter().position(|&i| i == idx) {
            self.selected_set.remove(pos);
        } else {
            self.selected_set.push(idx);
        }
        self.selected = self.selected_set.last().copied();
    }

    pub fn clear_selection(&mut self) {
        self.selected = None;
        self.selected_set.clear();
        self.selected_extra_layers.clear();
    }

    pub fn push_object(&mut self, obj: Obj) -> usize {
        let target_layer = if matches!(obj.kind, Shape::Slice { .. }) {
            self.doc
                .page()
                .layers
                .iter()
                .position(|l| l.is_web_layer)
                .unwrap_or(self.selected_layer)
        } else {
            self.selected_layer
        };
        let page = self.doc.page_mut();
        let layer_idx = target_layer.min(page.layers.len().saturating_sub(1));
        let Some(layer) = page.layers.get_mut(layer_idx) else {
            return 0;
        };
        layer.objects.push(obj);
        let obj_idx = layer.objects.len() - 1;
        page.sync_state();
        self.selected_layer = layer_idx;
        self.doc.dirty = true;
        obj_idx
    }

    pub fn load_logo(ctx: &Context) -> Option<egui::TextureHandle> {
        let bytes = include_bytes!("../assets/logo.png");
        if let Ok(img) = image::load_from_memory(bytes) {
            let w = img.width() as usize;
            let h = img.height() as usize;
            let mut pixels = Vec::with_capacity(w * h);
            for (_x, _y, c) in img.pixels() {
                pixels.push(Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]));
            }
            let image = egui::ColorImage::new([w, h], pixels);
            return Some(ctx.load_texture("firecraft-logo", image, egui::TextureOptions::LINEAR));
        }
        None
    }

    pub fn bitmap_texture(
        &mut self,
        key: (usize, usize),
        pixels: &[u8],
        size: Vec2,
    ) -> Option<TextureHandle> {
        if let Some(tex) = self.textures.get(&key) {
            return Some(tex.clone());
        }
        let w = size.x.max(1.0) as usize;
        let h = size.y.max(1.0) as usize;
        if pixels.len() < w * h * 4 {
            return None;
        }
        let mut data = Vec::with_capacity(w * h);
        for i in 0..w * h {
            data.push(Color32::from_rgba_unmultiplied(
                pixels[i * 4],
                pixels[i * 4 + 1],
                pixels[i * 4 + 2],
                pixels[i * 4 + 3],
            ));
        }
        let image = egui::ColorImage::new([w, h], data);
        let tex = self.ctx.load_texture(
            format!("bitmap-{key:?}"),
            image,
            egui::TextureOptions::LINEAR,
        );
        self.textures.insert(key, tex.clone());
        Some(tex)
    }

    // ---- Property mutators -------------------------------------------------

    fn with_selected_mut(&mut self, f: impl FnOnce(&mut Obj)) {
        if let Some(index) = self.selected {
            let layer = self.selected_layer;
            let page = self.doc.page_mut();
            if let Some(obj) = page
                .layers
                .get_mut(layer)
                .and_then(|l| l.objects.get_mut(index))
            {
                f(obj);
                page.sync_state();
                self.doc.dirty = true;
            }
        }
    }

    pub fn set_selected_name(&mut self, name: String) {
        self.with_selected_mut(|obj| obj.name = name);
    }

    /// Applies an appearance edit to every member of the selection on the active layer, plus any
    /// multi-layer selection entries. Used for fill/stroke/effects so multi-selection is not stale.
    fn with_selection_mut(&mut self, mut f: impl FnMut(&mut Obj)) {
        let mut targets: Vec<usize> = self.selected_set.clone();
        if let Some(s) = self.selected
            && !targets.contains(&s)
        {
            targets.push(s);
        }
        targets.sort_unstable();
        targets.dedup();
        let layer = self.selected_layer;
        if let Some(objs) = self
            .doc
            .page_mut()
            .layers
            .get_mut(layer)
            .map(|l| &mut l.objects)
        {
            for idx in targets {
                if let Some(obj) = objs.get_mut(idx) {
                    f(obj);
                }
            }
        }
        let extras = self.selected_extra_layers.clone();
        for (li, oi) in extras {
            if let Some(obj) = self
                .doc
                .page_mut()
                .layers
                .get_mut(li)
                .and_then(|l| l.objects.get_mut(oi))
            {
                f(obj);
            }
        }
        self.doc.page_mut().sync_state();
        self.doc.dirty = true;
    }

    pub fn set_selected_fill(&mut self, color: Option<Color32>) {
        self.with_selection_mut(|obj| {
            obj.fill = color;
            // Editing the solid Fill swatch switches the object back to a flat fill.
            obj.gradient = None;
        });
    }

    pub fn set_selected_stroke(&mut self, color: Option<Color32>) {
        self.with_selection_mut(|obj| obj.stroke = color);
    }

    pub fn set_selected_width(&mut self, width: f32) {
        self.with_selection_mut(|obj| obj.width = width);
    }

    pub fn set_selected_opacity(&mut self, opacity: f32) {
        self.with_selection_mut(|obj| obj.opacity = opacity.clamp(0.0, 1.0));
    }

    pub fn set_selected_blend_mode(&mut self, mode: crate::doc::BlendMode) {
        self.with_selection_mut(|obj| obj.blend_mode = mode);
    }

    pub fn set_selected_gradient(&mut self, gradient: Option<GradientFill>) {
        self.push_undo("Edit Gradient");
        self.with_selection_mut(|obj| {
            obj.gradient = gradient;
            if let Some(g) = gradient {
                obj.fill = Some(g.start);
            }
        });
    }

    pub fn set_selected_bounds(&mut self, lo: Vec2, hi: Vec2) {
        self.with_selected_mut(|obj| obj.set_bounds(lo, hi));
    }

    pub fn set_selected_corner_radius(&mut self, radius: f32) {
        self.with_selected_mut(|obj| {
            if let Shape::Rect { corner_radius, .. } = &mut obj.kind {
                *corner_radius = radius.max(0.0);
            }
        });
    }

    pub fn set_selected_polygon_params(&mut self, new_sides: u32, new_inner: f32) {
        self.polygon_sides = new_sides.clamp(3, 36);
        self.star_ratio = new_inner.clamp(0.1, 1.0);
        self.with_selected_mut(|obj| {
            if let Shape::Polygon {
                sides, star_inner, ..
            } = &mut obj.kind
            {
                *sides = new_sides.clamp(3, 36);
                *star_inner = new_inner.clamp(0.1, 1.0);
            }
        });
    }

    pub fn set_selected_text(&mut self, text: String) {
        self.with_selected_mut(|obj| {
            if let Shape::Text { text: t, .. } = &mut obj.kind {
                *t = text.clone();
            }
            obj.name = text;
        });
    }

    pub fn set_selected_text_size(&mut self, new_size: f32) {
        self.with_selected_mut(|obj| {
            if let Shape::Text { size, .. } = &mut obj.kind {
                *size = new_size.clamp(6.0, 240.0);
            }
        });
    }

    pub fn set_selected_font_family(&mut self, family: &str) {
        let fam = family.to_string();
        self.with_selected_mut(|obj| {
            if let Shape::Text { font_family, .. } = &mut obj.kind {
                *font_family = fam;
            }
        });
        self.status = format!("Font set to {family}");
    }

    pub fn toggle_selected_text_bold(&mut self) {
        self.with_selected_mut(|obj| {
            if let Shape::Text { bold, .. } = &mut obj.kind {
                *bold = !*bold;
            }
        });
    }

    pub fn toggle_selected_text_italic(&mut self) {
        self.with_selected_mut(|obj| {
            if let Shape::Text { italic, .. } = &mut obj.kind {
                *italic = !*italic;
            }
        });
    }

    pub fn set_selected_slice_meta(&mut self, new_url: String, new_alt: String) {
        self.with_selected_mut(|obj| {
            if let Shape::Slice { url, alt, .. } = &mut obj.kind {
                *url = new_url;
                *alt = new_alt;
            }
        });
    }

    pub fn set_selected_gradient_radial(&mut self, is_radial: bool) {
        self.with_selected_mut(|obj| {
            if let Shape::Gradient { radial, .. } = &mut obj.kind {
                *radial = is_radial;
            }
        });
    }

    // ---- Live Effects & Filters --------------------------------------------

    pub fn add_live_effect(&mut self, effect: LiveEffect) {
        if self.selected.is_none() && self.selected_set.is_empty() {
            self.status = "Select an object first to attach a Live Filter".to_string();
            return;
        }
        self.push_undo("Add Live Filter");
        let label = effect.label();
        self.last_effect = Some(effect.clone());
        self.with_selection_mut(|obj| obj.effects.push(effect.clone()));
        self.status = format!("Added Live Filter: {label}");
    }

    pub fn remove_live_effect(&mut self, eff_idx: usize) {
        self.push_undo("Remove Live Effect");
        self.with_selection_mut(|obj| {
            if eff_idx < obj.effects.len() {
                obj.effects.remove(eff_idx);
            }
        });
        self.textures.clear();
        self.status = "Removed live effect".to_string();
    }

    pub fn toggle_live_effect(&mut self, eff_idx: usize, enabled: bool) {
        self.with_selection_mut(|obj| {
            if let Some(eff) = obj.effects.get_mut(eff_idx) {
                eff.set_enabled(enabled);
            }
        });
    }

    pub fn clear_live_effects(&mut self) {
        self.push_undo("Clear Filters");
        self.with_selection_mut(|obj| obj.effects.clear());
        self.status = "Cleared live filters on selection".to_string();
    }

    pub fn repeat_last_filter(&mut self) {
        if let Some(eff) = self.last_effect.clone() {
            self.add_live_effect(eff);
        } else {
            self.run_bitmap_filter(Tool::Sharpen);
        }
    }

    // ---- Tool state --------------------------------------------------------

    /// Switch tools through one seam so leaving the Pen commits its in-progress contour
    /// instead of leaving stray preview nodes on the canvas.
    pub fn set_tool(&mut self, tool: Tool) {
        if tool != Tool::Path && !self.pen_nodes.is_empty() {
            self.finish_pen_path(false);
        }
        self.tool = tool;
    }

    /// Add one Pen anchor. Clicking the first anchor closes a path with at least three nodes.
    pub fn pen_add_node(&mut self, doc_pos: Vec2) {
        let tol = 8.0 / self.doc.zoom.max(0.1);
        if self.pen_nodes.len() >= 3 && (self.pen_nodes[0] - doc_pos).length() <= tol {
            self.finish_pen_path(true);
            return;
        }
        self.pen_nodes.push(doc_pos);
        self.status = if self.pen_nodes.len() == 1 {
            "Pen: click to place the next point · double-click or Enter to finish · click the first point to close".to_string()
        } else {
            format!(
                "Pen: {} points · double-click/Enter to finish",
                self.pen_nodes.len()
            )
        };
    }

    /// Commit the in-progress Pen contour. A closed contour needs three points; an open one
    /// needs two. Short or empty paths are discarded rather than creating degenerate objects.
    pub fn finish_pen_path(&mut self, closed: bool) {
        let closed = closed && self.pen_nodes.len() >= 3;
        if self.pen_nodes.len() < 2 {
            self.pen_nodes.clear();
            return;
        }
        self.push_undo("Draw Path");
        let points = std::mem::take(&mut self.pen_nodes);
        let obj = Obj::new(
            Shape::Path { points, closed },
            if closed { Some(self.fill) } else { None },
            Some(self.stroke),
        );
        let idx = self.push_object(obj);
        self.select_single(Some(idx));
        self.status = if closed {
            "Closed vector path created".to_string()
        } else {
            "Open vector path created — use Subselection (A) to edit nodes".to_string()
        };
    }

    /// Font families present in the active document, plus the CS6 built-in choices.
    /// Menus use this instead of a fixed list so imported and edited documents surface
    /// the fonts they actually contain.
    pub fn text_font_families(&self) -> Vec<String> {
        let mut families = Vec::new();
        for layer in &self.doc.page().layers {
            for obj in &layer.objects {
                if let Shape::Text { font_family, .. } = &obj.kind
                    && !families.iter().any(|f| f == font_family)
                {
                    families.push(font_family.clone());
                }
            }
        }
        for built_in in ["Sans-Serif", "Serif", "Monospace", "Display", "Geometric"] {
            if !families.iter().any(|f| f == built_in) {
                families.push(built_in.to_string());
            }
        }
        families
    }

    /// Text sizes used by the active document, ordered and de-duplicated, followed by
    /// CS6's common preset ladder. The menu marks the selected text's size.
    pub fn text_sizes(&self) -> Vec<f32> {
        let mut sizes = Vec::new();
        for layer in &self.doc.page().layers {
            for obj in &layer.objects {
                if let Shape::Text { size, .. } = &obj.kind
                    && !sizes.iter().any(|s: &f32| (*s - *size).abs() < 0.01)
                {
                    sizes.push(*size);
                }
            }
        }
        for preset in [12.0, 18.0, 24.0, 36.0, 48.0, 72.0] {
            if !sizes.iter().any(|s| (*s - preset).abs() < 0.01) {
                sizes.push(preset);
            }
        }
        sizes.sort_by(|a, b| a.total_cmp(b));
        sizes
    }

    /// The currently selected text style, when the selection is a text object.
    pub fn selected_text_style(&self) -> Option<(String, f32)> {
        match &self.selected_object()?.kind {
            Shape::Text {
                font_family, size, ..
            } => Some((font_family.clone(), *size)),
            _ => None,
        }
    }

    // ---- Pointer handling --------------------------------------------------

    pub fn snap_pos(&self, mut p: Vec2) -> Vec2 {
        let tol = 6.0 / self.doc.zoom.max(0.25);
        if self.snap_to_guides {
            let page = self.doc.page();
            let mut best_dx = tol;
            let mut best_dy = tol;
            let mut snap_x = p.x;
            let mut snap_y = p.y;
            for &vx in page
                .guides_v
                .iter()
                .chain([0.0, page.width * 0.5, page.width].iter())
            {
                let d = (p.x - vx).abs();
                if d < best_dx {
                    best_dx = d;
                    snap_x = vx;
                }
            }
            for &hy in page
                .guides_h
                .iter()
                .chain([0.0, page.height * 0.5, page.height].iter())
            {
                let d = (p.y - hy).abs();
                if d < best_dy {
                    best_dy = d;
                    snap_y = hy;
                }
            }
            p = Vec2::new(snap_x, snap_y);
        }
        if self.snap_to_grid && self.grid_size >= 2.0 {
            let g = self.grid_size;
            Vec2::new((p.x / g).round() * g, (p.y / g).round() * g)
        } else {
            p
        }
    }

    pub fn on_pointer_down(
        &mut self,
        raw_doc_pos: Vec2,
        _screen: Vec2,
        alt: bool,
        ctrl: bool,
        shift: bool,
    ) {
        let doc_pos = self.snap_pos(raw_doc_pos);
        match self.tool {
            Tool::Hand => {
                self.drag = Some(Drag {
                    kind: DragKind::Pan,
                    start: raw_doc_pos,
                    current: raw_doc_pos,
                    orig_bounds: None,
                    path: vec![],
                });
            }
            Tool::Zoom => {
                let factor = if alt {
                    1.0 / constants::ZOOM_STEP
                } else {
                    constants::ZOOM_STEP
                };
                self.set_zoom(self.doc.zoom * factor);
            }
            Tool::Fit => {
                self.fit_in_window();
            }
            Tool::Select | Tool::Subselect | Tool::Scale => {
                if self.tool == Tool::Subselect
                    && let Some(obj) = self.selected_object()
                    && let Shape::Path { points, .. } = &obj.kind
                {
                    let tol = 10.0 / self.doc.zoom.max(0.1);
                    if let Some((node_idx, _)) = points
                        .iter()
                        .enumerate()
                        .find(|(_, pt)| (**pt - raw_doc_pos).length() <= tol)
                    {
                        self.push_undo("Move Path Node");
                        self.drag = Some(Drag {
                            kind: DragKind::MoveNode(node_idx),
                            start: doc_pos,
                            current: doc_pos,
                            orig_bounds: None,
                            path: vec![],
                        });
                        return;
                    }
                }
                if let Some(auto_h) = canvas::hit_test_auto_handle(self, raw_doc_pos) {
                    let kind = if auto_h == 0 {
                        self.push_undo("Adjust Corner Radius");
                        DragKind::AdjustCornerRadius
                    } else {
                        self.push_undo("Adjust Star Pointiness");
                        DragKind::AdjustStarInner
                    };
                    self.drag = Some(Drag {
                        kind,
                        start: doc_pos,
                        current: doc_pos,
                        orig_bounds: None,
                        path: vec![],
                    });
                    return;
                }
                if let Some(handle_idx) = canvas::hit_test_handle(self, raw_doc_pos) {
                    let orig = self.selected_object().and_then(|o| o.bounds());
                    self.push_undo("Resize");
                    self.drag = Some(Drag {
                        kind: DragKind::Resize(handle_idx),
                        start: doc_pos,
                        current: doc_pos,
                        orig_bounds: orig,
                        path: vec![],
                    });
                    return;
                }
                let hit = canvas::hit_test(self, raw_doc_pos);
                match hit {
                    Some((layer, index)) => {
                        if shift || ctrl {
                            if self.selected_layer != layer {
                                self.selected_layer = layer;
                                self.select_single(Some(index));
                            } else {
                                self.toggle_select(index);
                            }
                            self.status = format!("{} object(s) selected", self.selected_set.len());
                        } else if !self.is_selected(index) || self.selected_layer != layer {
                            self.selected_layer = layer;
                            self.select_single(Some(index));
                        }
                        if alt {
                            self.duplicate_selected();
                        } else {
                            self.push_undo("Move");
                        }
                        self.drag = Some(Drag {
                            kind: DragKind::Move,
                            start: doc_pos,
                            current: doc_pos,
                            orig_bounds: None,
                            path: vec![],
                        });
                    }
                    None => {
                        if !shift && !ctrl {
                            self.clear_selection();
                        }
                        self.drag = Some(Drag {
                            kind: DragKind::Marquee,
                            start: doc_pos,
                            current: doc_pos,
                            orig_bounds: None,
                            path: vec![],
                        });
                    }
                }
            }
            Tool::Marquee | Tool::OvalMarquee | Tool::Crop => {
                self.drag = Some(Drag {
                    kind: DragKind::Marquee,
                    start: doc_pos,
                    current: doc_pos,
                    orig_bounds: None,
                    path: vec![],
                });
            }
            Tool::Lasso => {
                self.drag = Some(Drag {
                    kind: DragKind::Lasso,
                    start: doc_pos,
                    current: doc_pos,
                    orig_bounds: None,
                    path: vec![doc_pos],
                });
            }
            Tool::Wand => {
                if let Some((layer, index)) = canvas::hit_test(self, raw_doc_pos) {
                    let target_fill = self.doc.page().layers[layer].objects[index].fill;
                    let mut picked = if shift {
                        self.selected_set.clone()
                    } else {
                        Vec::new()
                    };
                    self.selected_layer = layer;
                    for (idx, obj) in self.doc.page().layers[layer].objects.iter().enumerate() {
                        if !obj.visible || obj.locked {
                            continue;
                        }
                        let matches_color = match (target_fill, obj.fill) {
                            (Some(a), Some(b)) => {
                                (a.r() as i32 - b.r() as i32).abs()
                                    + (a.g() as i32 - b.g() as i32).abs()
                                    + (a.b() as i32 - b.b() as i32).abs()
                                    <= 48
                            }
                            (None, None) => true,
                            _ => false,
                        };
                        if matches_color && !picked.contains(&idx) {
                            picked.push(idx);
                        }
                    }
                    self.selected = Some(index);
                    self.selected_set = picked;
                    self.status = format!(
                        "Magic Wand selected {} color-matched object(s)",
                        self.selected_set.len()
                    );
                } else {
                    self.clear_selection();
                    self.status =
                        "Magic Wand: click an object to select matching colors".to_string();
                }
            }
            Tool::Rectangle
            | Tool::RoundedRect
            | Tool::Ellipse
            | Tool::Line
            | Tool::Polygon
            | Tool::Star
            | Tool::Slice
            | Tool::Hotspot => {
                self.drag = Some(Drag {
                    kind: DragKind::Create,
                    start: doc_pos,
                    current: doc_pos,
                    orig_bounds: None,
                    path: vec![],
                });
            }
            Tool::Path => {
                // Pen anchors are click-placed, not a freehand drag. The canvas owns the
                // click path and calls `pen_add_node`; this branch handles pointer-down from
                // non-canvas callers (tests and accessibility paths) as a single anchor.
                if self.pen_nodes.is_empty() {
                    self.pen_add_node(doc_pos);
                }
            }
            Tool::Knife => {
                if self.selected.is_none()
                    && let Some((layer, index)) = canvas::hit_test(self, raw_doc_pos)
                {
                    self.selected_layer = layer;
                    self.select_single(Some(index));
                }
                self.split_selected_path();
            }
            Tool::Text => {
                self.push_undo("Add Text");
                let obj = Obj::new(
                    Shape::Text {
                        at: doc_pos,
                        text: String::new(),
                        size: 24.0,
                        bold: false,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(self.fill),
                    None,
                );
                let index = self.push_object(obj);
                self.select_single(Some(index));
                self.inline_text_edit_obj = Some(index);
                self.textures.clear();
                self.status =
                    "Type text directly on canvas (Esc or click outside to finish)".to_string();
            }
            Tool::ReplaceColor => {
                if let Some((layer, index)) = canvas::hit_test(self, raw_doc_pos) {
                    self.selected_layer = layer;
                    self.select_single(Some(index));
                    if let Some(color) = self.doc.page().layers[layer].objects[index].fill {
                        self.find_color = color;
                        self.find_replace_mode = FindReplaceMode::FillColor;
                        self.show_find_replace = true;
                        self.status = format!(
                            "Replace Color: {} primed from {} — choose a replacement color",
                            colors::hex_of(color),
                            self.doc.page().layers[layer].objects[index].name
                        );
                    } else {
                        self.status =
                            "This object has no flat fill to replace — use Find & Replace > Any object color for gradients/effects".to_string();
                    }
                }
            }
            Tool::RedEye => {
                self.run_red_eye_removal();
            }
            Tool::Picker => {
                if let Some((layer, index)) = canvas::hit_test(self, raw_doc_pos) {
                    let color = self.doc.page().layers[layer].objects[index].fill;
                    if let Some(c) = color {
                        self.fill = c;
                        self.status = format!("Picked {}", colors::hex_of(c));
                    }
                } else {
                    self.fill = self.doc.page().bg_color;
                    self.status = format!("Picked canvas {}", colors::hex_of(self.fill));
                }
            }
            Tool::Fill => {
                if let Some((layer, index)) = canvas::hit_test(self, raw_doc_pos) {
                    self.selected_layer = layer;
                    self.selected = Some(index);
                    self.push_undo("Fill Object");
                    self.set_selected_fill(Some(self.fill));
                    self.status = "Filled selected object".to_string();
                } else {
                    self.push_undo("Fill Canvas");
                    let (pw, ph) = (self.doc.page().width, self.doc.page().height);
                    let obj = Obj::new(
                        Shape::Rect {
                            min: Vec2::ZERO,
                            max: Vec2::new(pw, ph),
                            corner_radius: 0.0,
                            filled: true,
                        },
                        Some(self.fill),
                        None,
                    );
                    let index = self.push_object(obj);
                    self.selected = Some(index);
                    self.status = "Filled page background".to_string();
                }
            }
            Tool::Gradient => {
                self.drag = Some(Drag {
                    kind: DragKind::Create,
                    start: doc_pos,
                    current: doc_pos,
                    orig_bounds: None,
                    path: vec![],
                });
            }
            Tool::Brush | Tool::Pencil | Tool::Eraser | Tool::Freeform => {
                self.push_undo("Paint");
                self.drag = Some(Drag {
                    kind: DragKind::Paint,
                    start: doc_pos,
                    current: doc_pos,
                    orig_bounds: None,
                    path: vec![doc_pos],
                });
                if self.tool == Tool::Eraser {
                    self.erase_at(doc_pos);
                }
            }
            Tool::Stamp => {
                if self.selected.is_some() {
                    self.duplicate_selected();
                    if let Some((lo, hi)) = self.selected_object().and_then(|o| o.bounds()) {
                        let size = hi - lo;
                        self.set_selected_bounds(doc_pos - size * 0.5, doc_pos + size * 0.5);
                    }
                    self.status = "Stamped clone at cursor".to_string();
                } else {
                    self.status = "Select an object first to stamp clones".to_string();
                }
            }
            Tool::Blur | Tool::Sharpen | Tool::Dodge | Tool::Burn => {
                self.run_bitmap_filter(self.tool);
            }
        }
    }

    fn erase_at(&mut self, doc_pos: Vec2) {
        let radius = (self.brush_size / 2.0).max(2.0);
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        let Some(layer_ref) = page.layers.get_mut(layer) else {
            return;
        };
        layer_ref.objects.retain(|obj| match obj.bounds() {
            Some((lo, hi)) => {
                !(doc_pos.x >= lo.x - radius
                    && doc_pos.x <= hi.x + radius
                    && doc_pos.y >= lo.y - radius
                    && doc_pos.y <= hi.y + radius)
            }
            None => true,
        });
        page.sync_state();
        self.textures.clear();
    }

    pub fn run_bitmap_filter(&mut self, filter_tool: Tool) {
        let Some(index) = self.selected else {
            self.status = "Select an object first".to_string();
            return;
        };
        let layer = self.selected_layer;
        let Some(obj) = self
            .doc
            .page_mut()
            .layers
            .get_mut(layer)
            .and_then(|l| l.objects.get_mut(index))
        else {
            return;
        };
        let Shape::Bitmap { pixels, size, .. } = &mut obj.kind else {
            // If it's a vector object, apply a non-destructive Live Filter or color adjustment instead
            match filter_tool {
                Tool::Blur => self.add_live_effect(LiveEffect::GaussianBlur {
                    radius: 4.0,
                    enabled: true,
                }),
                Tool::Dodge => self.apply_color_command("brighten"),
                Tool::Burn => self.apply_color_command("darken"),
                _ => {
                    self.status = "Converted to Live Effect on vector object".to_string();
                }
            }
            return;
        };
        let count = pixels.len() / 4;
        let w = size.x.max(1.0) as usize;
        let rows = size.y.max(1.0) as usize;
        let original = pixels.clone();
        for i in 0..count {
            match filter_tool {
                Tool::Blur => {
                    let x = (i % w) as i64;
                    let y = (i / w) as i64;
                    let mut r = 0u32;
                    let mut g = 0u32;
                    let mut b = 0u32;
                    let mut n = 0u32;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = x + dx;
                            let ny = y + dy;
                            if nx < 0 || ny < 0 || nx as usize >= w || ny as usize >= rows {
                                continue;
                            }
                            let ni = (ny as usize) * w + nx as usize;
                            if ni >= count {
                                continue;
                            }
                            r += original[ni * 4] as u32;
                            g += original[ni * 4 + 1] as u32;
                            b += original[ni * 4 + 2] as u32;
                            n += 1;
                        }
                    }
                    if let (Some(rn), Some(gn), Some(bn)) =
                        (r.checked_div(n), g.checked_div(n), b.checked_div(n))
                    {
                        pixels[i * 4] = rn as u8;
                        pixels[i * 4 + 1] = gn as u8;
                        pixels[i * 4 + 2] = bn as u8;
                    }
                }
                Tool::Sharpen => {
                    let x = (i % w) as i64;
                    let y = (i / w) as i64;
                    if rows == 0 {
                        continue;
                    }
                    let cx = x.clamp(0, w as i64 - 1).max(0);
                    let cy = y.clamp(0, rows as i64 - 1).max(0);
                    let at = |px: i64, py: i64, c: usize| -> i32 {
                        let px = px.clamp(0, w as i64 - 1) as usize;
                        let py = py.clamp(0, rows as i64 - 1) as usize;
                        let idx = (py * w + px) * 4 + c;
                        original.get(idx).copied().unwrap_or(0) as i32
                    };
                    for c in 0..3 {
                        let centre = at(cx, cy, c);
                        let value = centre * 5
                            - at(cx - 1, cy, c)
                            - at(cx + 1, cy, c)
                            - at(cx, cy - 1, c)
                            - at(cx, cy + 1, c);
                        pixels[i * 4 + c] = value.clamp(0, 255) as u8;
                    }
                }
                Tool::Dodge => {
                    pixels[i * 4] = (original[i * 4] as f32 * 1.15).min(255.0) as u8;
                    pixels[i * 4 + 1] = (original[i * 4 + 1] as f32 * 1.15).min(255.0) as u8;
                    pixels[i * 4 + 2] = (original[i * 4 + 2] as f32 * 1.15).min(255.0) as u8;
                }
                Tool::Burn => {
                    pixels[i * 4] = (original[i * 4] as f32 * 0.85) as u8;
                    pixels[i * 4 + 1] = (original[i * 4 + 1] as f32 * 0.85) as u8;
                    pixels[i * 4 + 2] = (original[i * 4 + 2] as f32 * 0.85) as u8;
                }
                _ => {}
            }
        }
        self.doc.page_mut().sync_state();
        self.textures.clear();
        self.status = format!("{} applied", filter_tool.name());
    }

    /// Real red-eye reduction for the selected bitmap: neutralizes strongly red-dominant pixels
    /// (the same signature a camera flash leaves on a subject's pupils).
    pub fn run_red_eye_removal(&mut self) {
        let Some(index) = self.selected else {
            self.status = "Red Eye Removal: select a bitmap object first".to_string();
            return;
        };
        let layer = self.selected_layer;
        let is_bitmap = self
            .doc
            .page()
            .layers
            .get(layer)
            .and_then(|l| l.objects.get(index))
            .is_some_and(|o| matches!(o.kind, Shape::Bitmap { .. }));
        if !is_bitmap {
            self.status =
                "Red Eye Removal works on bitmap objects (flatten or import a photo first)"
                    .to_string();
            return;
        }
        self.push_undo("Red Eye Removal");
        let mut corrected = 0usize;
        if let Some(obj) = self
            .doc
            .page_mut()
            .layers
            .get_mut(layer)
            .and_then(|l| l.objects.get_mut(index))
            && let Shape::Bitmap { pixels, .. } = &mut obj.kind
        {
            for px in pixels.as_chunks_mut::<4>().0 {
                let r = px[0] as u32;
                let g = px[1] as u32;
                let b = px[2] as u32;
                if r > 65 && r > g * 3 / 2 && r > b * 3 / 2 {
                    px[0] = ((g + b) / 2).min(255) as u8;
                    corrected += 1;
                }
            }
        }
        self.textures.clear();
        self.doc.dirty = true;
        self.status = format!("Red Eye Removal corrected {corrected} red-dominant pixel(s)");
    }

    pub fn on_pointer_drag(&mut self, raw_doc_pos: Vec2, shift: bool) {
        let doc_pos = self.snap_pos(raw_doc_pos);
        let Some(drag) = self.drag.as_mut() else {
            return;
        };
        match drag.kind {
            DragKind::Pan => {
                // Screen-space panning is applied directly in canvas::show via pointer_delta.
            }
            DragKind::AdjustCornerRadius => {
                drag.current = doc_pos;
                let mut new_r = 0.0_f32;
                self.with_selected_mut(|obj| {
                    if let Shape::Rect {
                        min,
                        max,
                        corner_radius,
                        ..
                    } = &mut obj.kind
                    {
                        let lo = min.min(*max);
                        let hi = min.max(*max);
                        let max_r = ((hi.x - lo.x).min(hi.y - lo.y) * 0.5).max(0.0);
                        new_r = (doc_pos.x - lo.x).clamp(0.0, max_r);
                        *corner_radius = new_r;
                    }
                });
                self.status = format!("Corner Radius: {new_r:.0} px");
            }
            DragKind::AdjustStarInner => {
                drag.current = doc_pos;
                let mut new_ratio = 0.45_f32;
                self.with_selected_mut(|obj| {
                    if let Shape::Polygon {
                        center,
                        radius,
                        star_inner,
                        ..
                    } = &mut obj.kind
                    {
                        let dist = (doc_pos - *center).length();
                        new_ratio = (dist / radius.max(1.0)).clamp(0.10, 1.0);
                        *star_inner = new_ratio;
                    }
                });
                self.star_ratio = new_ratio;
                self.status = format!("Star Inner Ratio: {:.0}%", new_ratio * 100.0);
            }
            DragKind::MoveNode(node_idx) => {
                drag.current = doc_pos;
                self.with_selected_mut(|obj| {
                    if let Shape::Path { points, .. } = &mut obj.kind
                        && node_idx < points.len()
                    {
                        points[node_idx] = doc_pos;
                    }
                });
            }
            DragKind::Move => {
                let delta = doc_pos - drag.current;
                drag.current = doc_pos;
                let mut indices = self.selected_set.clone();
                if let Some(s) = self.selected
                    && !indices.contains(&s)
                {
                    indices.push(s);
                }
                let layer = self.selected_layer;
                let page = self.doc.page_mut();
                for &idx in &indices {
                    if let Some(obj) = page
                        .layers
                        .get_mut(layer)
                        .and_then(|l| l.objects.get_mut(idx))
                    {
                        obj.translate(delta);
                    }
                }
                page.sync_state();
                self.smart_guides_lines.clear();
                if (self.snap_to_guides || self.snap_to_objects)
                    && let Some(sel_idx) = self.selected
                    && let Some((lo, hi)) = self
                        .doc
                        .page()
                        .layers
                        .get(layer)
                        .and_then(|l| l.objects.get(sel_idx))
                        .and_then(|o| o.bounds())
                {
                    let tol = (6.0 / self.doc.zoom.max(0.25)).clamp(3.0, 14.0);
                    let pw = self.doc.page().width;
                    let ph = self.doc.page().height;
                    let mut x_targets: Vec<f32> = Vec::new();
                    let mut y_targets: Vec<f32> = Vec::new();
                    if self.snap_to_guides {
                        x_targets.extend([0.0, pw * 0.5, pw]);
                        y_targets.extend([0.0, ph * 0.5, ph]);
                        x_targets.extend(self.doc.page().guides_v.iter().copied());
                        y_targets.extend(self.doc.page().guides_h.iter().copied());
                    }
                    if self.snap_to_objects
                        && let Some(l) = self.doc.page().layers.get(layer)
                    {
                        for (oi, other) in l.objects.iter().enumerate() {
                            if oi == sel_idx || self.selected_set.contains(&oi) || !other.visible {
                                continue;
                            }
                            if let Some((oa, ob)) = other.bounds() {
                                x_targets.extend([oa.x, (oa.x + ob.x) * 0.5, ob.x]);
                                y_targets.extend([oa.y, (oa.y + ob.y) * 0.5, ob.y]);
                            }
                        }
                    }
                    let sel_xs = [lo.x, (lo.x + hi.x) * 0.5, hi.x];
                    let sel_ys = [lo.y, (lo.y + hi.y) * 0.5, hi.y];
                    let mut best_dx: Option<(f32, f32)> = None;
                    for &sx in &sel_xs {
                        for &tx in &x_targets {
                            let diff = tx - sx;
                            if diff.abs() <= tol
                                && best_dx.is_none_or(|(bd, _)| diff.abs() < bd.abs())
                            {
                                best_dx = Some((diff, tx));
                            }
                        }
                    }
                    let mut best_dy: Option<(f32, f32)> = None;
                    for &sy in &sel_ys {
                        for &ty in &y_targets {
                            let diff = ty - sy;
                            if diff.abs() <= tol
                                && best_dy.is_none_or(|(bd, _)| diff.abs() < bd.abs())
                            {
                                best_dy = Some((diff, ty));
                            }
                        }
                    }
                    let snap_delta = Vec2::new(
                        best_dx.map_or(0.0, |(d, _)| d),
                        best_dy.map_or(0.0, |(d, _)| d),
                    );
                    if snap_delta != Vec2::ZERO {
                        drag.current += snap_delta;
                        let page = self.doc.page_mut();
                        for &idx in &indices {
                            if let Some(obj) = page
                                .layers
                                .get_mut(layer)
                                .and_then(|l| l.objects.get_mut(idx))
                            {
                                obj.translate(snap_delta);
                            }
                        }
                        page.sync_state();
                    }
                    if self.show_guides {
                        if let Some((_, tx)) = best_dx {
                            self.smart_guides_lines
                                .push((Vec2::new(tx, 0.0), Vec2::new(tx, ph)));
                        }
                        if let Some((_, ty)) = best_dy {
                            self.smart_guides_lines
                                .push((Vec2::new(0.0, ty), Vec2::new(pw, ty)));
                        }
                    }
                }
            }
            DragKind::Resize(handle_idx) => {
                drag.current = doc_pos;
                if let Some((mut lo, mut hi)) = drag.orig_bounds {
                    match handle_idx {
                        0 => lo = doc_pos,
                        1 => lo.y = doc_pos.y,
                        2 => {
                            hi.x = doc_pos.x;
                            lo.y = doc_pos.y;
                        }
                        3 => hi.x = doc_pos.x,
                        4 => hi = doc_pos,
                        5 => hi.y = doc_pos.y,
                        6 => {
                            lo.x = doc_pos.x;
                            hi.y = doc_pos.y;
                        }
                        7 => lo.x = doc_pos.x,
                        _ => {}
                    }
                    if shift {
                        let s = (hi.x - lo.x).abs().max((hi.y - lo.y).abs());
                        hi = lo + Vec2::splat(s);
                    }
                    self.set_selected_bounds(lo, hi);
                }
            }
            DragKind::Paint | DragKind::Lasso => {
                drag.current = doc_pos;
                if (doc_pos - drag.path.last().copied().unwrap_or(doc_pos)).length() > 2.0 {
                    drag.path.push(doc_pos);
                    if self.tool == Tool::Eraser {
                        self.erase_at(doc_pos);
                    }
                }
            }
            _ => {
                let mut cur = doc_pos;
                if shift {
                    let d = cur - drag.start;
                    let side = d.x.abs().max(d.y.abs());
                    cur = drag.start + Vec2::new(side * d.x.signum(), side * d.y.signum());
                }
                drag.current = cur;
            }
        }
    }

    pub fn on_pointer_up(&mut self, raw_doc_pos: Vec2, shift: bool) {
        self.smart_guides_lines.clear();
        let mut doc_pos = self.snap_pos(raw_doc_pos);
        let Some(drag) = self.drag.take() else {
            return;
        };
        if shift && matches!(drag.kind, DragKind::Create) {
            let d = doc_pos - drag.start;
            let side = d.x.abs().max(d.y.abs());
            doc_pos = drag.start + Vec2::new(side * d.x.signum(), side * d.y.signum());
        }
        match drag.kind {
            DragKind::Lasso => {
                let mut picked_set = if shift {
                    self.selected_set.clone()
                } else {
                    Vec::new()
                };
                if drag.path.len() >= 3 {
                    let page = self.doc.page();
                    if let Some(layer) = page.layers.get(self.selected_layer) {
                        for (index, obj) in layer.objects.iter().enumerate() {
                            if !obj.visible || obj.locked {
                                continue;
                            }
                            if let Some((a, b)) = obj.bounds() {
                                let center = (a + b) * 0.5;
                                let corners =
                                    [center, a, Vec2::new(b.x, a.y), b, Vec2::new(a.x, b.y)];
                                if corners.iter().any(|&pt| point_in_polygon(pt, &drag.path))
                                    && !picked_set.contains(&index)
                                {
                                    picked_set.push(index);
                                }
                            }
                        }
                    }
                } else if let Some((layer, index)) = canvas::hit_test(self, raw_doc_pos) {
                    self.selected_layer = layer;
                    if !picked_set.contains(&index) {
                        picked_set.push(index);
                    }
                }
                self.selected = picked_set.last().copied();
                self.selected_set = picked_set;
                self.status = if !self.selected_set.is_empty() {
                    format!("Lasso selected {} object(s)", self.selected_set.len())
                } else {
                    "Lasso: no objects inside loop".to_string()
                };
            }
            DragKind::Marquee => {
                let lo = drag.start.min(doc_pos);
                let hi = drag.start.max(doc_pos);
                if self.tool == Tool::Crop && (hi.x - lo.x) >= 16.0 && (hi.y - lo.y) >= 16.0 {
                    self.crop_canvas_to_rect(lo, hi);
                    return;
                }
                let mut picked_set = if shift {
                    self.selected_set.clone()
                } else {
                    Vec::new()
                };
                let is_box = (hi.x - lo.x) > 4.0 || (hi.y - lo.y) > 4.0;
                let oval = self.tool == Tool::OvalMarquee;
                if oval && !is_box {
                    if let Some((layer, index)) = canvas::hit_test(self, raw_doc_pos) {
                        self.selected_layer = layer;
                        picked_set.push(index);
                    }
                } else {
                    let center = (lo + hi) * 0.5;
                    let radius = ((hi - lo) * 0.5).max(Vec2::splat(1.0));
                    let inside_oval = |p: Vec2| {
                        let d = (p - center) / radius;
                        d.x * d.x + d.y * d.y <= 1.0
                    };
                    let page = self.doc.page();
                    if let Some(layer) = page.layers.get(self.selected_layer) {
                        for (index, obj) in layer.objects.iter().enumerate() {
                            if !obj.visible || obj.locked {
                                continue;
                            }
                            if let Some((a, b)) = obj.bounds() {
                                let hit = if oval {
                                    [
                                        (a + b) * 0.5,
                                        a,
                                        Vec2::new(b.x, a.y),
                                        b,
                                        Vec2::new(a.x, b.y),
                                    ]
                                    .into_iter()
                                    .any(inside_oval)
                                } else {
                                    b.x >= lo.x && a.x <= hi.x && b.y >= lo.y && a.y <= hi.y
                                };
                                if hit {
                                    if !picked_set.contains(&index) {
                                        picked_set.push(index);
                                    }
                                    if !is_box {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                self.selected = picked_set.last().copied();
                self.selected_set = picked_set;
                self.status = if !self.selected_set.is_empty() {
                    format!(
                        "{} selected {} object(s)",
                        self.tool.name(),
                        self.selected_set.len()
                    )
                } else {
                    "Nothing selected".to_string()
                };
            }
            DragKind::Create => {
                let lo = drag.start.min(doc_pos);
                let hi = drag.start.max(doc_pos);
                if hi.x - lo.x < 2.0 && hi.y - lo.y < 2.0 {
                    return;
                }
                self.push_undo("Draw");
                let kind = match self.tool {
                    Tool::Rectangle => Shape::Rect {
                        min: lo,
                        max: hi,
                        corner_radius: 0.0,
                        filled: true,
                    },
                    Tool::RoundedRect => Shape::Rect {
                        min: lo,
                        max: hi,
                        corner_radius: 12.0,
                        filled: true,
                    },
                    Tool::Ellipse => Shape::Ellipse {
                        center: (lo + hi) / 2.0,
                        radius: (hi - lo) / 2.0,
                        filled: true,
                    },
                    Tool::Line => Shape::Line {
                        a: drag.start,
                        b: doc_pos,
                    },
                    Tool::Polygon => Shape::Polygon {
                        center: (lo + hi) * 0.5,
                        radius: ((hi - lo).length() * 0.5).max(4.0),
                        sides: self.polygon_sides,
                        star_inner: 1.0,
                        angle_rad: 0.0,
                    },
                    Tool::Star => Shape::Polygon {
                        center: (lo + hi) * 0.5,
                        radius: ((hi - lo).length() * 0.5).max(4.0),
                        sides: self.polygon_sides,
                        star_inner: self.star_ratio,
                        angle_rad: 0.0,
                    },
                    Tool::Slice | Tool::Hotspot => Shape::Slice {
                        min: lo,
                        max: hi,
                        url: "https://github.com/aliasfoxkde/FireCraft".to_string(),
                        alt: "FireCraft Web Slice".to_string(),
                        target: "_self".to_string(),
                        format: "PNG32".to_string(),
                        is_hotspot: self.tool == Tool::Hotspot,
                    },
                    Tool::Gradient => Shape::Gradient {
                        min: lo,
                        max: hi,
                        from: self.fill,
                        to: self.stroke,
                        radial: false,
                    },
                    _ => return,
                };
                let (fill, stroke) = match self.tool {
                    Tool::Line => (None, Some(self.stroke)),
                    Tool::Slice | Tool::Hotspot | Tool::Gradient => (None, None),
                    _ => (Some(self.fill), Some(self.stroke)),
                };
                let obj = Obj::new(kind, fill, stroke);
                let index = self.push_object(obj);
                self.selected = Some(index);
                self.status = format!("{} created", self.tool.name());
            }
            DragKind::Paint if drag.path.len() >= 2 && self.tool != Tool::Eraser => {
                let (stroke, width) = if self.tool == Tool::Freeform {
                    (self.stroke, 1.0)
                } else {
                    (self.fill, self.brush_size.max(1.0))
                };
                let mut obj = Obj::new(
                    Shape::Path {
                        points: drag.path,
                        closed: false,
                    },
                    None,
                    Some(stroke),
                );
                obj.width = width;
                let index = self.push_object(obj);
                self.selected = Some(index);
                self.status = if self.tool == Tool::Freeform {
                    "Freeform vector path created".to_string()
                } else {
                    "Brush stroke added".to_string()
                };
            }
            _ => {}
        }
    }

    // ---- Multi-Document Tabs & Document Lifecycle --------------------------

    pub fn sync_active_doc_slot(&mut self) {
        if self.active_doc < self.open_docs.len() {
            self.open_docs[self.active_doc] = self.doc.clone();
        }
    }

    pub fn swap_document_tabs(&mut self, a: usize, b: usize) {
        if a == b || a >= self.open_docs.len() || b >= self.open_docs.len() {
            return;
        }
        self.sync_active_doc_slot();
        self.open_docs.swap(a, b);
        if self.active_doc == a {
            self.active_doc = b;
        } else if self.active_doc == b {
            self.active_doc = a;
        }
        for fi in &mut self.floating_doc_tabs {
            if *fi == a {
                *fi = b;
            } else if *fi == b {
                *fi = a;
            }
        }
        for recent in &mut self.doc_tab_history {
            if *recent == a {
                *recent = b;
            } else if *recent == b {
                *recent = a;
            }
        }
    }

    /// Move a document to the front of the tab MRU list. Missing or stale indices are ignored;
    /// `active_doc` is always reinserted so an overflow strip can never hide every tab.
    pub fn touch_document_tab(&mut self, idx: usize) {
        if idx >= self.open_docs.len() {
            return;
        }
        self.doc_tab_history
            .retain(|&i| i != idx && i < self.open_docs.len());
        self.doc_tab_history.insert(0, idx);
    }

    pub fn new_document_tab(&mut self, width: f32, height: f32) {
        self.sync_active_doc_slot();
        let mut d = Document::new(width, height);
        d.name = format!("Untitled-{}.fw.png", self.open_docs.len() + 1);
        self.open_docs.push(d.clone());
        self.active_doc = self.open_docs.len() - 1;
        self.touch_document_tab(self.active_doc);
        self.doc = d;
        self.selected = None;
        self.selected_layer = 0;
        self.undo.clear();
        self.redo.clear();
        self.textures.clear();
        self.status = format!("New document tab: {}", self.doc.name);
    }

    pub fn switch_document_tab(&mut self, idx: usize) {
        if idx < self.open_docs.len() {
            self.sync_active_doc_slot();
            self.active_doc = idx;
            self.touch_document_tab(idx);
            self.doc = self.open_docs[idx].clone();
            self.current_path = self.doc.file_path.clone();
            self.selected = None;
            self.selected_layer = 0;
            self.undo.clear();
            self.redo.clear();
            self.textures.clear();
            // Return keyboard ownership to the canvas so the next shortcut or view toggle
            // is not swallowed by the tab strip that was just clicked.
            self.focus_canvas = true;
            self.status = format!("Switched to {}", self.doc.name);
        }
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.doc.dirty || self.open_docs.iter().any(|d| d.dirty)
    }

    /// Saves every dirty document that already has a file path. Untitled documents cannot be
    /// saved without asking for a path, so they are reported rather than silently discarded.
    pub fn new_document(&mut self, width: f32, height: f32) {
        self.new_document_with_options(
            width,
            height,
            constants::EXPORT_DEFAULT_DPI,
            Color32::WHITE,
            false,
        );
    }

    pub fn new_document_with_options(
        &mut self,
        width: f32,
        height: f32,
        dpi: u32,
        bg_color: Color32,
        transparent: bool,
    ) {
        self.sync_active_doc_slot();
        let mut doc = Document::new(width, height);
        doc.name = format!("Untitled-{}.fw.png", self.open_docs.len() + 1);
        doc.dpi = dpi;
        doc.page_mut().bg_color = bg_color;
        doc.page_mut().transparent_bg = transparent;
        self.open_docs.push(doc.clone());
        self.active_doc = self.open_docs.len() - 1;
        self.touch_document_tab(self.active_doc);
        self.doc = doc;
        self.selected = None;
        self.selected_layer = 0;
        self.undo.clear();
        self.redo.clear();
        self.textures.clear();
        self.current_path = None;
        self.status = format!("New document: {width:.0} × {height:.0} px @ {dpi} dpi");
    }

    pub fn close_document(&mut self) {
        self.doc = Document::new(constants::DEFAULT_WIDTH, constants::DEFAULT_HEIGHT);
        self.open_docs = vec![self.doc.clone()];
        self.floating_doc_tabs.clear();
        self.active_doc = 0;
        self.doc_tab_history = vec![0];
        self.selected = None;
        self.selected_layer = 0;
        self.undo.clear();
        self.redo.clear();
        self.textures.clear();
        self.status = "Document closed".to_string();
    }

    pub fn open_canvas_size_dialog(&mut self) {
        self.new_doc_width = self.doc.page().width;
        self.new_doc_height = self.doc.page().height;
        self.show_canvas_size_dialog = true;
    }

    pub fn resize_canvas(&mut self, width: f32, height: f32) {
        self.push_undo("Canvas Size");
        self.doc.width = width;
        self.doc.height = height;
        let page = self.doc.page_mut();
        page.width = width;
        page.height = height;
        self.status = format!("Canvas resized to {width:.0} × {height:.0} px");
    }

    pub fn preview_in_browser(&mut self) {
        let html = crate::formats::export_html_with_options(&self.doc, &self.html_setup);
        let path =
            std::env::temp_dir().join(format!("firecraft_preview{}", self.html_setup.extension));
        match std::fs::write(&path, html) {
            Ok(()) => {
                open_in_default_app(&path);
                self.status = format!("Opened browser preview: {}", path.display());
            }
            Err(e) => {
                self.status = format!("Preview failed: {e}");
            }
        }
    }

    pub fn build_print_html(&self) -> String {
        let svg = crate::formats::export_svg(&self.doc);
        let orient = if self.page_setup.landscape {
            "landscape"
        } else {
            "portrait"
        };
        let paper = self.page_setup.paper.css_name();
        let margin = self.page_setup.margin_in.clamp(0.0, 3.0);
        let scale_css = if self.page_setup.fit_to_page {
            "max-width: 100%; max-height: 92vh; width: auto; height: auto;".to_string()
        } else {
            let s = (self.page_setup.scale_pct / 100.0).clamp(0.1, 4.0);
            format!("transform: scale({s:.3}); transform-origin: top left;")
        };
        format!(
            "<!DOCTYPE html>\n<html>\n<head>\n  <meta charset=\"utf-8\">\n  <title>Print — {}</title>\n  <style>\n    @page {{ size: {paper} {orient}; margin: {margin:.2}in; }}\n    body {{ margin: 0; background: #fff; display: flex; justify-content: center; align-items: flex-start; }}\n    .fc-print-sheet svg {{ {scale_css} display: block; }}\n  </style>\n</head>\n<body onload=\"window.print()\">\n  <div class=\"fc-print-sheet\">{svg}</div>\n</body>\n</html>\n",
            self.doc.name
        )
    }

    pub fn print_document(&mut self) {
        let html = self.build_print_html();
        let path = std::env::temp_dir().join("firecraft_print.html");
        match std::fs::write(&path, html) {
            Ok(()) => {
                open_in_default_app(&path);
                self.status = format!(
                    "Sent {} to print preview ({})",
                    self.doc.name,
                    self.page_setup.paper.label()
                );
            }
            Err(e) => {
                self.status = format!("Print setup failed: {e}");
            }
        }
    }

    pub fn save_all_documents(&mut self) {
        self.sync_active_doc_slot();
        if self.current_path.is_some() {
            self.save_current();
        } else {
            self.begin_file_dialog(FileMode::Save);
        }
        let clean_count = self.open_docs.iter().filter(|d| !d.dirty).count();
        self.status = format!(
            "Saved active document ({clean_count}/{} open tabs clean)",
            self.open_docs.len()
        );
    }

    pub fn fit_canvas_to_contents(&mut self) {
        let mut min_pt = Vec2::splat(f32::MAX);
        let mut max_pt = Vec2::splat(f32::MIN);
        let mut found = false;
        for layer in &self.doc.page().layers {
            if !layer.visible {
                continue;
            }
            for obj in &layer.objects {
                if let Some((lo, hi)) = obj.bounds() {
                    min_pt = min_pt.min(lo);
                    max_pt = max_pt.max(hi);
                    found = true;
                }
            }
        }
        if !found {
            self.status = "No objects on canvas to fit".to_string();
            return;
        }
        self.push_undo("Fit Canvas");
        let offset = -min_pt;
        let new_w = (max_pt.x - min_pt.x).max(16.0);
        let new_h = (max_pt.y - min_pt.y).max(16.0);
        let page = self.doc.page_mut();
        for layer in &mut page.layers {
            for obj in &mut layer.objects {
                obj.translate(offset);
            }
        }
        page.width = new_w;
        page.height = new_h;
        page.sync_state();
        self.doc.width = new_w;
        self.doc.height = new_h;
        self.status = format!("Fit canvas to {new_w:.0} × {new_h:.0} px");
    }

    pub fn crop_canvas_to_rect(&mut self, lo: Vec2, hi: Vec2) {
        self.push_undo("Crop Canvas");
        let offset = -lo;
        let new_w = (hi.x - lo.x).max(16.0);
        let new_h = (hi.y - lo.y).max(16.0);
        let page = self.doc.page_mut();
        for layer in &mut page.layers {
            for obj in &mut layer.objects {
                obj.translate(offset);
            }
        }
        page.width = new_w;
        page.height = new_h;
        page.sync_state();
        self.doc.width = new_w;
        self.doc.height = new_h;
        self.set_tool(Tool::Select);
        self.status = format!("Cropped canvas to {new_w:.0} × {new_h:.0} px");
    }

    pub fn rotate_canvas_90(&mut self) {
        let (w, h) = (self.doc.page().width, self.doc.page().height);
        self.resize_canvas(h, w);
        self.status = "Rotated canvas 90°".to_string();
    }

    pub fn estimated_export_kb(&self) -> usize {
        let pixels = (self.doc.page().width * self.doc.page().height) as usize;
        let obj_factor = self.doc.total_objects_on_page().max(1) * 3;
        let base = match self.doc.optimize.format.as_str() {
            "PNG32" => pixels / 3200 + obj_factor,
            "PNG24" => pixels / 4000 + obj_factor,
            "PNG8" | "GIF WebSnap 128" => pixels / 7500 + obj_factor,
            "JPEG - Better Quality" => {
                (pixels / 6000) * (self.doc.optimize.quality as usize).max(10) / 80
            }
            "JPEG - Smaller File" => pixels / 11000,
            "SVG Vector" | "HTML5 + Images" => obj_factor.max(2),
            _ => pixels / 4500 + obj_factor,
        };
        base.max(2)
    }

    // ---- Pages, Layers, States, Symbols, Styles, Auto Shapes ---------------

    pub fn add_layer(&mut self) {
        self.push_undo("New Layer");
        let n = self.doc.page().layers.len() + 1;
        let layer = Layer::new(&format!("Layer {n}"));
        let page = self.doc.page_mut();
        // Insert below the Web Layer if present
        let insert_idx = page
            .layers
            .iter()
            .position(|l| l.is_web_layer)
            .unwrap_or(page.layers.len());
        page.layers.insert(insert_idx, layer);
        page.sync_state();
        self.selected_layer = insert_idx;
        self.status = format!("Added Layer {n}");
    }

    pub fn add_page(&mut self) {
        let (w, h) = (self.doc.page().width, self.doc.page().height);
        let n = self.doc.pages.len() + 1;
        self.doc.pages.push(Page::new(&format!("Page {n}"), w, h));
        self.doc.current_page = self.doc.pages.len() - 1;
        self.selected = None;
        self.selected_layer = 0;
        self.status = format!("Added Page {n}");
    }

    pub fn add_state(&mut self) {
        let page = self.doc.page_mut();
        page.sync_state();
        let n = page.states.len() + 1;
        let st = AnimationState::new(&format!("State {n}"));
        page.states.push(st);
        let last = page.states.len() - 1;
        page.select_state(last);
        self.selected = None;
        self.status = format!("Added Animation State {n}");
    }

    pub fn create_template_document(&mut self, title: &str, w: f32, h: f32) {
        self.new_document(w, h);
        self.doc.name = format!("{}.fw.png", title.replace(' ', "_"));
        self.load_demo_showcase();
        self.status = format!("Created document from template: {title}");
    }

    /// Populate the active page with a rich Fireworks CS6 vector + live-effect + web slice showcase.
    pub fn load_demo_showcase(&mut self) {
        self.push_undo("Load Demo Showcase");
        let (w, h) = (self.doc.page().width, self.doc.page().height);
        for obj in demo::showcase(w, h) {
            self.push_object(obj);
        }
        self.selected_layer = 0;
        self.select_single(None);
        self.pending_fit = true;
        self.status = "Loaded FireCraft Studio 1920×1080 Brand & Feature Showcase".to_string();
    }

    // ---- File Browser & I/O ------------------------------------------------

    pub fn begin_file_dialog(&mut self, mode: FileMode) {
        self.file_mode = mode;
        self.file_dialog_name = match mode {
            FileMode::Open | FileMode::Import => String::new(),
            FileMode::Save | FileMode::Export => self
                .current_path
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| match mode {
                    FileMode::Export => "export.png".to_string(),
                    _ => self.doc.name.clone(),
                }),
        };
        let dir = self
            .current_path
            .as_ref()
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| self.file_dialog_dir.clone());
        self.set_file_dialog_dir(dir);
        self.show_file_dialog = true;
    }

    pub fn set_file_dialog_dir(&mut self, dir: PathBuf) {
        self.file_dialog_dir = dir.clone();
        self.file_dialog_address = dir.display().to_string();
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.is_dir() || !p.to_string_lossy().ends_with(".exe"))
                    .collect()
            })
            .unwrap_or_default();
        entries.sort_by_key(|p| (!p.is_dir(), p.to_string_lossy().to_lowercase()));
        self.file_dialog_entries = entries;
    }

    pub fn commit_file_dialog(&mut self) {
        if self.file_dialog_name.trim().is_empty() {
            self.status = "No file name given".to_string();
            return;
        }
        let path = self.file_dialog_dir.join(self.file_dialog_name.trim());
        let result = match self.file_mode {
            FileMode::Open => self
                .run_file_intent(FileIntent::Open(path.clone()))
                .result(),
            FileMode::Save => self
                .run_file_intent(FileIntent::SaveAs(path.clone()))
                .result(),
            FileMode::Import => self.import_path(&path),
            FileMode::Export => self.export_to(&path),
        };
        if result.is_ok() {
            self.show_file_dialog = false;
            if let Some(idx) = self.pending_close_after_save.take() {
                self.close_document_tab(idx);
                self.continue_pending_tab_closes();
            }
            if self.pending_quit_after_save {
                self.pending_quit_after_save = false;
                match self.save_all_and_quit_step() {
                    Ok(true) => {}
                    Ok(false) => self.ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                    Err(err) => self.status = err,
                }
            }
        }
    }

    pub fn handle_dropped_files(&mut self, ctx: &egui::Context) {
        let (hovered, dropped, shift) = ctx.input(|i| {
            (
                i.raw.hovered_files.clone(),
                i.raw.dropped_files.clone(),
                i.modifiers.shift,
            )
        });
        if !hovered.is_empty() {
            let screen = ctx.content_rect();
            let p = ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground,
                egui::Id::from(egui::ViewportId::from_hash_of("firecraft_drop_overlay")),
            ));
            p.rect_filled(screen, 0.0, self.palette.selection.gamma_multiply(0.10));
            p.rect_stroke(
                screen.shrink(4.0),
                4.0,
                egui::Stroke::new(2.0, self.palette.accent),
                egui::StrokeKind::Inside,
            );
            let hint_r = egui::Rect::from_center_size(screen.center(), Vec2::new(340.0, 44.0));
            p.rect_filled(hint_r, 4.0, self.palette.panel.gamma_multiply(0.95));
            p.rect_stroke(
                hint_r,
                4.0,
                egui::Stroke::new(1.0, self.palette.accent),
                egui::StrokeKind::Middle,
            );
            p.text(
                hint_r.center(),
                egui::Align2::CENTER_CENTER,
                "Drop Image / SVG to Import onto Canvas (Shift: Open Tab)",
                egui::FontId::proportional(12.5),
                self.palette.text,
            );
        }
        if dropped.is_empty() {
            return;
        }
        for file in dropped {
            let path = file.path();
            if !path.as_os_str().is_empty() {
                let is_fw = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.to_ascii_lowercase().ends_with(".fw.png"));
                let res = if shift || is_fw {
                    self.open_path(path)
                } else {
                    self.import_path(path)
                };
                if res.is_ok() {
                    self.show_welcome = false;
                }
            } else {
                #[cfg(not(target_arch = "wasm32"))]
                if let Ok(bytes) = file.bytes()
                    && let Ok(img) = image::load_from_memory(&bytes)
                {
                    let rgba = img.to_rgba8();
                    let (w, h) = (rgba.width().max(1), rgba.height().max(1));
                    self.push_undo("Drop Image");
                    let mut obj = Obj::new(
                        Shape::Bitmap {
                            at: Vec2::new(40.0, 40.0),
                            size: Vec2::new(w as f32, h as f32),
                            pixels: rgba.into_raw(),
                        },
                        None,
                        None,
                    );
                    obj.name = "Bitmap".to_string();
                    let idx = self.push_object(obj);
                    self.select_single(Some(idx));
                    self.textures.clear();
                    self.show_welcome = false;
                    self.status = format!("Imported dropped image ({w}×{h})");
                }
            }
        }
    }

    // ---- Clipboard & Selection ---------------------------------------------

    pub fn set_zoom(&mut self, zoom: f32) {
        self.doc.zoom = zoom.clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);
        self.status = format!("Zoom: {:.0}%", self.doc.zoom * 100.0);
    }

    pub fn zoom_by(&mut self, factor: f32) {
        let zoom = self.doc.zoom * factor;
        self.set_zoom(zoom);
    }

    pub fn fit_in_window(&mut self) {
        self.apply_fit();
        self.status = "Fit in window".to_string();
    }

    /// Fit the page once the live canvas rect is known (first frame after a demo/open).
    pub fn apply_fit_from_canvas(&mut self) {
        self.apply_fit();
    }

    /// Frame the page in the current canvas rect without touching the status text.
    fn apply_fit(&mut self) {
        let page = self.doc.page();
        let (pw, ph) = (page.width.max(1.0), page.height.max(1.0));
        let (cw, ch) = (self.canvas_size.x.max(1.0), self.canvas_size.y.max(1.0));
        let zoom = ((cw * 0.90) / pw).min((ch * 0.90) / ph);
        self.set_zoom(zoom);
        self.doc.pan = Vec2::ZERO;
    }

    pub fn insert_common_library_item(&mut self, item: &str) {
        self.push_undo("Insert Common Library Widget");
        let cx = self.doc.page().width * 0.5;
        let cy = self.doc.page().height * 0.5;
        let mut children = Vec::new();
        match item {
            "Primary Button" => {
                let mut bg = Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 72.0, cy - 20.0),
                        max: Vec2::new(cx + 72.0, cy + 20.0),
                        corner_radius: 6.0,
                        filled: true,
                    },
                    Some(Color32::from_rgb(56, 132, 238)),
                    Some(Color32::from_rgb(35, 96, 190)),
                );
                bg.name = "Button Background".to_string();
                let mut lbl = Obj::new(
                    Shape::Text {
                        at: Vec2::new(cx - 42.0, cy - 10.0),
                        text: "Get Started".to_string(),
                        size: 16.0,
                        bold: true,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(Color32::WHITE),
                    None,
                );
                lbl.name = "Button Label".to_string();
                children.push(bg);
                children.push(lbl);
            }
            "Checkbox (Checked)" => {
                children.push(Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 10.0, cy - 10.0),
                        max: Vec2::new(cx + 10.0, cy + 10.0),
                        corner_radius: 4.0,
                        filled: true,
                    },
                    Some(Color32::from_rgb(56, 132, 238)),
                    Some(Color32::from_rgb(35, 96, 190)),
                ));
                children.push(Obj::new(
                    Shape::Text {
                        at: Vec2::new(cx + 18.0, cy - 9.0),
                        text: "Enable option".to_string(),
                        size: 14.0,
                        bold: false,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(Color32::from_rgb(40, 44, 52)),
                    None,
                ));
            }
            "Radio Button" => {
                children.push(Obj::new(
                    Shape::Ellipse {
                        center: Vec2::new(cx, cy),
                        radius: Vec2::splat(10.0),
                        filled: true,
                    },
                    Some(Color32::WHITE),
                    Some(Color32::from_rgb(56, 132, 238)),
                ));
                children.push(Obj::new(
                    Shape::Ellipse {
                        center: Vec2::new(cx, cy),
                        radius: Vec2::splat(4.5),
                        filled: true,
                    },
                    Some(Color32::from_rgb(56, 132, 238)),
                    None,
                ));
            }
            "Text Input Field" | "Dropdown ComboBox" => {
                children.push(Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 110.0, cy - 18.0),
                        max: Vec2::new(cx + 110.0, cy + 18.0),
                        corner_radius: 4.0,
                        filled: true,
                    },
                    Some(Color32::WHITE),
                    Some(Color32::from_rgb(160, 166, 178)),
                ));
                children.push(Obj::new(
                    Shape::Text {
                        at: Vec2::new(cx - 98.0, cy - 8.0),
                        text: if item == "Dropdown ComboBox" {
                            "Select option...  ▾".to_string()
                        } else {
                            "Enter text...".to_string()
                        },
                        size: 13.0,
                        bold: false,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(Color32::from_rgb(120, 126, 138)),
                    None,
                ));
            }
            "Toggle Switch" => {
                children.push(Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 24.0, cy - 12.0),
                        max: Vec2::new(cx + 24.0, cy + 12.0),
                        corner_radius: 12.0,
                        filled: true,
                    },
                    Some(Color32::from_rgb(52, 180, 110)),
                    None,
                ));
                children.push(Obj::new(
                    Shape::Ellipse {
                        center: Vec2::new(cx + 12.0, cy),
                        radius: Vec2::splat(9.0),
                        filled: true,
                    },
                    Some(Color32::WHITE),
                    None,
                ));
            }
            "Modal Card" => {
                children.push(Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 160.0, cy - 100.0),
                        max: Vec2::new(cx + 160.0, cy + 100.0),
                        corner_radius: 8.0,
                        filled: true,
                    },
                    Some(Color32::from_rgb(248, 249, 252)),
                    Some(Color32::from_rgb(190, 196, 208)),
                ));
                children.push(Obj::new(
                    Shape::Text {
                        at: Vec2::new(cx - 138.0, cy - 78.0),
                        text: "Card Title".to_string(),
                        size: 18.0,
                        bold: true,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(Color32::from_rgb(30, 34, 42)),
                    None,
                ));
            }
            "Progress Bar" => {
                children.push(Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 120.0, cy - 6.0),
                        max: Vec2::new(cx + 120.0, cy + 6.0),
                        corner_radius: 6.0,
                        filled: true,
                    },
                    Some(Color32::from_rgb(220, 224, 232)),
                    None,
                ));
                children.push(Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 120.0, cy - 6.0),
                        max: Vec2::new(cx + 40.0, cy + 6.0),
                        corner_radius: 6.0,
                        filled: true,
                    },
                    Some(Color32::from_rgb(56, 132, 238)),
                    None,
                ));
            }
            "Navigation Bar" => {
                children.push(Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 240.0, cy - 22.0),
                        max: Vec2::new(cx + 240.0, cy + 22.0),
                        corner_radius: 4.0,
                        filled: true,
                    },
                    Some(Color32::from_rgb(36, 40, 48)),
                    None,
                ));
                children.push(Obj::new(
                    Shape::Text {
                        at: Vec2::new(cx - 220.0, cy - 9.0),
                        text: "Brand   Home   Features   Pricing   Docs".to_string(),
                        size: 14.0,
                        bold: true,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(Color32::WHITE),
                    None,
                ));
            }
            _ => {
                children.push(Obj::new(
                    Shape::Rect {
                        min: Vec2::new(cx - 46.0, cy - 13.0),
                        max: Vec2::new(cx + 46.0, cy + 13.0),
                        corner_radius: 13.0,
                        filled: true,
                    },
                    Some(Color32::from_rgb(232, 163, 61)),
                    None,
                ));
                children.push(Obj::new(
                    Shape::Text {
                        at: Vec2::new(cx - 24.0, cy - 8.0),
                        text: "NEW".to_string(),
                        size: 12.0,
                        bold: true,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(Color32::WHITE),
                    None,
                ));
            }
        }
        let mut grp = Obj::new(Shape::Group { children }, None, None);
        grp.name = item.to_string();
        let idx = self.push_object(grp);
        self.select_single(Some(idx));
        self.status = format!("Inserted Common Library widget: {item}");
    }

    pub fn tick_animation(&mut self, ctx: &Context, now: f64) {
        if !self.anim_playing {
            return;
        }
        let state_count = self.doc.page().states.len();
        if state_count <= 1 {
            self.anim_playing = false;
            return;
        }
        let cur = self.doc.page().current_state.min(state_count - 1);
        let delay_sec = (self.doc.page().states[cur].delay_cs as f64 * 0.01).max(0.05);
        if now - self.anim_last_tick >= delay_sec {
            self.anim_last_tick = now;
            let next = (cur + 1) % state_count;
            self.doc.page_mut().select_state(next);
            self.selected = None;
            self.selected_set.clear();
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(40));
    }

    pub fn toggle_window_fullscreen(&mut self) {
        self.window_fullscreen = !self.window_fullscreen;
        self.ctx
            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.window_fullscreen));
        self.status = if self.window_fullscreen {
            "Window Fullscreen ON (F11 to exit)".to_string()
        } else {
            "Window Fullscreen OFF".to_string()
        };
    }

    pub fn ai_key_present(&self) -> bool {
        !self.ai_key.trim().is_empty()
            || std::env::var("FIRECRAFT_AI_API_KEY")
                .map(|k| !k.trim().is_empty())
                .unwrap_or(false)
    }

    pub fn start_ai_request(&mut self) {
        if self.ai_rx.is_some() {
            return;
        }
        if !self.ai_key_present() {
            self.ai_status =
                "Missing API key: set FIRECRAFT_AI_API_KEY or enter a key.".to_string();
            return;
        }
        if self.ai_model.trim().is_empty() {
            self.ai_status =
                "Missing model name (for example, your provider's chat model id).".to_string();
            return;
        }
        let prompt = self.ai_prompt.trim().to_string();
        if prompt.is_empty() {
            self.ai_status = "Enter a question or instruction first.".to_string();
            return;
        }
        let key = if !self.ai_key.trim().is_empty() {
            self.ai_key.trim().to_string()
        } else {
            std::env::var("FIRECRAFT_AI_API_KEY").unwrap_or_default()
        };
        let base = self.ai_base_url.trim().trim_end_matches('/').to_string();
        if base.is_empty() {
            self.ai_status = "Missing API base URL.".to_string();
            return;
        }
        let model = self.ai_model.trim().to_string();
        let context = match self.selected_object() {
            Some(obj) => format!(
                "Selection: {} ({}) at bounds {:?}, fill {:?}, stroke {:?}, opacity {:.0}%, {} live filter(s).",
                obj.name,
                obj.type_label(),
                obj.bounds(),
                obj.fill,
                obj.stroke,
                obj.opacity * 100.0,
                obj.effects.len()
            ),
            None => format!(
                "No object selected. Document '{}' is {:.0}x{:.0} px at {} dpi with {} layer(s) and {} state(s).",
                self.doc.name,
                self.doc.page().width,
                self.doc.page().height,
                self.doc.dpi,
                self.doc.page().layers.len(),
                self.doc.page().states.len()
            ),
        };
        let user_content = format!("{prompt}\n\nEditor context:\n{context}");
        let payload = format!(
            "{{\"model\":{},\"messages\":[{{\"role\":\"system\",\"content\":{}}},{{\"role\":\"user\",\"content\":{}}}]}}",
            json_escape(&model),
            json_escape(
                "You are FireCraft's design assistant. Answer concisely and use the supplied editor context. Never claim to have changed the document."
            ),
            json_escape(&user_content)
        );
        let url = format!("{base}/chat/completions");
        let (tx, rx) = std::sync::mpsc::channel();
        self.ai_rx = Some(rx);
        self.ai_status = "Requesting response…".to_string();
        std::thread::spawn(move || {
            let mut cmd = std::process::Command::new("curl");
            cmd.args([
                "-sS",
                "--max-time",
                "90",
                "-X",
                "POST",
                &url,
                "-H",
                "Content-Type: application/json",
                "-H",
                &format!("Authorization: Bearer {key}"),
                "--data-raw",
                &payload,
            ]);
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                const CREATE_NO_WINDOW: u32 = 0x0800_0000;
                cmd.creation_flags(CREATE_NO_WINDOW);
            }
            let result = match cmd.output() {
                Ok(out) => {
                    let body = String::from_utf8_lossy(&out.stdout).to_string();
                    if out.status.success() {
                        match ai_extract_content(&body) {
                            Some(text) if !text.trim().is_empty() => Ok(text),
                            _ => Err(format!(
                                "Provider returned no assistant message. Response: {}",
                                body.trim()
                            )),
                        }
                    } else {
                        let err = String::from_utf8_lossy(&out.stderr);
                        Err(format!(
                            "Request failed ({}). {}{}",
                            out.status,
                            err.trim(),
                            if body.trim().is_empty() {
                                String::new()
                            } else {
                                format!(" Body: {}", body.trim())
                            }
                        ))
                    }
                }
                Err(e) => Err(format!("Could not run curl: {e}")),
            };
            let _ = tx.send(result);
        });
    }

    pub fn poll_ai(&mut self) {
        let Some(rx) = &self.ai_rx else {
            return;
        };
        match rx.try_recv() {
            Ok(Ok(text)) => {
                self.ai_response = text;
                self.ai_status = "Response received.".to_string();
                self.ai_rx = None;
            }
            Ok(Err(err)) => {
                self.ai_status = err;
                self.ai_rx = None;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                self.ctx
                    .request_repaint_after(std::time::Duration::from_millis(120));
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.ai_status = "AI request worker stopped unexpectedly.".to_string();
                self.ai_rx = None;
            }
        }
    }

    pub fn check_for_updates(&mut self) {
        let cur = env!("CARGO_PKG_VERSION");
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut cmd = std::process::Command::new("curl");
            cmd.args([
                "-sL",
                "--max-time",
                "6",
                "-H",
                "User-Agent: FireCraft-Updater",
                "https://api.github.com/repos/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/latest",
            ]);
            #[cfg(target_os = "windows")]
            {
                use std::os::windows::process::CommandExt;
                cmd.creation_flags(0x0800_0000);
            }
            match cmd.output() {
                Ok(out) if out.status.success() => {
                    let body = String::from_utf8_lossy(&out.stdout);
                    if let Some(tag) = crate::mcp::extract_json_str(&body, "tag_name") {
                        let url = crate::mcp::extract_json_str(&body, "html_url").unwrap_or_else(|| {
                            "https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/latest".to_string()
                        });
                        let clean_tag = tag.trim_start_matches('v');
                        self.update_latest_version = Some(tag.clone());
                        self.update_release_url = Some(url);
                        if clean_tag == cur {
                            self.update_status = format!("You are up to date (v{cur}).");
                        } else {
                            self.update_status =
                                format!("Update available: {tag} (installed: v{cur})");
                        }
                    } else {
                        self.update_status =
                            format!("Current: v{cur} — check GitHub Releases for latest builds.");
                    }
                }
                _ => {
                    self.update_status =
                        format!("Offline or API rate-limited (installed: v{cur}).");
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.update_status = format!("Web build v{cur} — reload page for latest release.");
        }
        self.status = self.update_status.clone();
    }

    pub fn open_update_release_page(&self) {
        let url = self.update_release_url.as_deref().unwrap_or(
            "https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/latest",
        );
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("explorer.exe").arg(url).spawn();
        }
        #[cfg(all(not(target_os = "windows"), not(target_arch = "wasm32")))]
        {
            let _ = std::process::Command::new("xdg-open").arg(url).spawn();
        }
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(w) = web_sys::window() {
                let _ = w.open_with_url_and_target(url, "_blank");
            }
        }
    }

    pub fn set_theme(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
        self.palette = theme::resolve(mode);
        self.status = format!("Theme: {}", mode.label());
    }
}

impl eframe::App for FireCraft {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        crate::layout::show(ui, self);
    }

    /// Keep the native backbuffer the same dark tone as the workspace. Without this the default
    /// light clear color can appear for a frame on Windows when the window is resized (entering or
    /// leaving full-screen) or when DWM re-composites after an app switch.
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        let p = self.palette.bg;
        [
            p.r() as f32 / 255.0,
            p.g() as f32 / 255.0,
            p.b() as f32 / 255.0,
            1.0,
        ]
    }
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_unescape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('t') => out.push('\t'),
            Some('b') => out.push('\u{8}'),
            Some('f') => out.push('\u{c}'),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            Some('/') => out.push('/'),
            Some('u') => {
                let hex: String = chars.by_ref().take(4).collect();
                if let Ok(code) = u32::from_str_radix(&hex, 16)
                    && let Some(ch) = char::from_u32(code)
                {
                    out.push(ch);
                }
            }
            Some(other) => out.push(other),
            None => break,
        }
    }
    out
}

/// Pulls the first non-empty assistant `content` string out of an OpenAI-compatible response.
fn ai_extract_content(json: &str) -> Option<String> {
    let mut search_from = 0;
    while let Some(rel) = json[search_from..].find("\"content\"") {
        let mut i = search_from + rel + "\"content\"".len();
        let bytes = json.as_bytes();
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b':' {
            search_from = i;
            continue;
        }
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'"' {
            search_from = i;
            continue;
        }
        i += 1;
        let start = i;
        let mut escaped = false;
        while i < bytes.len() {
            let b = bytes[i];
            if b == b'"' && !escaped {
                break;
            }
            escaped = b == b'\\' && !escaped;
            if b != b'\\' {
                escaped = false;
            }
            i += 1;
        }
        if i <= bytes.len() {
            let text = json_unescape(&json[start..i]);
            if !text.trim().is_empty() {
                return Some(text);
            }
        }
        search_from = i.saturating_add(1).min(json.len());
    }
    None
}

fn open_in_default_app(path: &std::path::Path) {
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/C", "start", "", &path.to_string_lossy()])
            .spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(path).spawn();
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    }
    #[cfg(target_arch = "wasm32")]
    let _ = path;
}

fn point_in_polygon(pt: Vec2, poly: &[Vec2]) -> bool {
    if poly.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = poly.len() - 1;
    for i in 0..poly.len() {
        let pi = poly[i];
        let pj = poly[j];
        let intersects = ((pi.y > pt.y) != (pj.y > pt.y))
            && (pt.x < (pj.x - pi.x) * (pt.y - pi.y) / (pj.y - pi.y + 1e-6) + pi.x);
        if intersects {
            inside = !inside;
        }
        j = i;
    }
    inside
}

#[cfg(test)]
mod tests {
    use super::object::{polygon_area, polygon_boolean};
    use super::*;

    /// Intent-level test fixture from the architecture review. It absorbs the layer,
    /// visibility, and selection setup that command tests used to repeat.
    struct DocumentFixture {
        app: FireCraft,
        last: Option<usize>,
    }

    impl DocumentFixture {
        fn new() -> Self {
            Self {
                app: FireCraft::new(None),
                last: None,
            }
        }

        fn push(mut self, kind: Shape, fill: Option<Color32>) -> Self {
            let idx = self.app.push_object(Obj::new(kind, fill, None));
            self.last = Some(idx);
            self
        }

        fn rect(self) -> Self {
            self.push(
                Shape::Rect {
                    min: Vec2::new(20.0, 20.0),
                    max: Vec2::new(120.0, 80.0),
                    corner_radius: 0.0,
                    filled: true,
                },
                Some(Color32::from_rgb(40, 90, 160)),
            )
        }

        fn text(self, value: &str) -> Self {
            self.push(
                Shape::Text {
                    at: Vec2::new(24.0, 100.0),
                    text: value.to_string(),
                    size: 24.0,
                    bold: false,
                    italic: false,
                    font_family: "Serif".to_string(),
                },
                Some(Color32::BLACK),
            )
        }

        fn hidden(mut self) -> Self {
            if let Some(idx) = self.last {
                let layer = self.app.selected_layer;
                if let Some(obj) = self.app.doc.page_mut().layers[layer].objects.get_mut(idx) {
                    obj.visible = false;
                }
            }
            self
        }

        fn selected(mut self) -> Self {
            self.app.select_single(self.last);
            self
        }

        fn build(self) -> FireCraft {
            self.app
        }
    }

    #[test]
    fn expanded_mode_is_the_startup_workspace() {
        let app = FireCraft::new(None);
        assert_eq!(app.workspace_preset, "Expanded Mode");
        assert!(app.show_main_toolbar);
        assert!(app.show_tools_dock);
        assert!(app.show_right_dock);
        assert_eq!(
            app.panel_groups
                .iter()
                .filter(|g| g.zone == DockZone::RightOuter && !g.tabs.is_empty())
                .count(),
            5
        );
    }

    #[test]
    fn pen_builds_a_path_from_clicks_and_commits_it() {
        let mut app = FireCraft::new(None);
        app.set_tool(Tool::Path);
        app.pen_add_node(Vec2::new(20.0, 20.0));
        app.pen_add_node(Vec2::new(120.0, 30.0));
        app.pen_add_node(Vec2::new(80.0, 100.0));
        assert_eq!(app.pen_nodes.len(), 3);
        app.finish_pen_path(true);
        assert!(app.pen_nodes.is_empty());
        let obj = app.selected_object().expect("committed path is selected");
        match obj.kind {
            Shape::Path { points, closed } => {
                assert_eq!(points.len(), 3);
                assert!(closed);
            }
            other => panic!("expected path, got {other:?}"),
        }
    }

    #[test]
    fn switching_tabs_preserves_view_and_returns_canvas_focus() {
        let mut app = FireCraft::new(None);
        app.view_mode = CanvasViewMode::TwoUp;
        app.new_document_tab(800.0, 600.0);
        app.switch_document_tab(0);
        assert_eq!(app.view_mode, CanvasViewMode::TwoUp);
        assert!(app.focus_canvas);
    }

    #[test]
    fn discard_and_quit_arms_the_close_gate() {
        let mut app = DocumentFixture::new().rect().selected().build();
        app.doc.dirty = true;
        app.show_quit_confirm = true;
        assert!(app.should_intercept_close());
        app.confirm_discard_and_quit();
        assert!(!app.show_quit_confirm);
        assert!(!app.should_intercept_close());
    }

    #[test]
    fn closing_an_earlier_tab_keeps_the_active_document() {
        let mut app = FireCraft::new(None);
        app.doc.name = "First".to_string();
        app.doc.dirty = true;
        app.sync_active_doc_slot();
        app.new_document_tab(800.0, 600.0);
        app.doc.name = "Second".to_string();
        app.doc.dirty = true;
        app.sync_active_doc_slot();
        app.request_close_document_tab(0);
        assert_eq!(app.pending_close_tab, Some(0));
        app.close_document_tab(0);
        assert_eq!(app.open_docs.len(), 1);
        assert_eq!(app.active_doc, 0);
        assert_eq!(app.doc.name, "Second");
    }

    #[test]
    fn tab_history_tracks_recent_switches_and_close_remaps() {
        let mut app = FireCraft::new(None);
        app.new_document_tab(800.0, 600.0);
        app.new_document_tab(800.0, 600.0);
        app.switch_document_tab(0);
        app.switch_document_tab(1);
        assert_eq!(&app.doc_tab_history[..3], &[1, 0, 2]);
        app.close_document_tab(0);
        assert_eq!(&app.doc_tab_history[..2], &[0, 1]);
    }

    #[test]
    fn bulk_close_stops_for_dirty_tab_and_then_finishes() {
        let mut app = FireCraft::new(None);
        app.new_document_tab(800.0, 600.0);
        app.new_document_tab(800.0, 600.0);
        app.open_docs[1].dirty = true;
        app.request_close_document_tabs(vec![0, 1, 2]);
        assert_eq!(app.open_docs.len(), 2);
        assert_eq!(app.pending_close_tab, Some(1));
        assert_eq!(app.pending_close_tabs, vec![0]);
        app.close_document_tab(1);
        app.pending_close_tab = None;
        app.continue_pending_tab_closes();
        assert_eq!(app.open_docs.len(), 1);
        assert_eq!(app.active_doc, 0);
        assert!(!app.doc.dirty);
    }

    #[test]
    fn file_intent_saves_and_clears_dirty_state() {
        let mut app = DocumentFixture::new().rect().selected().build();
        app.doc.dirty = true;
        let path = std::env::temp_dir().join("firecraft-file-intent-test.fw.png");
        let outcome = app.run_file_intent(FileIntent::SaveAs(path.clone()));
        assert_eq!(outcome, FileOutcome::Saved(path.clone()));
        assert!(!app.doc.dirty);
        assert_eq!(app.current_path, Some(path.clone()));
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn text_menu_lists_document_fonts_and_sizes() {
        let app = DocumentFixture::new()
            .rect()
            .text("Dynamic type")
            .selected()
            .build();
        assert!(app.text_font_families().iter().any(|f| f == "Serif"));
        assert!(app.text_sizes().iter().any(|s| (*s - 24.0).abs() < 0.01));
        assert_eq!(app.selected_text_style().unwrap().0, "Serif");
    }

    #[test]
    fn hidden_fixture_can_be_selected_for_command_tests() {
        let app = DocumentFixture::new()
            .rect()
            .text("Hidden label")
            .hidden()
            .selected()
            .build();
        let obj = app.selected_object().expect("fixture stays selected");
        assert!(!obj.visible);
    }

    #[test]
    fn app_workflow_auto_shapes_effects_and_undo() {
        let mut app = FireCraft::new(None);
        assert_eq!(app.open_docs.len(), 1);
        assert_eq!(app.doc.symbols.len(), 1);

        app.insert_auto_shape(AutoShapeKind::Star);
        assert!(app.selected.is_some());
        assert_eq!(app.doc.total_objects_on_page(), 1);

        app.add_live_effect(LiveEffect::DropShadow {
            distance: 5.0,
            angle_deg: 45.0,
            blur: 4.0,
            color: Color32::BLACK,
            enabled: true,
        });
        assert_eq!(app.selected_object().unwrap().effects.len(), 1);

        app.flip_horizontal();
        app.rotate_90();
        app.undo_action(); // undo rotate
        app.undo_action(); // undo flip
        app.undo_action(); // undo live filter
        assert_eq!(app.selected_object().unwrap().effects.len(), 0);
        app.redo_action();
        assert_eq!(app.selected_object().unwrap().effects.len(), 1);
    }

    #[test]
    fn polygon_booleans_are_area_correct() {
        let a = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(100.0, 0.0),
            Vec2::new(100.0, 100.0),
            Vec2::new(0.0, 100.0),
        ];
        let b = vec![
            Vec2::new(50.0, 50.0),
            Vec2::new(150.0, 50.0),
            Vec2::new(150.0, 150.0),
            Vec2::new(50.0, 150.0),
        ];
        let union = polygon_boolean(&a, &b, "union").expect("union");
        assert!((polygon_area(&union).abs() - 17_500.0).abs() < 1.0);
        let intersect = polygon_boolean(&a, &b, "intersect").expect("intersect");
        assert!((polygon_area(&intersect).abs() - 2_500.0).abs() < 1.0);
        let punch = polygon_boolean(&a, &b, "punch").expect("punch");
        assert!((polygon_area(&punch).abs() - 7_500.0).abs() < 1.0);
    }

    #[test]
    fn polygon_boolean_area_identities_hold() {
        let square = |x: f32, y: f32, size: f32| {
            vec![
                Vec2::new(x, y),
                Vec2::new(x + size, y),
                Vec2::new(x + size, y + size),
                Vec2::new(x, y + size),
            ]
        };
        let a = square(0.0, 0.0, 100.0);
        for offset in [0.0, 25.0, 50.0, 99.0] {
            let b = square(offset, offset * 0.5, 100.0);
            let area_a = polygon_area(&a).abs();
            let area_b = polygon_area(&b).abs();
            let union = polygon_area(&polygon_boolean(&a, &b, "union").unwrap()).abs();
            let inter =
                polygon_area(&polygon_boolean(&a, &b, "intersect").unwrap_or_default()).abs();
            let punch = polygon_area(&polygon_boolean(&a, &b, "punch").unwrap_or_default()).abs();

            assert!(
                (union + inter - (area_a + area_b)).abs() < 2.0,
                "offset {offset}: union {union} + intersect {inter} != {area_a} + {area_b}"
            );
            assert!(
                (punch - (area_a - inter)).abs() < 2.0,
                "offset {offset}: punch {punch} != area {area_a} - intersect {inter}"
            );
            let self_union = polygon_boolean(&a, &a, "union").expect("a union a");
            assert!((polygon_area(&self_union).abs() - area_a).abs() < 1.0);
            let self_inter = polygon_boolean(&a, &a, "intersect").expect("a intersect a");
            assert!((polygon_area(&self_inter).abs() - area_a).abs() < 1.0);
        }

        // Disjoint and holed results cannot be stored in a single closed path. Reject them rather
        // than silently keeping only the largest region.
        let far = square(500.0, 500.0, 100.0);
        assert!(polygon_boolean(&a, &far, "union").is_none());
        let island = square(40.0, 40.0, 20.0);
        assert!(
            polygon_boolean(&a, &island, "punch").is_none(),
            "punch that leaves a hole must be reported, not filled"
        );
    }

    #[test]
    fn four_quarter_turns_restore_the_bounds() {
        let mut app = FireCraft::new(None);
        app.insert_auto_shape(AutoShapeKind::ChamferRect);
        app.select_all();
        let (before_min, before_max) = app
            .selected_object()
            .expect("selection")
            .bounds()
            .expect("bounds");
        let before_w = before_max.x - before_min.x;
        let before_h = before_max.y - before_min.y;
        for turn in 0..4 {
            app.rotate_90();
            let (min, max) = app
                .selected_object()
                .expect("selection")
                .bounds()
                .expect("bounds");
            let w = max.x - min.x;
            let h = max.y - min.y;
            if turn < 3 {
                let same = (w - before_w).abs() < 0.5 && (h - before_h).abs() < 0.5;
                let swapped = (w - before_h).abs() < 0.5 && (h - before_w).abs() < 0.5;
                assert!(
                    same || swapped,
                    "turn {}: {w}x{h} vs {before_w}x{before_h}",
                    turn + 1
                );
            } else {
                assert!(
                    (w - before_w).abs() < 0.5 && (h - before_h).abs() < 0.5,
                    "four quarter turns should restore {before_w}x{before_h}, got {w}x{h}"
                );
            }
        }
    }

    #[test]
    fn style_gradient_applies_to_every_selected_object() {
        let mut app = FireCraft::new(None);
        app.insert_auto_shape(AutoShapeKind::RoundedRect);
        app.insert_auto_shape(AutoShapeKind::Star);
        app.select_all();
        let preset = StylePreset {
            name: "Test Gradient".to_string(),
            fill: Some(Color32::from_rgb(255, 0, 0)),
            gradient_end: Some(Color32::from_rgb(0, 0, 255)),
            stroke: Some(Color32::BLACK),
            stroke_width: 2.0,
            effects: Vec::new(),
        };
        app.apply_style_preset_value(&preset);
        let objects = &app.doc.page().layers[0].objects;
        assert_eq!(objects.len(), 2);
        for obj in objects {
            let gradient = obj.gradient.expect("gradient applied");
            assert_eq!(gradient.end, Color32::from_rgb(0, 0, 255));
        }
    }

    #[test]
    fn multi_tab_and_demo_showcase() {
        let mut app = FireCraft::new(None);
        app.load_demo_showcase();
        assert!(app.doc.total_objects_on_page() >= 6);

        app.new_document_tab(1024.0, 768.0);
        assert_eq!(app.open_docs.len(), 2);
        assert_eq!(app.active_doc, 1);
        assert_eq!(app.doc.page().width, 1024.0);

        app.switch_document_tab(0);
        assert_eq!(app.active_doc, 0);
        assert!(app.doc.total_objects_on_page() >= 6);
    }

    #[test]
    fn multi_selection_and_group_ungroup() {
        let mut app = FireCraft::new(None);
        app.insert_auto_shape(AutoShapeKind::RoundedRect);
        app.insert_auto_shape(AutoShapeKind::Star);
        assert_eq!(app.doc.total_objects_on_page(), 2);

        app.select_all();
        assert_eq!(app.selected_set.len(), 2);

        app.group_selected();
        assert_eq!(app.doc.total_objects_on_page(), 1);
        assert!(matches!(
            app.selected_object().unwrap().kind,
            Shape::Group { .. }
        ));

        app.ungroup_selected();
        assert_eq!(app.doc.total_objects_on_page(), 2);
        assert_eq!(app.selected_set.len(), 2);
    }

    #[test]
    fn clipboard_copy_cut_paste_and_welcome_default() {
        let mut app = FireCraft::new(None);
        assert!(app.show_welcome);
        app.insert_auto_shape(AutoShapeKind::RoundedRect);
        app.insert_auto_shape(AutoShapeKind::Star);
        app.select_all();
        app.copy_selected();
        assert_eq!(app.clipboard_set.len(), 2);
        app.paste_with_text(Some("FIRECRAFT_CLIPBOARD:2"));
        assert_eq!(app.doc.total_objects_on_page(), 4);
        assert_eq!(app.selected_set.len(), 2);

        app.paste_with_text(Some("External Pasted Headline"));
        assert_eq!(app.doc.total_objects_on_page(), 5);

        app.select_inverse();
        assert_eq!(app.selected_set.len(), 4);

        app.toggle_full_screen_canvas();
        assert!(app.full_screen_canvas);
        assert!(!app.show_tools_dock);
        assert!(!app.show_right_dock);
        assert!(!app.show_bottom_inspector);
        assert!(!app.show_main_toolbar);

        app.toggle_full_screen_canvas();
        assert!(!app.full_screen_canvas);
        assert!(app.show_tools_dock);
        assert!(app.show_right_dock);
        assert!(app.show_bottom_inspector);
        assert!(app.show_main_toolbar);
    }

    #[test]
    fn select_all_and_delete_across_layers() {
        let mut app = FireCraft::new(None);
        app.insert_auto_shape(AutoShapeKind::RoundedRect);
        app.add_layer();
        app.insert_auto_shape(AutoShapeKind::Star);
        app.insert_slice(false); // goes to Web Layer
        assert_eq!(app.doc.total_objects_on_page(), 3);

        // Select all even when current selected_layer is empty or different
        app.clear_selection();
        app.select_all();
        assert!(!app.selected_set.is_empty());
        assert!(!app.selected_extra_layers.is_empty());

        app.delete_selected();
        assert_eq!(app.doc.total_objects_on_page(), 0);
    }

    #[test]
    fn combine_paths_boolean_ops() {
        let mut app = FireCraft::new(None);
        app.push_object(Obj::new(
            Shape::Rect {
                min: Vec2::new(100.0, 100.0),
                max: Vec2::new(220.0, 220.0),
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::RED),
            None,
        ));
        app.push_object(Obj::new(
            Shape::Rect {
                min: Vec2::new(160.0, 140.0),
                max: Vec2::new(280.0, 260.0),
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::BLUE),
            None,
        ));
        app.select_all();
        app.combine_paths_op("intersect");
        assert_eq!(app.doc.total_objects_on_page(), 1);
        let (lo, hi) = app.selected_object().unwrap().bounds().unwrap();
        assert!((lo.x - 160.0).abs() < 1.0 && (lo.y - 140.0).abs() < 1.0);
        assert!((hi.x - 220.0).abs() < 1.0 && (hi.y - 220.0).abs() < 1.0);
    }

    #[test]
    fn flip_horizontal_and_vertical_keeps_bounds() {
        let mut app = FireCraft::new(None);
        app.push_object(Obj::new(
            Shape::Polygon {
                center: Vec2::new(300.0, 300.0),
                radius: 60.0,
                sides: 5,
                star_inner: 0.45,
                angle_rad: 0.3,
            },
            Some(Color32::GOLD),
            None,
        ));
        app.select_single(Some(0));
        let before_bounds = app.selected_object().unwrap().bounds().unwrap();
        app.flip_horizontal();
        let after_h = app.selected_object().unwrap();
        let (blo, bhi) = before_bounds;
        let (alo, ahi) = after_h.bounds().unwrap();
        assert!((alo.x - blo.x).abs() < 1e-3 && (ahi.x - bhi.x).abs() < 1e-3);
        assert!((alo.y - blo.y).abs() < 1e-3 && (ahi.y - bhi.y).abs() < 1e-3);
        if let Shape::Polygon { angle_rad, .. } = after_h.kind {
            assert!((angle_rad - (-0.3)).abs() < 1e-4);
        } else {
            panic!("Expected Polygon");
        }
    }

    #[test]
    fn file_menu_html_and_print_setup_generation() {
        let mut app = FireCraft::new(None);
        app.html_setup.style = crate::formats::HtmlStyle::CssGrid;
        app.html_setup.include_viewport_meta = true;
        app.html_setup.center_in_browser = true;
        let html = crate::formats::export_html_with_options(&app.doc, &app.html_setup);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("display: grid"));
        assert!(html.contains("viewport"));

        app.page_setup.paper = PaperSize::A4;
        app.page_setup.landscape = true;
        app.page_setup.margin_in = 0.25;
        let print_html = app.build_print_html();
        assert!(print_html.contains("@page { size: A4 landscape; margin: 0.25in; }"));
        assert!(print_html.contains("window.print()"));
    }

    #[test]
    fn styles_library_categories_and_apply() {
        let mut app = FireCraft::new(None);
        for lib in crate::doc::StyleLibrary::ALL {
            let list = StylePreset::library_styles(lib);
            assert!(
                !list.is_empty(),
                "Expected non-empty style library for {:?}",
                lib
            );
        }
        let dark_chrome = StylePreset::library_styles(crate::doc::StyleLibrary::DarkChrome);
        assert!(dark_chrome.len() >= 20);
        let layer_idx = app.selected_layer;
        app.doc.page_mut().layers[layer_idx].objects.push(Obj::new(
            Shape::Rect {
                min: Vec2::new(10.0, 10.0),
                max: Vec2::new(100.0, 60.0),
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::GRAY),
            None,
        ));
        app.select_single(Some(0));
        app.apply_style_preset_value(&dark_chrome[0]);
        let obj = app.selected_object().unwrap();
        assert_eq!(obj.fill, dark_chrome[0].fill);
        assert_eq!(obj.stroke, dark_chrome[0].stroke);
    }

    #[test]
    fn document_tabs_swap_keeps_active_doc() {
        let mut app = FireCraft::new(None);
        app.doc.name = "First".to_string();
        app.new_document_tab(800.0, 600.0);
        app.doc.name = "Second".to_string();
        app.sync_active_doc_slot();
        assert_eq!(app.active_doc, 1);
        app.swap_document_tabs(0, 1);
        assert_eq!(app.active_doc, 0);
        assert_eq!(app.open_docs[0].name, "Second");
        assert_eq!(app.open_docs[1].name, "First");
    }

    #[test]
    fn lasso_and_wand_selection_tools() {
        let mut app = FireCraft::new(None);
        let layer_idx = app.selected_layer;
        app.doc.page_mut().layers[layer_idx].objects.push(Obj::new(
            Shape::Rect {
                min: Vec2::new(100.0, 100.0),
                max: Vec2::new(160.0, 160.0),
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::from_rgb(200, 60, 40)),
            None,
        ));
        app.doc.page_mut().layers[layer_idx].objects.push(Obj::new(
            Shape::Rect {
                min: Vec2::new(400.0, 400.0),
                max: Vec2::new(460.0, 460.0),
                corner_radius: 0.0,
                filled: true,
            },
            Some(Color32::from_rgb(200, 60, 40)),
            None,
        ));

        // Lasso around first rectangle only
        app.tool = Tool::Lasso;
        app.on_pointer_down(Vec2::new(80.0, 80.0), Vec2::ZERO, false, false, false);
        app.on_pointer_drag(Vec2::new(180.0, 80.0), false);
        app.on_pointer_drag(Vec2::new(180.0, 180.0), false);
        app.on_pointer_drag(Vec2::new(80.0, 180.0), false);
        app.on_pointer_up(Vec2::new(80.0, 80.0), false);
        assert_eq!(app.selected_set, vec![0]);

        // Magic Wand clicks first rectangle and selects both color-matched rectangles
        app.tool = Tool::Wand;
        app.on_pointer_down(Vec2::new(120.0, 120.0), Vec2::ZERO, false, false, false);
        assert_eq!(app.selected_set, vec![0, 1]);
    }

    #[test]
    fn convert_to_path_works_for_shapes_and_text() {
        let mut app = FireCraft::new(None);
        let layer_idx = app.selected_layer;
        app.doc.page_mut().layers[layer_idx].objects.push(Obj::new(
            Shape::Rect {
                min: Vec2::new(10.0, 10.0),
                max: Vec2::new(110.0, 70.0),
                corner_radius: 12.0,
                filled: true,
            },
            Some(Color32::BLUE),
            None,
        ));
        app.select_single(Some(0));
        app.convert_selected_to_path();
        assert!(matches!(
            app.selected_object().unwrap().kind,
            Shape::Path { ref points, closed: true } if points.len() == 16
        ));

        app.doc.page_mut().layers[layer_idx].objects.push(Obj::new(
            Shape::Text {
                at: Vec2::new(40.0, 100.0),
                text: "FireCraft".to_string(),
                size: 24.0,
                bold: false,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(Color32::WHITE),
            None,
        ));
        app.select_single(Some(1));
        app.convert_selected_to_path();
        assert!(matches!(
            app.selected_object().unwrap().kind,
            Shape::Path { closed: true, .. }
        ));
    }

    #[test]
    fn flatten_selection_renders_transparent_glyphs_not_solid_box() {
        let mut app = FireCraft::new(None);
        let layer_idx = app.selected_layer;
        app.doc.page_mut().layers[layer_idx].objects.push(Obj::new(
            Shape::Text {
                at: Vec2::new(40.0, 100.0),
                text: "HI".to_string(),
                size: 28.0,
                bold: false,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(Color32::from_rgb(255, 80, 20)),
            None,
        ));
        app.select_single(Some(0));
        app.flatten_selected_to_bitmap();
        let flat = app.selected_object().unwrap();
        if let Shape::Bitmap { pixels, .. } = &flat.kind {
            let alphas: Vec<u8> = pixels.as_chunks::<4>().0.iter().map(|p| p[3]).collect();
            assert!(
                alphas.contains(&0),
                "Expected transparent background pixels"
            );
            assert!(
                alphas.iter().any(|&a| a > 0),
                "Expected rasterized glyph pixels"
            );
        } else {
            panic!("Expected flattened Bitmap");
        }
    }

    #[test]
    fn smart_snapping_aligns_object_to_canvas_center_and_guides() {
        let app = FireCraft::new(None);
        assert!(app.snap_to_guides);
        assert!(app.snap_to_objects);
        let pw = app.doc.page().width;
        let ph = app.doc.page().height;
        let snapped = app.snap_pos(Vec2::new(pw * 0.5 + 3.0, ph * 0.5 - 2.0));
        assert!((snapped.x - pw * 0.5).abs() < 1e-3);
        assert!((snapped.y - ph * 0.5).abs() < 1e-3);
    }

    #[test]
    fn find_replace_color_slots_and_live_effects() {
        let mut app = FireCraft::new(None);
        let from = Color32::from_rgb(20, 40, 60);
        let to = Color32::from_rgb(200, 120, 40);
        let mut obj = Obj::new(
            Shape::Rect {
                min: Vec2::new(10.0, 10.0),
                max: Vec2::new(80.0, 80.0),
                corner_radius: 0.0,
                filled: true,
            },
            Some(from),
            Some(from),
        );
        obj.gradient = Some(GradientFill {
            start: from,
            end: from,
        });
        obj.effects.push(LiveEffect::DropShadow {
            distance: 4.0,
            angle_deg: 45.0,
            blur: 3.0,
            color: from,
            enabled: true,
        });
        app.push_object(obj);
        app.select_single(Some(0));
        app.find_replace_mode = FindReplaceMode::AnyColor;
        app.find_color = from;
        app.replace_color = to;
        assert_eq!(app.find_replace_match_count(), 1);
        app.run_find_and_replace();
        let replaced = app.selected_object().unwrap();
        assert_eq!(replaced.fill, Some(to));
        assert_eq!(replaced.stroke, Some(to));
        let gradient = replaced.gradient.unwrap();
        assert_eq!(gradient.start, to);
        assert_eq!(gradient.end, to);
        match &replaced.effects[0] {
            LiveEffect::DropShadow { color, .. } => assert_eq!(*color, to),
            other => panic!("unexpected effect {other:?}"),
        }
        assert!(app.doc.dirty);
    }

    #[test]
    fn find_replace_text_matches_names_and_hidden_text_objects() {
        let mut app = FireCraft::new(None);
        let mut hidden = Obj::new(
            Shape::Text {
                at: Vec2::new(20.0, 40.0),
                text: "brand headline".to_string(),
                size: 24.0,
                bold: false,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(Color32::BLACK),
            None,
        );
        hidden.name = "Brand Layer Asset".to_string();
        hidden.visible = false;
        app.push_object(hidden);
        app.select_single(Some(0));
        app.find_replace_mode = FindReplaceMode::Text;
        app.find_query = "brand".to_string();
        app.replace_query = "Studio".to_string();
        app.find_case_sensitive = false;
        assert_eq!(app.find_replace_match_count(), 1);
        app.run_find_and_replace();
        let obj = app.selected_object().unwrap();
        assert_eq!(obj.name, "Studio Layer Asset");
        if let Shape::Text { text, .. } = &obj.kind {
            assert_eq!(text, "Studio headline");
        } else {
            panic!("expected text object");
        }
    }

    #[test]
    fn join_and_split_paths_preserve_contiguous_geometry() {
        let mut app = FireCraft::new(None);
        app.push_object(Obj::new(
            Shape::Path {
                points: vec![Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0)],
                closed: false,
            },
            None,
            Some(Color32::BLACK),
        ));
        app.push_object(Obj::new(
            Shape::Path {
                points: vec![Vec2::new(20.0, 0.0), Vec2::new(30.0, 0.0)],
                closed: false,
            },
            None,
            Some(Color32::BLACK),
        ));
        app.select_single(Some(0));
        app.selected_set = vec![0, 1];
        app.combine_paths_op("join");
        assert_eq!(app.doc.total_objects_on_page(), 1);
        let joined = app.selected_object().unwrap();
        if let Shape::Path { points, closed } = joined.kind {
            assert!(!closed);
            assert_eq!(points.len(), 4);
            assert!((points[1] - Vec2::new(10.0, 0.0)).length() < 1e-3);
            assert!((points[2] - Vec2::new(20.0, 0.0)).length() < 1e-3);
        } else {
            panic!("expected joined path");
        }

        app.doc.page_mut().layers[0].objects[0].kind = Shape::Path {
            points: (0..6).map(|i| Vec2::new(i as f32 * 10.0, 0.0)).collect(),
            closed: false,
        };
        app.select_single(Some(0));
        app.split_selected_path();
        assert_eq!(
            app.doc.total_objects_on_page(),
            2,
            "split should create a second contour"
        );
    }
}
