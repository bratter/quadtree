// TODO: Should test these, perhaps juat pull in some tests from turf.js
// TODO: Consider a unit overlay to help with conversions

use std::f64::consts::PI;

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

/// Raw haversine formaula to calculate the great-circle distance in radian
/// between a point `p` and a reference point `p1`, both also in radians.
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