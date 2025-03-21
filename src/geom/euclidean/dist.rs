use geo::{Distance, Euclidean, Line, LineString, Point, Polygon, Rect};

use super::math::dist_rect_rect;
use crate::{AsGeom, Geometry, GeometryRef, geom::QtFloat};

/// Trait applied to all valid geometries to calculate the Euclidean distance
/// between geometries.
///
/// The trait is implemented for all [`Geometry`] geometries, and the enum
/// itself.
///
/// This trait should not need to be used outside the crate as it is abstracted
/// through the [`CalcMethod`] enum.
pub trait DistEuclidean<T, Rhs = Self> {
    fn dist_euclidean(&self, rhs: &Rhs) -> T;
}

impl<T> DistEuclidean<T, GeometryRef<'_, T>> for Point<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &GeometryRef<T>) -> T {
        match rhs {
            &GeometryRef::Point(d) => Euclidean::distance(self, d),
            &GeometryRef::Line(d) => Euclidean::distance(self, d),
            &GeometryRef::LineString(d) => Euclidean::distance(self, d),
            &GeometryRef::Polygon(d) => Euclidean::distance(self, d),
            &GeometryRef::Rect(d) => Euclidean::distance(self, &d.to_polygon()),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for Point<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        self.dist_euclidean(&rhs.as_geom())
    }
}

impl<T> DistEuclidean<T, GeometryRef<'_, T>> for Line<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &GeometryRef<T>) -> T {
        match rhs {
            &GeometryRef::Point(d) => Euclidean::distance(self, d),
            &GeometryRef::Line(d) => Euclidean::distance(self, d),
            &GeometryRef::LineString(d) => Euclidean::distance(self, d),
            &GeometryRef::Polygon(d) => Euclidean::distance(self, d),
            &GeometryRef::Rect(d) => Euclidean::distance(self, &d.to_polygon()),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for Line<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        self.dist_euclidean(&rhs.as_geom())
    }
}

impl<T> DistEuclidean<T, GeometryRef<'_, T>> for LineString<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &GeometryRef<T>) -> T {
        match rhs {
            &GeometryRef::Point(d) => Euclidean::distance(self, d),
            &GeometryRef::Line(d) => Euclidean::distance(self, d),
            &GeometryRef::LineString(d) => Euclidean::distance(self, d),
            &GeometryRef::Polygon(d) => Euclidean::distance(self, d),
            &GeometryRef::Rect(d) => Euclidean::distance(self, &d.to_polygon()),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for LineString<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        self.dist_euclidean(&rhs.as_geom())
    }
}

impl<T> DistEuclidean<T, GeometryRef<'_, T>> for Polygon<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &GeometryRef<T>) -> T {
        match rhs {
            &GeometryRef::Point(d) => Euclidean::distance(self, d),
            &GeometryRef::Line(d) => Euclidean::distance(self, d),
            &GeometryRef::LineString(d) => Euclidean::distance(self, d),
            &GeometryRef::Polygon(d) => Euclidean::distance(self, d),
            &GeometryRef::Rect(d) => Euclidean::distance(self, &d.to_polygon()),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for Polygon<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        self.dist_euclidean(&rhs.as_geom())
    }
}

impl<T> DistEuclidean<T, GeometryRef<'_, T>> for Rect<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &GeometryRef<T>) -> T {
        match rhs {
            &GeometryRef::Point(d) => Euclidean::distance(&self.to_polygon(), d),
            &GeometryRef::Line(d) => Euclidean::distance(&self.to_polygon(), d),
            &GeometryRef::LineString(d) => Euclidean::distance(&self.to_polygon(), d),
            &GeometryRef::Polygon(d) => Euclidean::distance(&self.to_polygon(), d),
            &GeometryRef::Rect(d) => dist_rect_rect(self, d),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for Rect<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        self.dist_euclidean(&rhs.as_geom())
    }
}

impl<T> DistEuclidean<T, GeometryRef<'_, T>> for GeometryRef<'_, T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &GeometryRef<T>) -> T {
        match self {
            GeometryRef::Point(d) => d.dist_euclidean(rhs),
            GeometryRef::Line(d) => d.dist_euclidean(rhs),
            GeometryRef::LineString(d) => d.dist_euclidean(rhs),
            GeometryRef::Polygon(d) => d.dist_euclidean(rhs),
            GeometryRef::Rect(d) => d.dist_euclidean(rhs),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for GeometryRef<'_, T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        self.dist_euclidean(&rhs.as_geom())
    }
}

impl<T> DistEuclidean<T, GeometryRef<'_, T>> for Geometry<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &GeometryRef<'_, T>) -> T {
        rhs.dist_euclidean(&self.as_geom())
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for Geometry<T>
where
    T: QtFloat,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        self.dist_euclidean(&rhs.as_geom())
    }
}
