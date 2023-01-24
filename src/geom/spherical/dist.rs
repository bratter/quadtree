use crate::{AsGeom, Error, Geometry, GeometryRef};
use geo::{GeoFloat, Line, LineString, Point, Polygon, Rect};

use super::math::*;

/// Trait applied to all valid geometries to calculate the Haversine distance
/// between geometries.
///
/// The trait is implemented for all [`Geometry`] geometries, and the enum
/// itself, but may produce an error if the Haversine distance cannot be
/// calculated.
///
/// This trait should not need to be used outside the crate as it is abstracted
/// through the [`CalcMethod`] enum.
pub trait DistHaversine<T, Rhs = Self> {
    fn dist_haversine(&self, rhs: &Rhs) -> Result<T, Error>;
}

impl<T> DistHaversine<T, GeometryRef<'_, T>> for Point<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &GeometryRef<T>) -> Result<T, Error> {
        match rhs {
            GeometryRef::Point(d) => Ok(dist_pt_pt(self, d)),
            GeometryRef::Line(d) => Ok(dist_pt_line(self, d)),
            GeometryRef::LineString(d) => Ok(dist_pt_linestring(self, d)),
            GeometryRef::Polygon(d) => Ok(dist_pt_poly(self, d)),
            GeometryRef::Rect(d) => Ok(dist_pt_rect(self, d)),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Point<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        self.dist_haversine(&rhs.as_geom())
    }
}

impl<T> DistHaversine<T, GeometryRef<'_, T>> for Line<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &GeometryRef<T>) -> Result<T, Error> {
        match rhs {
            GeometryRef::Point(d) => Ok(dist_pt_line(d, self)),
            GeometryRef::Line(_) => Err(Error::InvalidDistance),
            GeometryRef::LineString(_) => Err(Error::InvalidDistance),
            GeometryRef::Polygon(_) => Err(Error::InvalidDistance),
            GeometryRef::Rect(_) => Err(Error::InvalidDistance),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Line<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        self.dist_haversine(&rhs.as_geom())
    }
}

impl<T> DistHaversine<T, GeometryRef<'_, T>> for LineString<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &GeometryRef<T>) -> Result<T, Error> {
        match rhs {
            GeometryRef::Point(_) => Err(Error::InvalidDistance),
            GeometryRef::Line(_) => Err(Error::InvalidDistance),
            GeometryRef::LineString(_) => Err(Error::InvalidDistance),
            GeometryRef::Polygon(_) => Err(Error::InvalidDistance),
            GeometryRef::Rect(_) => Err(Error::InvalidDistance),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for LineString<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        self.dist_haversine(&rhs.as_geom())
    }
}

impl<T> DistHaversine<T, GeometryRef<'_, T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &GeometryRef<T>) -> Result<T, Error> {
        match rhs {
            GeometryRef::Point(p) => Ok(dist_pt_poly(p, self)),
            GeometryRef::Line(_) => Err(Error::InvalidDistance),
            GeometryRef::LineString(_) => Err(Error::InvalidDistance),
            GeometryRef::Polygon(_) => Err(Error::InvalidDistance),
            GeometryRef::Rect(_) => Err(Error::InvalidDistance),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        self.dist_haversine(&rhs.as_geom())
    }
}

impl<T> DistHaversine<T, GeometryRef<'_, T>> for Rect<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &GeometryRef<T>) -> Result<T, Error> {
        match rhs {
            GeometryRef::Point(d) => Ok(dist_pt_rect(d, self)),
            GeometryRef::Line(_) => Err(Error::InvalidDistance),
            GeometryRef::LineString(_) => Err(Error::InvalidDistance),
            GeometryRef::Polygon(_) => Err(Error::InvalidDistance),
            GeometryRef::Rect(d) => Ok(dist_rect_rect(self, d)),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Rect<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        self.dist_haversine(&rhs.as_geom())
    }
}

impl<T> DistHaversine<T, GeometryRef<'_, T>> for GeometryRef<'_, T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &GeometryRef<T>) -> Result<T, Error> {
        match self {
            GeometryRef::Point(d) => d.dist_haversine(rhs),
            GeometryRef::Line(d) => d.dist_haversine(rhs),
            GeometryRef::LineString(d) => d.dist_haversine(rhs),
            GeometryRef::Polygon(d) => d.dist_haversine(rhs),
            GeometryRef::Rect(d) => d.dist_haversine(rhs),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for GeometryRef<'_, T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        self.dist_haversine(&rhs.as_geom())
    }
}

impl<T> DistHaversine<T, GeometryRef<'_, T>> for Geometry<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &GeometryRef<T>) -> Result<T, Error> {
        rhs.dist_haversine(self)
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Geometry<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        self.dist_haversine(&rhs.as_geom())
    }
}
