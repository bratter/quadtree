/*
 * Quadtree Package.
 * 
 * Multiple quadtree implementations for various geometries.
 * 
 * TODO: Maybe the best way of doing this overall is to have a trait that replaces the X in find and KNN, there is no other reason why a quadtree needs distance - not required for insert or retrieve
 *       Then datum is just a point, and bounds datum just needs to be something that can calc a bounding rectangle (the BoundingRect trait)
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

use geo::{Rect, BoundingRect};

use geom::*;
use node::*;
use iter::*;
use knn::*;

pub use quadtrees::point::PointQuadTree;
pub use quadtrees::bounds::BoundsQuadTree;

pub const DEFAULT_MAX_CHILDREN: usize = 4;
pub const DEFAULT_MAX_DEPTH: u8 = 4;

pub trait Datum {
    fn point(&self) -> geo::Point<f64>;
}

pub trait BoundsDatum: Datum {
    fn bounds(&self) -> Rect;
}

// We turn a Point into a datum so it can be used in the qt directly
impl Datum for geo::Point {
    fn point(&self) -> geo::Point {
        *self
    }
}

// Nothing stopping a point having a zero-sized bounds for a Bounds qt
impl BoundsDatum for geo::Point {
    fn bounds(&self) -> Rect {
        self.bounding_rect()
    }
}

// TODO: Delete these?
// Also turn a Segment into a BoundsDatum so it can be used in a Bounds qt directly
// impl <Geom: System<Geometry = Geom>> Datum<Geom> for Segment<Geom> {
//     fn point(&self) -> Point<Geom> {
//         self.a()
//     }
// }

// impl <Geom: System<Geometry = Geom>> BoundsDatum<Geom> for Segment<Geom> {
//     fn bounds(&self) -> Bounds<Geom> {
//         Bounds::from_points(self.a(), self.b())
//     }
// }

pub trait QuadTree<T> {
    /// Create a new Quadtree.
    fn new(bounds: Rect, max_depth: u8, max_children: usize) -> Self;

    /// Create a new QuadTree using default values for max_depth and max_children.
    fn default(bounds: Rect) -> Self;

    /// Return the number of datums added to the quadtree.
    fn size(&self) -> usize;

    fn insert(&mut self, datum: T);

    fn retrieve(&self, datum: &T) -> Vec<&T>;
}

pub trait SearchDatum<T, Geom>: Distance<Bounds<Geom>> + Distance<T>
where
    T: Datum,
    Geom: System<Geometry = Geom>,
{}

// TODO: These should be deleted
// impl<Geom> SearchDatum<Point<Geom>, Geom> for Point<Geom>
// where
//     Geom: System<Geometry = Geom>,
// {}

// impl<Geom> SearchDatum<Segment<Geom>, Geom> for Point<Geom>
// where
//     Geom: System<Geometry = Geom>,
// {}

// TODO: This should take a geom that the implementations then specify
//       This can then enforce that the SearchDatum has the right distance Geom 
// TODO: Consider using a different way to have private access to the root

// For a std qt, A Datum just needs to be able to be or produce a geo::Point
//       or can produce a tuple position, or implements geo::Centroid for a general solution (Centroid sometimes returns an option, is it easy enough to generalize over this)
// For a bounds qt, a Datum just needs to implement geo::BoundingRect, but this also sometimes returns none, so need to handle this
//       Can start just by supporting where Output = geo::Point for both, but then expand to the some case later on with an extra impl and some wrapping logic
// For anything being searched, we need to know what distance implementations to use
//       So perhaps the trait can take a type param that is the Geom
//       Then the SearchDistance? trait also takes the same type param
//       Then only if the X implements both distance to Rect and T for the same Geom will it work 
pub trait QuadTreeSearch<T, Geom>
where
    T: Datum,
    Geom: System<Geometry = Geom>,
{
    /// Find the closest datum in the quadtree to the passed comparator.
    /// Returns the datum and the distance to the point in a tuple. The
    /// comparator must implement the Distance trait for Bounds and T.
    /// 
    /// This will often require an `impl Distance<T> for X` block, which will
    /// be trivial in most cases, as it can delegate to the underlying geometry.
    fn find<X>(&self, cmp: &X) -> Option<(&T, f64)>
    where X: SearchDatum<T, Geom>;

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
    where X: SearchDatum<T, Geom>;
}