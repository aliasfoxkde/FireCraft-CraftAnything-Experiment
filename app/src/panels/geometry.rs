//! Pure panel geometry: group heights, magnetic snapping and stack placement.
//!
//! No widget state and no `&mut` access — the dock code passes plain rectangles in and gets
//! plain rectangles back, so every rule here is unit-testable without a display.

use eframe::egui::{Pos2, Rect, Vec2};

/// Distance at which a floating group snaps to an edge, another window or a corner.
pub const SNAP_THRESHOLD: f32 = 12.0;
/// Looser horizontal tolerance for aligning a window into a column with another one.
pub const COLUMN_SNAP: f32 = 28.0;
/// Gap between stacked windows.
pub const STACK_GAP: f32 = 2.0;
/// Gap kept from the workspace edge after snapping.
pub const EDGE_INSET: f32 = 2.0;

/// Height of a panel group, collapsed or expanded, including its header.
pub fn group_height(collapsed: bool, body_height: f32) -> f32 {
    if collapsed {
        24.0
    } else {
        (body_height + 30.0).clamp(60.0, 480.0)
    }
}

/// A floating window's rectangle, used as a snap target.
#[derive(Clone, Copy, Debug)]
pub struct Slot {
    pub pos: Pos2,
    pub size: Vec2,
}

impl Slot {
    pub fn rect(self) -> Rect {
        Rect::from_min_size(self.pos, self.size)
    }
}

/// Snap `pos` (top-left of a window of `size`) to the workspace bounds and to the other
/// floating windows. Edge snapping wins over column snapping; column snapping keeps rows aligned.
pub fn snap_pos(pos: Pos2, size: Vec2, bounds: Rect, others: &[Slot]) -> Pos2 {
    let mut pos = pos;

    // 1. Workspace edges.
    if (pos.x - bounds.left()).abs() < SNAP_THRESHOLD {
        pos.x = bounds.left() + EDGE_INSET;
    }
    if ((pos.x + size.x) - bounds.right()).abs() < SNAP_THRESHOLD {
        pos.x = bounds.right() - size.x - EDGE_INSET;
    }
    if (pos.y - bounds.top()).abs() < SNAP_THRESHOLD {
        pos.y = bounds.top() + EDGE_INSET;
    }
    if ((pos.y + size.y) - bounds.bottom()).abs() < SNAP_THRESHOLD {
        pos.y = bounds.bottom() - size.y - EDGE_INSET;
    }

    // 2. Other floating windows: stack vertically, align horizontally.
    for other in others {
        let rect = other.rect();
        if (pos.x - rect.left()).abs() < COLUMN_SNAP {
            if (pos.y - rect.bottom()).abs() < SNAP_THRESHOLD {
                pos.y = rect.bottom() + STACK_GAP;
                pos.x = rect.left();
            } else if ((pos.y + size.y) - rect.top()).abs() < SNAP_THRESHOLD {
                pos.y = rect.top() - size.y - STACK_GAP;
                pos.x = rect.left();
            }
        }
        if (pos.x - rect.right()).abs() < SNAP_THRESHOLD {
            pos.x = rect.right() + STACK_GAP + 1.0;
            if (pos.y - rect.top()).abs() < SNAP_THRESHOLD {
                pos.y = rect.top();
            }
        } else if ((pos.x + size.x) - rect.left()).abs() < SNAP_THRESHOLD {
            pos.x = rect.left() - size.x - STACK_GAP - 1.0;
            if (pos.y - rect.top()).abs() < SNAP_THRESHOLD {
                pos.y = rect.top();
            }
        }
    }
    pos
}

/// Where the child window goes when stacked below a target.
pub fn stacked_below(target: Slot) -> Pos2 {
    target.pos + Vec2::new(0.0, target.size.y + STACK_GAP)
}

/// Where a target moves when another window is stacked above it.
pub fn shifted_for_window_above(target_pos: Pos2, child_height: f32) -> Pos2 {
    target_pos + Vec2::new(0.0, child_height + STACK_GAP)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bounds() -> Rect {
        Rect::from_min_size(Pos2::ZERO, Vec2::new(1400.0, 900.0))
    }

    #[test]
    fn group_height_is_clamped_and_headers_are_compact() {
        assert_eq!(group_height(true, 500.0), 24.0);
        assert_eq!(group_height(false, 0.0), 60.0);
        assert_eq!(group_height(false, 100.0), 130.0);
        assert_eq!(group_height(false, 1_000.0), 480.0);
    }

    #[test]
    fn workspace_edges_snap_within_the_threshold_only() {
        let size = Vec2::new(200.0, 150.0);
        assert_eq!(
            snap_pos(Pos2::new(5.0, 300.0), size, bounds(), &[]),
            Pos2::new(2.0, 300.0)
        );
        assert_eq!(
            snap_pos(Pos2::new(60.0, 300.0), size, bounds(), &[]),
            Pos2::new(60.0, 300.0)
        );
        assert_eq!(
            snap_pos(Pos2::new(1195.0, 300.0), size, bounds(), &[]),
            Pos2::new(1400.0 - 200.0 - 2.0, 300.0)
        );
        assert_eq!(
            snap_pos(Pos2::new(300.0, 4.0), size, bounds(), &[]),
            Pos2::new(300.0, 2.0)
        );
        assert_eq!(
            snap_pos(Pos2::new(300.0, 747.0), size, bounds(), &[]),
            Pos2::new(300.0, 900.0 - 150.0 - 2.0)
        );
    }

    #[test]
    fn window_stacks_below_a_neighbour_and_aligns_the_column() {
        let other = Slot {
            pos: Pos2::new(300.0, 200.0),
            size: Vec2::new(220.0, 160.0),
        };
        // Dropped just under the neighbour: snaps flush beneath it, same left edge.
        let snapped = snap_pos(
            Pos2::new(305.0, 366.0),
            Vec2::new(200.0, 120.0),
            bounds(),
            &[other],
        );
        assert_eq!(snapped, Pos2::new(300.0, 362.0));
        assert_eq!(stacked_below(other), Pos2::new(300.0, 362.0));
    }

    #[test]
    fn window_snaps_to_the_left_of_a_neighbour_without_overlapping() {
        let other = Slot {
            pos: Pos2::new(600.0, 200.0),
            size: Vec2::new(220.0, 160.0),
        };
        let size = Vec2::new(180.0, 140.0);
        let snapped = snap_pos(Pos2::new(425.0, 202.0), size, bounds(), &[other]);
        assert_eq!(snapped.x, 600.0 - 180.0 - 3.0);
        assert_eq!(snapped.y, 200.0);
        assert!(
            snapped.x + size.x <= other.pos.x,
            "snapped window overlaps its neighbour"
        );
    }

    #[test]
    fn stacking_above_moves_the_target_down_by_the_child_height() {
        let target = Pos2::new(100.0, 100.0);
        assert_eq!(
            shifted_for_window_above(target, 120.0),
            Pos2::new(100.0, 222.0)
        );
    }

    #[test]
    fn a_window_far_from_everything_is_left_alone() {
        let pos = Pos2::new(500.0, 400.0);
        let size = Vec2::new(200.0, 150.0);
        let other = Slot {
            pos: Pos2::new(100.0, 100.0),
            size: Vec2::new(100.0, 100.0),
        };
        assert_eq!(snap_pos(pos, size, bounds(), &[other]), pos);
    }
}
