//! Pure polygon operations shared by canvas rendering and picking.
//!
//! `triangulate_simple_polygon` returns triangle indices into the caller's original point slice.
//! It ignores duplicate closing points and redundant forward collinear anchors. Degenerate and
//! self-intersecting contours return `None`; callers should keep their editable outline but not
//! invent a filled silhouette.

use eframe::egui::Vec2;
use std::cell::RefCell;
use std::sync::Arc;

const MIN_GEOMETRY_SCALE: f32 = 1.0;
const RELATIVE_DISTANCE_EPSILON: f32 = 1.0e-6;
const RELATIVE_AREA_EPSILON: f32 = 1.0e-7;

#[derive(Clone, Debug, Default)]
pub(crate) struct PathTriangulationCache {
    entry: RefCell<Option<CachedTriangulation>>,
}

#[derive(Clone, Debug)]
struct CachedTriangulation {
    point_bits: Vec<[u32; 2]>,
    triangles: Option<Arc<[[usize; 3]]>>,
}

impl PartialEq for PathTriangulationCache {
    // This derived cache is not part of an object's document or equality semantics.
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl PathTriangulationCache {
    pub(crate) fn triangulate(&self, points: &[Vec2]) -> Option<Arc<[[usize; 3]]>> {
        let mut entry = self.entry.borrow_mut();
        if let Some(cached) = entry.as_ref()
            && cached.point_bits.len() == points.len()
            && cached.point_bits.iter().zip(points).all(|(cached, point)| {
                cached[0] == point.x.to_bits() && cached[1] == point.y.to_bits()
            })
        {
            return cached.triangles.clone();
        }

        let triangles = triangulate_simple_polygon(points)
            .map(|triangles| Arc::<[[usize; 3]]>::from(triangles));
        *entry = Some(CachedTriangulation {
            point_bits: points
                .iter()
                .map(|point| [point.x.to_bits(), point.y.to_bits()])
                .collect(),
            triangles: triangles.clone(),
        });
        triangles
    }
}

pub(crate) fn triangulate_simple_polygon(points: &[Vec2]) -> Option<Vec<[usize; 3]>> {
    if points.len() < 3
        || points
            .iter()
            .any(|point| !point.x.is_finite() || !point.y.is_finite())
    {
        return None;
    }

    let (min_x, max_x) = points
        .iter()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |range, p| {
            (range.0.min(p.x), range.1.max(p.x))
        });
    let (min_y, max_y) = points
        .iter()
        .fold((f32::INFINITY, f32::NEG_INFINITY), |range, p| {
            (range.0.min(p.y), range.1.max(p.y))
        });
    let scale = (max_x - min_x).max(max_y - min_y).max(MIN_GEOMETRY_SCALE);
    let distance_epsilon = scale * RELATIVE_DISTANCE_EPSILON;
    let distance_epsilon_sq = distance_epsilon * distance_epsilon;
    let area_epsilon = scale * scale * RELATIVE_AREA_EPSILON;

    let mut polygon: Vec<usize> = Vec::with_capacity(points.len());
    for (index, point) in points.iter().enumerate() {
        if polygon
            .last()
            .is_none_or(|&previous| (points[previous] - *point).length_sq() > distance_epsilon_sq)
        {
            polygon.push(index);
        }
    }
    while polygon.len() > 1
        && polygon
            .first()
            .zip(polygon.last())
            .is_some_and(|(&first, &last)| {
                (points[first] - points[last]).length_sq() <= distance_epsilon_sq
            })
    {
        polygon.pop();
    }

    // Remove forward collinear anchors: they do not change the filled contour but can prevent
    // an otherwise valid ear from being found when the point lies on a candidate diagonal.
    loop {
        if polygon.len() <= 3 {
            break;
        }
        let mut removed = false;
        for current_index in 0..polygon.len() {
            let previous = points[polygon[(current_index + polygon.len() - 1) % polygon.len()]];
            let current = points[polygon[current_index]];
            let next = points[polygon[(current_index + 1) % polygon.len()]];
            let incoming = current - previous;
            let outgoing = next - current;
            if cross_2d(incoming, outgoing).abs() <= area_epsilon
                && incoming.dot(outgoing) >= -distance_epsilon_sq
            {
                polygon.remove(current_index);
                removed = true;
                break;
            }
        }
        if !removed {
            break;
        }
    }

    if polygon.len() < 3 {
        return None;
    }

    let twice_area: f32 = (0..polygon.len())
        .map(|index| {
            let a = points[polygon[index]];
            let b = points[polygon[(index + 1) % polygon.len()]];
            a.x * b.y - b.x * a.y
        })
        .sum();
    if twice_area.abs() <= area_epsilon {
        return None;
    }
    let orientation = twice_area.signum();

    // A self-intersecting contour has no single simple-polygon interior. Leave it unfilled rather
    // than silently painting a convex hull or an arbitrary subset of the bow-tie.
    for first in 0..polygon.len() {
        let first_next = (first + 1) % polygon.len();
        for second in first + 1..polygon.len() {
            let second_next = (second + 1) % polygon.len();
            if first_next == second || second_next == first {
                continue;
            }
            if segments_intersect(
                points[polygon[first]],
                points[polygon[first_next]],
                points[polygon[second]],
                points[polygon[second_next]],
                area_epsilon,
                distance_epsilon,
            ) {
                return None;
            }
        }
    }

    let mut remaining = polygon;
    let mut triangles = Vec::with_capacity(remaining.len() - 2);
    let max_attempts = remaining.len().saturating_mul(remaining.len());
    let mut attempts = 0;
    while remaining.len() > 3 {
        let mut ear = None;
        for current_index in 0..remaining.len() {
            let previous_index = (current_index + remaining.len() - 1) % remaining.len();
            let next_index = (current_index + 1) % remaining.len();
            let a = points[remaining[previous_index]];
            let b = points[remaining[current_index]];
            let c = points[remaining[next_index]];
            if cross_2d(b - a, c - b) * orientation <= area_epsilon {
                continue;
            }
            if remaining
                .iter()
                .enumerate()
                .any(|(candidate_index, &candidate)| {
                    candidate_index != previous_index
                        && candidate_index != current_index
                        && candidate_index != next_index
                        && point_in_triangle(points[candidate], a, b, c, orientation, area_epsilon)
                })
            {
                continue;
            }
            ear = Some((
                current_index,
                [
                    remaining[previous_index],
                    remaining[current_index],
                    remaining[next_index],
                ],
            ));
            break;
        }

        let (current_index, triangle) = ear?;
        triangles.push(triangle);
        remaining.remove(current_index);
        attempts += 1;
        if attempts > max_attempts {
            return None;
        }
    }
    triangles.push([remaining[0], remaining[1], remaining[2]]);
    Some(triangles)
}

pub(crate) fn point_in_polygon(points: &[Vec2], point: Vec2) -> bool {
    if points.len() < 3 {
        return false;
    }
    let bounds_scale = points
        .iter()
        .map(|vertex| (vertex.x - point.x).abs().max((vertex.y - point.y).abs()))
        .fold(MIN_GEOMETRY_SCALE, f32::max);
    let epsilon = bounds_scale * RELATIVE_DISTANCE_EPSILON;
    let mut inside = false;
    for index in 0..points.len() {
        let a = points[index];
        let b = points[(index + 1) % points.len()];
        if point_on_segment(point, a, b, epsilon * bounds_scale, epsilon) {
            return true;
        }
        if (a.y > point.y) != (b.y > point.y) {
            let intersection_x = (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x;
            if point.x < intersection_x {
                inside = !inside;
            }
        }
    }
    inside
}

fn cross_2d(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

fn point_in_triangle(
    point: Vec2,
    a: Vec2,
    b: Vec2,
    c: Vec2,
    orientation: f32,
    epsilon: f32,
) -> bool {
    cross_2d(b - a, point - a) * orientation >= -epsilon
        && cross_2d(c - b, point - b) * orientation >= -epsilon
        && cross_2d(a - c, point - c) * orientation >= -epsilon
}

fn point_on_segment(
    point: Vec2,
    a: Vec2,
    b: Vec2,
    area_epsilon: f32,
    distance_epsilon: f32,
) -> bool {
    cross_2d(b - a, point - a).abs() <= area_epsilon
        && (point - a).dot(point - b) <= distance_epsilon * distance_epsilon
}

fn segments_intersect(
    a: Vec2,
    b: Vec2,
    c: Vec2,
    d: Vec2,
    area_epsilon: f32,
    distance_epsilon: f32,
) -> bool {
    let ab_c = cross_2d(b - a, c - a);
    let ab_d = cross_2d(b - a, d - a);
    let cd_a = cross_2d(d - c, a - c);
    let cd_b = cross_2d(d - c, b - c);
    if ((ab_c > area_epsilon && ab_d < -area_epsilon)
        || (ab_c < -area_epsilon && ab_d > area_epsilon))
        && ((cd_a > area_epsilon && cd_b < -area_epsilon)
            || (cd_a < -area_epsilon && cd_b > area_epsilon))
    {
        return true;
    }
    point_on_segment(c, a, b, area_epsilon, distance_epsilon)
        || point_on_segment(d, a, b, area_epsilon, distance_epsilon)
        || point_on_segment(a, c, d, area_epsilon, distance_epsilon)
        || point_on_segment(b, c, d, area_epsilon, distance_epsilon)
}

#[cfg(test)]
mod tests {
    use super::{PathTriangulationCache, point_in_polygon, triangulate_simple_polygon};
    use eframe::egui::Vec2;
    use std::sync::Arc;

    fn l_shape() -> Vec<Vec2> {
        vec![
            Vec2::new(10.0, 10.0),
            Vec2::new(50.0, 10.0),
            Vec2::new(50.0, 20.0),
            Vec2::new(20.0, 20.0),
            Vec2::new(20.0, 50.0),
            Vec2::new(10.0, 50.0),
        ]
    }

    fn area(points: &[Vec2], triangle: [usize; 3]) -> f32 {
        let [a, b, c] = triangle.map(|index| points[index]);
        cross(b - a, c - a).abs() * 0.5
    }

    fn cross(a: Vec2, b: Vec2) -> f32 {
        a.x * b.y - a.y * b.x
    }

    #[test]
    fn triangulates_concave_contour_and_keeps_notch_outside() {
        let points = l_shape();
        let triangles = triangulate_simple_polygon(&points).expect("simple L contour");
        assert_eq!(triangles.len(), points.len() - 2);
        let covered_area: f32 = triangles
            .iter()
            .copied()
            .map(|triangle| area(&points, triangle))
            .sum();
        assert!((covered_area - 700.0).abs() < 0.001);
        assert!(point_in_polygon(&points, Vec2::new(15.0, 15.0)));
        assert!(!point_in_polygon(&points, Vec2::new(40.0, 40.0)));
    }

    #[test]
    fn triangulation_handles_reversed_winding_and_redundant_anchors() {
        let mut reversed = l_shape();
        reversed.reverse();
        assert_eq!(
            triangulate_simple_polygon(&reversed).map(|triangles| triangles.len()),
            Some(4)
        );

        let redundant = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(2.0, 0.0),
            Vec2::new(4.0, 0.0),
            Vec2::new(4.0, 4.0),
            Vec2::new(0.0, 4.0),
            Vec2::new(0.0, 0.0),
        ];
        assert_eq!(
            triangulate_simple_polygon(&redundant).map(|triangles| triangles.len()),
            Some(2)
        );
    }

    #[test]
    fn degenerate_and_self_intersecting_contours_are_not_filled() {
        let line = vec![Vec2::ZERO, Vec2::new(1.0, 1.0), Vec2::splat(2.0)];
        assert!(triangulate_simple_polygon(&line).is_none());

        let bow_tie = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(10.0, 10.0),
            Vec2::new(0.0, 10.0),
            Vec2::new(10.0, 0.0),
        ];
        assert!(triangulate_simple_polygon(&bow_tie).is_none());
    }

    #[test]
    fn path_triangulation_cache_reuses_geometry_and_invalidates_after_edit() {
        let cache = PathTriangulationCache::default();
        let mut points = l_shape();
        let original = cache.triangulate(&points).expect("simple L contour");
        let reused = cache.triangulate(&points).expect("cached L contour");
        assert!(Arc::ptr_eq(&original, &reused));

        points[1].x += 10.0;
        let edited = cache.triangulate(&points).expect("edited simple L contour");
        assert!(!Arc::ptr_eq(&original, &edited));
    }
}
