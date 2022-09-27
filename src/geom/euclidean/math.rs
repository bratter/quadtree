type Pt = (f64, f64);

/// Calculate the square of the distance between a point `p` and reference
/// point `p1`. This version producing the square is provided to avoid an
/// expensive `.sqrt()` operation when not required.
pub fn dist_sq_pt_pt(p: Pt, p1: Pt) -> f64 {
    let (x, y) = p;
    let (x1, y1) = p1;

    (x - x1).powi(2) + (y - y1).powi(2)
}

/// Calculate the distance between a point `p` and a second reference point
/// `p1`.
pub fn dist_pt_pt(p: Pt, p1: Pt) -> f64 {
    dist_sq_pt_pt(p, p1).sqrt()
}

/// Calculate the square of the distance between a point `p` and a line segment
/// defined by its two endpoints `p1` and `p2` using vector projection. This
/// version producing the square is provised to avoid an expensive `.sqrt()`
/// operation when not required.
/// 
/// Base version from https://stackoverflow.com/a/6853926
pub fn dist_sq_pt_line(p: Pt, p1: Pt, p2: Pt) -> f64 {
    let (x, y) = p;
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let (a, b, c, d) = (x - x1, y - y1, x2 - x1, y2 - y1);

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;

    // Wrap in an `if` to account for a zero line length
    // Just has to be <0 to work so we pick distance to p1
    let param = if len_sq == 0.0 { -1.0 } else { dot / len_sq };

    let (xx, yy) = if param < 0.0 {
        // Closest to p1, so reduces to pt-pt
        (x1, y1)
    } else if param > 1.0 {
        // Closest to p2, so pt-pt again
        (x2, y2)
    } else {
        // Here we project onto the segment
        (x1 + param * c, y1 + param * d)
    };

    (x - xx).powi(2) + (y - yy).powi(2)
}

/// Calculate the distance between a point `p` and a line segment defined by
/// its two endpoints `p1` and `p2`.
pub fn dist_pt_line(p: Pt, p1: Pt, p2: Pt) -> f64 {
    dist_sq_pt_line(p, p1, p2).sqrt()
}
