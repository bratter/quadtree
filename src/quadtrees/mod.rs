pub mod datum;
pub mod point;
pub mod bounds;
mod knn;

use crate::Distance;
pub use datum::Datum;
use geo::GeoFloat;

pub const DEFAULT_MAX_CHILDREN: usize = 4;
pub const DEFAULT_MAX_DEPTH: u8 = 4;

// TODO: Docs
pub trait QuadTree<D> {
    /// Return the number of datums currently stored in the quadtree.
    fn size(&self) -> usize;

    fn insert(&mut self, datum: D);

    fn retrieve(&self, datum: &D) -> Vec<&D>;
}

pub trait QuadTreeSearch<D, T>
where
    T: GeoFloat,
{
    /// Find the closest datum in the quadtree to the passed comparator.
    /// Returns the datum and the distance to the point in a tuple. The
    /// comparator must implement the Distance trait for Bounds and T.
    /// 
    /// This will often require an `impl Distance<T> for X` block, which will
    /// be trivial in most cases, as it can delegate to the underlying geometry.
    /// TODO: Distance has to take a generic T instead, or maybe CoordFloat + whatever else for haversine is fine here?
    fn find<X>(&self, cmp: &X) -> Option<(&D, T)>
    where
        X: Distance<T>;

    /// Find `k` nearest neighbors within a radius of `r` of the comparator`cmp`.
    /// 
    /// Requires the comparator to implement `Distance` for both bounds and the
    /// specific type of the datum used. As with find this will often be required,
    /// but trivial.
    /// 
    /// Performs a partial unstable sort on nodes in any order, so (a) calls to
    /// distance that return `NaN` will panic, and (b) the method makes no
    /// ordering promise when data are at the same distance.
    fn knn<X>(&self, cmp: &X, k: usize, r: T) -> Vec<(&D, T)>
    where
        X: Distance<T>;
}