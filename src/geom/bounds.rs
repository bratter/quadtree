use super::*;

#[derive(Debug)]
pub struct Bounds<Geom> {
    origin: Point<Geom>,
    width: f64,
    height: f64,
}

impl <Geom: System> Bounds<Geom> {
    pub fn new(origin: Point<Geom>, width: f64, height: f64) -> Bounds<Geom> {
        Self::from_origin(origin, width, height)
    }

    pub fn from_points(a: Point<Geom>, b: Point<Geom>) -> Bounds<Geom> {
        let x = if a.x < b.x { a.x } else { b.x };
        let y = if a.y < b.y { a.y } else { b.y };
        let width = (b.x - a.x).abs();
        let height = (b.y - a.y).abs();
        
        Bounds { origin: Point::new(x, y), width, height }
    }

    pub fn from_origin(origin: Point<Geom>, width: f64, height: f64) -> Bounds<Geom> {
        Bounds { origin, width, height }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn x_min(&self) -> f64 {
        self.origin.x
    }

    pub fn y_min(&self) -> f64 {
        self.origin.y
    }

    pub fn x_max(&self) -> f64 {
        self.origin.x + self.width
    }

    pub fn y_max(&self) -> f64 {
        self.origin.y + self.height
    }

    pub fn points(&self) -> [Point<Geom>; 4] {
        let (x, y) = self.origin.as_tuple();

        [
            Point::new(x, y),
            Point::new(x + self.width, y),
            Point::new(x + self.width, y + self.height),
            Point::new(x, y + self.height),
        ]
    }

    pub fn segments(&self) -> [Segment<Geom>; 4] {
        let [tl, tr, br, bl] = self.points();

        [
            Segment::new(tl, tr),
            Segment::new(tr, br),
            Segment::new(br, bl),
            Segment::new(bl, tl),
        ]
    }

    // TODO: Should this be a trait? Then can have multiple implementations - point, segment, bounds
    pub fn contains(&self, pt: Point<Geom>) -> bool {
        let (x1, y1) = self.origin.as_tuple();
        let (x2, y2) = (x1 + self.width, y1 + self.height);
        let (x, y) = pt.as_tuple();

        x >= x1 && x <= x2 && y >= y1 && x <= y2
    }

    /// Checks that the passed bounds is completely contained by this bounds.
    pub fn contains_bounds(&self, bounds: Bounds<Geom>) -> bool {
        bounds.x_min() >= self.x_min()
        && bounds.x_max() <= self.x_max()
        && bounds.y_min() >= self.y_min()
        && bounds.y_max() <= self.y_max()
    }
}

impl <Geom: System<Geometry = Geom>> Distance<Point<Geom>> for Bounds<Geom> {
    fn dist(&self, cmp: &Point<Geom>) -> f64 {
        let (x, y) = cmp.as_tuple();
        let [top, right, bottom, left] = self.segments();
        
        if x < self.x_min() { left.dist(cmp) }
        else if x > self.x_max() { right.dist(cmp) }
        else if y < self.y_min() { top.dist(cmp) }
        else if y > self.y_max() { bottom.dist(cmp) }
        else { 0.0 }
    }
}

impl <Geom: System<Geometry = Geom>> Distance<Bounds<Geom>> for Bounds<Geom> {
    fn dist(&self, cmp: &Bounds<Geom>) -> f64 {
        Geom::dist_bounds_bounds(self, cmp)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use euclidean::*;
    use spherical::*;

    #[test]
    fn build_euclidean_bounds() {
        // Can construct points either way
        let p1 = Euclidean::point(0.0, 0.0);
        let p2 = Point::<Euclidean>::new(3.0, 4.0);

        let b1 = Bounds::new(p1, 1.0, 2.0);
        let b2 = Bounds::from_points(p2, p1);

        assert_eq!(
            b1.points().map(|p| p.as_tuple()),
            [(0.0, 0.0), (1.0, 0.0), (1.0, 2.0), (0.0, 2.0)]
        );

        assert_eq!(
            b2.points().map(|p| p.as_tuple()),
            [(0.0, 0.0), (3.0, 0.0), (3.0, 4.0), (0.0, 4.0)]
        );

        // This won't compile because the types don't match
        // let sp = Point::<Spherical>::new(0.0, 0.0);
        // assert_eq!(p1.dist(&sp), 0.0);

        // Also testing the PartialEq
        let segs = b1.segments();
        assert_eq!(segs[0].a, Point::new(0.0, 0.0));
        assert_eq!(segs[0].b, Point::new(1.0, 0.0));
    }

    #[test]
    fn build_spherical_bounds() {
        // Can construct points either way
        let p1 = Spherical::point(0.0, 0.0);
        let p2 = Point::<Spherical>::new(PI, -PI / 2.0);

        let b1 = Bounds::new(p1, PI / 2.0, PI / 4.0);
        let b2 = Bounds::from_points(p2, p1);

        assert_eq!(
            b1.points().map(|p| p.as_tuple()),
            [(0.0, 0.0), (PI / 2.0, 0.0), (PI / 2.0, PI / 4.0), (0.0, PI / 4.0)]
        );

        assert_eq!(
            b2.points().map(|p| p.as_tuple()),
            [(0.0, -PI / 2.0), (PI, -PI / 2.0), (PI, 0.0), (0.0, 0.0)]
        );
    }
}