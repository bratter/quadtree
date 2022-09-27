use math::*;
use super::*;

/// Module containing spherical coordinate math
pub mod math;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spherical {}
impl System for Spherical {
    type Geometry = Spherical;

    fn dist_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64 {
        dist_pt_pt(p1.as_tuple(), p2.as_tuple())
    }

    fn dist_pt_line(pt: &Point<Self::Geometry>, line: &Segment<Self::Geometry>) -> f64 {
        dist_pt_line(pt.as_tuple(), line.a.as_tuple(), line.b.as_tuple())
    }

    // Calculate Spherical bounds distances.
    // Note that the antimeridian is a problem that is not easy to solve, see:
    // https://macwright.com/2016/09/26/the-180th-meridian.html. All calcs
    // in this module assume that no shape can cross 180 deg lng. Everything
    // must be a separate shape in a composite.
    fn dist_bounds_bounds(b1: &Bounds<Self::Geometry>, b2: &Bounds<Self::Geometry>) -> f64 {
        // Overlap logic works the same as Euclidean
        let overlap_x = b1.x_max() >= b2.x_min() && b2.x_max() >= b1.x_min();
        let overlap_y = b1.y_max() >= b2.y_min() && b2.y_max() >= b1.y_min();

        match (overlap_x, overlap_y) {
            // If any overlap, then 0
            (true, true) => 0.0,
            // If x (lng) overlaps, then find the closest pair of lats and
            // return the difference - no need to run through haversine
            // as latitude math maps directly to radians
            (true, false) => {
                let d1 = (b1.y_min() - b2.y_max()).abs();
                let d2 = (b2.y_min() - b1.y_max()).abs();

                if d1 < d2 { d1 } else { d2 }
            },
            // If y (lat) overlaps, then find the point of overlap with the
            // maximum abs value of lat (closest to the poles) and calc
            // distance for this lat and the respective lngs
            (false, true) => {
                // Point of max overlap is the min of the bounds maxes
                let min_lat = b1.y_min().max(b2.y_min());
                let max_lat = b1.y_max().min(b2.y_max());
                let lat = if min_lat.abs() > max_lat.abs() { min_lat } else { max_lat };

                // Easiest way to adjust for lng wrapping is to take the pair
                // with the min lng delta, because Lng::sub deals with wrapping
                let delta_xa = f64::from(Lng::from(b1.x_max()) - Lng::from(b2.x_min())).abs();
                let delta_xi = f64::from(Lng::from(b1.x_min()) - Lng::from(b2.x_max())).abs();

                if delta_xa < delta_xi {
                    dist_pt_pt((b1.x_max(), lat), (b2.x_min(), lat))
                } else {
                    dist_pt_pt((b1.x_min(), lat), (b2.x_max(), lat))
                }
            },
            // When neither overlaps, take the distance from the closest
            // corners, accounting for wrapping lngs
            (false, false) => {
                // Easiest way to adjust for lng wrapping is to take the pair
                // with the min lng delta, because Lng::sub deals with wrapping
                let delta_xa = f64::from(Lng::from(b1.x_max()) - Lng::from(b2.x_min())).abs();
                let delta_xi = f64::from(Lng::from(b1.x_min()) - Lng::from(b2.x_max())).abs();

                let (x1, x2) = if delta_xa < delta_xi {
                    (b1.x_max(), b2.x_min())
                } else {
                    (b1.x_min(), b2.x_max())
                };
                let (y1, y2) = if b1.y_max() < b2.y_min() {
                    (b1.y_max(), b2.y_min())
                } else {
                    (b1.y_min(), b2.y_max())
                };

                dist_pt_pt((x1, y1), (x2, y2))
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
    use std::f64::consts::*;
    use super::*;

    const EPSILON: f64 = 1e-6;

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
    fn calculate_pt_pt_and_pt_segment_distance() {
        // Create directly in radians
        let p1 = Spherical::point(0.0, 0.0);
        // Create from degrees
        let p2 = Point::from_deg(180.0, 0.0);
        let p3 = Spherical::point(PI, PI / 2.0);

        let seg = Spherical::segment(p1, p2);

        assert_eq!(p1.dist(&p2), PI);
        assert_eq!(p2.dist(&p1), PI);
        assert_eq!(p2.dist(&p3), PI / 2.0);

        assert_eq!(p2.dist(&seg), 0.0);
        assert_eq!(p1.dist(&seg), 0.0);
        assert_eq!(seg.dist(&p3), PI / 2.0);
    }

    #[test]
    fn distance_calc_wraps_lng_bounds() {
        // They equal each other
        let p1 = Spherical::point(7.0 * FRAC_PI_8, 0.0);
        let p2 = Spherical::point(-7.0 * FRAC_PI_8, 0.0);
        assert_eq!(p1.dist(&p2), p2.dist(&p1));

        // They equal something that doesn't wrap
        let p3 = Spherical::point(0.0, 0.0);
        let p4 = Spherical::point(FRAC_PI_4, 0.0);
        assert!(p1.dist(&p2) - p3.dist(&p4) < EPSILON);
    }

    #[test]
    fn bounds_dist_works_for_simple_bounds() {
        // 0.4 is approx pi/8
        let b1 = Bounds::new(
            Spherical::point(0.1, -0.1),
            0.4,
            0.5,
        );

        // Test an overlap
        let b2 = Bounds::new(Spherical::point(0.2, 0.0), 0.8, 0.8);
        assert_eq!(b1.dist(&b2), 0.0);
        
        // Test touching
        let b2 = Bounds::new(Spherical::point(0.4, 0.0), 0.2, 0.2);
        assert_eq!(b1.dist(&b2), 0.0);

        // Test lat above - simple as the distance should just be the delta in radians
        let b2 = Bounds::new(Spherical::point(0.1, 0.6), 0.2, 0.2);
        assert!(b1.dist(&b2) - 0.2 < EPSILON);

        // Test lat below
        let b2 = Bounds::new(Spherical::point(0.1, -0.4), 0.2, 0.1);
        assert!(b1.dist(&b2) - 0.3 < EPSILON);

        // Test lng greater than, min dist @ 0.4, b2 ends higher
        let b2 = Bounds::new(Spherical::point(0.6, 0.2), 0.2, 0.4);
        let d = Spherical::point(0.5, 0.4).dist(&Spherical::point(0.6, 0.4));
        assert!(b1.dist(&b2) - d < EPSILON);
        
        // Test lng greater than, min dist @ -0.1, b2 starts lower
        let b2 = Bounds::new(Spherical::point(0.7, -0.2), 0.1, 0.1);
        let d = Spherical::point(0.5, -0.1).dist(&Spherical::point(0.7, -0.1));
        assert!(b1.dist(&b2) - d < EPSILON);

        // Test lng less than - min dist @ 0.3, b1 ends higher
        let b2 = Bounds::new(Spherical::point(-0.2, 0.0), 0.1, 0.3);
        let d = Spherical::point(0.1, 0.3).dist(&Spherical::point(-0.1, 0.3));
        assert!(b1.dist(&b2) - d < EPSILON);
        
        // Test corner - top left
        let b2 = Bounds::new(Spherical::point(-0.2, -0.3), 0.1, 0.1);
        let d = Spherical::point(0.1, -0.1).dist(&Spherical::point(-0.1, -0.2));
        assert!(b1.dist(&b2) - d < EPSILON);

        // Test corner - top right
        let b2 = Bounds::new(Spherical::point(0.8, -0.4), 0.1, 0.1);
        let d = Spherical::point(0.5, -0.1).dist(&Spherical::point(0.8, -0.3));
        assert!(b1.dist(&b2) - d < EPSILON);

        // Test corner - bottom right
        let b2 = Bounds::new(Spherical::point(0.9, 0.6), 0.1, 0.1);
        let d = Spherical::point(0.5, 0.4).dist(&Spherical::point(0.9, 0.6));
        assert!(b1.dist(&b2) - d < EPSILON);

        // Test corner - bottom left
        let b2 = Bounds::new(Spherical::point(-0.8, 0.7), 0.2, 0.1);
        let d = Spherical::point(0.1, 0.4).dist(&Spherical::point(-0.6, 0.7));
        assert!(b1.dist(&b2) - d < EPSILON);
    }
    
    #[test]
    fn bounds_dist_works_when_on_other_side_of_antimeridian() {
        let b1 = Bounds::new(Spherical::point(2.9, 0.0), 0.1, 0.4);

        // Test overlapping latitude
        let b2 = Bounds::new(Spherical::point(-3.0, 0.1), 0.1, 0.2);
        let d = Spherical::point(3.0, 0.3).dist(&Spherical::point(-3.0, 0.3));
        assert!(b1.dist(&b2) - d < EPSILON);

        // Test corner
        let b2 = Bounds::new(Spherical::point(-2.8, -0.4), 0.1, 0.2);
        let d = Spherical::point(3.0, 0.0).dist(&Spherical::point(-2.8, 0.2));
        assert!(b1.dist(&b2) - d < EPSILON);
    }
}