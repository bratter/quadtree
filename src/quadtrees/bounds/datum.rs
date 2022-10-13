use geo::{GeoNum, Rect, BoundingRect, Coordinate, Point, Line};

// TODO: Expand impls to the other main geo-types. Most of these return
//       Option<Rect<T>>, so need to think through how to handle this,
//       Likely error on the insert, but how to handle in the type system?

/// A trait required for an item to be useable in a `BoundsQuadTree`. It simply
/// requires that the item can produce a `geo::Rect` for comparisons.
/// 
/// This trait comes implemented for all geo-types that implement
/// `geo::BoundingRect`, so these types can be used in the quadtree directly.
/// 
/// We only constrain by GeoNum here as floats are not required if we are not
/// using distance-based search methods.
pub trait BoundsDatum<T = f64>
where
    T: GeoNum,
{
    fn bounds(&self) -> Rect<T>;
}

// Nothing stopping a point/coord having a zero-sized bounds for a Bounds qt
impl<T> BoundsDatum<T> for Coordinate<T>
where
    T: GeoNum,
{
    fn bounds(&self) -> Rect<T> {
        self.bounding_rect()
    }
}

impl<T> BoundsDatum<T> for Point<T>
where
    T: GeoNum,
{
    fn bounds(&self) -> Rect<T> {
        self.bounding_rect()
    }
}

impl<T> BoundsDatum<T> for Rect<T>
where
    T: GeoNum,
{
    fn bounds(&self) -> Rect<T> {
        *self
    }
}

impl<T> BoundsDatum<T> for Line<T>
where
    T: GeoNum,
{
    fn bounds(&self) -> Rect<T> {
        self.bounding_rect()
    }
}