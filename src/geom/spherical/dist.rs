use crate::{Error, Geometry};
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
/// through the [`crate::Distance`] trait using the [`crate::Spherical`] type.
pub trait DistHaversine<T, Rhs = Self> {
    fn dist_haversine(&self, rhs: &Rhs) -> Result<T, Error>;
}

impl<T> DistHaversine<T, Geometry<T>> for Point<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        match rhs {
            Geometry::Point(d) => Ok(dist_pt_pt(self, d)),
            Geometry::Line(d) => Ok(dist_pt_line(self, d)),
            Geometry::LineString(d) => Ok(dist_pt_linestring(self, d)),
            Geometry::Polygon(_) => Err(Error::InvalidDistance),
            Geometry::Rect(d) => Ok(dist_pt_rect(self, d)),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Line<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        match rhs {
            Geometry::Point(d) => Ok(dist_pt_line(d, self)),
            Geometry::Line(_) => Err(Error::InvalidDistance),
            Geometry::LineString(_) => Err(Error::InvalidDistance),
            Geometry::Polygon(_) => Err(Error::InvalidDistance),
            Geometry::Rect(_) => Err(Error::InvalidDistance),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for LineString<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        match rhs {
            Geometry::Point(_) => Err(Error::InvalidDistance),
            Geometry::Line(_) => Err(Error::InvalidDistance),
            Geometry::LineString(_) => Err(Error::InvalidDistance),
            Geometry::Polygon(_) => Err(Error::InvalidDistance),
            Geometry::Rect(_) => Err(Error::InvalidDistance),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        match rhs {
            Geometry::Point(_) => Err(Error::InvalidDistance),
            Geometry::Line(_) => Err(Error::InvalidDistance),
            Geometry::LineString(_) => Err(Error::InvalidDistance),
            Geometry::Polygon(_) => Err(Error::InvalidDistance),
            Geometry::Rect(_) => Err(Error::InvalidDistance),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Rect<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        match rhs {
            Geometry::Point(d) => Ok(dist_pt_rect(d, self)),
            Geometry::Line(_) => Err(Error::InvalidDistance),
            Geometry::LineString(_) => Err(Error::InvalidDistance),
            Geometry::Polygon(_) => Err(Error::InvalidDistance),
            Geometry::Rect(d) => Ok(dist_rect_rect(self, d)),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Geometry<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> Result<T, Error> {
        match self {
            Geometry::Point(d) => d.dist_haversine(rhs),
            Geometry::Line(d) => d.dist_haversine(rhs),
            Geometry::LineString(d) => d.dist_haversine(rhs),
            Geometry::Polygon(d) => d.dist_haversine(rhs),
            Geometry::Rect(d) => d.dist_haversine(rhs),
        }
    }
}
