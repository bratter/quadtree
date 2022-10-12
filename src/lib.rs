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
 * TODO: Make a PR for the geo crate to add extra euclidean and haversine distance measures for Rect
 */

pub mod geom;
mod quadtrees;
mod node;
mod iter;
mod knn;

use geo::{Rect, BoundingRect, EuclideanDistance, HaversineDistance, InteriorPoint};

use geom::*;
// TODO: Fix this import
use geom::rect::{DistEuclidean, DistHaversine};
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

// TODO: Expand these impls
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

impl Datum for geo::Line {
    fn point(&self) -> geo::Point<f64> {
        self.interior_point()
    }
}

impl BoundsDatum for geo::Line {
    fn bounds(&self) -> Rect {
        self.bounding_rect()
    }
}

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

// TODO: This should take a geom that the implementations then specify
//       This can then enforce that the SearchDatum has the right distance Geom 
// TODO: Consider using a different way to have private access to the root
// TODO: Find bounds funtion to enable find on a bbox comparisons only find in bounds qts?

// For a std qt, A Datum just needs to be able to be or produce a geo::Point
//       or can produce a tuple position, or implements geo::Centroid for a general solution (Centroid sometimes returns an option, is it easy enough to generalize over this)
// For a bounds qt, a Datum just needs to implement geo::BoundingRect, but this also sometimes returns none, so need to handle this
//       Can start just by supporting where Output = geo::Point for both, but then expand to the some case later on with an extra impl and some wrapping logic
// One of two solutions is possible:
//       (a) Use the Datum construct where Datum returns an `Option<Point>` instead of just a point, then qt's return option and implementors wrap non Option versions in `Some`
//       (b) Create multiple implementations and we delegate to underlying functions
//       Option (a) is probably better, and can still turn a point, line, line_string, etc. into a Datum easily
// For anything being searched, we need to know what distance implementations to use
//       So perhaps the trait can take a type param that is the Geom
//       Then the SearchDistance? trait also takes the same type param
//       Then only if the X implements both distance to Rect and T for the same Geom will it work
// So what do we need to do minimally
//       Call self.dist(cmp) or cmp.dist(self)
//       This needs a geom that both self and cmp know about
//       Should we wrap each Test in a struct that then provides the methods

// TODO: The T's in the QuadTreeSearch can be copletely defined by this create, and users must simply pass an approrpate geo type wrapped in a coord system into the methods
//       There is no need for custom types to be passed in. Therefore need to scrub segE
//       Then just need to consider if we keep datum as-is or just make them implement Centroid for Datum and BoundingRect for Bounds
// TODO: Move these and rename
// TODO: Rename generics everywhere
pub trait SearchDistance<Datum> {
    fn dist_datum(&self, datum: &Datum) -> f64;
    fn dist_bbox(&self, bbox: &Rect) -> f64;
}

pub struct Eucl<Test> (Test);
impl<Test> Eucl<Test> {
    pub fn new(t: Test) -> Self {
        Self(t)
    }
}
impl<Test, Datum> SearchDistance<Datum> for Eucl<Test>
where
    Test: DistEuclidean<f64, Datum> + DistEuclidean<f64, Rect>,
{
    fn dist_datum(&self, datum: &Datum) -> f64 {
        self.dist_euclidean(datum)
    }

    fn dist_bbox(&self, bbox: &Rect) -> f64 {
        self.dist_euclidean(bbox)
    }
}
impl<Test> core::ops::Deref for Eucl<Test> {
    type Target = Test;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TODO: May need to use some of my math here in haversine implementations
pub struct Sph<Test> (Test);
impl<Test> Sph<Test> {
    pub fn new(t: Test) -> Self {
        Self(t)
    }
}
impl<Test, Datum> SearchDistance<Datum> for Sph<Test>
where
    Test: DistHaversine<f64, Datum> + DistHaversine<f64, Rect>,
{
    fn dist_datum(&self, datum: &Datum) -> f64 {
        self.dist_haversine(datum)
    }

    fn dist_bbox(&self, bbox: &Rect) -> f64 {
        self.dist_haversine(bbox)
    }
}
impl<Test> core::ops::Deref for Sph<Test> {
    type Target = Test;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait QuadTreeSearch<D>
where
    D: Datum,
{
    /// Find the closest datum in the quadtree to the passed comparator.
    /// Returns the datum and the distance to the point in a tuple. The
    /// comparator must implement the Distance trait for Bounds and T.
    /// 
    /// This will often require an `impl Distance<T> for X` block, which will
    /// be trivial in most cases, as it can delegate to the underlying geometry.
    fn find<T>(&self, cmp: &T) -> Option<(&D, f64)>
    where T: SearchDistance<D>;

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
    where T: SearchDistance<D>;
}