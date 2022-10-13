use geo::{Rect, coord, EuclideanDistance};

/// Module containing Euclidean coordinate math
pub mod math;

pub fn dist_rect_rect(r1: &Rect, r2: &Rect) -> f64 {
    let overlap_x = r1.max().x >= r2.min().x && r2.max().x >= r1.min().x;
    let overlap_y = r1.max().y >= r2.min().y && r2.max().y >= r1.min().y;

    match (overlap_x, overlap_y) {
        // If there is any overlap, then the distance is zero
        (true, true) => 0.0,
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

            coord!(x: x1, y: y1).euclidean_distance(&coord!(x: x2, y: y2))
        }
    }
}

// TODO: Add dist rect rect tests, potentially remove ones not needed because of geo
//       The current tests are basically useless as they only test geo functionality
#[cfg(test)]
mod tests {
    use geo::{Point, Line, EuclideanDistance};

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

    #[test]
    fn calculate_pt_pt_and_pt_segment_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        let p3 = Point::new(3.0, 0.0);

        let seg = Line::new(p1.0, p2.0);

        assert_eq!(p1.euclidean_distance(&p2), 5.0);

        // TODO: Clean up tests
        assert_eq!(p1.euclidean_distance(&seg), 0.0);
        assert_eq!(seg.euclidean_distance(&p2), 4.0);
    }
}