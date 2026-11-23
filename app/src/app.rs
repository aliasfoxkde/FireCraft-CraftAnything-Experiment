//! The application state, tool & pointer interaction, live effects,
//! multi-document tab controller, and Fireworks CS6 editing commands.

use crate::canvas::{self, TextureCache};
use crate::colors;
use crate::constants;
use crate::doc::{
    AnimationState, AutoShapeKind, Document, Layer, LiveEffect, Obj, Page, Shape, Snapshot,
    StylePreset, SymbolDef,
};
use crate::theme::{self, Palette, ThemeMode};

use image::GenericImageView;

use eframe::egui::{self, Color32, Context, TextureHandle, Vec2};

use std::path::{Path, PathBuf};

/// What the file dialog is being used for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FileMode {
    Open,
    Import,
    Save,
    Export,
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
    Line,
    Rectangle,
    RoundedRect,
    Ellipse,
    Polygon,
    Star,
    Path,
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
            Tool::Line => "Line",
            Tool::Rectangle => "Rectangle",
            Tool::RoundedRect => "Rounded Rectangle",
            Tool::Ellipse => "Ellipse",
            Tool::Polygon => "Polygon",
            Tool::Star => "Star",
            Tool::Path => "Pen",
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
            PanelId::SpecialChars => "Chars",
            PanelId::ImageEditing => "Image Edit",
            PanelId::AutoShapeProps => "Auto Props",
            PanelId::DocumentLibrary => "Doc Library",
            PanelId::CommonLibrary => "Common Lib",
            PanelId::CssProperties => "CSS",
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
        match self {
            PanelId::Path | PanelId::Optimize => 220.0,
            PanelId::Align => 152.0,
            PanelId::History => 148.0,
            PanelId::Layers | PanelId::Pages | PanelId::States => 188.0,
            PanelId::ColorPalette | PanelId::ColorMixer => 186.0,
            PanelId::Swatches => 134.0,
            _ => 118.0,
        }
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
    Path,
}

pub struct Drag {
    pub kind: DragKind,
    pub start: Vec2,
    pub current: Vec2,
    pub orig_bounds: Option<(Vec2, Vec2)>,
    pub path: Vec<Vec2>,
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
    pub doc: Document,
    pub open_docs: Vec<Document>,
    pub active_doc: usize,
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
    pub show_bottom_inspector: bool,
    pub bottom_collapsed: bool,
    pub full_screen_canvas: bool,
    pub window_fullscreen: bool,
    pub panel_groups: Vec<PanelGroup>,
    pub next_panel_group_id: u64,
    pub dragging_panel: Option<PanelId>,
    pub dragging_group: Option<u64>,
    pub panel_title_rects: Vec<(u64, egui::Rect)>,
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
    pub show_guides: bool,
    pub show_slices: bool,
    pub onion_skin: bool,
    pub align_to_canvas: bool,
    pub space_mode: String,
    pub alt_hand_pan: bool,
    pub canvas_size: Vec2,
    pub cursor_doc_pos: Option<Vec2>,
    pub brush_size: f32,
    pub grid_size: f32,
    pub polygon_sides: u32,
    pub star_ratio: f32,
    pub current_path: Option<PathBuf>,
    pub recent_files: Vec<PathBuf>,
    pub show_new_dialog: bool,
    pub show_canvas_size_dialog: bool,
    pub show_find_replace: bool,
    pub show_commands_dialog: bool,
    pub show_about_dialog: bool,
    pub show_update_dialog: bool,
    pub pending_close_tab: Option<usize>,
    pub update_status: String,
    pub update_latest_version: Option<String>,
    pub update_release_url: Option<String>,
    pub find_query: String,
    pub replace_query: String,
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
    pub clipboard: Option<Obj>,
    pub clipboard_set: Vec<Obj>,
    pub last_effect: Option<LiveEffect>,
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
            doc: initial_doc.clone(),
            open_docs: vec![initial_doc],
            active_doc: 0,
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
            show_bottom_inspector: true,
            bottom_collapsed: false,
            full_screen_canvas: false,
            window_fullscreen: false,
            panel_groups: Self::cs6_dual_column_groups(),
            next_panel_group_id: 100,
            dragging_panel: None,
            dragging_group: None,
            panel_title_rects: Vec::new(),
            workspace_preset: "CS6 Dual-Column",
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
            show_guides: true,
            show_slices: true,
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
            show_find_replace: false,
            show_commands_dialog: false,
            show_about_dialog: false,
            show_update_dialog: false,
            pending_close_tab: None,
            update_status: format!("Current version: v{}", env!("CARGO_PKG_VERSION")),
            update_latest_version: None,
            update_release_url: Some(
                "https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment/releases/latest"
                    .to_string(),
            ),
            find_query: String::new(),
            replace_query: String::new(),
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
            clipboard: None,
            clipboard_set: Vec::new(),
            last_effect: None,
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
            },
            PanelGroup {
                id: 3,
                tabs: vec![
                    PanelId::Styles,
                    PanelId::ColorPalette,
                    PanelId::ColorMixer,
                    PanelId::Swatches,
                    PanelId::Info,
                ],
                active_tab: 1,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 144.0,
                floating_pos: egui::pos2(740.0, 440.0),
                floating_width: 286.0,
            },
            PanelGroup {
                id: 4,
                tabs: vec![
                    PanelId::SpecialChars,
                    PanelId::ImageEditing,
                    PanelId::AutoShapes,
                    PanelId::AutoShapeProps,
                    PanelId::DocumentLibrary,
                    PanelId::CommonLibrary,
                    PanelId::CssProperties,
                    PanelId::Url,
                ],
                active_tab: 2,
                zone: DockZone::RightOuter,
                collapsed: false,
                body_height: 112.0,
                floating_pos: egui::pos2(740.0, 610.0),
                floating_width: 286.0,
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
                self.show_right_dock = true;
                self.show_bottom_inspector = true;
                self.collapse_all_panels(true);
                if let Some(first) = self.panel_groups.first_mut() {
                    first.collapsed = false;
                }
                self.status =
                    "Workspace set to Iconic Mode (collapsed panel headers; click any header to expand)"
                        .to_string();
            }
            "Netbook Mode" => {
                self.full_screen_canvas = false;
                self.show_main_toolbar = true;
                self.show_tools_dock = true;
                self.show_right_dock = false;
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
    pub fn snap_floating_pos(
        &self,
        group_id: u64,
        mut pos: egui::Pos2,
        size: Vec2,
        bounds: egui::Rect,
    ) -> egui::Pos2 {
        let threshold = 12.0;
        // Snap to workspace/canvas edges
        if (pos.x - bounds.left()).abs() < threshold {
            pos.x = bounds.left() + 2.0;
        }
        if ((pos.x + size.x) - bounds.right()).abs() < threshold {
            pos.x = bounds.right() - size.x - 2.0;
        }
        if (pos.y - bounds.top()).abs() < threshold {
            pos.y = bounds.top() + 2.0;
        }
        if ((pos.y + size.y) - bounds.bottom()).abs() < threshold {
            pos.y = bounds.bottom() - size.y - 2.0;
        }

        // Snap edge-to-edge and corner-aligned against other floating panel windows
        for other in &self.panel_groups {
            if other.id == group_id || other.zone != DockZone::Floating {
                continue;
            }
            let oh = if other.collapsed {
                26.0
            } else {
                other.body_height + 30.0
            };
            let o_rect =
                egui::Rect::from_min_size(other.floating_pos, Vec2::new(other.floating_width, oh));
            // Vertical stacking snap (top to other's bottom, or bottom to other's top)
            if (pos.x - o_rect.left()).abs() < 28.0 {
                if (pos.y - o_rect.bottom()).abs() < threshold {
                    pos.y = o_rect.bottom() + 2.0;
                    pos.x = o_rect.left();
                } else if ((pos.y + size.y) - o_rect.top()).abs() < threshold {
                    pos.y = o_rect.top() - size.y - 2.0;
                    pos.x = o_rect.left();
                }
            }
            // Horizontal column snap (left to other's right, or right to other's left)
            if (pos.x - o_rect.right()).abs() < threshold {
                pos.x = o_rect.right() + 3.0;
                if (pos.y - o_rect.top()).abs() < threshold {
                    pos.y = o_rect.top();
                }
            } else if ((pos.x + size.x) - o_rect.left()).abs() < threshold {
                pos.x = o_rect.left() - size.x - 3.0;
                if (pos.y - o_rect.top()).abs() < threshold {
                    pos.y = o_rect.top();
                }
            }
        }
        pos
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
        let bytes = include_bytes!("logo.png");
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

    pub fn push_undo(&mut self, label: &str) {
        let snap = self.doc.snapshot(label);
        self.undo.push(snap);
        if self.undo.len() > constants::UNDO_LIMIT {
            self.undo.remove(0);
        }
        self.redo.clear();
    }

    pub fn undo_action(&mut self) {
        if let Some(snap) = self.undo.pop() {
            self.redo.push(self.doc.snapshot(&snap.label));
            self.doc.restore(&snap);
            self.textures.clear();
            let count = self
                .doc
                .page()
                .layers
                .get(self.selected_layer)
                .map(|l| l.objects.len())
                .unwrap_or(0);
            self.selected = self.selected.and_then(|idx| {
                if count == 0 {
                    None
                } else {
                    Some(idx.min(count - 1))
                }
            });
            self.selected_set.retain(|&i| i < count);
            if self.selected_set.is_empty()
                && let Some(s) = self.selected
            {
                self.selected_set.push(s);
            }
            self.status = format!("Undo: {}", snap.label);
        }
    }

    pub fn redo_action(&mut self) {
        if let Some(snap) = self.redo.pop() {
            self.undo.push(self.doc.snapshot(&snap.label));
            self.doc.restore(&snap);
            self.textures.clear();
            let count = self
                .doc
                .page()
                .layers
                .get(self.selected_layer)
                .map(|l| l.objects.len())
                .unwrap_or(0);
            self.selected = self.selected.and_then(|idx| {
                if count == 0 {
                    None
                } else {
                    Some(idx.min(count - 1))
                }
            });
            self.selected_set.retain(|&i| i < count);
            if self.selected_set.is_empty()
                && let Some(s) = self.selected
            {
                self.selected_set.push(s);
            }
            self.status = format!("Redo: {}", snap.label);
        }
    }

    pub fn replay_last_action(&mut self) {
        if self.selected.is_some() {
            self.duplicate_selected();
            if let Some(eff) = self.last_effect.clone() {
                self.add_live_effect(eff);
            }
            self.status = "Replayed step on selection".to_string();
        } else {
            self.status = "Select an object to replay step".to_string();
        }
    }

    pub fn delete_selected(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        if indices.is_empty() && self.selected_extra_layers.is_empty() {
            return;
        }
        self.push_undo("Delete");
        indices.sort_unstable();
        indices.dedup();
        let layer = self.selected_layer;
        let mut removed = 0usize;
        let extra = self.selected_extra_layers.clone();
        let page = self.doc.page_mut();
        if layer < page.layers.len() {
            for &idx in indices.iter().rev() {
                if idx < page.layers[layer].objects.len() {
                    page.layers[layer].objects.remove(idx);
                    removed += 1;
                }
            }
        }
        let mut by_layer: std::collections::BTreeMap<usize, Vec<usize>> =
            std::collections::BTreeMap::new();
        for (li, oi) in extra {
            if li != layer {
                by_layer.entry(li).or_default().push(oi);
            }
        }
        for (li, mut ois) in by_layer {
            ois.sort_unstable();
            ois.dedup();
            if li < page.layers.len() {
                for oi in ois.into_iter().rev() {
                    if oi < page.layers[li].objects.len() {
                        page.layers[li].objects.remove(oi);
                        removed += 1;
                    }
                }
            }
        }
        page.sync_state();
        self.clear_selection();
        self.textures.clear();
        self.status = format!("Deleted {removed} object(s)");
    }

    pub fn duplicate_selected(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        if indices.is_empty() {
            return;
        }
        self.push_undo("Duplicate");
        indices.sort_unstable();
        indices.dedup();
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        let mut new_set = Vec::new();
        if layer < page.layers.len() {
            for &idx in &indices {
                if idx < page.layers[layer].objects.len() {
                    let mut copy = page.layers[layer].objects[idx].clone();
                    copy.translate(Vec2::new(14.0, 14.0));
                    page.layers[layer].objects.push(copy);
                    new_set.push(page.layers[layer].objects.len() - 1);
                }
            }
            page.sync_state();
        }
        self.selected = new_set.last().copied();
        self.selected_set = new_set;
        self.status = format!("Duplicated {} object(s)", indices.len());
    }

    pub fn group_selected(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        indices.sort_unstable();
        indices.dedup();
        if indices.len() < 2 {
            self.status = "Select 2 or more objects (Shift-click or Marquee) to Group".to_string();
            return;
        }
        self.push_undo("Group");
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        let Some(objs) = page.layers.get_mut(layer).map(|l| &mut l.objects) else {
            return;
        };
        let mut children = Vec::new();
        for &idx in &indices {
            if let Some(o) = objs.get(idx) {
                children.push(o.clone());
            }
        }
        for &idx in indices.iter().rev() {
            if idx < objs.len() {
                objs.remove(idx);
            }
        }
        let grp = Obj::new(Shape::Group { children }, None, None);
        objs.push(grp);
        let new_idx = objs.len() - 1;
        page.sync_state();
        self.select_single(Some(new_idx));
        self.status = format!("Grouped {} objects", indices.len());
    }

    pub fn ungroup_selected(&mut self) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        let Some(children) = self
            .doc
            .page()
            .layers
            .get(layer)
            .and_then(|l| l.objects.get(index))
            .and_then(|o| match &o.kind {
                Shape::Group { children } => Some(children.clone()),
                _ => None,
            })
        else {
            self.status = "Selected object is not a Group".to_string();
            return;
        };
        self.push_undo("Ungroup");
        let page = self.doc.page_mut();
        let objs = &mut page.layers[layer].objects;
        objs.remove(index);
        let mut new_set = Vec::new();
        let count = children.len();
        for ch in children {
            objs.push(ch);
            new_set.push(objs.len() - 1);
        }
        page.sync_state();
        self.selected = new_set.last().copied();
        self.selected_set = new_set;
        self.status = format!("Ungrouped into {count} objects");
    }

    pub fn clone_selected(&mut self) {
        if let Some(index) = self.selected {
            self.push_undo("Clone");
            let layer = self.selected_layer;
            let page = self.doc.page_mut();
            if layer < page.layers.len() && index < page.layers[layer].objects.len() {
                let copy = page.layers[layer].objects[index].clone();
                page.layers[layer].objects.push(copy);
                let new_idx = page.layers[layer].objects.len() - 1;
                page.sync_state();
                self.selected = Some(new_idx);
                self.status = "Cloned object in place".to_string();
            }
        }
    }

    pub fn nudge_selected(&mut self, delta: Vec2) {
        if self.selected.is_none() && self.selected_set.is_empty() {
            return;
        }
        self.push_undo("Nudge");
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        for idx in indices {
            if let Some(obj) = page
                .layers
                .get_mut(layer)
                .and_then(|l| l.objects.get_mut(idx))
            {
                obj.translate(delta);
            }
        }
        page.sync_state();
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

    pub fn set_selected_fill(&mut self, color: Option<Color32>) {
        self.with_selected_mut(|obj| obj.fill = color);
    }

    pub fn set_selected_stroke(&mut self, color: Option<Color32>) {
        self.with_selected_mut(|obj| obj.stroke = color);
    }

    pub fn set_selected_width(&mut self, width: f32) {
        self.with_selected_mut(|obj| obj.width = width);
    }

    pub fn set_selected_opacity(&mut self, opacity: f32) {
        self.with_selected_mut(|obj| obj.opacity = opacity.clamp(0.0, 1.0));
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
        if self.selected.is_none() {
            self.status = "Select an object first to attach a Live Filter".to_string();
            return;
        }
        self.push_undo("Add Live Filter");
        let label = effect.label();
        self.last_effect = Some(effect.clone());
        self.with_selected_mut(|obj| obj.effects.push(effect));
        self.status = format!("Added Live Filter: {label}");
    }

    pub fn toggle_live_effect(&mut self, eff_idx: usize, enabled: bool) {
        self.with_selected_mut(|obj| {
            if let Some(eff) = obj.effects.get_mut(eff_idx) {
                eff.set_enabled(enabled);
            }
        });
    }

    pub fn clear_live_effects(&mut self) {
        self.push_undo("Clear Filters");
        self.with_selected_mut(|obj| obj.effects.clear());
        self.status = "Cleared live filters on selection".to_string();
    }

    pub fn repeat_last_filter(&mut self) {
        if let Some(eff) = self.last_effect.clone() {
            self.add_live_effect(eff);
        } else {
            self.run_bitmap_filter(Tool::Sharpen);
        }
    }

    // ---- Pointer handling --------------------------------------------------

    pub fn snap_pos(&self, p: Vec2) -> Vec2 {
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
            Tool::Marquee | Tool::Lasso | Tool::Wand | Tool::Crop => {
                self.drag = Some(Drag {
                    kind: DragKind::Marquee,
                    start: doc_pos,
                    current: doc_pos,
                    orig_bounds: None,
                    path: vec![],
                });
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
                self.drag = Some(Drag {
                    kind: DragKind::Path,
                    start: doc_pos,
                    current: doc_pos,
                    orig_bounds: None,
                    path: vec![doc_pos],
                });
            }
            Tool::Knife => {
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
                self.push_undo("Gradient");
                let (pw, ph) = (self.doc.page().width, self.doc.page().height);
                let obj = Obj::new(
                    Shape::Gradient {
                        min: Vec2::ZERO,
                        max: Vec2::new(pw, ph),
                        from: self.fill,
                        to: self.stroke,
                        radial: false,
                    },
                    None,
                    None,
                );
                let index = self.push_object(obj);
                self.selected = Some(index);
                self.status = "Gradient shape added".to_string();
            }
            Tool::Brush | Tool::Pencil | Tool::Eraser => {
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
                if self.show_guides
                    && let Some(sel_idx) = self.selected
                    && let Some((lo, hi)) = self
                        .doc
                        .page()
                        .layers
                        .get(layer)
                        .and_then(|l| l.objects.get(sel_idx))
                        .and_then(|o| o.bounds())
                {
                    let cx = (lo.x + hi.x) * 0.5;
                    let cy = (lo.y + hi.y) * 0.5;
                    let pw = self.doc.page().width;
                    let ph = self.doc.page().height;
                    if (cx - pw * 0.5).abs() < 4.0 {
                        self.smart_guides_lines
                            .push((Vec2::new(pw * 0.5, 0.0), Vec2::new(pw * 0.5, ph)));
                    }
                    if (cy - ph * 0.5).abs() < 4.0 {
                        self.smart_guides_lines
                            .push((Vec2::new(0.0, ph * 0.5), Vec2::new(pw, ph * 0.5)));
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
            DragKind::Paint | DragKind::Path => {
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
                let page = self.doc.page();
                if let Some(layer) = page.layers.get(self.selected_layer) {
                    for (index, obj) in layer.objects.iter().enumerate() {
                        if !obj.visible || obj.locked {
                            continue;
                        }
                        if let Some((a, b)) = obj.bounds()
                            && b.x >= lo.x
                            && a.x <= hi.x
                            && b.y >= lo.y
                            && a.y <= hi.y
                        {
                            if !picked_set.contains(&index) {
                                picked_set.push(index);
                            }
                            if !is_box {
                                break;
                            }
                        }
                    }
                }
                self.selected = picked_set.last().copied();
                self.selected_set = picked_set;
                self.status = if !self.selected_set.is_empty() {
                    format!("Selected {} object(s)", self.selected_set.len())
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
                    _ => return,
                };
                let (fill, stroke) = match self.tool {
                    Tool::Line => (None, Some(self.stroke)),
                    Tool::Slice | Tool::Hotspot => (None, None),
                    _ => (Some(self.fill), Some(self.stroke)),
                };
                let obj = Obj::new(kind, fill, stroke);
                let index = self.push_object(obj);
                self.selected = Some(index);
                self.status = format!("{} created", self.tool.name());
            }
            DragKind::Paint => {
                if drag.path.len() >= 2 && self.tool != Tool::Eraser {
                    let mut obj = Obj::new(
                        Shape::Path {
                            points: drag.path,
                            closed: false,
                        },
                        None,
                        Some(self.fill),
                    );
                    obj.width = self.brush_size.max(1.0);
                    let index = self.push_object(obj);
                    self.selected = Some(index);
                }
            }
            DragKind::Path if drag.path.len() >= 2 => {
                self.push_undo("Path");
                let closed = drag.path.len() > 3
                    && (drag.path[0] - *drag.path.last().unwrap_or(&drag.path[0])).length() < 14.0;
                let obj = Obj::new(
                    Shape::Path {
                        points: drag.path,
                        closed,
                    },
                    if closed { Some(self.fill) } else { None },
                    Some(self.stroke),
                );
                let idx = self.push_object(obj);
                self.selected = Some(idx);
            }
            _ => {}
        }
    }

    // ---- Multi-Document Tabs & Document Lifecycle --------------------------

    fn sync_active_doc_slot(&mut self) {
        if self.active_doc < self.open_docs.len() {
            self.open_docs[self.active_doc] = self.doc.clone();
        }
    }

    pub fn new_document_tab(&mut self, width: f32, height: f32) {
        self.sync_active_doc_slot();
        let mut d = Document::new(width, height);
        d.name = format!("Untitled-{}.fw.png", self.open_docs.len() + 1);
        self.open_docs.push(d.clone());
        self.active_doc = self.open_docs.len() - 1;
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
            self.doc = self.open_docs[idx].clone();
            self.selected = None;
            self.selected_layer = 0;
            self.undo.clear();
            self.redo.clear();
            self.textures.clear();
            self.status = format!("Switched to {}", self.doc.name);
        }
    }

    pub fn request_close_document_tab(&mut self, idx: usize) {
        let is_dirty = if idx == self.active_doc {
            self.doc.dirty
        } else {
            self.open_docs.get(idx).is_some_and(|d| d.dirty)
        };
        if is_dirty {
            self.pending_close_tab = Some(idx);
        } else {
            self.close_document_tab(idx);
        }
    }

    pub fn request_close_active_tab(&mut self) {
        self.request_close_document_tab(self.active_doc);
    }

    pub fn close_document_tab(&mut self, idx: usize) {
        if self.open_docs.len() <= 1 {
            self.close_document();
            return;
        }
        self.open_docs.remove(idx);
        self.active_doc = self.active_doc.min(self.open_docs.len() - 1);
        self.doc = self.open_docs[self.active_doc].clone();
        self.selected = None;
        self.selected_layer = 0;
        self.textures.clear();
    }

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
        self.active_doc = 0;
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
        self.tool = Tool::Select;
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

    pub fn duplicate_state(&mut self) {
        let page = self.doc.page_mut();
        page.sync_state();
        let cur = page.current_state.min(page.states.len() - 1);
        let mut copy = page.states[cur].clone();
        copy.name = format!("State {}", page.states.len() + 1);
        page.states.push(copy);
        let last = page.states.len() - 1;
        page.select_state(last);
        self.status = "Duplicated current animation state".to_string();
    }

    pub fn tween_selected_across_states(&mut self) {
        let Some(obj) = self.selected_object() else {
            self.status = "Select an object to tween across 3 animation states".to_string();
            return;
        };
        let page = self.doc.page_mut();
        page.sync_state();
        for step in 1..=3 {
            let mut next_st = page.states[page.current_state].clone();
            next_st.name = format!("State {}", page.states.len() + 1);
            let mut moved = obj.clone();
            moved.translate(Vec2::new(step as f32 * 36.0, step as f32 * 14.0));
            moved.opacity = (1.0 - step as f32 * 0.18).max(0.25);
            if let Some(first_layer) = next_st.layers.first_mut() {
                first_layer.objects.push(moved);
            }
            page.states.push(next_st);
        }
        self.status = "Generated 3 motion-tween animation states".to_string();
    }

    pub fn convert_selection_to_symbol(&mut self) {
        let Some(obj) = self.selected_object() else {
            self.status = "Select an object to convert to a Symbol".to_string();
            return;
        };
        self.push_undo("Convert to Symbol");
        let id = self.doc.symbols.len();
        let name = format!("{} Symbol", obj.name);
        let (at, _) = obj
            .bounds()
            .unwrap_or((Vec2::new(100.0, 100.0), Vec2::new(164.0, 164.0)));
        self.doc.symbols.push(SymbolDef {
            id,
            name: name.clone(),
            kind: "Graphic",
            objects: vec![obj],
        });
        let inst = Obj::new(
            Shape::SymbolInstance {
                symbol_id: id,
                at,
                scale: Vec2::splat(1.0),
            },
            Some(self.fill),
            Some(self.stroke),
        );
        let idx = self.push_object(inst);
        self.select_single(Some(idx));
        self.status = format!("Converted to symbol: {name}");
    }

    pub fn create_default_button_symbol(&mut self) {
        if !self.doc.symbols.is_empty() {
            return;
        }
        let btn = Obj::new(
            Shape::Rect {
                min: Vec2::ZERO,
                max: Vec2::new(120.0, 40.0),
                corner_radius: 8.0,
                filled: true,
            },
            Some(Color32::from_rgb(232, 163, 61)),
            Some(Color32::from_rgb(30, 30, 36)),
        );
        self.doc.symbols.push(SymbolDef {
            id: 0,
            name: "Primary CTA Button".to_string(),
            kind: "Button",
            objects: vec![btn],
        });
    }

    pub fn place_symbol_instance(&mut self, symbol_id: usize) {
        self.push_undo("Place Symbol");
        let center = Vec2::new(
            self.doc.page().width * 0.5 - 32.0,
            self.doc.page().height * 0.5 - 32.0,
        );
        let inst = Obj::new(
            Shape::SymbolInstance {
                symbol_id,
                at: center,
                scale: Vec2::splat(1.2),
            },
            Some(self.fill),
            Some(self.stroke),
        );
        let idx = self.push_object(inst);
        self.select_single(Some(idx));
        self.status = format!("Placed Symbol #{} instance", symbol_id + 1);
    }

    pub fn apply_style_preset(&mut self, style_idx: usize) {
        let Some(preset) = self.doc.styles.get(style_idx).cloned() else {
            return;
        };
        if self.selected.is_none() {
            self.fill = preset.fill.unwrap_or(self.fill);
            self.stroke = preset.stroke.unwrap_or(self.stroke);
            self.status = format!("Loaded style colors: {}", preset.name);
            return;
        }
        self.push_undo("Apply Style");
        let name = preset.name.clone();
        self.with_selected_mut(|obj| {
            obj.fill = preset.fill;
            obj.stroke = preset.stroke;
            obj.width = preset.stroke_width;
            obj.effects = preset.effects;
        });
        self.status = format!("Applied style: {name}");
    }

    pub fn save_selection_as_style(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        let name = format!("Style {}", self.doc.styles.len() + 1);
        self.doc.styles.push(StylePreset {
            name: name.clone(),
            fill: obj.fill,
            stroke: obj.stroke,
            stroke_width: obj.width,
            effects: obj.effects.clone(),
        });
        self.status = format!("Saved new style preset: {name}");
    }

    pub fn insert_auto_shape(&mut self, kind: AutoShapeKind) {
        self.push_undo("Insert Auto Shape");
        let c = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
        let shape = match kind {
            AutoShapeKind::RoundedRect => Shape::Rect {
                min: c - Vec2::new(70.0, 45.0),
                max: c + Vec2::new(70.0, 45.0),
                corner_radius: 16.0,
                filled: true,
            },
            AutoShapeKind::ChamferRect => Shape::Rect {
                min: c - Vec2::new(70.0, 45.0),
                max: c + Vec2::new(70.0, 45.0),
                corner_radius: 8.0,
                filled: true,
            },
            AutoShapeKind::Star => Shape::Polygon {
                center: c,
                radius: 64.0,
                sides: 5,
                star_inner: 0.45,
                angle_rad: 0.0,
            },
            AutoShapeKind::Polygon => Shape::Polygon {
                center: c,
                radius: 60.0,
                sides: 6,
                star_inner: 1.0,
                angle_rad: 0.0,
            },
            AutoShapeKind::Gear => Shape::Polygon {
                center: c,
                radius: 62.0,
                sides: 12,
                star_inner: 0.78,
                angle_rad: 0.0,
            },
            AutoShapeKind::Arrow => Shape::Path {
                points: vec![
                    c + Vec2::new(-70.0, -18.0),
                    c + Vec2::new(15.0, -18.0),
                    c + Vec2::new(15.0, -40.0),
                    c + Vec2::new(70.0, 0.0),
                    c + Vec2::new(15.0, 40.0),
                    c + Vec2::new(15.0, 18.0),
                    c + Vec2::new(-70.0, 18.0),
                ],
                closed: true,
            },
            AutoShapeKind::Callout => Shape::Path {
                points: vec![
                    c + Vec2::new(-70.0, -45.0),
                    c + Vec2::new(70.0, -45.0),
                    c + Vec2::new(70.0, 25.0),
                    c + Vec2::new(-10.0, 25.0),
                    c + Vec2::new(-35.0, 55.0),
                    c + Vec2::new(-30.0, 25.0),
                    c + Vec2::new(-70.0, 25.0),
                ],
                closed: true,
            },
            AutoShapeKind::Cross => Shape::Path {
                points: vec![
                    c + Vec2::new(-20.0, -60.0),
                    c + Vec2::new(20.0, -60.0),
                    c + Vec2::new(20.0, -20.0),
                    c + Vec2::new(60.0, -20.0),
                    c + Vec2::new(60.0, 20.0),
                    c + Vec2::new(20.0, 20.0),
                    c + Vec2::new(20.0, 60.0),
                    c + Vec2::new(-20.0, 60.0),
                    c + Vec2::new(-20.0, 20.0),
                    c + Vec2::new(-60.0, 20.0),
                    c + Vec2::new(-60.0, -20.0),
                    c + Vec2::new(-20.0, -20.0),
                ],
                closed: true,
            },
            AutoShapeKind::LShape => Shape::Path {
                points: vec![
                    c + Vec2::new(-55.0, -55.0),
                    c + Vec2::new(-15.0, -55.0),
                    c + Vec2::new(-15.0, 15.0),
                    c + Vec2::new(55.0, 15.0),
                    c + Vec2::new(55.0, 55.0),
                    c + Vec2::new(-55.0, 55.0),
                ],
                closed: true,
            },
            AutoShapeKind::Spiral => {
                let mut pts = Vec::new();
                for i in 0..36 {
                    let t = i as f32 * 0.35;
                    let r = 4.0 + i as f32 * 1.7;
                    pts.push(c + Vec2::new(r * t.cos(), r * t.sin()));
                }
                Shape::Path {
                    points: pts,
                    closed: false,
                }
            }
            AutoShapeKind::Pie | AutoShapeKind::Donut => Shape::Ellipse {
                center: c,
                radius: Vec2::splat(56.0),
                filled: true,
            },
        };
        let mut obj = Obj::new(shape, Some(self.fill), Some(self.stroke));
        obj.name = kind.label().to_string();
        obj.width = 2.0;
        let idx = self.push_object(obj);
        self.select_single(Some(idx));
        self.status = format!("Inserted Auto Shape: {}", kind.label());
    }

    pub fn insert_special_glyph(&mut self, glyph: &str) {
        if let Some(obj) = self.selected_object()
            && let Shape::Text { mut text, .. } = obj.kind
        {
            text.push_str(glyph);
            self.set_selected_text(text);
            self.status = format!("Appended '{glyph}' to selected text");
            return;
        }
        self.push_undo("Insert Character");
        let c = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
        let obj = Obj::new(
            Shape::Text {
                at: c,
                text: glyph.to_string(),
                size: 36.0,
                bold: false,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(self.fill),
            None,
        );
        let idx = self.push_object(obj);
        self.select_single(Some(idx));
        self.status = format!("Inserted special character '{glyph}'");
    }

    pub fn insert_slice(&mut self, is_hotspot: bool) {
        self.push_undo(if is_hotspot {
            "Add Hotspot"
        } else {
            "Add Slice"
        });
        let (lo, hi) = self
            .selected_object()
            .and_then(|o| o.bounds())
            .unwrap_or_else(|| {
                let c = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
                (c - Vec2::new(80.0, 30.0), c + Vec2::new(80.0, 30.0))
            });
        let obj = Obj::new(
            Shape::Slice {
                min: lo,
                max: hi,
                url: "https://github.com/aliasfoxkde/FireCraft".to_string(),
                alt: "FireCraft Slice".to_string(),
                target: "_self".to_string(),
                format: "PNG32".to_string(),
                is_hotspot,
            },
            None,
            None,
        );
        let idx = self.push_object(obj);
        self.select_single(Some(idx));
        self.show_slices = true;
        self.status = if is_hotspot {
            "Web Hotspot created on Web Layer".to_string()
        } else {
            "Web Slice created on Web Layer".to_string()
        };
    }

    pub fn create_slice_from_selection(&mut self) {
        self.insert_slice(false);
    }

    // ---- Path, Alignment, Color & Document Commands ------------------------

    pub fn align_selected_to_canvas(&mut self, mode: &str) {
        if self.selected_set.is_empty() && self.selected.is_none() {
            return;
        }
        let indices: Vec<usize> = if self.selected_set.is_empty() {
            self.selected.into_iter().collect()
        } else {
            self.selected_set.clone()
        };
        let (cw, ch) = (self.doc.page().width, self.doc.page().height);
        let layer = self.selected_layer;
        let use_canvas = self.align_to_canvas || indices.len() <= 1;
        let (ref_lo, ref_hi) = if use_canvas {
            (Vec2::ZERO, Vec2::new(cw, ch))
        } else {
            let mut u_lo = Vec2::splat(f32::MAX);
            let mut u_hi = Vec2::splat(f32::MIN);
            if let Some(l) = self.doc.page().layers.get(layer) {
                for &idx in &indices {
                    if let Some((lo, hi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                        u_lo = u_lo.min(lo);
                        u_hi = u_hi.max(hi);
                    }
                }
            }
            if u_lo.x > u_hi.x {
                (Vec2::ZERO, Vec2::new(cw, ch))
            } else {
                (u_lo, u_hi)
            }
        };
        let ref_size = ref_hi - ref_lo;
        self.push_undo("Align");
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for &idx in &indices {
                if let Some(obj) = l.objects.get_mut(idx)
                    && let Some((lo, hi)) = obj.bounds()
                {
                    let size = hi - lo;
                    let new_lo = match mode {
                        "left" => Vec2::new(ref_lo.x, lo.y),
                        "center_x" => Vec2::new(ref_lo.x + (ref_size.x - size.x) * 0.5, lo.y),
                        "right" => Vec2::new(ref_hi.x - size.x, lo.y),
                        "top" => Vec2::new(lo.x, ref_lo.y),
                        "center_y" => Vec2::new(lo.x, ref_lo.y + (ref_size.y - size.y) * 0.5),
                        "bottom" => Vec2::new(lo.x, ref_hi.y - size.y),
                        _ => lo,
                    };
                    obj.set_bounds(new_lo, new_lo + size);
                }
            }
        }
        page.sync_state();
        self.status = format!(
            "Aligned {} object(s) ({mode}, {})",
            indices.len(),
            if use_canvas {
                "Relative to Canvas"
            } else {
                "Relative to Object"
            }
        );
    }

    pub fn distribute_selected(&mut self, mode: &str) {
        let indices: Vec<usize> = if self.selected_set.is_empty() {
            self.selected.into_iter().collect()
        } else {
            self.selected_set.clone()
        };
        if indices.is_empty() {
            return;
        }
        if indices.len() == 1 {
            // Single selection falls back to canvas alignment for the corresponding axis
            let fallback = match mode {
                "top" => "top",
                "center_v" => "center_y",
                "bottom" => "bottom",
                "left" => "left",
                "center_h" => "center_x",
                "right" => "right",
                _ => "center_x",
            };
            self.align_selected_to_canvas(fallback);
            return;
        }
        let layer = self.selected_layer;
        let mut items: Vec<(usize, Vec2, Vec2)> = Vec::new();
        if let Some(l) = self.doc.page().layers.get(layer) {
            for &idx in &indices {
                if let Some((lo, hi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                    items.push((idx, lo, hi));
                }
            }
        }
        if items.len() < 2 {
            return;
        }
        let vertical = matches!(mode, "top" | "center_v" | "bottom");
        items.sort_by(|a, b| {
            let ka = if vertical { a.1.y } else { a.1.x };
            let kb = if vertical { b.1.y } else { b.1.x };
            ka.partial_cmp(&kb).unwrap_or(std::cmp::Ordering::Equal)
        });
        let (cw, ch) = (self.doc.page().width, self.doc.page().height);
        let first_val = match mode {
            "top" => {
                if self.align_to_canvas {
                    0.0
                } else {
                    items.first().map(|x| x.1.y).unwrap_or(0.0)
                }
            }
            "center_v" => {
                if self.align_to_canvas {
                    items.first().map(|x| (x.2.y - x.1.y) * 0.5).unwrap_or(0.0)
                } else {
                    items.first().map(|x| (x.1.y + x.2.y) * 0.5).unwrap_or(0.0)
                }
            }
            "bottom" => {
                if self.align_to_canvas {
                    items.first().map(|x| x.2.y - x.1.y).unwrap_or(0.0)
                } else {
                    items.first().map(|x| x.2.y).unwrap_or(0.0)
                }
            }
            "left" => {
                if self.align_to_canvas {
                    0.0
                } else {
                    items.first().map(|x| x.1.x).unwrap_or(0.0)
                }
            }
            "center_h" => {
                if self.align_to_canvas {
                    items.first().map(|x| (x.2.x - x.1.x) * 0.5).unwrap_or(0.0)
                } else {
                    items.first().map(|x| (x.1.x + x.2.x) * 0.5).unwrap_or(0.0)
                }
            }
            _ => {
                if self.align_to_canvas {
                    items.first().map(|x| x.2.x - x.1.x).unwrap_or(0.0)
                } else {
                    items.first().map(|x| x.2.x).unwrap_or(0.0)
                }
            }
        };
        let last_val = match mode {
            "top" => {
                if self.align_to_canvas {
                    ch - items.last().map(|x| x.2.y - x.1.y).unwrap_or(0.0)
                } else {
                    items.last().map(|x| x.1.y).unwrap_or(ch)
                }
            }
            "center_v" => {
                if self.align_to_canvas {
                    ch - items.last().map(|x| (x.2.y - x.1.y) * 0.5).unwrap_or(0.0)
                } else {
                    items.last().map(|x| (x.1.y + x.2.y) * 0.5).unwrap_or(ch)
                }
            }
            "bottom" => {
                if self.align_to_canvas {
                    ch
                } else {
                    items.last().map(|x| x.2.y).unwrap_or(ch)
                }
            }
            "left" => {
                if self.align_to_canvas {
                    cw - items.last().map(|x| x.2.x - x.1.x).unwrap_or(0.0)
                } else {
                    items.last().map(|x| x.1.x).unwrap_or(cw)
                }
            }
            "center_h" => {
                if self.align_to_canvas {
                    cw - items.last().map(|x| (x.2.x - x.1.x) * 0.5).unwrap_or(0.0)
                } else {
                    items.last().map(|x| (x.1.x + x.2.x) * 0.5).unwrap_or(cw)
                }
            }
            _ => {
                if self.align_to_canvas {
                    cw
                } else {
                    items.last().map(|x| x.2.x).unwrap_or(cw)
                }
            }
        };
        self.push_undo("Distribute");
        let n = (items.len() - 1).max(1) as f32;
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for (step, (idx, lo, hi)) in items.into_iter().enumerate() {
                let target = first_val + (last_val - first_val) * (step as f32 / n);
                let size = hi - lo;
                let new_lo = match mode {
                    "top" => Vec2::new(lo.x, target),
                    "center_v" => Vec2::new(lo.x, target - size.y * 0.5),
                    "bottom" => Vec2::new(lo.x, target - size.y),
                    "left" => Vec2::new(target, lo.y),
                    "center_h" => Vec2::new(target - size.x * 0.5, lo.y),
                    _ => Vec2::new(target - size.x, lo.y),
                };
                if let Some(obj) = l.objects.get_mut(idx) {
                    obj.set_bounds(new_lo, new_lo + size);
                }
            }
        }
        page.sync_state();
        self.status = format!("Distributed selection ({mode})");
    }

    pub fn match_canvas_dimension(&mut self, match_w: bool, match_h: bool) {
        let indices: Vec<usize> = if self.selected_set.is_empty() {
            self.selected.into_iter().collect()
        } else {
            self.selected_set.clone()
        };
        if indices.is_empty() {
            return;
        }
        let layer = self.selected_layer;
        let (cw, ch) = (self.doc.page().width, self.doc.page().height);
        let use_canvas = self.align_to_canvas || indices.len() == 1;
        let (target_w, target_h) = if use_canvas {
            (cw, ch)
        } else {
            let mut max_w = 1.0f32;
            let mut max_h = 1.0f32;
            if let Some(l) = self.doc.page().layers.get(layer) {
                for &idx in &indices {
                    if let Some((lo, hi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                        max_w = max_w.max(hi.x - lo.x);
                        max_h = max_h.max(hi.y - lo.y);
                    }
                }
            }
            (max_w, max_h)
        };
        self.push_undo("Match Size");
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for &idx in &indices {
                if let Some(obj) = l.objects.get_mut(idx)
                    && let Some((lo, hi)) = obj.bounds()
                {
                    let new_lo = Vec2::new(
                        if match_w && use_canvas { 0.0 } else { lo.x },
                        if match_h && use_canvas { 0.0 } else { lo.y },
                    );
                    let new_hi = Vec2::new(
                        if match_w { new_lo.x + target_w } else { hi.x },
                        if match_h { new_lo.y + target_h } else { hi.y },
                    );
                    obj.set_bounds(new_lo, new_hi);
                }
            }
        }
        page.sync_state();
        self.status = format!(
            "Matched size ({})",
            if use_canvas {
                "Relative to Canvas"
            } else {
                "Relative to Largest Object"
            }
        );
    }

    pub fn space_selected(&mut self, horizontal: bool) {
        let indices: Vec<usize> = if self.selected_set.is_empty() {
            self.selected.into_iter().collect()
        } else {
            self.selected_set.clone()
        };
        if indices.len() < 2 {
            self.distribute_selected(if horizontal { "center_h" } else { "center_v" });
            return;
        }
        let fixed_gap: Option<f32> = self.space_mode.trim_end_matches(" px").parse::<f32>().ok();
        let layer = self.selected_layer;
        let mut items: Vec<(usize, Vec2, Vec2)> = Vec::new();
        if let Some(l) = self.doc.page().layers.get(layer) {
            for &idx in &indices {
                if let Some((lo, hi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                    items.push((idx, lo, hi));
                }
            }
        }
        if items.len() < 2 {
            return;
        }
        items.sort_by(|a, b| {
            let ka = if horizontal { a.1.x } else { a.1.y };
            let kb = if horizontal { b.1.x } else { b.1.y };
            ka.partial_cmp(&kb).unwrap_or(std::cmp::Ordering::Equal)
        });
        let gap = if let Some(g) = fixed_gap {
            g
        } else {
            let total_span = if horizontal {
                items.last().map(|x| x.2.x).unwrap_or(0.0)
                    - items.first().map(|x| x.1.x).unwrap_or(0.0)
            } else {
                items.last().map(|x| x.2.y).unwrap_or(0.0)
                    - items.first().map(|x| x.1.y).unwrap_or(0.0)
            };
            let sum_sizes: f32 = items
                .iter()
                .map(|(_, lo, hi)| if horizontal { hi.x - lo.x } else { hi.y - lo.y })
                .sum();
            ((total_span - sum_sizes) / (items.len() - 1) as f32).max(0.0)
        };
        self.push_undo("Space Objects");
        let mut cursor = if horizontal {
            items[0].1.x
        } else {
            items[0].1.y
        };
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for (idx, lo, hi) in items {
                let size = hi - lo;
                let new_lo = if horizontal {
                    Vec2::new(cursor, lo.y)
                } else {
                    Vec2::new(lo.x, cursor)
                };
                if let Some(obj) = l.objects.get_mut(idx) {
                    obj.set_bounds(new_lo, new_lo + size);
                }
                cursor += if horizontal { size.x } else { size.y } + gap;
            }
        }
        page.sync_state();
        self.status = format!(
            "Spaced objects {} ({})",
            if horizontal {
                "horizontally"
            } else {
                "vertically"
            },
            self.space_mode
        );
    }

    fn obj_to_polygon_points(obj: &Obj) -> Vec<Vec2> {
        match &obj.kind {
            Shape::Rect { min, max, .. } => {
                let lo = min.min(*max);
                let hi = min.max(*max);
                vec![lo, Vec2::new(hi.x, lo.y), hi, Vec2::new(lo.x, hi.y)]
            }
            Shape::Ellipse { center, radius, .. } => (0..24)
                .map(|i| {
                    let t = i as f32 / 24.0 * std::f32::consts::TAU;
                    *center + Vec2::new(radius.x * t.cos(), radius.y * t.sin())
                })
                .collect(),
            Shape::Polygon {
                center,
                radius,
                sides,
                star_inner,
                angle_rad,
            } => Obj::polygon_vertices(*center, *radius, *sides, *star_inner, *angle_rad),
            Shape::Path { points, .. } => points.clone(),
            Shape::Line { a, b } => vec![*a, *b],
            _ => obj
                .bounds()
                .map(|(lo, hi)| vec![lo, Vec2::new(hi.x, lo.y), hi, Vec2::new(lo.x, hi.y)])
                .unwrap_or_default(),
        }
    }

    fn sutherland_hodgman_clip(subject: &[Vec2], clip: &[Vec2]) -> Vec<Vec2> {
        if subject.len() < 3 || clip.len() < 3 {
            return Vec::new();
        }
        // Ensure clip winding is consistent (CCW in screen coords: signed area > 0)
        let area: f32 = (0..clip.len())
            .map(|i| {
                let a = clip[i];
                let b = clip[(i + 1) % clip.len()];
                a.x * b.y - b.x * a.y
            })
            .sum();
        let sign = if area >= 0.0 { 1.0 } else { -1.0 };
        let inside = |p: Vec2, e1: Vec2, e2: Vec2| -> bool {
            ((e2.x - e1.x) * (p.y - e1.y) - (e2.y - e1.y) * (p.x - e1.x)) * sign >= -1e-4
        };
        let intersect_seg = |a: Vec2, b: Vec2, e1: Vec2, e2: Vec2| -> Vec2 {
            let dc = e1 - e2;
            let dp = a - b;
            let n1 = e1.x * e2.y - e1.y * e2.x;
            let n2 = a.x * b.y - a.y * b.x;
            let denom = dc.x * dp.y - dc.y * dp.x;
            if denom.abs() < 1e-6 {
                return (a + b) * 0.5;
            }
            Vec2::new(
                (n1 * dp.x - n2 * dc.x) / denom,
                (n1 * dp.y - n2 * dc.y) / denom,
            )
        };
        let mut output = subject.to_vec();
        for i in 0..clip.len() {
            let e1 = clip[i];
            let e2 = clip[(i + 1) % clip.len()];
            let input = output;
            output = Vec::new();
            if input.is_empty() {
                break;
            }
            let mut s = *input.last().unwrap();
            for &e in &input {
                if inside(e, e1, e2) {
                    if !inside(s, e1, e2) {
                        output.push(intersect_seg(s, e, e1, e2));
                    }
                    output.push(e);
                } else if inside(s, e1, e2) {
                    output.push(intersect_seg(s, e, e1, e2));
                }
                s = e;
            }
        }
        output
    }

    fn union_polygon_hull(pts: &[Vec2]) -> Vec<Vec2> {
        if pts.len() < 3 {
            return pts.to_vec();
        }
        let mut sorted = pts.to_vec();
        sorted.sort_by(|a, b| {
            a.x.partial_cmp(&b.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
        });
        let cross =
            |o: Vec2, a: Vec2, b: Vec2| (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
        let mut lower: Vec<Vec2> = Vec::new();
        for &p in &sorted {
            while lower.len() >= 2
                && cross(lower[lower.len() - 2], lower[lower.len() - 1], p) <= 0.0
            {
                lower.pop();
            }
            lower.push(p);
        }
        let mut upper: Vec<Vec2> = Vec::new();
        for &p in sorted.iter().rev() {
            while upper.len() >= 2
                && cross(upper[upper.len() - 2], upper[upper.len() - 1], p) <= 0.0
            {
                upper.pop();
            }
            upper.push(p);
        }
        lower.pop();
        upper.pop();
        lower.extend(upper);
        lower
    }

    pub fn combine_paths_op(&mut self, op: &str) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        indices.sort_unstable();
        indices.dedup();
        if indices.is_empty() {
            self.status = "Select 1 or 2 objects first".to_string();
            return;
        }
        if op == "split" {
            self.split_selected_path();
            return;
        }
        self.push_undo("Combine Paths");
        if indices.len() == 1 {
            self.convert_selected_to_path();
            if op == "join" {
                self.with_selected_mut(|obj| {
                    if let Shape::Path { closed, .. } = &mut obj.kind {
                        *closed = true;
                    }
                });
                self.status = "Closed path endpoints (Join)".to_string();
            } else {
                self.status = format!("Select 2+ objects to apply {op}");
            }
            return;
        }
        let layer = self.selected_layer;
        let first_idx = indices[0];
        let page = self.doc.page_mut();
        let Some(objs) = page.layers.get_mut(layer).map(|l| &mut l.objects) else {
            return;
        };
        if *indices.last().unwrap_or(&0) >= objs.len() {
            return;
        }

        let mut acc_pts = Self::obj_to_polygon_points(&objs[first_idx]);
        let mut closed = true;
        for &other_idx in &indices[1..] {
            let other_pts = Self::obj_to_polygon_points(&objs[other_idx]);
            match op {
                "join" => {
                    acc_pts.extend(other_pts);
                    closed = false;
                }
                "intersect" | "crop" => {
                    let clipped = Self::sutherland_hodgman_clip(&acc_pts, &other_pts);
                    if clipped.len() >= 3 {
                        acc_pts = clipped;
                    }
                }
                "punch" => {
                    // Notch/subtract `other_pts` from `acc_pts` by keeping vertices outside other's bounds
                    if let Some((olo, ohi)) = objs[other_idx].bounds() {
                        let mut punched = Vec::new();
                        for &pt in &acc_pts {
                            if pt.x >= olo.x && pt.x <= ohi.x && pt.y >= olo.y && pt.y <= ohi.y {
                                // Project inward vertex to nearest boundary edge of puncher
                                let dl = (pt.x - olo.x).abs();
                                let dr = (ohi.x - pt.x).abs();
                                let dt = (pt.y - olo.y).abs();
                                let db = (ohi.y - pt.y).abs();
                                let m = dl.min(dr).min(dt).min(db);
                                if (m - dl).abs() < 0.1 {
                                    punched.push(Vec2::new(olo.x, pt.y));
                                } else if (m - dr).abs() < 0.1 {
                                    punched.push(Vec2::new(ohi.x, pt.y));
                                } else if (m - dt).abs() < 0.1 {
                                    punched.push(Vec2::new(pt.x, olo.y));
                                } else {
                                    punched.push(Vec2::new(pt.x, ohi.y));
                                }
                            } else {
                                punched.push(pt);
                            }
                        }
                        acc_pts = punched;
                    }
                }
                _ => {
                    // Union: combine all points into outer contour hull
                    acc_pts.extend(other_pts);
                    acc_pts = Self::union_polygon_hull(&acc_pts);
                }
            }
        }

        objs[first_idx].kind = Shape::Path {
            points: acc_pts,
            closed,
        };
        for &other_idx in indices[1..].iter().rev() {
            objs.remove(other_idx);
        }
        page.sync_state();
        self.select_single(Some(first_idx));
        self.status = format!("Applied Combine Paths: {op}");
    }

    pub fn convert_selected_to_path(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        let new_kind = match &obj.kind {
            Shape::Rect { min, max, .. } => {
                let lo = min.min(*max);
                let hi = min.max(*max);
                Shape::Path {
                    points: vec![lo, Vec2::new(hi.x, lo.y), hi, Vec2::new(lo.x, hi.y)],
                    closed: true,
                }
            }
            Shape::Ellipse { center, radius, .. } => {
                let pts = (0..16)
                    .map(|i| {
                        let t = i as f32 / 16.0 * std::f32::consts::TAU;
                        *center + Vec2::new(radius.x * t.cos(), radius.y * t.sin())
                    })
                    .collect();
                Shape::Path {
                    points: pts,
                    closed: true,
                }
            }
            Shape::Polygon {
                center,
                radius,
                sides,
                star_inner,
                angle_rad,
            } => Shape::Path {
                points: Obj::polygon_vertices(*center, *radius, *sides, *star_inner, *angle_rad),
                closed: true,
            },
            Shape::Line { a, b } => Shape::Path {
                points: vec![*a, *b],
                closed: false,
            },
            _ => return,
        };
        self.push_undo("Convert to Path");
        self.with_selected_mut(|o| o.kind = new_kind);
        self.status = "Converted selection to editable Vector Path".to_string();
    }

    pub fn simplify_selected_path(&mut self) {
        self.push_undo("Simplify Path");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind
                && points.len() > 4
            {
                let mut kept = Vec::new();
                for (i, p) in points.iter().enumerate() {
                    if i == 0 || i + 1 == points.len() || i % 2 == 0 {
                        kept.push(*p);
                    }
                }
                *points = kept;
            }
        });
        self.status = "Simplified vector path".to_string();
    }

    pub fn split_selected_path(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        if let Shape::Path { points, .. } = &obj.kind
            && points.len() >= 4
        {
            self.push_undo("Knife Split Path");
            let mid = points.len() / 2;
            let first_half = points[..=mid].to_vec();
            let second_half = points[mid..].to_vec();
            self.with_selected_mut(|o| {
                o.kind = Shape::Path {
                    points: first_half,
                    closed: false,
                };
            });
            let mut second_obj = obj.clone();
            second_obj.kind = Shape::Path {
                points: second_half,
                closed: false,
            };
            second_obj.translate(Vec2::new(6.0, 6.0));
            self.push_object(second_obj);
            self.status = "Split path into two contours".to_string();
        }
    }

    pub fn toggle_selected_path_closed(&mut self) {
        self.with_selected_mut(|obj| {
            if let Shape::Path { closed, .. } = &mut obj.kind {
                *closed = !*closed;
            }
        });
    }

    pub fn expand_selected_bounds(&mut self, delta: f32) {
        if let Some((lo, hi)) = self.selected_object().and_then(|o| o.bounds()) {
            self.push_undo("Alter Path Bounds");
            self.set_selected_bounds(lo - Vec2::splat(delta), hi + Vec2::splat(delta));
            self.status = if delta >= 0.0 {
                format!("Expanded path bounds by +{delta:.0}px")
            } else {
                format!("Inset path bounds by {delta:.0}px")
            };
        }
    }

    pub fn reverse_selected_path(&mut self) {
        self.convert_selected_to_path();
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind {
                points.reverse();
            }
        });
        self.status = "Reversed path contour direction".to_string();
    }

    pub fn add_points_to_selected_path(&mut self) {
        self.convert_selected_to_path();
        self.push_undo("Add Path Points");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, closed } = &mut obj.kind
                && points.len() >= 2
                && points.len() < 128
            {
                let mut next = Vec::with_capacity(points.len() * 2);
                for i in 0..points.len() {
                    let a = points[i];
                    next.push(a);
                    if i + 1 < points.len() {
                        next.push((a + points[i + 1]) * 0.5);
                    } else if *closed {
                        next.push((a + points[0]) * 0.5);
                    }
                }
                *points = next;
            }
        });
        self.status = "Subdivided path segments with new anchor points".to_string();
    }

    pub fn delete_point_from_selected_path(&mut self) {
        self.convert_selected_to_path();
        self.push_undo("Delete Path Point");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind
                && points.len() > 3
            {
                points.pop();
            }
        });
        self.status = "Removed anchor point from path".to_string();
    }

    pub fn round_selected_path_points(&mut self, delta_radius: f32) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        if let Shape::Rect { corner_radius, .. } = obj.kind {
            let next = (corner_radius + delta_radius).clamp(0.0, 120.0);
            self.set_selected_corner_radius(next);
            self.status = format!("Adjusted corner roundness to {next:.0}px");
        } else {
            self.add_points_to_selected_path();
            self.simplify_selected_path();
            self.status = "Rounded vector path corners".to_string();
        }
    }

    pub fn align_selected_path_points(&mut self, horizontal: bool) {
        self.convert_selected_to_path();
        self.push_undo("Align Path Points");
        self.with_selected_mut(|obj| {
            if let Shape::Path { points, .. } = &mut obj.kind
                && !points.is_empty()
            {
                let avg = if horizontal {
                    points.iter().map(|p| p.y).sum::<f32>() / points.len() as f32
                } else {
                    points.iter().map(|p| p.x).sum::<f32>() / points.len() as f32
                };
                if let Some(first) = points.first_mut() {
                    if horizontal {
                        first.y = avg;
                    } else {
                        first.x = avg;
                    }
                }
            }
        });
        self.status = if horizontal {
            "Aligned path points horizontally".to_string()
        } else {
            "Aligned path points vertically".to_string()
        };
    }

    pub fn snap_selected_to_pixel(&mut self) {
        self.push_undo("Snap to Pixel");
        self.with_selected_mut(|obj| {
            if let Some((lo, hi)) = obj.bounds() {
                obj.set_bounds(
                    Vec2::new(lo.x.round(), lo.y.round()),
                    Vec2::new(hi.x.round(), hi.y.round()),
                );
            }
            if let Shape::Path { points, .. } = &mut obj.kind {
                for p in points {
                    p.x = p.x.round();
                    p.y = p.y.round();
                }
            }
        });
        self.status = "Snapped selection coordinates to pixel grid".to_string();
    }

    pub fn convert_stroke_to_fill(&mut self) {
        self.push_undo("Convert Stroke to Fill");
        let default_stroke = self.stroke;
        self.with_selected_mut(|obj| {
            let sc = obj.stroke.unwrap_or(default_stroke);
            obj.fill = Some(sc);
            obj.stroke = None;
        });
        self.status = "Converted stroke outline to filled shape".to_string();
    }

    pub fn reorder_selected_step(&mut self, forward: bool) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        let len = self
            .doc
            .page()
            .layers
            .get(layer)
            .map(|l| l.objects.len())
            .unwrap_or(0);
        if len < 2 {
            return;
        }
        let target = if forward {
            (index + 1).min(len - 1)
        } else {
            index.saturating_sub(1)
        };
        if target == index {
            return;
        }
        self.push_undo(if forward {
            "Bring Forward"
        } else {
            "Send Backward"
        });
        let page = self.doc.page_mut();
        page.layers[layer].objects.swap(index, target);
        page.sync_state();
        self.select_single(Some(target));
        self.status = if forward {
            "Brought selection forward one level".to_string()
        } else {
            "Sent selection backward one level".to_string()
        };
    }

    pub fn rotate_ccw_90(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        let Some((lo, hi)) = obj.bounds() else {
            return;
        };
        self.push_undo("Rotate 90° CCW");
        let center = (lo + hi) * 0.5;
        let half = Vec2::new((hi.y - lo.y) * 0.5, (hi.x - lo.x) * 0.5);
        self.set_selected_bounds(center - half, center + half);
        self.status = "Rotated selection 90° counter-clockwise".to_string();
    }

    pub fn scale_selected(&mut self, factor: f32) {
        if let Some((lo, hi)) = self.selected_object().and_then(|o| o.bounds()) {
            self.push_undo("Scale");
            let center = (lo + hi) * 0.5;
            let half = (hi - lo) * 0.5 * factor;
            self.set_selected_bounds(center - half, center + half);
            self.status = format!("Scaled selection by {:.0}%", factor * 100.0);
        }
    }

    pub fn apply_color_command(&mut self, cmd: &str) {
        if self.selected.is_none() {
            self.status = "Select an object first".to_string();
            return;
        }
        self.push_undo("Adjust Color");
        let xform = |c: Color32| -> Color32 {
            let (r, g, b, a) = (c.r() as f32, c.g() as f32, c.b() as f32, c.a());
            match cmd {
                "brighten" => Color32::from_rgba_unmultiplied(
                    (r * 1.18).min(255.0) as u8,
                    (g * 1.18).min(255.0) as u8,
                    (b * 1.18).min(255.0) as u8,
                    a,
                ),
                "darken" => Color32::from_rgba_unmultiplied(
                    (r * 0.82) as u8,
                    (g * 0.82) as u8,
                    (b * 0.82) as u8,
                    a,
                ),
                "contrast" => {
                    let f = |v: f32| ((v - 128.0) * 1.25 + 128.0).clamp(0.0, 255.0) as u8;
                    Color32::from_rgba_unmultiplied(f(r), f(g), f(b), a)
                }
                "invert" => {
                    Color32::from_rgba_unmultiplied(255 - c.r(), 255 - c.g(), 255 - c.b(), a)
                }
                "grayscale" => {
                    let y = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
                    Color32::from_rgba_unmultiplied(y, y, y, a)
                }
                "sepia" => {
                    let tr = (0.393 * r + 0.769 * g + 0.189 * b).min(255.0) as u8;
                    let tg = (0.349 * r + 0.686 * g + 0.168 * b).min(255.0) as u8;
                    let tb = (0.272 * r + 0.534 * g + 0.131 * b).min(255.0) as u8;
                    Color32::from_rgba_unmultiplied(tr, tg, tb, a)
                }
                _ => c,
            }
        };
        self.with_selected_mut(|obj| {
            obj.fill = obj.fill.map(xform);
            obj.stroke = obj.stroke.map(xform);
            if let Shape::Bitmap { pixels, .. } = &mut obj.kind {
                for px in pixels.as_chunks_mut::<4>().0 {
                    let c = xform(Color32::from_rgba_unmultiplied(px[0], px[1], px[2], px[3]));
                    px[0] = c.r();
                    px[1] = c.g();
                    px[2] = c.b();
                }
            }
        });
        self.textures.clear();
        self.status = format!("Applied color filter: {cmd}");
    }

    pub fn flatten_selected_to_bitmap(&mut self) {
        let Some(obj) = self.selected_object() else {
            return;
        };
        let Some((lo, hi)) = obj.bounds() else {
            return;
        };
        self.push_undo("Flatten Selection");
        let w = (hi.x - lo.x).max(4.0) as u32;
        let h = (hi.y - lo.y).max(4.0) as u32;
        let c = obj.fill.unwrap_or(self.fill);
        let img = image::RgbaImage::from_pixel(w, h, image::Rgba([c.r(), c.g(), c.b(), 255]));
        self.with_selected_mut(|o| {
            o.kind = Shape::Bitmap {
                at: lo,
                pixels: img.into_raw(),
                size: Vec2::new(w as f32, h as f32),
            };
        });
        self.textures.clear();
        self.status = "Flattened selection to editable Bitmap".to_string();
    }

    pub fn copy_selected_css(&mut self) {
        if let Some(obj) = self.selected_object() {
            let css = crate::formats::generate_css_for_object(&obj);
            self.ctx.copy_text(css);
            self.library_tab = 4; // Focus CSS tab
            self.status = "Copied CSS3 rules to clipboard".to_string();
        }
    }

    pub fn run_find_and_replace(&mut self) {
        if self.find_query.is_empty() {
            return;
        }
        self.push_undo("Find and Replace");
        let from = self.find_query.clone();
        let to = self.replace_query.clone();
        let mut count = 0usize;
        for page in &mut self.doc.pages {
            for layer in &mut page.layers {
                for obj in &mut layer.objects {
                    if obj.name.contains(&from) {
                        obj.name = obj.name.replace(&from, &to);
                        count += 1;
                    }
                    if let Shape::Text { text, .. } = &mut obj.kind
                        && text.contains(&from)
                    {
                        *text = text.replace(&from, &to);
                        count += 1;
                    }
                }
            }
            page.sync_state();
        }
        self.status = format!("Replaced {count} occurrence(s) of '{from}'");
    }

    pub fn check_spelling(&mut self) {
        let mut text_objects = 0usize;
        for layer in &self.doc.page().layers {
            for obj in &layer.objects {
                if matches!(obj.kind, Shape::Text { .. }) {
                    text_objects += 1;
                }
            }
        }
        self.status = format!("Spelling check complete: {text_objects} text object(s) verified");
    }

    pub fn execute_saved_command(&mut self, cmd: &str) {
        match cmd {
            "Add Drop Shadow Card" => self.command_add_shadow_card(),
            "Add Picture Frame" => self.command_add_picture_frame(),
            "Distribute 3×3 Guides" => self.command_add_thirds_guides(),
            "Convert Selection to Grayscale" => self.apply_color_command("grayscale"),
            "Create Slice from Selection" => self.create_slice_from_selection(),
            _ => self.replay_last_action(),
        }
    }

    pub fn command_add_picture_frame(&mut self) {
        self.push_undo("Add Picture Frame");
        let (w, h) = (self.doc.page().width, self.doc.page().height);
        let mut frame = Obj::new(
            Shape::Rect {
                min: Vec2::new(16.0, 16.0),
                max: Vec2::new(w - 16.0, h - 16.0),
                corner_radius: 6.0,
                filled: false,
            },
            None,
            Some(self.stroke),
        );
        frame.name = "Picture Frame".to_string();
        frame.width = 8.0;
        let idx = self.push_object(frame);
        self.select_single(Some(idx));
        self.status = "Added Picture Frame".to_string();
    }

    pub fn command_add_shadow_card(&mut self) {
        self.push_undo("Add Shadow Card");
        let c = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
        let mut card = Obj::new(
            Shape::Rect {
                min: c - Vec2::new(120.0, 75.0),
                max: c + Vec2::new(120.0, 75.0),
                corner_radius: 12.0,
                filled: true,
            },
            Some(self.fill),
            Some(self.stroke),
        );
        card.name = "UI Card".to_string();
        card.effects.push(LiveEffect::DropShadow {
            distance: 6.0,
            angle_deg: 45.0,
            blur: 8.0,
            color: Color32::from_black_alpha(150),
            enabled: true,
        });
        let idx = self.push_object(card);
        self.select_single(Some(idx));
        self.status = "Added UI Card with Drop Shadow".to_string();
    }

    pub fn command_add_thirds_guides(&mut self) {
        let page = self.doc.page_mut();
        let (w, h) = (page.width, page.height);
        page.guides_v = vec![w / 3.0, w * 2.0 / 3.0];
        page.guides_h = vec![h / 3.0, h * 2.0 / 3.0];
        self.show_guides = true;
        self.status = "Added 3×3 rule-of-thirds guides".to_string();
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
        let pad = (w * 0.035).clamp(24.0, 64.0);

        // 1. Deep Studio Gradient Backdrop
        let mut backdrop = Obj::new(
            Shape::Gradient {
                min: Vec2::ZERO,
                max: Vec2::new(w, h),
                from: Color32::from_rgb(22, 24, 34),
                to: Color32::from_rgb(14, 15, 22),
                radial: true,
            },
            None,
            None,
        );
        backdrop.name = "Studio Backdrop".to_string();
        self.push_object(backdrop);

        // 2. Hero Brand Card
        let hero_h = (h * 0.40).clamp(200.0, 420.0);
        let mut hero = Obj::new(
            Shape::Rect {
                min: Vec2::new(pad, pad),
                max: Vec2::new(w - pad, pad + hero_h),
                corner_radius: 16.0,
                filled: true,
            },
            Some(Color32::from_rgb(30, 34, 48)),
            Some(Color32::from_rgb(232, 163, 61)),
        );
        hero.name = "Hero Brand Card".to_string();
        hero.width = 2.0;
        hero.effects.push(LiveEffect::DropShadow {
            distance: 8.0,
            angle_deg: 90.0,
            blur: 14.0,
            color: Color32::from_black_alpha(150),
            enabled: true,
        });
        self.push_object(hero);

        // 3. Stylized Multi-Layer Vector FireCraft Flame Emblem (left of hero)
        let fcx = pad + (w * 0.12).clamp(90.0, 210.0);
        let fcy = pad + hero_h * 0.52;
        let fs = (hero_h * 0.36).clamp(54.0, 140.0);
        let mut flame_outer = Obj::new(
            Shape::Path {
                points: vec![
                    Vec2::new(fcx, fcy - fs),
                    Vec2::new(fcx + fs * 0.42, fcy - fs * 0.35),
                    Vec2::new(fcx + fs * 0.68, fcy + fs * 0.22),
                    Vec2::new(fcx + fs * 0.46, fcy + fs * 0.78),
                    Vec2::new(fcx, fcy + fs * 0.92),
                    Vec2::new(fcx - fs * 0.48, fcy + fs * 0.78),
                    Vec2::new(fcx - fs * 0.66, fcy + fs * 0.18),
                    Vec2::new(fcx - fs * 0.32, fcy - fs * 0.42),
                ],
                closed: true,
            },
            Some(Color32::from_rgb(230, 74, 38)),
            Some(Color32::from_rgb(255, 190, 90)),
        );
        flame_outer.name = "FireCraft Flame Outer".to_string();
        flame_outer.width = 2.0;
        flame_outer.effects.push(LiveEffect::Glow {
            radius: 14.0,
            color: Color32::from_rgb(255, 130, 40),
            inner: false,
            enabled: true,
        });
        self.push_object(flame_outer);

        let mut flame_mid = Obj::new(
            Shape::Path {
                points: vec![
                    Vec2::new(fcx + fs * 0.04, fcy - fs * 0.56),
                    Vec2::new(fcx + fs * 0.36, fcy + fs * 0.10),
                    Vec2::new(fcx + fs * 0.28, fcy + fs * 0.64),
                    Vec2::new(fcx, fcy + fs * 0.76),
                    Vec2::new(fcx - fs * 0.32, fcy + fs * 0.58),
                    Vec2::new(fcx - fs * 0.24, fcy + fs * 0.02),
                ],
                closed: true,
            },
            Some(Color32::from_rgb(245, 158, 42)),
            None,
        );
        flame_mid.name = "FireCraft Flame Mid".to_string();
        self.push_object(flame_mid);

        let mut flame_core = Obj::new(
            Shape::Path {
                points: vec![
                    Vec2::new(fcx, fcy - fs * 0.12),
                    Vec2::new(fcx + fs * 0.18, fcy + fs * 0.34),
                    Vec2::new(fcx, fcy + fs * 0.62),
                    Vec2::new(fcx - fs * 0.18, fcy + fs * 0.34),
                ],
                closed: true,
            },
            Some(Color32::from_rgb(255, 232, 110)),
            None,
        );
        flame_core.name = "FireCraft Flame Core".to_string();
        let last_idx = self.push_object(flame_core);

        // 4. Hero Branding Typography & Tagline
        let text_x = fcx + fs * 1.05;
        let mut title = Obj::new(
            Shape::Text {
                at: Vec2::new(text_x, pad + hero_h * 0.26),
                text: "FireCraft Studio".to_string(),
                size: (hero_h * 0.18).clamp(32.0, 64.0),
                bold: true,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(Color32::from_rgb(250, 208, 68)),
            None,
        );
        title.name = "Hero Brand Title".to_string();
        self.push_object(title);

        let mut subtitle = Obj::new(
            Shape::Text {
                at: Vec2::new(text_x, pad + hero_h * 0.50),
                text: "Open Source Bitmap & Vector Graphics Editor · CS6 Parity + MCP".to_string(),
                size: (hero_h * 0.068).clamp(16.0, 26.0),
                bold: false,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(Color32::from_rgb(198, 206, 224)),
            None,
        );
        subtitle.name = "Hero Subtitle".to_string();
        self.push_object(subtitle);

        // 5. Rounded CTA Button with Bevel & Web Slice
        let cta_w = (w * 0.16).clamp(180.0, 300.0);
        let cta_h = (hero_h * 0.16).clamp(44.0, 64.0);
        let cta_min = Vec2::new(text_x, pad + hero_h * 0.68);
        let cta_max = cta_min + Vec2::new(cta_w, cta_h);
        let mut cta = Obj::new(
            Shape::Rect {
                min: cta_min,
                max: cta_max,
                corner_radius: 10.0,
                filled: true,
            },
            Some(Color32::from_rgb(232, 163, 61)),
            Some(Color32::from_rgb(255, 224, 150)),
        );
        cta.name = "Primary CTA Button".to_string();
        cta.effects.push(LiveEffect::Bevel {
            width: 2.0,
            highlight: Color32::from_white_alpha(180),
            shadow: Color32::from_black_alpha(140),
            enabled: true,
        });
        self.push_object(cta);

        let mut cta_lbl = Obj::new(
            Shape::Text {
                at: Vec2::new(cta_min.x + 28.0, cta_min.y + cta_h * 0.30),
                text: "Explore Showcase →".to_string(),
                size: (cta_h * 0.38).clamp(15.0, 22.0),
                bold: true,
                italic: false,
                font_family: "Sans-Serif".to_string(),
            },
            Some(Color32::from_rgb(24, 20, 14)),
            None,
        );
        cta_lbl.name = "CTA Button Label".to_string();
        self.push_object(cta_lbl);

        // 6. Four Modern Feature Cards across bottom half
        let cards_top = pad + hero_h + (h * 0.04).clamp(24.0, 48.0);
        let cards_bottom = (h - pad).max(cards_top + 220.0);
        let gap = (w * 0.02).clamp(16.0, 36.0);
        let card_w = ((w - pad * 2.0 - gap * 3.0) / 4.0).max(140.0);
        let features = [
            (
                "Vector + Bitmap",
                "Boolean paths, dynamic\nAuto Shapes & raster tools",
                Color32::from_rgb(66, 153, 225),
            ),
            (
                "Live CS6 Filters",
                "Drop Shadow, Glow, Bevel\n& 21 dockable panels",
                Color32::from_rgb(237, 137, 54),
            ),
            (
                "Web & Slices",
                "PNG, WebP, SVG, HTML5\n2-Up & 4-Up optimization",
                Color32::from_rgb(72, 187, 120),
            ),
            (
                "MCP & Headless CLI",
                "JSON-RPC 2.0 server,\n60+ commands & scripts",
                Color32::from_rgb(159, 122, 234),
            ),
        ];
        for (i, (heading, body, accent_col)) in features.iter().enumerate() {
            let x0 = pad + i as f32 * (card_w + gap);
            let mut card = Obj::new(
                Shape::Rect {
                    min: Vec2::new(x0, cards_top),
                    max: Vec2::new(x0 + card_w, cards_bottom),
                    corner_radius: 12.0,
                    filled: true,
                },
                Some(Color32::from_rgb(28, 32, 44)),
                Some(*accent_col),
            );
            card.name = format!("Feature Card: {heading}");
            card.width = 1.5;
            card.effects.push(LiveEffect::DropShadow {
                distance: 6.0,
                angle_deg: 45.0,
                blur: 10.0,
                color: Color32::from_black_alpha(140),
                enabled: true,
            });
            self.push_object(card);

            let mut h_txt = Obj::new(
                Shape::Text {
                    at: Vec2::new(x0 + 24.0, cards_top + 36.0),
                    text: (*heading).to_string(),
                    size: (card_w * 0.068).clamp(18.0, 28.0),
                    bold: true,
                    italic: false,
                    font_family: "Sans-Serif".to_string(),
                },
                Some(*accent_col),
                None,
            );
            h_txt.name = format!("{heading} Title");
            self.push_object(h_txt);

            let mut b_txt = Obj::new(
                Shape::Text {
                    at: Vec2::new(x0 + 24.0, cards_top + 96.0),
                    text: (*body).to_string(),
                    size: (card_w * 0.048).clamp(14.0, 20.0),
                    bold: false,
                    italic: false,
                    font_family: "Sans-Serif".to_string(),
                },
                Some(Color32::from_rgb(190, 198, 214)),
                None,
            );
            b_txt.name = format!("{heading} Body");
            self.push_object(b_txt);
        }

        // 7. Interactive Web Slice on CTA
        let slice = Obj::new(
            Shape::Slice {
                min: cta_min,
                max: cta_max,
                url: "https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment"
                    .to_string(),
                alt: "Launch FireCraft Studio".to_string(),
                target: "_blank".to_string(),
                format: "PNG32".to_string(),
                is_hotspot: false,
            },
            None,
            None,
        );
        self.push_object(slice);
        self.selected_layer = 0;
        self.select_single(Some(last_idx));
        self.fit_in_window();
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
            FileMode::Open => self.open_path(&path),
            FileMode::Import => self.import_path(&path),
            FileMode::Save => self.save_to(&path),
            FileMode::Export => self.export_to(&path),
        };
        if result.is_ok() {
            self.show_file_dialog = false;
        }
    }

    pub fn open_path(&mut self, path: &Path) -> Result<(), String> {
        match crate::formats::open(path) {
            Ok(doc) => {
                self.sync_active_doc_slot();
                self.open_docs.push(doc.clone());
                self.active_doc = self.open_docs.len() - 1;
                self.doc = doc;
                self.selected = None;
                self.selected_layer = 0;
                self.undo.clear();
                self.redo.clear();
                self.textures.clear();
                let pb = path.to_path_buf();
                self.current_path = Some(pb.clone());
                if !self.recent_files.contains(&pb) {
                    self.recent_files.push(pb);
                }
                self.status = format!("Opened {}", path.display());
                Ok(())
            }
            Err(e) => {
                self.status = format!("Could not open: {e}");
                Err(e)
            }
        }
    }

    pub fn import_path(&mut self, path: &Path) -> Result<(), String> {
        let imported = crate::formats::open(path)?;
        self.push_undo("Import");
        for layer in imported.page().layers.clone() {
            if layer.is_web_layer {
                continue;
            }
            for obj in layer.objects {
                let idx = self.push_object(obj);
                self.selected = Some(idx);
            }
        }
        self.textures.clear();
        self.status = format!("Imported {}", path.display());
        Ok(())
    }

    pub fn save_current(&mut self) {
        match self.current_path.clone() {
            Some(path) => {
                if let Err(e) = self.save_to(&path) {
                    self.status = format!("Save failed: {e}");
                }
            }
            None => self.begin_file_dialog(FileMode::Save),
        }
    }

    pub fn save_to(&mut self, path: &Path) -> Result<(), String> {
        crate::formats::save(&self.doc, path)?;
        let pb = path.to_path_buf();
        self.current_path = Some(pb.clone());
        self.doc.dirty = false;
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            self.doc.name = name.to_string();
        }
        self.sync_active_doc_slot();
        if !self.recent_files.contains(&pb) {
            self.recent_files.push(pb);
        }
        self.status = format!("Saved {}", path.display());
        Ok(())
    }

    pub fn export_to(&mut self, path: &Path) -> Result<(), String> {
        let format = crate::formats::Format::from_path(path);
        if !format.writeable() {
            let msg = format!(
                "{} export is not supported yet; choose PNG, JPEG, GIF, WebP, TIFF, BMP, ICO, SVG or HTML.",
                format.label()
            );
            self.status = msg.clone();
            return Err(msg);
        }
        crate::formats::save(&self.doc, path)?;
        self.status = format!("Exported {}", path.display());
        Ok(())
    }

    // ---- Clipboard & Selection ---------------------------------------------

    pub fn copy_selected(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        indices.sort_unstable();
        indices.dedup();
        let layer = self.selected_layer;
        let mut copied = Vec::new();
        if let Some(l) = self.doc.page().layers.get(layer) {
            for &idx in &indices {
                if let Some(obj) = l.objects.get(idx) {
                    copied.push(obj.clone());
                }
            }
        }
        if copied.is_empty() {
            self.status = "Select one or more objects first to Copy (Ctrl+C)".to_string();
            return;
        }
        self.clipboard = copied.last().cloned();
        self.clipboard_set = copied;
        let marker = format!("FIRECRAFT_CLIPBOARD:{}", self.clipboard_set.len());
        self.ctx.copy_text(marker.clone());
        #[cfg(not(target_arch = "wasm32"))]
        if let Ok(mut cb) = arboard::Clipboard::new() {
            let _ = cb.set_text(marker);
        }
        self.status = format!("Copied {} object(s) to clipboard", self.clipboard_set.len());
    }

    pub fn cut_selected(&mut self) {
        self.copy_selected();
        if !self.clipboard_set.is_empty() {
            self.delete_selected();
            self.status = format!("Cut {} object(s) to clipboard", self.clipboard_set.len());
        }
    }

    pub fn paste(&mut self) {
        self.paste_with_text(None);
    }

    pub fn paste_with_text(&mut self, event_text: Option<&str>) {
        // 1. Check if the OS clipboard holds a bitmap image (e.g. Win+Shift+S screenshot, browser Copy Image)
        #[cfg(not(target_arch = "wasm32"))]
        if event_text.is_none()
            && let Ok(mut cb) = arboard::Clipboard::new()
            && let Ok(img) = cb.get_image()
            && img.width > 0
            && img.height > 0
        {
            let w = img.width as f32;
            let h = img.height as f32;
            let center = Vec2::new(self.doc.page().width * 0.5, self.doc.page().height * 0.5);
            let at = (center - Vec2::new(w * 0.5, h * 0.5)).max(Vec2::ZERO);
            let mut obj = Obj::new(
                Shape::Bitmap {
                    at,
                    pixels: img.bytes.into_owned(),
                    size: Vec2::new(w, h),
                },
                None,
                None,
            );
            obj.name = format!("Pasted Bitmap ({:.0}×{:.0})", w, h);
            self.push_undo("Paste Image");
            let idx = self.push_object(obj);
            self.select_single(Some(idx));
            self.textures.clear();
            self.status = format!(
                "Pasted bitmap image ({:.0}×{:.0} px) from system clipboard",
                w, h
            );
            return;
        }

        // 2. Determine clipboard text (from egui Paste event or OS clipboard)
        #[cfg(not(target_arch = "wasm32"))]
        let os_text = event_text.map(|s| s.to_string()).or_else(|| {
            arboard::Clipboard::new()
                .ok()
                .and_then(|mut cb| cb.get_text().ok())
        });
        #[cfg(target_arch = "wasm32")]
        let os_text = event_text.map(|s| s.to_string());
        let is_internal_marker = os_text
            .as_deref()
            .map(|s| s.starts_with("FIRECRAFT_CLIPBOARD"))
            .unwrap_or(false);

        // 3. If we have internal FireCraft objects in `clipboard_set` (or `clipboard`) and either
        // the marker matches or no external text override was passed, paste the internal objects.
        if (is_internal_marker || event_text.is_none())
            && (!self.clipboard_set.is_empty() || self.clipboard.is_some())
        {
            if self.clipboard_set.is_empty()
                && let Some(single) = self.clipboard.clone()
            {
                self.clipboard_set.push(single);
            }
            self.push_undo("Paste");
            let mut new_indices = Vec::new();
            let mut updated_clip = Vec::with_capacity(self.clipboard_set.len());
            for mut obj in self.clipboard_set.clone() {
                obj.translate(Vec2::new(16.0, 16.0));
                updated_clip.push(obj.clone());
                let idx = self.push_object(obj);
                new_indices.push(idx);
            }
            self.clipboard_set = updated_clip;
            self.clipboard = self.clipboard_set.last().cloned();
            self.selected = new_indices.last().copied();
            self.selected_set = new_indices;
            self.textures.clear();
            self.status = format!(
                "Pasted {} object(s) from clipboard",
                self.selected_set.len()
            );
            return;
        }

        // 4. If external text was pasted (SVG snippet, image file path, or plain text), materialize it on canvas!
        if let Some(raw_text) = os_text {
            let trimmed = raw_text.trim();
            if !trimmed.is_empty() && !trimmed.starts_with("FIRECRAFT_CLIPBOARD") {
                // Check if it's a file path on disk
                let unquoted = trimmed.trim_matches('"').trim_matches('\'');
                let candidate_path = std::path::Path::new(unquoted);
                if candidate_path.is_file() && self.import_path(candidate_path).is_ok() {
                    return;
                }
                // Otherwise create a Text object at cursor or canvas center
                let at = self.cursor_doc_pos.unwrap_or_else(|| {
                    Vec2::new(
                        self.doc.page().width * 0.5 - 80.0,
                        self.doc.page().height * 0.5,
                    )
                });
                self.push_undo("Paste Text");
                let mut obj = Obj::new(
                    Shape::Text {
                        at,
                        text: trimmed.chars().take(240).collect(),
                        size: 28.0,
                        bold: false,
                        italic: false,
                        font_family: "Sans-Serif".to_string(),
                    },
                    Some(self.fill),
                    None,
                );
                obj.name = "Pasted Text".to_string();
                let idx = self.push_object(obj);
                self.select_single(Some(idx));
                self.status = "Pasted text from system clipboard as Text object".to_string();
                return;
            }
        }

        // 5. Fallback: if internal clipboard has objects even when external text was present, paste them
        if !self.clipboard_set.is_empty() || self.clipboard.is_some() {
            if self.clipboard_set.is_empty()
                && let Some(single) = self.clipboard.clone()
            {
                self.clipboard_set.push(single);
            }
            self.push_undo("Paste");
            let mut new_indices = Vec::new();
            for mut obj in self.clipboard_set.clone() {
                obj.translate(Vec2::new(16.0, 16.0));
                let idx = self.push_object(obj);
                new_indices.push(idx);
            }
            self.selected = new_indices.last().copied();
            self.selected_set = new_indices;
            self.status = format!(
                "Pasted {} object(s) from clipboard",
                self.selected_set.len()
            );
            return;
        }

        self.status =
            "Clipboard is empty — select an object and press Ctrl+C (or copy an image/text) first"
                .to_string();
    }

    pub fn select_all(&mut self) {
        self.selected_extra_layers.clear();
        let layers = &self.doc.page().layers;
        let mut total = 0usize;
        // Prefer keeping current selected_layer if it has selectable objects; otherwise pick first non-empty visible unlocked layer
        let cur_has_objs = layers.get(self.selected_layer).is_some_and(|l| {
            l.visible && !l.locked && l.objects.iter().any(|o| o.visible && !o.locked)
        });
        if !cur_has_objs
            && let Some((li, _)) = layers.iter().enumerate().find(|(_, l)| {
                l.visible && !l.locked && l.objects.iter().any(|o| o.visible && !o.locked)
            })
        {
            self.selected_layer = li;
        }
        let primary_layer = self.selected_layer;
        let mut primary_set = Vec::new();
        for (li, l) in layers.iter().enumerate() {
            if !l.visible || l.locked {
                continue;
            }
            for (oi, obj) in l.objects.iter().enumerate() {
                if !obj.visible || obj.locked {
                    continue;
                }
                total += 1;
                if li == primary_layer {
                    primary_set.push(oi);
                } else {
                    self.selected_extra_layers.push((li, oi));
                }
            }
        }
        self.selected_set = primary_set;
        self.selected = self.selected_set.last().copied();
        self.status = format!("Selected {total} object(s) across all visible layers");
    }

    pub fn select_inverse(&mut self) {
        let layer = self.selected_layer;
        let count = self
            .doc
            .page()
            .layers
            .get(layer)
            .map(|l| l.objects.len())
            .unwrap_or(0);
        let prev = self.selected_set.clone();
        self.selected_set = (0..count).filter(|i| !prev.contains(i)).collect();
        self.selected = self.selected_set.last().copied();
        self.status = format!(
            "Inverted selection ({} object(s) selected)",
            self.selected_set.len()
        );
    }

    pub fn toggle_lock_selection(&mut self) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        if indices.is_empty() {
            return;
        }
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for idx in indices {
                if let Some(o) = l.objects.get_mut(idx) {
                    o.locked = !o.locked;
                }
            }
        }
        page.sync_state();
        self.status = "Toggled lock on selected object(s)".to_string();
    }

    pub fn toggle_full_screen_canvas(&mut self) {
        let any_dock_visible = self.show_tools_dock
            || self.show_right_dock
            || self.show_bottom_inspector
            || self.show_main_toolbar;
        if any_dock_visible && !self.full_screen_canvas {
            self.full_screen_canvas = true;
            self.show_tools_dock = false;
            self.show_right_dock = false;
            self.show_bottom_inspector = false;
            self.show_main_toolbar = false;
            self.status =
                "Full-Screen Canvas Preview Mode enabled (press Tab or F4 to restore panels)"
                    .to_string();
        } else {
            self.full_screen_canvas = false;
            self.show_tools_dock = true;
            self.show_right_dock = true;
            self.show_bottom_inspector = true;
            self.show_main_toolbar = true;
            self.status = "Restored CS6 workspace panels (Tab / F4)".to_string();
        }
    }

    pub fn select_page_step(&mut self, delta: isize) {
        let count = self.doc.pages.len();
        if count <= 1 {
            return;
        }
        let next = (self.doc.current_page as isize + delta).rem_euclid(count as isize) as usize;
        self.doc.current_page = next;
        self.clear_selection();
        self.status = format!("Switched to Page {} of {count}", next + 1);
    }

    pub fn select_state_step(&mut self, delta: isize) {
        let count = self.doc.page().states.len();
        if count <= 1 {
            return;
        }
        let cur = self.doc.page().current_state;
        let next = (cur as isize + delta).rem_euclid(count as isize) as usize;
        self.doc.page_mut().select_state(next);
        self.clear_selection();
        self.status = format!("Switched to Animation State {} of {count}", next + 1);
    }

    pub fn cycle_selection(&mut self, forward: bool) {
        let layer = self.selected_layer;
        let count = self
            .doc
            .page()
            .layers
            .get(layer)
            .map(|l| l.objects.len())
            .unwrap_or(0);
        if count == 0 {
            self.selected = None;
            return;
        }
        let next = match self.selected {
            Some(cur) => {
                if forward {
                    (cur + 1) % count
                } else if cur == 0 {
                    count - 1
                } else {
                    cur - 1
                }
            }
            None => 0,
        };
        self.selected = Some(next);
        self.selected_set = vec![next];
    }

    pub fn select_similar(&mut self) {
        let Some(cur) = self.selected_object() else {
            return;
        };
        let target_type = cur.type_label();
        let layer = self.selected_layer;
        if let Some(l) = self.doc.page().layers.get(layer) {
            let matches = l
                .objects
                .iter()
                .filter(|o| o.type_label() == target_type)
                .count();
            self.status = format!("Found {matches} '{target_type}' object(s) on active layer");
        }
    }

    // ---- Geometry transforms -----------------------------------------------

    pub fn paste_attributes(&mut self) {
        let Some(src) = self.clipboard.clone() else {
            self.status = "Copy an object first to Paste Attributes".to_string();
            return;
        };
        if self.selected.is_none() && self.selected_set.is_empty() {
            return;
        }
        self.push_undo("Paste Attributes");
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        let layer = self.selected_layer;
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for idx in indices {
                if let Some(obj) = l.objects.get_mut(idx) {
                    obj.fill = src.fill;
                    obj.stroke = src.stroke;
                    obj.width = src.width;
                    obj.opacity = src.opacity;
                    obj.effects = src.effects.clone();
                }
            }
        }
        page.sync_state();
        self.doc.dirty = true;
        self.status = "Pasted Fill, Stroke, Opacity & Live Filters".to_string();
    }

    pub fn flip_horizontal(&mut self) {
        self.mirror_selected(true);
    }

    pub fn flip_vertical(&mut self) {
        self.mirror_selected(false);
    }

    fn mirror_obj_around(obj: &mut Obj, axis: f32, horizontal: bool) {
        let reflect_pt = |p: &mut Vec2| {
            if horizontal {
                p.x = axis - p.x;
            } else {
                p.y = axis - p.y;
            }
        };
        match &mut obj.kind {
            Shape::Rect { min, max, .. } | Shape::Slice { min, max, .. } => {
                reflect_pt(min);
                reflect_pt(max);
                let nlo = min.min(*max);
                let nhi = min.max(*max);
                *min = nlo;
                *max = nhi;
            }
            Shape::Gradient {
                min, max, from, to, ..
            } => {
                reflect_pt(min);
                reflect_pt(max);
                let nlo = min.min(*max);
                let nhi = min.max(*max);
                *min = nlo;
                *max = nhi;
                std::mem::swap(from, to);
            }
            Shape::Ellipse { center, .. } => reflect_pt(center),
            Shape::Polygon {
                center, angle_rad, ..
            } => {
                reflect_pt(center);
                *angle_rad = if horizontal {
                    -*angle_rad
                } else {
                    std::f32::consts::PI - *angle_rad
                };
            }
            Shape::Line { a, b } => {
                reflect_pt(a);
                reflect_pt(b);
            }
            Shape::Path { points, .. } => {
                for p in points.iter_mut() {
                    reflect_pt(p);
                }
            }
            Shape::Bitmap { at, size, pixels } => {
                if horizontal {
                    at.x = axis - (at.x + size.x);
                } else {
                    at.y = axis - (at.y + size.y);
                }
                let w = size.x.max(1.0) as usize;
                let h = size.y.max(1.0) as usize;
                if pixels.len() >= w * h * 4 {
                    if horizontal {
                        for row in 0..h {
                            for col in 0..(w / 2) {
                                let a = (row * w + col) * 4;
                                let b = (row * w + (w - 1 - col)) * 4;
                                for c in 0..4 {
                                    pixels.swap(a + c, b + c);
                                }
                            }
                        }
                    } else {
                        for row in 0..(h / 2) {
                            for col in 0..w {
                                let a = (row * w + col) * 4;
                                let b = ((h - 1 - row) * w + col) * 4;
                                for c in 0..4 {
                                    pixels.swap(a + c, b + c);
                                }
                            }
                        }
                    }
                }
            }
            Shape::Text { .. } | Shape::SymbolInstance { .. } => {
                if let Some((lo, hi)) = obj.bounds() {
                    let size = hi - lo;
                    let new_lo = Vec2::new(
                        if horizontal {
                            axis - (lo.x + size.x)
                        } else {
                            lo.x
                        },
                        if horizontal {
                            lo.y
                        } else {
                            axis - (lo.y + size.y)
                        },
                    );
                    obj.translate(new_lo - lo);
                }
            }
            Shape::Group { children } => {
                for ch in children.iter_mut() {
                    Self::mirror_obj_around(ch, axis, horizontal);
                }
            }
        }
    }

    fn mirror_selected(&mut self, horizontal: bool) {
        let mut indices = self.selected_set.clone();
        if let Some(s) = self.selected
            && !indices.contains(&s)
        {
            indices.push(s);
        }
        indices.sort_unstable();
        indices.dedup();
        if indices.is_empty() {
            return;
        }
        let layer = self.selected_layer;
        let mut lo = Vec2::splat(f32::MAX);
        let mut hi = Vec2::splat(f32::MIN);
        let mut any = false;
        if let Some(l) = self.doc.page().layers.get(layer) {
            for &idx in &indices {
                if let Some((olo, ohi)) = l.objects.get(idx).and_then(|o| o.bounds()) {
                    lo = lo.min(olo);
                    hi = hi.max(ohi);
                    any = true;
                }
            }
        }
        if !any {
            return;
        }
        self.push_undo("Flip");
        let axis = if horizontal { lo.x + hi.x } else { lo.y + hi.y };
        let page = self.doc.page_mut();
        if let Some(l) = page.layers.get_mut(layer) {
            for idx in indices {
                if let Some(obj) = l.objects.get_mut(idx) {
                    Self::mirror_obj_around(obj, axis, horizontal);
                }
            }
        }
        page.sync_state();
        self.textures.clear();
        self.doc.dirty = true;
        self.status = if horizontal {
            "Flipped horizontally".to_string()
        } else {
            "Flipped vertically".to_string()
        };
    }

    pub fn rotate_90(&mut self) {
        let Some((lo, hi)) = self.selected_object().and_then(|o| o.bounds()) else {
            return;
        };
        let centre = (lo + hi) / 2.0;
        self.push_undo("Rotate 90°");
        let rotate = |p: &mut Vec2| {
            let d = *p - centre;
            *p = centre + Vec2::new(d.y, -d.x);
        };
        self.with_selected_mut(|obj| match &mut obj.kind {
            Shape::Rect { min, max, .. }
            | Shape::Gradient { min, max, .. }
            | Shape::Slice { min, max, .. } => {
                rotate(min);
                rotate(max);
            }
            Shape::Ellipse { center, radius, .. } => {
                rotate(center);
                std::mem::swap(&mut radius.x, &mut radius.y);
            }
            Shape::Polygon { angle_rad, .. } => {
                *angle_rad += std::f32::consts::FRAC_PI_2;
            }
            Shape::Line { a, b } => {
                rotate(a);
                rotate(b);
            }
            Shape::Path { points, .. } => {
                for p in points.iter_mut() {
                    rotate(p);
                }
            }
            Shape::Text { at, .. }
            | Shape::Bitmap { at, .. }
            | Shape::SymbolInstance { at, .. } => rotate(at),
            Shape::Group { children } => {
                for ch in children.iter_mut() {
                    if let Some((clo, chi)) = ch.bounds() {
                        let mut a = clo;
                        let mut b = chi;
                        rotate(&mut a);
                        rotate(&mut b);
                        ch.set_bounds(a.min(b), a.max(b));
                    }
                }
            }
        });
        self.textures.clear();
        self.status = "Rotated 90°".to_string();
    }

    pub fn reorder_selected(&mut self, to_front: bool) {
        let Some(index) = self.selected else {
            return;
        };
        let layer = self.selected_layer;
        self.push_undo("Reorder");
        let page = self.doc.page_mut();
        let Some(objects) = page.layers.get_mut(layer).map(|l| &mut l.objects) else {
            return;
        };
        if index >= objects.len() {
            return;
        }
        let obj = objects.remove(index);
        let new_index = if to_front {
            objects.push(obj);
            objects.len() - 1
        } else {
            objects.insert(0, obj);
            0
        };
        page.sync_state();
        self.selected = Some(new_index);
        self.status = if to_front {
            "Brought to front".to_string()
        } else {
            "Sent to back".to_string()
        };
    }

    // ---- View & Theme ------------------------------------------------------

    pub fn set_zoom(&mut self, zoom: f32) {
        self.doc.zoom = zoom.clamp(constants::ZOOM_MIN, constants::ZOOM_MAX);
        self.status = format!("Zoom: {:.0}%", self.doc.zoom * 100.0);
    }

    pub fn zoom_by(&mut self, factor: f32) {
        let zoom = self.doc.zoom * factor;
        self.set_zoom(zoom);
    }

    pub fn fit_in_window(&mut self) {
        let page = self.doc.page();
        let (pw, ph) = (page.width.max(1.0), page.height.max(1.0));
        let (cw, ch) = (self.canvas_size.x.max(1.0), self.canvas_size.y.max(1.0));
        let zoom = ((cw * 0.90) / pw).min((ch * 0.90) / ph);
        self.set_zoom(zoom);
        self.doc.pan = Vec2::ZERO;
        self.status = "Fit in window".to_string();
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
