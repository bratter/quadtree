pub mod bounds;
pub mod datum;
mod knn;
pub mod point;
mod sorted;

use crate::{iter::DatumIter, node::Node, Distance, Error};
pub use datum::Datum;
use geo::{GeoFloat, GeoNum};

use self::sorted::SortIter;

pub const DEFAULT_MAX_CHILDREN: usize = 4;
pub const DEFAULT_MAX_DEPTH: u8 = 4;

/// Minimal trait that all QuadTree types must implement.
///
/// Enables reporting on the number of contained elements, insert, and collision
/// detection focused retrieve operations.
///
/// Requires a single generic `D`, the types of the Datum that the QuadTree
/// will hold.
pub trait QuadTree<D, T>
where
    T: GeoNum,
{
    /// The specific node built for this QuadTree implementation
    type Node: Node<D, T>;

    /// Return the number of datums currently stored in the quadtree.
    fn size(&self) -> usize;

    /// Insert a datum into the QuadTree. Returns a result, so will return Err
    /// if the insertion fails. Err will contain a Quadtree [`Error`].
    fn insert(&mut self, datum: D) -> Result<(), Error>;

    /// Retrieve "nearby" datums to the passed datum in an iterator.
    ///
    /// This retrieval is useful for collision detection and other spatial
    /// approximations, and works best when the quadtree is evenly populated.
    fn retrieve(&self, datum: &D) -> DatumIter<'_, Self::Node, D, T>;
}

/// Add on QuadTree trait that adds distance-based search methods to a
/// [`QuadTree`] implementation.
///
/// This trait constrains the available numeric type implementations to
/// [`GeoFloat`] as floating point math is required to measure distances.
pub trait QuadTreeSearch<D, T>
where
    D: Datum<T>,
    T: GeoFloat,
{
    /// The specific node built for this QuadTree implementation
    type Node: Node<D, T>;

    /// Find the closest datum in the quadtree to the passed comparator.
    ///
    /// Returns the datum and the distance to the point in a tuple, wrapped in
    /// a `Result`. The `Err` branch contains a [`Error`] code.
    ///
    /// In order to provide polymorphic distance calculations for a variety of
    /// coordinate systems, the type passed directly must implement the
    /// [`Distance`] trait. This provides formula to measure distances relative
    /// to any allowable [`crate::Geometry`].
    ///
    /// In general use, the consumer will wrap their own type in a premade
    /// wrapper that provides appropriate distance functions for a given
    /// coordinate system. e.g.:
    ///
    /// - [`crate::Euclidean`] for Euclidean distances
    /// - [`crate::Spherical`] for Haversine formula distances
    ///
    /// Helper function [`crate::eucl`] and [`crate::sphere`] are provided to
    /// make this easy.
    ///
    /// The underlying type then only needs to implement [`Datum`] and does not
    /// need to be the same [`Datum`] as inserted in the QuadTree. Because it
    /// only requires a Datum, [`geo::Point`], etc. work out of the box:
    ///
    /// ```
    /// use geo::{Point, Rect, coord};
    /// use quadtree::{PointQuadTree, QuadTreeSearch, eucl};
    ///
    /// let bounds = Rect::new(coord!(x: 0.0, y: 0.0), coord!(x: 1.0, y: 1.0));
    /// let mut qt: PointQuadTree<Point, f64> = PointQuadTree::from_bounds(bounds);
    ///
    /// qt.find(&eucl(Point::new(0.0, 0.0)));
    ///
    /// ```
    fn find<X>(&self, cmp: &X) -> Result<(&D, T), Error>
    where
        X: Distance<T>,
    {
        let infinity = T::from(f64::INFINITY).ok_or(Error::CannotCastInfinity)?;
        self.find_r(cmp, infinity)
    }

    /// Similar to [`QuadTreeSearch::find`], but takes a maximum distance
    /// parameter to constrain the maximum search radius. Will return an
    /// [`Error::NoneInRadius`] if no match is found inside `r`.
    fn find_r<X>(&self, cmp: &X, r: T) -> Result<(&D, T), Error>
    where
        X: Distance<T>;

    /// Find `k` nearest neighbors of the comparator `cmp`.
    ///
    /// The algorithm uses a partial unstable sort on nodes in any order, so
    /// the method makes no ordering promise when data are at the same distance.
    ///
    /// Returns a vector with a maximum length of k, but the result maybe
    /// shorter if insufficient points can be found. The vector's members
    /// are tuples of the found Datum and its distance to the comparator.
    /// The return is wrapped in `Result`, with the Err branch containing an
    /// [`Error`] code.
    ///
    /// As with [`QuadTreeSearch::find`], the comparator must implement
    /// [`Distance`], but this is usually provided by wrapping another type
    /// in [`crate::eucl`] and [`crate::sphere`]. See [`QuadTreeSearch::find`]
    /// for more information. Similarly the underlying type must implement
    /// [`Datum`], which comes for free with any [`crate::Geometry`] type.
    fn knn<X>(&self, cmp: &X, k: usize) -> Result<Vec<(&D, T)>, Error>
    where
        X: Distance<T>,
    {
        let infinity = T::from(f64::INFINITY).ok_or(Error::CannotCastInfinity)?;
        self.knn_r(cmp, k, infinity)
    }

    /// Similar to [`QuadTreeSearch::knn`], but takes a maximum distance
    /// parameter to constrain the maximum search radius. Returns an empty
    /// vector if no data is found within the search radius.
    fn knn_r<X>(&self, cmp: &X, k: usize, r: T) -> Result<Vec<(&D, T)>, Error>
    where
        X: Distance<T>;

    /// Iterate through all data in the QuadTree in distance-sorted order.
    ///
    /// The algorithm uses a partial unstable sort, so it makes no ordering
    /// promise when data are at the same distance.
    ///
    /// The iterator is designed to be more forgiving than [`QuadTreeSearch::find`]
    /// and [`QuadTreeSearch::knn`], attempting to skip items on error rather
    /// than returning an [`Err`].
    ///
    /// As with the other search methods, the comparator must implement
    /// [`Distance`], but this is usually provided by wrapping another type.
    /// See [`QuadTreeSearch::find`] for more information. Similarly the
    /// underlying type must implement [`Datum`], which comes for free with
    /// any [`crate::Geometry`] type.
    fn sorted<'a, X>(&'a self, cmp: &'a X) -> SortIter<'a, Self::Node, D, X, T>
    where
        X: Distance<T>;
}
