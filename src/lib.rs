/*
 * TODO: Force constraints on spherical coords
 * TODO: Should we use Error semantics for insertion? Probably yes
 * TODO: Should this have an integer-with-power-2-bounds version?
 * TODO: Use https://georust.org/ for geographic primitives instead
 * TODO: Possible to simplify the generics? Ref https://www.youtube.com/watch?v=yozQ9C69pNs
 * TODO: Should part of node be modeled as an enum to account for children vs nodes?
 * TODO: Should spherical units deal with degrees rather than radians? Degrees are useful for readability, but radians make for easy conversions
 * TODO: Should nodes and children be private on the node structs?
 * TODO: Are the find/knn semantics the best? Should we constrain the X by Datum instead as we know the point will always work
 */

pub mod geom;
mod quadtrees;
mod node;
mod iter;
mod knn;

use geom::*;
use node::*;
use iter::*;
use knn::*;

pub use quadtrees::point::PointQuadTree;
pub use quadtrees::bounds::BoundsQuadTree;

pub const DEFAULT_MAX_CHILDREN: usize = 4;
pub const DEFAULT_MAX_DEPTH: u8 = 4;

pub enum SubNode {
    TopLeft = 0,
    TopRight = 1,
    BottomRight = 2,
    BottomLeft = 3,
}

pub trait Datum<Geom: System<Geometry = Geom>> {
    fn point(&self) -> Point<Geom>;
}

pub trait BoundsDatum<Geom: System<Geometry = Geom>>: Datum<Geom> {
    fn bounds(&self) -> Bounds<Geom>;
}

// We turn a Point into a datum so it can be used in the qt directly
impl <Geom: System<Geometry = Geom>> Datum<Geom> for Point<Geom> {
    fn point(&self) -> Point<Geom> {
        *self
    }
}

// Nothing stopping a point having a zero-sized bounds for a Bounds qt
impl <Geom: System<Geometry = Geom>> BoundsDatum<Geom> for Point<Geom> {
    fn bounds(&self) -> Bounds<Geom> {
        Bounds::from_origin(*self, 0.0, 0.0)
    }
}

// Also turn a Segment into a BoundsDatum so it can be used in a Bounds qt directly
impl <Geom: System<Geometry = Geom>> Datum<Geom> for Segment<Geom> {
    fn point(&self) -> Point<Geom> {
        self.a()
    }
}

impl <Geom: System<Geometry = Geom>> BoundsDatum<Geom> for Segment<Geom> {
    fn bounds(&self) -> Bounds<Geom> {
        Bounds::from_points(self.a(), self.b())
    }
}

pub trait QuadTree<T: Datum<Geom>, Geom: System<Geometry = Geom>> {
    /// Create a new Quadtree.
    fn new(bounds: Bounds<Geom>, max_depth: u8, max_children: usize) -> Self;

    /// Create a new QuadTree using default values for max_depth and max_children.
    fn default(bounds: Bounds<Geom>) -> Self;

    /// Return the number of datums added to the quadtree.
    fn size(&self) -> usize;

    fn insert(&mut self, datum: T);

    fn retrieve(&self, datum: &T) -> Vec<&T>;

    /// Find the closest datum in the quadtree to the passed comparator.
    /// Returns the datum and the distance to the point in a tuple. The
    /// comparator must implement the Distance trait for Bounds and T.
    /// 
    /// This will often require an `impl Distance<T> for X` block, which will
    /// be trivial in most cases, as it can delegate to the underlying geometry.
    fn find<X>(&self, cmp: &X) -> Option<(&T, f64)>
    where X: Distance<Bounds<Geom>> + Distance<T>;

    /// Find `k` nearest neighbors within a radius of `r` of the comparator`cmp`.
    /// 
    /// Requires the comparator to implement `Distance` for both bounds and the
    /// specific type of the datum used. As with find this will often be required,
    /// but trivial.
    /// 
    /// Performs a partial unstable sort on nodes in any order, so (a) calls to
    /// distance that return `NaN` will panic, and (b) the method makes no
    /// ordering promise when data are at the same distance.
    fn knn<X>(&self, cmp: &X, k: usize, r: f64) -> Vec<(&T, f64)>
    where X: Distance<Bounds<Geom>> + Distance<T>;
}