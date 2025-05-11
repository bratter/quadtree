use geo::{BoundingRect, GeoNum, Line, LineString, Point, Polygon, Rect};

use crate::Error;

use super::{AsGeom, CalcMethod, GeomCalc};

//
// -------------------- GeometryRef (borrowed data)  -------------------- //
//

/// Enum to capture borrowed geo-types that can be used in QuadTrees. This is the destiniation type
/// for as_geom and therefore can be used directly as data, and is also the type that must be
/// returned from implementations of [`AsGeom`] on custome datum types.
///
/// We provide wrappers for [`Point`], [`Line`], [`LineString`], [`Polygon`], and [`Rect`]. This
/// wrapper implements [`AsGeom`] so it can be used directly  in QuadTrees, and also uses owned
/// data, enabling radians coversion.
///
/// Because this wraps borrowed data, neither [`From`] or [`ToRadians`] is implemented. If these
/// are required, the operations should be performed on the individual geometries or [`Geometry`]
/// prior to conversion.
#[derive(Debug, Clone, Copy)]
pub enum GeometryRef<'a, T>
where
    T: GeoNum,
{
    Point(&'a Point<T>),
    Line(&'a Line<T>),
    LineString(&'a LineString<T>),
    Polygon(&'a Polygon<T>),
    Rect(&'a Rect<T>),
}

// Implement AsGeom and return self so it can be used in a QuadTree directly.
impl<T> AsGeom<T> for GeometryRef<'_, T>
where
    T: GeoNum,
{
    fn as_geom(&self) -> GeometryRef<T> {
        *self
    }
}

impl<'a, T> GeometryRef<'a, T>
where
    T: GeoNum,
{
    pub fn into_calc(self, method: CalcMethod) -> GeomCalc<'a, T> {
        GeomCalc { geom: self, method }
    }
}

impl<T> BoundingRect<T> for GeometryRef<'_, T>
where
    T: GeoNum,
{
    type Output = Option<Rect<T>>;

    fn bounding_rect(&self) -> Self::Output {
        match self {
            GeometryRef::Point(d) => Some(d.bounding_rect()),
            GeometryRef::Line(d) => Some(d.bounding_rect()),
            GeometryRef::LineString(d) => d.bounding_rect(),
            GeometryRef::Polygon(d) => d.bounding_rect(),
            GeometryRef::Rect(d) => Some(d.bounding_rect()),
        }
    }
}

//
// -------------------- Geometry (owned data)  -------------------- //
//

/// Enum to capture owned geo-types that can be used in QuadTrees.
///
/// We provide wrappers for [`Point`], [`Line`], [`LineString`], [`Polygon`],
/// and [`Rect`]. This wrapper implements [`AsGeom`] so it can be used directly
/// in QuadTrees, and also uses owned data, enabling radians coversion.
///
/// [`From`] is implemented on Geometry for each of these geo-types to ease
/// creation.
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

// Ensure we can convert an owned geometry into a reference.
impl<T> AsGeom<T> for Geometry<T>
where
    T: GeoNum,
{
    fn as_geom(&self) -> GeometryRef<T> {
        match self {
            Self::Point(d) => GeometryRef::Point(d),
            Self::Line(d) => GeometryRef::Line(d),
            Self::LineString(d) => GeometryRef::LineString(d),
            Self::Polygon(d) => GeometryRef::Polygon(d),
            Self::Rect(d) => GeometryRef::Rect(d),
        }
    }
}

impl<T> BoundingRect<T> for Geometry<T>
where
    T: GeoNum,
{
    type Output = Option<Rect<T>>;

    fn bounding_rect(&self) -> Self::Output {
        self.as_geom().bounding_rect()
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

impl<T> TryFrom<geo::Geometry<T>> for Geometry<T>
where
    T: GeoNum,
{
    type Error = Error;

    fn try_from(value: geo::Geometry<T>) -> Result<Self, Error> {
        match value {
            geo::Geometry::Point(point) => Ok(point.into()),
            geo::Geometry::Line(line) => Ok(line.into()),
            geo::Geometry::LineString(line_string) => Ok(line_string.into()),
            geo::Geometry::Polygon(polygon) => Ok(polygon.into()),
            geo::Geometry::Rect(rect) => Ok(rect.into()),
            _ => Err(Error::UnsupportedGeometry),
        }
    }
}
