/*
 * Quadtree Package.
 * 
 * Multiple quadtree implementations for various geometries.
 * 
 * TODO: I think the way consumers can work this is have a wrapper type that is a datum that for find/knn impls DistEuclidean (should test this)
 * TODO: Document or test that retrieve.filter can give intersections or contains; convert retrieve to an iterator?
 * TODO: Cascade geo's numeric types into the quadtree itself
 * TODO: Force constraints on spherical coords?
 * TODO: Should we use Error semantics for insertion? Probably yes
 * TODO: Should this have an integer-with-power-2-bounds version?
 * TODO: Should part of node be modeled as an enum to account for children vs nodes?
 * TODO: Should spherical units deal with degrees rather than radians? Degrees are useful for readability, but radians make for easy conversions
 * TODO: Should nodes and children be private on the node structs?
 * TODO: Are the find/knn semantics the best? Should we constrain the X by Datum instead as we know the point will always work
 * TODO: Make a PR for the geo crate to add extra euclidean and haversine distance measures for Rect
 */

pub mod geom;
mod quadtrees;
mod node;
mod iter;
mod knn;

use geo::Rect;

// TODO: Fix these imports
use geom::Distance;
use node::*;
use iter::*;
use knn::*;

pub use quadtrees::point::*;
pub use quadtrees::bounds::*;

pub use geom::euclidean::*;
pub use geom::spherical::*;

pub const DEFAULT_MAX_CHILDREN: usize = 4;
pub const DEFAULT_MAX_DEPTH: u8 = 4;

// TODO: Move these to the quadtrees module?
pub trait QuadTree<D> {
    /// Create a new Quadtree.
    fn new(bounds: Rect, max_depth: u8, max_children: usize) -> Self;

    /// Create a new QuadTree using default values for max_depth and max_children.
    fn default(bounds: Rect) -> Self;

    /// Return the number of datums added to the quadtree.
    fn size(&self) -> usize;

    fn insert(&mut self, datum: D);

    fn retrieve(&self, datum: &D) -> Vec<&D>;
}

pub trait QuadTreeSearch<D> {
    /// Find the closest datum in the quadtree to the passed comparator.
    /// Returns the datum and the distance to the point in a tuple. The
    /// comparator must implement the Distance trait for Bounds and T.
    /// 
    /// This will often require an `impl Distance<T> for X` block, which will
    /// be trivial in most cases, as it can delegate to the underlying geometry.
    fn find<T>(&self, cmp: &T) -> Option<(&D, f64)>
    where T: Distance<D>;

    /// Find `k` nearest neighbors within a radius of `r` of the comparator`cmp`.
    /// 
    /// Requires the comparator to implement `Distance` for both bounds and the
    /// specific type of the datum used. As with find this will often be required,
    /// but trivial.
    /// 
    /// Performs a partial unstable sort on nodes in any order, so (a) calls to
    /// distance that return `NaN` will panic, and (b) the method makes no
    /// ordering promise when data are at the same distance.
    fn knn<T>(&self, cmp: &T, k: usize, r: f64) -> Vec<(&D, f64)>
    where T: Distance<D>;
}