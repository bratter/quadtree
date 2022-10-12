use geo::Line;

use super::*;

/// Module containing Euclidean coordinate math
pub mod math;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Euclidean;
impl System for Euclidean {
    type Geometry = Euclidean;

    fn dist_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64 {
        math::dist_pt_pt(p1.x_y(), p2.x_y())
    }

    fn dist_pt_line(pt: &Point<Self::Geometry>, line: &Line) -> f64 {
        math::dist_pt_line(pt.x_y(), line.start_point().x_y(), line.end_point().x_y())
    }

    fn dist_bounds_bounds(b1: &Bounds<Self::Geometry>, b2: &Bounds<Self::Geometry>) -> f64 {
        let overlap_x = b1.x_max() >= b2.x_min() && b2.x_max() >= b1.x_min();
        let overlap_y = b1.y_max() >= b2.y_min() && b2.y_max() >= b1.y_min();

        match (overlap_x, overlap_y) {
            // If there is any overlap, then the distance is zero
            (true, true) => 0.0,
            // When x overlaps, distance is the smallest y-difference,
            // and similarly for y-overlaps
            (true, false) => (b1.y_min() - b2.y_max()).min(b2.y_min() - b1.y_max()),
            (false, true) => (b1.x_min() - b2.x_max()).min(b2.x_min() - b1.x_max()),
            // When neither overlaps, take the distance from closest corners
            (false, false) => {
                let (x1, x2) = if b1.x_max() < b2.x_min() {
                    (b1.x_max(), b2.x_min())
                } else {
                    (b1.x_min(), b2.x_max())
                };
                let (y1, y2) = if b1.y_max() < b2.y_min() {
                    (b1.y_max(), b2.y_min())
                } else {
                    (b1.y_min(), b2.y_max())
                };

                math::dist_pt_pt((x1, y1), (x2, y2))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_construct_points_and_line_segments() {
        let p1 = point!(0.0, 0.0, Euclidean);
        let p2 = point!(3.0, 4.0);

        let seg = Line::new(p1.0, p2.0);

        assert_eq!(p1.x_y(), (0.0, 0.0));
        assert_eq!(p2.x_y(), (3.0, 4.0));

        assert_eq!(seg.start_point().x_y(), (0.0, 0.0));
        assert_eq!(seg.end_point().x_y(), (3.0, 4.0));
    }

    #[test]
    fn calculate_pt_pt_and_pt_segment_distance() {
        let p1 = point!(0.0, 0.0);
        let p2 = point!(3.0, 4.0);
        let p3 = point!(3.0, 0.0);

        let seg = Line::new(p1.0, p2.0);

        assert_eq!(p1.dist(&p2), 5.0);

        // TODO: Clean up tests
        // assert_eq!(p1.dist(&seg), 0.0);
        // assert_eq!(seg.dist(&p2), 4.0);
    }
}