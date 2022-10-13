use geo::{Rect, Point, HaversineDistance};
use math::*;

/// Module containing spherical coordinate math
pub mod math;

fn p(x: f64, y: f64) -> Point {
    Point::new(x, y)
}

// Calculate Spherical bounds distances.
// Note that the antimeridian is a problem that is not easy to solve, see:
// https://macwright.com/2016/09/26/the-180th-meridian.html. All calcs
// in this module assume that no shape can cross 180 deg lng. Everything
// must be a separate shape in a composite.
pub fn dist_rect_rect(r1: &Rect, r2: &Rect) -> f64 {
    // Overlap logic works the same as Euclidean
    let overlap_x = r1.max().x >= r2.min().x && r2.max().x >= r1.min().x;
    let overlap_y = r1.max().y >= r2.min().y && r2.max().y >= r1.min().y;

    match (overlap_x, overlap_y) {
        // If any overlap, then 0
        (true, true) => 0.0,
        // If x (lng) overlaps, then find the closest pair of lats and
        // return the difference - no need to run through haversine
        // as latitude math maps directly to radians
        (true, false) => {
            let d1 = (r1.min().y - r2.max().y).abs();
            let d2 = (r2.min().y - r1.max().y).abs();

            if d1 < d2 { d1 } else { d2 }
        },
        // If y (lat) overlaps, then find the point of overlap with the
        // maximum abs value of lat (closest to the poles) and calc
        // distance for this lat and the respective lngs
        (false, true) => {
            // Point of max overlap is the min of the bounds maxes
            let min_lat = r1.min().y.max(r2.min().y);
            let max_lat = r1.max().y.min(r2.max().y);
            let lat = if min_lat.abs() > max_lat.abs() { min_lat } else { max_lat };

            // Easiest way to adjust for lng wrapping is to take the pair
            // with the min lng delta, because Lng::sub deals with wrapping
            let delta_xa = f64::from(Lng::from(r1.max().x) - Lng::from(r2.min().x)).abs();
            let delta_xi = f64::from(Lng::from(r1.min().x) - Lng::from(r2.max().x)).abs();

            if delta_xa < delta_xi {
                p(r1.max().x, lat).haversine_distance(&p(r2.min().x, lat))
            } else {
                p(r1.min().x, lat).haversine_distance(&p(r2.max().x, lat))
            }
        },
        // When neither overlaps, take the distance from the closest
        // corners, accounting for wrapping lngs
        (false, false) => {
            // Easiest way to adjust for lng wrapping is to take the pair
            // with the min lng delta, because Lng::sub deals with wrapping
            let delta_xa = f64::from(Lng::from(r1.max().x) - Lng::from(r2.min().x)).abs();
            let delta_xi = f64::from(Lng::from(r1.min().x) - Lng::from(r2.max().x)).abs();

            let (x1, x2) = if delta_xa < delta_xi {
                (r1.max().x, r2.min().x)
            } else {
                (r1.min().x, r2.max().x)
            };
            let (y1, y2) = if r1.max().y < r2.min().y {
                (r1.max().y, r2.min().y)
            } else {
                (r1.min().y, r2.max().y)
            };

            p(x1, y1).haversine_distance(&p(x2, y2))
        },
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::*;
    use geo::{Point, Line};
    use crate::geom::rect::DistHaversine;

    use super::*;

    const EPSILON: f64 = 1e-6;

    // TODO: Remove useless test
    #[test]
    fn can_construct_points_and_line_segments() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);

        let seg = Line::new(p1.0, p2.0);

        assert_eq!(p1.x_y(), (0.0, 0.0));
        assert_eq!(p2.x_y(), (3.0, 4.0));

        assert_eq!(seg.start_point().x_y(), (0.0, 0.0));
        assert_eq!(seg.end_point().x_y(), (3.0, 4.0));
    }

    // TODO: This is likely a useless test
    #[test]
    fn calculate_pt_pt_and_pt_segment_distance() {
        // Create directly in radians
        let p1 = Point::new(0.0, 0.0);
        // Create from degrees
        let p2 = Point::new(180.0, 0.0).to_radians();
        let p3 = Point::new(PI, PI / 2.0);

        let seg = Line::new(p1.0, p2.0);

        assert_eq!(p1.dist_haversine(&p2), PI);
        assert_eq!(p2.dist_haversine(&p1), PI);
        assert_eq!(p2.dist_haversine(&p3), PI / 2.0);

        // TODO: Clean up the tests
        // assert_eq!(p2.dist(&seg), 0.0);
        // assert_eq!(p1.dist(&seg), 0.0);
        // assert_eq!(seg.dist(&p3), PI / 2.0);
    }

    // TODO: Check if this is useless, it probably is, but should checkbehavior of geo
    #[test]
    fn distance_calc_wraps_lng_bounds() {
        // They equal each other
        let p1 = Point::new(7.0 * FRAC_PI_8, 0.0,);
        let p2 = Point::new(-7.0 * FRAC_PI_8, 0.0);
        assert_eq!(p1.dist_haversine(&p2), p2.dist_haversine(&p1));

        // They equal something that doesn't wrap
        let p3 = Point::new(0.0, 0.0);
        let p4 = Point::new(FRAC_PI_4, 0.0,);
        assert!(p1.dist_haversine(&p2) - p3.dist_haversine(&p4) < EPSILON);
    }

    #[test]
    fn rect_dist_works_for_simple_rects() {
        // 0.4 is approx pi/8
        let b1 = Rect::new(
            Point::new(0.1, -0.1),
            Point::new(0.4, 0.5),
        );

        // Test an overlap
        let b2 = Rect::new(Point::new(0.2, 0.0), Point::new(0.8 , 0.8));
        assert_eq!(b1.dist_haversine(&b2), 0.0);
        
        // Test touching
        let b2 = Rect::new(Point::new(0.4, 0.0), Point::new(0.2 , 0.2));
        assert_eq!(b1.dist_haversine(&b2), 0.0);

        // Test lat above - simple as the distance should just be the delta in radians
        let b2 = Rect::new(Point::new(0.1, 0.6), Point::new(0.2 , 0.2));
        assert!(b1.dist_haversine(&b2) - 0.2 < EPSILON);

        // Test lat below
        let b2 = Rect::new(Point::new(0.1, -0.4), Point::new(0.2 , 0.1));
        assert!(b1.dist_haversine(&b2) - 0.3 < EPSILON);

        // Test lng greater than, min dist @ 0.4, b2 ends higher
        let b2 = Rect::new(Point::new(0.6, 0.2), Point::new(0.2 , 0.4));
        let d = Point::new(0.5, 0.4).dist_haversine(&Point::new(0.6, 0.4));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);
        
        // Test lng greater than, min dist @ -0.1, b2 starts lower
        let b2 = Rect::new(Point::new(0.7, -0.2), Point::new(0.1, 0.1));
        let d = Point::new(0.5, -0.1).dist_haversine(&Point::new(0.7, -0.1));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);

        // Test lng less than - min dist @ 0.3, b1 ends higher
        let b2 = Rect::new(Point::new(-0.2, 0.0), Point::new(0.1, 0.3));
        let d = Point::new(0.1, 0.3).dist_haversine(&Point::new(-0.1, 0.3));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);
        
        // Test corner - top left
        let b2 = Rect::new(Point::new(-0.2, -0.3), Point::new(0.1, 0.1));
        let d = Point::new(0.1, -0.1).dist_haversine(&Point::new(-0.1, -0.2));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);

        // Test corner - top right
        let b2 = Rect::new(Point::new(0.8, -0.4), Point::new(0.1, 0.1));
        let d = Point::new(0.5, -0.1).dist_haversine(&Point::new(0.8, -0.3));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);

        // Test corner - bottom right
        let b2 = Rect::new(Point::new(0.9, 0.6), Point::new(0.1, 0.1));
        let d = Point::new(0.5, 0.4).dist_haversine(&Point::new(0.9, 0.6));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);

        // Test corner - bottom left
        let b2 = Rect::new(Point::new(-0.8, 0.7), Point::new(0.2, 0.1));
        let d = Point::new(0.1, 0.4).dist_haversine(&Point::new(-0.6, 0.7));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);
    }
    
    #[test]
    fn rect_dist_works_when_on_other_side_of_antimeridian() {
        let b1 = Rect::new(Point::new(2.9, 0.0), Point::new(0.1, 0.4));

        // Test overlapping latitude
        let b2 = Rect::new(Point::new(-3.0, 0.1), Point::new(0.1, 0.2));
        let d = Point::new(3.0, 0.3).dist_haversine(&Point::new(-3.0, 0.3));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);

        // Test corner
        let b2 = Rect::new(Point::new(-2.8, -0.4), Point::new(0.1, 0.2));
        let d = Point::new(3.0, 0.0).dist_haversine(&Point::new(-2.8, 0.2));
        assert!(b1.dist_haversine(&b2) - d < EPSILON);
    }
}