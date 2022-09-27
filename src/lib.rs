/*
 * TODO: Should this have an integer-with-power-2-bounds version?
 * TODO: Use https://georust.org/ for geographic primitives instead
 * TODO: Possible to simplify the generics? Ref https://www.youtube.com/watch?v=yozQ9C69pNs
 * TODO: Implement KNN
 */

pub mod geom;
mod quadtrees;
mod node;
mod iter;

use geom::*;
use node::*;
use iter::*;

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

pub trait QuadTree<T: Datum<Geom>, Geom: System<Geometry = Geom>> {
    /// Create a new Quadtree.
    fn new(bounds: Bounds<Geom>, max_depth: u8, max_children: usize) -> Self;

    /// Create a new QuadTree using default values for max_depth and max_children.
    fn default(bounds: Bounds<Geom>) -> Self;

    /// Return the number of datums added to the quadtree.
    fn size(&self) -> usize;

    fn insert(&mut self, datum: T);

    fn retrieve(&self, datum: &T) -> Vec<&T>;

    /// Find the closest datum in the quadtree to the passed point.
    fn find(&self, datum: &Point<Geom>) -> Option<&T>;
}