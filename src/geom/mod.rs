use std::marker::PhantomData;

// TODO: Possible to simplfy the generics?
pub mod euclidean;
pub mod spherical;

// Re-export the coordinate system for easier access
// Should be the only thing commonly used
pub use euclidean::Euclidean;
pub use spherical::Spherical;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<Geom> {
    x: f64,
    y: f64,
    geometry: PhantomData<Geom>,
}

impl <Geom: System> Point<Geom> {
    pub fn new(x: f64, y: f64) -> Point<Geom> {
        Point { x, y, geometry: PhantomData }
    }

    pub fn as_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment<Geom> {
    a: Point<Geom>,
    b: Point<Geom>,
    geometry: PhantomData<Geom>,
}

impl <Geom: System> Segment<Geom> {
    pub fn new(a: Point<Geom>, b: Point<Geom>) -> Segment<Geom> {
        Segment { a, b, geometry: PhantomData }
    }
}

pub trait System: core::fmt::Debug + Clone + Copy + PartialEq {
    type Geometry;

    fn point(x: f64, y: f64) -> Point<Self::Geometry> {
        Point { x, y, geometry: PhantomData }
    }

    fn segment(a: Point<Self::Geometry>, b: Point<Self::Geometry>) -> Segment<Self::Geometry> {
        Segment { a, b, geometry: PhantomData }
    }

    fn dist_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64;

    fn dist_rel_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64 {
        Self::dist_pt_pt(p1, p2)
    }

    fn dist_pt_line(pt: &Point<Self::Geometry>, line: &Segment<Self::Geometry>) -> f64;

    fn dist_rel_pt_line(pt: &Point<Self::Geometry>, line: &Segment<Self::Geometry>) -> f64 {
        Self::dist_pt_line(pt, line)
    }
}

/// Flexible trait to calculate the distance between two objects.
pub trait Distance<T> {
    /// Calculate the distance between two items.
    fn dist(&self, cmp: &T) -> f64;

    /// Relative distance measure. Does not mean anything on its own, but the
    /// result will be correctly ordered relative to other calls to the same
    /// function. Does not come for free as it becomes too dangerous, but
    /// can be implemented simply by delegating to `self.dist()` when there is
    /// not a more efficient implementation, e.g. euclidean distance squared
    /// avoids an expensive square root.
    fn dist_rel(&self, cmp: &T) -> f64;
}

impl <Geom: System<Geometry = Geom>> Distance<Point<Geom>> for Point<Geom> {
    fn dist(&self, cmp: &Point<Geom>) -> f64 {
        Geom::dist_pt_pt(self, cmp)
    }

    fn dist_rel(&self, cmp: &Point<Geom>) -> f64 {
        Geom::dist_rel_pt_pt(self, cmp)
    }
}

impl <Geom: System<Geometry = Geom>> Distance<Point<Geom>> for Segment<Geom> {
    fn dist(&self, cmp: &Point<Geom>) -> f64 {
        Geom::dist_pt_line(cmp, self)
    }

    fn dist_rel(&self, cmp: &Point<Geom>) -> f64 {
        Geom::dist_rel_pt_line(cmp, self)
    }
}

impl <Geom: System<Geometry = Geom>> Distance<Segment<Geom>> for Point<Geom> {
    fn dist(&self, cmp: &Segment<Geom>) -> f64 {
        Geom::dist_pt_line(self, cmp)
    }

    fn dist_rel(&self, cmp: &Segment<Geom>) -> f64 {
        Geom::dist_rel_pt_line(self, cmp)
    }
}

// TODO: Make these private and provide accessors?
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

    // TODO: Should this be a trait?
    pub fn contains(&self, pt: Point<Geom>) -> bool {
        let (x1, y1) = self.origin.as_tuple();
        let (x2, y2) = (x1 + self.width, y1 + self.height);
        let (x, y) = pt.as_tuple();

        x >= x1 && x <= x2 && y >= y1 && x <= y2
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

    fn dist_rel(&self, cmp: &Point<Geom>) -> f64 {
        let (x, y) = cmp.as_tuple();
        let [top, right, bottom, left] = self.segments();
        
        if x < self.x_min() { left.dist_rel(cmp) }
        else if x > self.x_max() { right.dist_rel(cmp) }
        else if y < self.y_min() { top.dist_rel(cmp) }
        else if y > self.y_max() { bottom.dist_rel(cmp) }
        else { 0.0 }
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