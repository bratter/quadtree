// Module declarations
pub mod euclidean;
pub mod spherical;
mod bounds;

use std::marker::PhantomData;

// Re-export the coordinate system for easier access
// Should be the only thing commonly used, everything else can be accessed from
// the namespace itself
pub use euclidean::Euclidean;
pub use spherical::Spherical;

// Re-export everything from bounds so its available under the same namespace
pub use bounds::*;

pub trait System: core::fmt::Debug + Clone + Copy + PartialEq {
    type Geometry;

    fn point(x: f64, y: f64) -> Point<Self::Geometry> {
        Point { x, y, geometry: PhantomData }
    }

    fn segment(a: Point<Self::Geometry>, b: Point<Self::Geometry>) -> Segment<Self::Geometry> {
        Segment { a, b, geometry: PhantomData }
    }

    fn dist_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64;

    fn dist_pt_line(pt: &Point<Self::Geometry>, line: &Segment<Self::Geometry>) -> f64;

    fn dist_bounds_bounds(b1: &Bounds<Self::Geometry>, b2: &Bounds<Self::Geometry>) -> f64;
}

/// Flexible trait to calculate the distance between two objects.
pub trait Distance<T> {
    /// Calculate the distance between two items.
    fn dist(&self, cmp: &T) -> f64;

    // TODO: Consider re-introducing relative distances
    // Relative distance measure. Does not mean anything on its own, but the
    // result will be correctly ordered relative to other calls to the same
    // function. Does not come for free as it becomes too dangerous, but
    // can be implemented simply by delegating to `self.dist()` when there is
    // not a more efficient implementation, e.g. euclidean distance squared
    // avoids an expensive square root.
    // fn dist_rel(&self, cmp: &T) -> f64;
}

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

impl <Geom: System<Geometry = Geom>> Distance<Point<Geom>> for Point<Geom> {
    fn dist(&self, cmp: &Point<Geom>) -> f64 {
        Geom::dist_pt_pt(self, cmp)
    }
}

impl <Geom: System<Geometry = Geom>> Distance<Segment<Geom>> for Point<Geom> {
    fn dist(&self, cmp: &Segment<Geom>) -> f64 {
        Geom::dist_pt_line(self, cmp)
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

impl <Geom: System<Geometry = Geom>> Distance<Point<Geom>> for Segment<Geom> {
    fn dist(&self, cmp: &Point<Geom>) -> f64 {
        Geom::dist_pt_line(cmp, self)
    }
}