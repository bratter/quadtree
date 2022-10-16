use geo::{Rect, Point, GeoNum};

use crate::{Datum, Geometry};

// Module declarations
pub mod geometry;
pub mod euclidean;
pub mod spherical;

// TODO: Document better/consider: User test points must implement one or both of DistEuclidean or DistSpherical
//       But would encourage just using geotypes for this where implementations come pre-made if the Datum is also a geotype
//       There might also be some way to impl for Derefs - see the section in rect.rs and try it out

/// Trait implemented by the geometry wrapper types which provides polymorphism
/// for distance calculations. The implementors should generally forward the
/// call to an appropriate underlying distance calculation, for example that
/// provided by the `EuclideanDistance` trait.
/// 
/// This trait should not need to be used outside the crate, as the only place
/// it is necessary is in the wrapper types. However any consumer can implement
/// custom distance calcs on any type they wish, so is still part of the public
/// API.
/// TODO: This needs to take a T
pub trait Distance<T>
where
    T: GeoNum,
{
    // TODO: This can probably just be dist_geom, and we don't need datum here at all
    /// Calculate the distance between a datum `D` and the test type
    /// implementing this trait.
    fn dist_datum(&self, datum: &dyn Datum<T>) -> T;


    fn dist_geom(&self, geom: &Geometry<T>) -> T;

    /// Calculate the distance between a geo::Rect` and the test type
    /// implementing this trait.
    fn dist_bbox(&self, bbox: &Rect<T>) -> T;
}

/// Determine whether a `Point` in contained within or sits on the boundary of
/// a `Rect`.
/// 
/// We cannot use Rect::contains for this purpose because the DE-9IM semantics
/// (https://en.wikipedia.org/wiki/DE-9IM) that geo-rust uses does not return
/// true when the `Point` site on the boundary of the `Rect`. However this is
/// still valid for most QuadTree operations.
/// 
/// Note that even 0-sized `Rect` shapes on the boundary of a quadtree will be
/// contained by another `Rect`, so this is not required for bounds-bounds
/// calculations.
pub fn pt_in_rect<T>(rect: &Rect<T>, pt: &Point<T>) -> bool
where
    T: GeoNum,
{
    let (x, y) = pt.x_y();
    let (x1, y1) = rect.min().x_y();
    let (x2, y2) = rect.max().x_y();

    x >= x1 && x <= x2 && y >= y1 && y <= y2
}

/// Determine whether the first rectangle `r1` contains or has on its border,
/// in degenerate cases, `r2`.
/// 
/// Currently this mirrors the behavior of contains for rects in geo,-rust, but
/// this appears to be erroneous behavior, so we will not rely on it here.
pub fn rect_in_rect<T>(r1: &Rect<T>, r2: &Rect<T>) -> bool
where
    T: GeoNum,
{
    r1.min().x <= r2.min().x
        && r1.max().x >= r2.max().x
        && r1.min().y <= r2.min().y
        && r1.max().y >= r2.max().y
}