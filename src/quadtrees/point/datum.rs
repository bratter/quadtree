use geo::{Coordinate, Point, GeoNum};

// TODO: Does is make sense to have PointDatum impls for non-point like
//       entities? If so, then expand the impls, but it probably doesn't
//       Because it is not entirely clear what the point should be, it
//       should up up to the implementor to decide how to produce point.

/// A trait required for an item to be useable in a `PointQuadTree`. It simply
/// requires that the item can produce a `geo::Point` for comparisons.
/// 
/// This trait comes implemented for `geo::Coordinate` and `geo::Point`, so
/// coords and points can be used in quadtrees directly.
/// 
/// We only constrain by GeoNum here as floats are not required if we are not
/// using distance-based search methods.
pub trait PointDatum<T = f64>
where
    T: GeoNum,
{
    fn point(&self) -> Point<T>;
}

// We turn a Point into a datum so it can be used in the qt directly
impl<T> PointDatum<T> for Coordinate<T>
where
    T: GeoNum,
{
    fn point(&self) -> Point<T> {
        Point::from(*self)
    }
}

impl<T> PointDatum<T> for Point<T>
where
    T: GeoNum,
{
    fn point(&self) -> Point<T> {
        *self
    }
}