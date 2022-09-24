use super::*;

/// Module containing Euclidean coordinate math
pub mod math;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Euclidean;
impl System for Euclidean {
    type Geometry = Euclidean;

    fn dist_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64 {
        math::dist_pt_pt(p1.as_tuple(), p2.as_tuple())
    }

    fn dist_rel_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64 {
        math::dist_sq_pt_pt(p1.as_tuple(), p2.as_tuple())
    }

    fn dist_pt_line(pt: &Point<Self::Geometry>, line: &Segment<Self::Geometry>) -> f64 {
        math::dist_pt_line(pt.as_tuple(), line.a.as_tuple(), line.b.as_tuple())
    }

    fn dist_rel_pt_line(pt: &Point<Self::Geometry>, line: &Segment<Self::Geometry>) -> f64 {
        math::dist_sq_pt_line(pt.as_tuple(), line.a.as_tuple(), line.b.as_tuple())
    }

    fn dist_bounds_bounds(b1: &Bounds<Self::Geometry>, b2: &Bounds<Self::Geometry>) -> f64 {
        let overlap_x = b1.x_max() >= b2.x_min() && b2.x_max() >= b1.y_min();
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
        let p1 = Euclidean::point(0.0, 0.0);
        let p2 = Euclidean::point(3.0, 4.0);

        let seg = Euclidean::segment(p1, p2);

        assert_eq!(p1.as_tuple(), (0.0, 0.0));
        assert_eq!(p2.as_tuple(), (3.0, 4.0));

        assert_eq!(seg.a.as_tuple(), (0.0, 0.0));
        assert_eq!(seg.b.as_tuple(), (3.0, 4.0));
    }

    #[test]
    fn calculate_pt_pt_and_pt_segment_distance_and_rel() {
        let p1 = Euclidean::point(0.0, 0.0);
        let p2 = Euclidean::point(3.0, 4.0);
        let p3 = Euclidean::point(3.0, 0.0);

        let seg = Euclidean::segment(p1, p3);

        assert_eq!(p1.dist(&p2), 5.0);
        assert_eq!(p1.dist_rel(&p3), 9.0);

        assert_eq!(p1.dist(&seg), 0.0);
        assert_eq!(seg.dist(&p2), 4.0);
    }
}