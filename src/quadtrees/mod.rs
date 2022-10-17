pub mod datum;
pub mod point;
pub mod bounds;
mod knn;

use crate::{Distance, Error};
pub use datum::Datum;
use geo::GeoFloat;

pub const DEFAULT_MAX_CHILDREN: usize = 4;
pub const DEFAULT_MAX_DEPTH: u8 = 4;

// TODO: Docs, fix and write
pub trait QuadTree<D> {
    /// Return the number of datums currently stored in the quadtree.
    fn size(&self) -> usize;

    fn insert(&mut self, datum: D) -> Result<(), Error>;

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
    /// 
    /// A new note: This returns a `Result<(&D, T), Error>` rather than an
    /// `Option` to give the consumer more insight into the failure reason.
    /// TODO: If this ends up returning an error instead, can make of the err enums a replacement for the unwrap here and in knn
    fn find<X>(&self, cmp: &X) -> Result<(&D, T), Error>
    where
        X: Distance<T>,
    {
        self.find_r(cmp, T::from(f64::INFINITY).unwrap())
    }

    fn find_r<X>(&self, cmp: &X, r: T) -> Result<(&D, T), Error>
    where
        X: Distance<T>;

    fn knn<X>(&self, cmp: &X, k: usize) -> Result<Vec<(&D, T)>, Error>
    where
        X: Distance<T>,
    {
        self.knn_r(cmp, k, T::from(f64::INFINITY).unwrap())
    }
    
    /// Find `k` nearest neighbors within a radius of `r` of the comparator `cmp`.
    /// 
    /// Requires the comparator to implement `Distance` for both bounds and the
    /// specific type of the datum used. As with find this will often be required,
    /// but trivial.
    /// 
    /// Performs a partial unstable sort on nodes in any order, so (a) calls to
    /// distance that return `NaN` will panic, and (b) the method makes no
    /// ordering promise when data are at the same distance.
    fn knn_r<X>(&self, cmp: &X, k: usize, r: T) -> Result<Vec<(&D, T)>, Error>
    where
        X: Distance<T>;
}