use std::f64::consts::PI;
use std::ops::{Add, Sub};
use geo::{Point, Line, Rect, HaversineDistance, CoordNum, CoordFloat};
use num_traits::FromPrimitive;

/// Helper function for making points
fn p<T>(x: T, y: T) -> Point<T>
where
    T: CoordNum
{
    Point::new(x, y)
}

#[derive(Debug, Clone, Copy)]
pub struct Lng<T>(T)
where
    T: CoordFloat;

impl<T> From<T> for Lng<T>
where
    T: CoordFloat,
{
    fn from(n: T) -> Self {
        let pi = T::from(PI).unwrap();
        // Must be in radians in the domain [-Pi, Pi]
        let n = (n + pi) % (T::from(2).unwrap() * pi);
        Lng(n - (n.signum() * pi))
    }
}

impl<T> From<Lng<T>> for f64
where
    T: CoordFloat,
{
    fn from(n: Lng<T>) -> Self {
        n.0.to_f64().unwrap()
    }
}

impl<T> PartialEq for Lng<T>
where
    T: CoordFloat,
{
    // Equal if the underlying f64 are equal, or if they are on PI/-PI
    fn eq(&self, other: &Self) -> bool {
        let pi = T::from(PI).unwrap();
        self.0 == other.0 || self.0.abs() == pi && other.0.abs() == pi
    }
}

impl<T> Add for Lng<T>
where
    T: CoordFloat,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Lng::from(self.0 + rhs.0)  
    }
}

impl<T> Sub for Lng<T>
where
    T: CoordFloat,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Lng::from(self.0 - rhs.0)
    }
}

// TODO: Update doc comments
/// Calculate the great circle distance between a point `p` and a line segment
/// defined by its two endpoints `p1` and `p2`. Inputs and outputs are in
/// radians. Convert radians to any distance unit by multiplying by the radius.
/// 
/// Inputs must be provided as `(lng, lat)` radian tuples.
/// 
/// Squared versions are not required as there is no final `.sqrt()` in the
/// haversine formula.
/// 
/// Adapted from: https://github.com/Turfjs/turf/blob/master/packages/turf-point-to-line-distance/index.ts
pub fn dist_pt_line<T>(pt: &Point<T>, line: &Line<T>) -> T
where
    T: CoordFloat + FromPrimitive
{
    // Projection logic is identical to the euclidean case,
    // but distance calc is different
    let (x, y) = pt.x_y();
    let (x1, y1) = line.start_point().x_y();
    let (x2, y2) = line.end_point().x_y();

    let (a, b, c, d) = (x - x1, y - y1, x2 - x1, y2 - y1);

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;

    // Wrap in an `if` to account for a zero line length
    // Just has to be <0 to work so we pick distance to p1
    let param = if len_sq == T::zero() { -T::one() } else { dot / len_sq };

    if param < T::zero() {
        // Closest to start point, so reduces to pt-pt
        pt.haversine_distance(&line.start_point())
    } else if param > T::one() {
        // Closest to end point, so pt-pt again
        pt.haversine_distance(&line.end_point())
    } else {
        // Here we project onto the segment
        let projected = p(x1 + param * c, y1 + param * d);
        pt.haversine_distance(&projected)
    }
}

// Calculate Spherical bounds distances.
// Note that the antimeridian is a problem that is not easy to solve, see:
// https://macwright.com/2016/09/26/the-180th-meridian.html. All calcs
// in this module assume that no shape can cross 180 deg lng. Everything
// must be a separate shape in a composite.
pub fn dist_rect_rect<T>(r1: &Rect<T>, r2: &Rect<T>) -> T
where
    T: CoordFloat + FromPrimitive
{
    // Overlap logic works the same as Euclidean
    let overlap_x = r1.max().x >= r2.min().x && r2.max().x >= r1.min().x;
    let overlap_y = r1.max().y >= r2.min().y && r2.max().y >= r1.min().y;

    match (overlap_x, overlap_y) {
        // If any overlap, then 0
        (true, true) => T::zero(),
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
    use geo::{Point, Rect};
    use crate::DistHaversine;

    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn create_eq_add_subtract_lngs() {
        // Into works for f64, from works for Lng
        let l1 = Lng::from(FRAC_PI_2);
        let l2 = Lng::from(-FRAC_PI_2);

        // Basic equals works
        assert_eq!(FRAC_PI_2, l1.into());
        assert_eq!(-FRAC_PI_2, l2.into());

        // Wrap into +-PI
        let l3 = Lng::from(3.0 * PI / 2.0);
        assert_eq!(-FRAC_PI_2, l3.into());

        // -PI == PI, and implements PartialEq
        let l4 = Lng::from(PI);
        let l5 = Lng::from(-PI);
        assert_eq!(l4, l5);

        // Can add and subtract in a wrapping manner
        let l7 = Lng::from(FRAC_PI_4);
        let l8 = Lng::from(FRAC_PI_4 * 3.0);
        assert_eq!(l1 + l7, l8);
        assert_eq!(l4 + l1, l2);
        assert_eq!(l1 - l7, l7);
        assert_eq!(l7 - l1, Lng::from(-FRAC_PI_4));

        // Check that wrappung subtraction works for large negatives
        // Using an approximate match to avoid floating point issues
        let res: f64 = (Lng::from(-7.0 * FRAC_PI_8) - Lng::from(7.0 * FRAC_PI_8) - Lng::from(FRAC_PI_4)).into();
        assert!(res < 1e-6);
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