// Module declarations
pub mod euclidean;
pub mod spherical;
mod bounds;
mod point;
pub mod rect;

use geo;

// Re-export the coordinate system for easier access
// Should be the only thing commonly used, everything else can be accessed from
// the namespace itself
pub use euclidean::Euclidean;
pub use spherical::Spherical;

// Re-export everything from bounds so its available under the same namespace
pub use bounds::*;

pub use point::*;
// pub use segment::*;

pub trait System: core::fmt::Debug + Clone + Copy + PartialEq {
    type Geometry;

    fn dist_pt_pt(p1: &Point<Self::Geometry>, p2: &Point<Self::Geometry>) -> f64;

    fn dist_pt_line(pt: &Point<Self::Geometry>, line: &geo::Line) -> f64;

    fn dist_bounds_bounds(b1: &Bounds<Self::Geometry>, b2: &Bounds<Self::Geometry>) -> f64;
}

pub trait Dist2<Geom> {
    fn dist<T>(&self, cmp: &T) -> f64;
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

impl <Geom: System<Geometry = Geom>> Distance<Point<Geom>> for Point<Geom> {
    fn dist(&self, cmp: &Point<Geom>) -> f64 {
        Geom::dist_pt_pt(self, cmp)
    }
}

impl <Geom: System<Geometry = Geom>> Distance<geo::Line> for Point<Geom> {
    fn dist(&self, cmp: &geo::Line) -> f64 {
        Geom::dist_pt_line(self, cmp)
    }
}

impl <Geom: System<Geometry = Geom>> Distance<Point<Geom>> for geo::Line {
    fn dist(&self, cmp: &Point<Geom>) -> f64 {
        Geom::dist_pt_line(cmp, self)
    }
}

// TODO: Move this re-export
pub use geo::coord;