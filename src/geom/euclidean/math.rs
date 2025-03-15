use geo::{Distance, Euclidean, Rect, coord};

use crate::geom::QtFloat;

/// Calculate the euclidean distance between two [`Rect`]'s.
pub fn dist_rect_rect<T>(r1: &Rect<T>, r2: &Rect<T>) -> T
where
    T: QtFloat,
{
    let overlap_x = r1.max().x >= r2.min().x && r2.max().x >= r1.min().x;
    let overlap_y = r1.max().y >= r2.min().y && r2.max().y >= r1.min().y;

    match (overlap_x, overlap_y) {
        // If there is any overlap, then the distance is zero
        (true, true) => T::zero(),
        // When x overlaps, distance is the smallest y-difference,
        // and similarly for y-overlaps
        (true, false) => (r1.min().y - r2.max().y).min(r2.min().y - r1.max().y),
        (false, true) => (r1.min().x - r2.max().x).min(r2.min().x - r1.max().x),
        // When neither overlaps, take the distance from closest corners
        (false, false) => {
            let (x1, x2) = if r1.max().x < r2.min().x {
                (r1.max().x, r2.min().x)
            } else {
                (r1.min().x, r2.max().x)
            };
            let (y1, y2) = if r1.max().y < r2.min().y {
                (r1.max().y, r2.min().y)
            } else {
                (r1.min().y, r2.max().y)
            };

            Euclidean::distance(coord!(x: x1, y: y1), coord!(x: x2, y: y2))
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO: Write tests for rect-rect dist
}
