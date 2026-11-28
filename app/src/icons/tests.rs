use super::{AlignIcon, Icon, PathIcon, draw, draw_align, draw_path, install_phosphor_font};
use eframe::egui::{self, Color32, Pos2, Rect, Vec2};

const ICONS: &[Icon] = &[
    Icon::New,
    Icon::Open,
    Icon::Save,
    Icon::Import,
    Icon::Export,
    Icon::Cut,
    Icon::Copy,
    Icon::Paste,
    Icon::Duplicate,
    Icon::Undo,
    Icon::Redo,
    Icon::Select,
    Icon::Subselect,
    Icon::Scale,
    Icon::Marquee,
    Icon::Lasso,
    Icon::Wand,
    Icon::Crop,
    Icon::Hand,
    Icon::Zoom,
    Icon::Brush,
    Icon::Pencil,
    Icon::Eraser,
    Icon::Stamp,
    Icon::Line,
    Icon::Rectangle,
    Icon::Rounded,
    Icon::Ellipse,
    Icon::Polygon,
    Icon::Star,
    Icon::Pen,
    Icon::Knife,
    Icon::Text,
    Icon::Picker,
    Icon::Fill,
    Icon::Gradient,
    Icon::Blur,
    Icon::Sharpen,
    Icon::Dodge,
    Icon::Burn,
    Icon::Slice,
    Icon::Hotspot,
    Icon::ZoomIn,
    Icon::ZoomOut,
    Icon::Fit,
    Icon::Group,
    Icon::Ungroup,
    Icon::BringFront,
    Icon::BringForward,
    Icon::SendBackward,
    Icon::SendBack,
    Icon::RotateCCW,
    Icon::RotateCW,
    Icon::FlipH,
    Icon::FlipV,
    Icon::Eye,
    Icon::EyeOff,
    Icon::Lock,
    Icon::Unlock,
    Icon::Trash,
    Icon::Folder,
    Icon::StateFirst,
    Icon::StatePrev,
    Icon::StatePlay,
    Icon::StatePause,
    Icon::StateNext,
    Icon::StateLast,
    Icon::Swap,
    Icon::SquareHalf,
    Icon::DropSlash,
    Icon::CornersOut,
];

const ALIGN_ICONS: &[AlignIcon] = &[
    AlignIcon::RelativeToObject,
    AlignIcon::RelativeToCanvas,
    AlignIcon::AlignLeft,
    AlignIcon::AlignCenterH,
    AlignIcon::AlignRight,
    AlignIcon::AlignTop,
    AlignIcon::AlignCenterV,
    AlignIcon::AlignBottom,
    AlignIcon::DistributeTop,
    AlignIcon::DistributeCenterV,
    AlignIcon::DistributeBottom,
    AlignIcon::DistributeLeft,
    AlignIcon::DistributeCenterH,
    AlignIcon::DistributeRight,
    AlignIcon::MatchWidth,
    AlignIcon::MatchHeight,
    AlignIcon::MatchBoth,
    AlignIcon::SpaceVertical,
    AlignIcon::SpaceHorizontal,
];

const PATH_ICONS: &[PathIcon] = &[
    PathIcon::Join,
    PathIcon::Split,
    PathIcon::Union,
    PathIcon::Intersect,
    PathIcon::Punch,
    PathIcon::Crop,
    PathIcon::Simplify,
    PathIcon::ExpandStroke,
    PathIcon::InsetExpand,
    PathIcon::HardEdge,
    PathIcon::AntiAlias,
    PathIcon::FeatherEdge,
    PathIcon::StrokeToFill,
    PathIcon::ReverseDirection,
    PathIcon::ClosePath,
    PathIcon::OpenPath,
    PathIcon::PathToMarquee,
    PathIcon::SnapToPixel,
    PathIcon::FlattenPath,
    PathIcon::AddPoints,
    PathIcon::DeletePoints,
    PathIcon::CornerPoint,
    PathIcon::SmoothPoint,
    PathIcon::RoundPoints,
    PathIcon::FilletPoints,
    PathIcon::ChamferPoints,
    PathIcon::BreakPath,
    PathIcon::JoinPoints,
    PathIcon::AlignPointsH,
    PathIcon::AlignPointsV,
    PathIcon::KnifeSplit,
    PathIcon::ExtendPath,
    PathIcon::SharpApex,
    PathIcon::InversePoints,
    PathIcon::SelectAllPoints,
    PathIcon::FirstPoint,
    PathIcon::PrevPoint,
    PathIcon::NextPoint,
];

fn paint_each<T: std::fmt::Debug + Copy>(items: &[T], mut paint: impl FnMut(&egui::Painter, T)) {
    let ctx = egui::Context::default();
    install_phosphor_font(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(640.0, 48.0))),
        ..Default::default()
    };

    for (index, item) in items.iter().copied().enumerate() {
        let mut output = ctx.run_ui(input.clone(), |ui| {
            paint(ui.painter(), item);
        });
        assert!(
            !output.shapes.is_empty(),
            "painting {item:?} at table index {index} emitted no egui shapes"
        );
        // No renderer consumes font-atlas uploads in this headless test.
        output.textures_delta.clear();
    }
}

#[test]
fn every_toolbar_icon_paints_with_the_bundled_phosphor_font() {
    paint_each(ICONS, |painter, icon| {
        draw(
            painter,
            Rect::from_min_size(Pos2::new(4.0, 4.0), Vec2::splat(32.0)),
            icon,
            Color32::WHITE,
            1.25,
        );
    });
}

#[test]
fn every_toolbar_icon_has_a_paintable_legacy_fallback() {
    paint_each(ICONS, |painter, icon| {
        super::draw_legacy(
            painter,
            Rect::from_min_size(Pos2::new(4.0, 4.0), Vec2::splat(32.0)),
            icon,
            Color32::from_rgb(220, 180, 120),
            0.0,
        );
    });
}

#[test]
fn every_alignment_icon_paints() {
    paint_each(ALIGN_ICONS, |painter, icon| {
        draw_align(
            painter,
            Rect::from_min_size(Pos2::new(4.0, 4.0), Vec2::splat(32.0)),
            icon,
            Color32::WHITE,
        );
    });
}

#[test]
fn every_path_icon_paints() {
    paint_each(PATH_ICONS, |painter, icon| {
        draw_path(
            painter,
            Rect::from_min_size(Pos2::new(4.0, 4.0), Vec2::splat(32.0)),
            icon,
            Color32::WHITE,
        );
    });
}
