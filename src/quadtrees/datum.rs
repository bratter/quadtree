use geo::{Coordinate, GeoNum, Line, LineString, Point, Polygon, Rect};

use crate::Geometry;

/// Trait to define a QuadTree Datum.
///
/// The trait enables polymorphic extraction of underlying geo-types to enable
/// measurements for insertion and search operations.
///
/// This trait must be implemented by any type that is inserted into a QuadTree.
/// The library provides implementations for all geo-types that are supported.
/// These supported Geometries are listed in the [`Geometry`] enum.
///
/// The only requirement for a valid QuadTree `Datum` is that it has a `geometry`
/// method that returns one of the valid geo-type geometries. For custom `Datum`
/// this will usually be a simple matter of return an already underlying geo-type
/// and wrapping in the appropriate Geometry enum variant:
///
/// ```
/// // For example for a Point wrapper using the default f64 numeric type
/// # use geo::Point;
/// # use quadtree::{Geometry, Datum};
/// struct MyDatum {
///     id: usize,
///     location: Point,
/// }
///
/// impl Datum for MyDatum {
///     fn geometry(&self) -> Geometry<f64> {
///         Geometry::Point(self.location)
///     }
/// }
/// ```
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
