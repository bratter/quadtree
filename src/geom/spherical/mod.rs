use math::deg_to_rad;

use super::*;

/// Module containing spherical coordinate math
pub mod math;

// TODO: What units should this take and return?
//       Accepting degrees is useful, because that's how lng/lat are usually noted
//       Returning radians is useful because its easy to convert
//       Would ideally be consistent, so starting with accepting radians, returning radians
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spherical {}
impl System for Spherical {
    type Geometry = Spherical;

    fn dist_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64 {
        math::dist_pt_pt(p1.as_tuple(), p2.as_tuple())
    }

    fn dist_pt_line(pt: &Point<Self::Geometry>, line: &Segment<Self::Geometry>) -> f64 {
        math::dist_pt_line(pt.as_tuple(), line.a.as_tuple(), line.b.as_tuple())
    }

    fn dist_bounds_bounds(b1: &Bounds<Self::Geometry>, b2: &Bounds<Self::Geometry>) -> f64 {
        // Overlap logic works the same as Euclidean
        // TODO: Does overlap_x, or any comparison depend on longitude wrapping?
        //       For longitude differences, perhaps subtract Pi if long diff is > Pi
        //       For longitude comparisons
        let overlap_x = b1.x_max() >= b2.x_min() && b2.x_max() >= b1.y_min();
        let overlap_y = b1.y_max() >= b2.y_min() && b2.y_max() >= b1.y_min();

        match (overlap_x, overlap_y) {
            // If any overlap, then 0
            (true, true) => 0.0,
            // If x (lng) overlaps, then find the closest pair of lats and
            // calculate distance for any lng, so choose 0 for simplicity
            (true, false) => {
                if b1.y_min() - b2.y_max() < b2.y_min() - b1.y_max() {
                    math::dist_pt_pt((0.0, b1.y_min()), (0.0, b2.y_max()))
                } else {
                    math::dist_pt_pt((0.0, b2.y_min()), (0.0, b1.y_max()))
                }
            },
            // If y (lat) overlaps, then find the point of overlap with the
            // maximum abs value of lat (closest to the poles) and calc
            // distance for this lat and the respective lngs
            (false, true) => {
                // Point of max overlap is the min of the bounds maxes
                let min_lat = b1.y_min().max(b2.y_min());
                let max_lat = b1.y_max().min(b2.y_max());
                let lat = if min_lat.abs() > max_lat.abs() { min_lat } else { max_lat };

                // TODO: Does this need to be adjusted for longitude wrapping?
                if b1.x_min() - b2.x_max() < b2.x_min() - b1.x_max() {
                    math::dist_pt_pt((b1.x_min(), lat), (b2.x_max(), lat))
                } else {
                    math::dist_pt_pt((b2.x_min(), lat), (b1.x_max(), lat))
                }
            },
            // When neither overlaps, take the distance from the closest
            // corners, accounting for wrapping lngs
            (false, false) => {
                // TODO: Adjust for longitude wrapping
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
            },
        }
    }
}

impl Point<Spherical> {
    pub fn from_deg(lng: f64, lat: f64) -> Point<Spherical> {
        Self::new(deg_to_rad(lng), deg_to_rad(lat))
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn can_construct_points_and_line_segments() {
        let p1 = Spherical::point(0.0, 0.0);
        let p2 = Spherical::point(3.0, 4.0);

        let seg = Spherical::segment(p1, p2);

        assert_eq!(p1.as_tuple(), (0.0, 0.0));
        assert_eq!(p2.as_tuple(), (3.0, 4.0));

        assert_eq!(seg.a.as_tuple(), (0.0, 0.0));
        assert_eq!(seg.b.as_tuple(), (3.0, 4.0));
    }

    #[test]
    fn calculate_pt_pt_and_pt_segment_distance_and_rel() {
        // Create directly in radians
        let p1 = Spherical::point(0.0, 0.0);
        // Create from degrees
        let p2 = Point::from_deg(180.0, 0.0);
        let p3 = Spherical::point(PI, PI / 2.0);

        let seg = Spherical::segment(p1, p2);

        assert_eq!(p1.dist(&p2), PI);
        assert_eq!(p2.dist(&p1), PI);
        assert_eq!(p2.dist(&p3), PI / 2.0);
        assert_eq!(p2.dist_rel(&p3), PI / 2.0);

        assert_eq!(p2.dist(&seg), 0.0);
        assert_eq!(p1.dist(&seg), 0.0);
        assert_eq!(seg.dist(&p3), PI / 2.0);
    }
}