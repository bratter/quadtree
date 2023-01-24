use geo::{Coord, GeoFloat, Line, LineString, MapCoords, MapCoordsInPlace, Point, Polygon, Rect};

use crate::Geometry;

/// Ergonomic trait to enable easy conversion of valid geo-types from degrees
/// to radians.
///
/// The trait is implemented for all types in [`Geometry`] and the enum itself.
///
/// This is useful because most input sources will contain Lng/Lat coordinates
/// in degrees, but all quadtree crate spherical geometry methods accept and
/// return radians only.
///
/// Note: Radians was a design choice to make it easy to use Spherical distances
/// on any spherical body, as opposed to an approach that multiplies by Earth's
/// radius before returning.
pub trait ToRadians<T>
where
    T: GeoFloat,
{
    /// Convert all coordinates in a geometry from degrees to radians. This
    /// method maps over all the underlying coordinates and returns a new
    /// geometry.
    ///
    /// Note that all [`QuadTree`] distance methods require input geometries to
    /// be in radians.
    fn to_radians(&self) -> Self;

    /// Convert all coordinates in a geometry from degrees to radians. This
    /// method maps over all coordinates *in place*, modifying the underlying
    /// geometry.
    ///
    /// Note that all [`QuadTree`] distance methods require input geometries to
    /// be in radians.
    fn to_radians_in_place(&mut self);
}

fn coord_to_radians<T>(c: Coord<T>) -> Coord<T>
where
    T: GeoFloat,
{
    Coord {
        x: c.x.to_radians(),
        y: c.y.to_radians(),
    }
}

impl<T> ToRadians<T> for Point<T>
where
    T: GeoFloat,
{
    fn to_radians(&self) -> Self {
        Point::from(coord_to_radians(self.0))
    }

    fn to_radians_in_place(&mut self) {
        self.0 = coord_to_radians(self.0);
    }
}

impl<T> ToRadians<T> for Line<T>
where
    T: GeoFloat,
{
    fn to_radians(&self) -> Self {
        Line::new(coord_to_radians(self.start), coord_to_radians(self.end))
    }

    fn to_radians_in_place(&mut self) {
        self.start = coord_to_radians(self.start);
        self.end = coord_to_radians(self.end);
    }
}

impl<T> ToRadians<T> for LineString<T>
where
    T: GeoFloat,
{
    fn to_radians(&self) -> Self {
        self.map_coords(coord_to_radians)
    }

    fn to_radians_in_place(&mut self) {
        self.map_coords_in_place(coord_to_radians)
    }
}

impl<T> ToRadians<T> for Polygon<T>
where
    T: GeoFloat,
{
    fn to_radians(&self) -> Self {
        self.map_coords(coord_to_radians)
    }

    fn to_radians_in_place(&mut self) {
        self.map_coords_in_place(coord_to_radians)
    }
}

impl<T> ToRadians<T> for Rect<T>
where
    T: GeoFloat,
{
    fn to_radians(&self) -> Self {
        self.map_coords(coord_to_radians)
    }

    fn to_radians_in_place(&mut self) {
        self.map_coords_in_place(coord_to_radians)
    }
}

impl<T> ToRadians<T> for Geometry<T>
where
    T: GeoFloat,
{
    fn to_radians(&self) -> Self {
        match self {
            Geometry::Point(d) => Geometry::Point(d.to_radians()),
            Geometry::Line(d) => Geometry::Line(d.to_radians()),
            Geometry::LineString(d) => Geometry::LineString(d.to_radians()),
            Geometry::Polygon(d) => Geometry::Polygon(d.to_radians()),
            Geometry::Rect(d) => Geometry::Rect(d.to_radians()),
        }
    }

    fn to_radians_in_place(&mut self) {
        match self {
            Geometry::Point(d) => d.to_radians_in_place(),
            Geometry::Line(d) => d.to_radians_in_place(),
            Geometry::LineString(d) => d.to_radians_in_place(),
            Geometry::Polygon(d) => d.to_radians_in_place(),
            Geometry::Rect(d) => d.to_radians_in_place(),
        };
    }
}
