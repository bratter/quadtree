use super::*;

/// Module containing Euclidean coordinate math
pub mod math;

#[derive(Clone, Copy)]
pub struct Euclidean;
impl System for Euclidean {
    type Geometry = Euclidean;
}

impl Distance<Point<Euclidean>> for Point<Euclidean> {
    fn dist(&self, cmp: &Point<Euclidean>) -> f64 {
        math::dist_to_pt(self.as_tuple(), cmp.as_tuple())
    }

    fn dist_rel(&self, cmp: &Point<Euclidean>) -> f64 {
        math::dist_sq_to_pt(self.as_tuple(), cmp.as_tuple())
    }
}

impl Distance<Segment<Euclidean>> for Point<Euclidean> {
    fn dist(&self, cmp: &Segment<Euclidean>) -> f64 {
        math::dist_to_line_seg(
            self.as_tuple(),
            cmp.a.as_tuple(),
            cmp.b.as_tuple(),
        )
    }

    fn dist_rel(&self, cmp: &Segment<Euclidean>) -> f64 {
        math::dist_sq_to_line_seg(
            self.as_tuple(),
            cmp.a.as_tuple(),
            cmp.b.as_tuple(),
        )
    }
}

impl Distance<Point<Euclidean>> for Segment<Euclidean> {
    fn dist(&self, cmp: &Point<Euclidean>) -> f64 {
        cmp.dist(self)
    }

    fn dist_rel(&self, cmp: &Point<Euclidean>) -> f64 {
        cmp.dist(self)
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