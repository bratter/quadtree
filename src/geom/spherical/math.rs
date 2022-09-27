use std::f64::consts::PI;
use std::ops::{Add, Sub};

type Pt = (f64, f64);

/// Radius of the Earth in meters.
pub const EARTH_RADIUS: f64 = 6371008.8;

pub fn deg_to_rad(deg: f64) -> f64 {
    (deg % 360.0) * PI / 180.0
}

pub fn rad_to_deg(rad: f64) -> f64 {
    let pi2 = 2.0 * PI;
    (rad % pi2) * 180.0 / PI
}

#[derive(Debug, Clone, Copy)]
pub struct Lng(f64);

impl From<f64> for Lng {
    fn from(n: f64) -> Self {
        // Must be in radians in the domain [-Pi, Pi]
        let n = (n + PI) % (2.0 * PI);
        Lng(n - (n.signum() * PI))
    }
}

impl From<Lng> for f64 {
    fn from(n: Lng) -> Self {
        n.0
    }
}

impl PartialEq for Lng {
    // Equal if the underlying f64 are equal, or if they are on PI/-PI
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 || self.0.abs() == PI && other.0.abs() == PI
    }
}

impl Add for Lng {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Lng::from(self.0 + rhs.0)  
    }
}

impl Sub for Lng {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Lng::from(self.0 - rhs.0)
    }
}

/// Raw haversine formaula to calculate the great-circle distance in radian
/// between a point `p1` and a reference point `p2`, both also in radians.
/// Because all inputs and outputs are radians, this formula is unitless.
/// 
/// Input points must be provided as `(lng, lat)` radian tuples.
/// 
/// Haversine formula adapted from:
/// https://github.com/Turfjs/turf/blob/master/packages/turf-distance/index.ts
pub fn haversine(p1: Pt, p2: Pt) -> f64 {
    let d_lng = p2.0 - p1.0;
    let d_lat = p2.1 - p1.1;
    let lat_p1 = p1.1;
    let lat_p2 = p2.1;

    let a =
        (d_lat / 2.0).sin().powi(2) +
        (d_lng / 2.0).sin().powi(2) * lat_p1.cos() * lat_p2.cos();

    2.0 * a.sqrt().atan2((1.0 - a).sqrt())
}

/// Calculate the great circle distance between a point `p`and a second
/// reference point `p1`. Inputs and outputs are in radians. Convert radians
/// to any distance unit by multiplying by the radius.
/// 
/// Inputs must be provided as `(lng, lat)` radian tuples.
/// 
/// Squared versions are not required as there is no final `.sqrt()` in the
/// haversine formula.
pub fn dist_pt_pt(p: Pt, p1: Pt) -> f64 {
    haversine(p, p1)
}

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
pub fn dist_pt_line(p: Pt, p1: Pt, p2: Pt) -> f64 {
    // Projection logic is identical to the euclidean case,
    // but distance calc is different
    let (x, y) = p;
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let (a, b, c, d) = (x - x1, y - y1, x2 - x1, y2 - y1);

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;

    // Wrap in an `if` to account for a zero line length
    // Just has to be <0 to work so we pick distance to p1
    let param = if len_sq == 0.0 { -1.0 } else { dot / len_sq };

    if param < 0.0 {
        // Closest to p1, so reduces to pt-pt
        haversine(p, p1)
    } else if param > 1.0 {
        // Closest to p2, so pt-pt again
        haversine(p, p2)
    } else {
        // Here we project onto the segment
        let projected = (x1 + param * c, y1 + param * d);
        haversine(p, projected)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::*;
    use super::*;

    #[test]
    fn create_eq_add_subtract_lngs() {
        // Into works for f64, from works for Lng
        let l1: Lng = FRAC_PI_2.into();
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
}