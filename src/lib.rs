/*
 * TODO: Should this have both bounded and point versions?
 *       If yes, maybe do them as separate objects with a trait?
 *       What about an integer-with-power-2-bounds
 * TODO: Use https://georust.org/ for geographic primitives instead
 */

// TODO: Implement Default for Quadtree and make it a bound on the trait
// TODO: Make a prelude to simplify use statements
// TODO: Possible to simplify the generics? Ref https://www.youtube.com/watch?v=yozQ9C69pNs, could remove the generic on System, but keep the associated type
// TODO: Fix iterator implementation to only use into_iter, and only make that work for references
// TODO: Rearrange the node/qt modules
// TODO: Implement KNN

pub mod geom;
mod node;
mod iter;

pub mod quadtrees;

use geom::*;
use node::*;
use iter::*;

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

// TODO: Does this need a T constrained this way?
pub trait QuadTree<T: Datum<Geom>, Geom: System<Geometry = Geom>> {
    /// Create a new Quadtree.
    fn new(bounds: Bounds<Geom>, max_depth: u8, max_children: usize) -> Self;

    /// Create a new QuadTree using default values for max_depth and max_children.
    fn new_def(bounds: Bounds<Geom>) -> Self;

    /// Return the number of datums added to the quadtree.
    fn size(&self) -> usize;

    fn insert(&mut self, datum: T);

    fn retrieve(&self, datum: &T) -> Vec<&T>;

    // fn iter(&self) -> QuadTreeIter<'_, T, Geom>;

    /// Find the closest datum in the quadtree to the passed point.
    fn find(&self, datum: &Point<Geom>) -> Option<&T>;
}