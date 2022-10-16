use geo::{Rect, GeoNum, Point, Line, LineString, Polygon, BoundingRect};

/// Enum to capture the geo-types that can be used as Datums
#[derive(Debug, Clone)]
pub enum Geometry<T>
where
    T: GeoNum,
{
    Point(Point<T>),
    Line(Line<T>),
    LineString(LineString<T>),
    Polygon(Polygon<T>),
    Rect(Rect<T>),
}

impl<T> BoundingRect<T> for Geometry<T>
where
    T: GeoNum,
{
    type Output = Option<Rect<T>>;

    fn bounding_rect(&self) -> Self::Output {
        match self {
            Geometry::Point(d) => Some(d.bounding_rect()),
            Geometry::Line(d) => Some(d.bounding_rect()),
            Geometry::LineString(d) => d.bounding_rect(),
            Geometry::Polygon(d) => d.bounding_rect(),
            Geometry::Rect(d) => Some(d.bounding_rect()),
        }
    }
}

impl<T> From<Point<T>> for Geometry<T>
where
    T: GeoNum,
{
    fn from(d: Point<T>) -> Self {
        Geometry::Point(d)
    }
}

impl<T> From<Line<T>> for Geometry<T>
where
    T: GeoNum,
{
    fn from(d: Line<T>) -> Self {
        Geometry::Line(d)
    }
}

impl<T> From<LineString<T>> for Geometry<T>
where
    T: GeoNum,
{
    fn from(d: LineString<T>) -> Self {
        Geometry::LineString(d)
    }
}

impl<T> From<Polygon<T>> for Geometry<T>
where
    T: GeoNum,
{
    fn from(d: Polygon<T>) -> Self {
        Geometry::Polygon(d)
    }
}

impl<T> From<Rect<T>> for Geometry<T>
where
    T: GeoNum,
{
    fn from(d: Rect<T>) -> Self {
        Geometry::Rect(d)
    }
}