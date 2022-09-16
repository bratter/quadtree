use self::math::deg_to_rad;

use super::*;

/// Module containing spherical coordinate math
pub mod math;

// TODO: What units should this take and return?
//       Accepting degrees is useful, because that's how lng/lat are usually noted
//       Returning radians is useful because its easy to convert
//       Would ideally be consistent, so starting with accepting radians, returning radians
#[derive(Clone, Copy)]
pub struct Spherical {}
impl System for Spherical {
    type Geometry = Spherical;
}

impl Point<Spherical> {
    pub fn from_deg(lng: f64, lat: f64) -> Point<Spherical> {
        Self::new(deg_to_rad(lng), deg_to_rad(lat))
    }
}

impl Distance<Point<Spherical>> for Point<Spherical> {
    fn dist(&self, cmp: &Point<Spherical>) -> f64 {
        math::dist_to_pt(self.as_tuple(), cmp.as_tuple())
    }
}

impl Distance<Segment<Spherical>> for Point<Spherical> {
    fn dist(&self, cmp: &Segment<Spherical>) -> f64 {
        math::dist_to_line_seg(self.as_tuple(), cmp.a.as_tuple(), cmp.b.as_tuple())
    }
}

impl Distance<Point<Spherical>> for Segment<Spherical> {
    fn dist(&self, cmp: &Point<Spherical>) -> f64 {
        cmp.dist(self)
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
        assert_eq!(p2.dist_rel(&p3), PI / 2.0);

        assert_eq!(p2.dist(&seg), 0.0);
        assert_eq!(p1.dist(&seg), 0.0);
        assert_eq!(seg.dist(&p3), PI / 2.0);
    }
}