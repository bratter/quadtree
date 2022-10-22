use geo::{
    Coordinate, GeoFloat, Line, LineString, MapCoords, MapCoordsInPlace, Point, Polygon, Rect,
};

use crate::Geometry;

pub trait ToRadians<T>
where
    T: GeoFloat,
{
    fn to_radians(&self) -> Self;

    fn to_radians_in_place(&mut self);
}

fn coord_to_radians<T>(c: Coordinate<T>) -> Coordinate<T>
where
    T: GeoFloat,
{
    Coordinate {
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
