use geo::{GeoNum, Coordinate, Point, Rect, Line, LineString, Polygon};

use crate::Geometry;

// TODO: Adjust this to talk about geometries, bounding rect, and comparisons
/// A trait required for an item to be useable in a `BoundsQuadTree`. It simply
/// requires that the item can produce a `geo::Rect` for comparisons.
/// 
/// This trait comes implemented for all geo-types that implement
/// `geo::BoundingRect`, so these types can be used in the quadtree directly.
/// 
/// We only constrain by GeoNum here as floats are not required if we are not
/// using distance-based search methods.
pub trait Datum<T = f64>
where
    T: GeoNum,
{
    fn geometry(&self) -> Geometry<T>;
}

/// Helper macro to generate impl blocks for Datum
macro_rules! impl_datum_helper {
    ($t:ident) => {
        impl<T> Datum<T> for $t<T>
        where
            T: GeoNum,
        {
            fn geometry(&self) -> Geometry<T> {
                Geometry::from(self.clone())
            }
        }
    };
}

impl<T> Datum<T> for Coordinate<T>
where
    T: GeoNum,
{
    fn geometry(&self) -> Geometry<T> {
        Geometry::from(Point::from(*self))
    }
}

impl_datum_helper!(Point);
impl_datum_helper!(Line);
impl_datum_helper!(LineString);
impl_datum_helper!(Polygon);
impl_datum_helper!(Rect);