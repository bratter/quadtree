use geo::{Coordinate, GeoNum, Point};

/// A trait required for an item to be useable in a [`crate::PointQuadTree`].
/// It simply requires that the item can produce a [`Point`] for comparisons.
///
/// This trait comes implemented for [`Coordinate`] and [`Point`], so
/// coords and points can be used in quadtrees directly.
///
/// We only constrain by [`GeoNum`] here as floats are not required if we are not
/// using distance-based search methods.
///
/// We do not provide implementations for non-point shapes. It is up for the
/// user to decide how to generate a point from a polygon, etc.
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
