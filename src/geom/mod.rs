// Module declarations
pub mod euclidean;
pub mod geometry;
pub mod math;
pub mod spherical;

use geo::{GeoFloat, GeoNum, Line, LineString, Point, Polygon, Rect};
use num_traits::{FloatConst, Signed};
use rstar::RTreeNum;

pub use euclidean::dist::DistEuclidean;
pub(crate) use math::*;
pub use spherical::dist::DistHaversine;

use crate::{Error, GeometryRef};

/// Wrapper trait to simplfy bounds for distance calculations. Comes implmented for `f64` and `f32`
/// native types.
pub trait QtFloat: GeoFloat + Signed + FloatConst + RTreeNum {}

impl QtFloat for f32 {}
impl QtFloat for f64 {}

/// Set of valid calculation methods with which to instantiate a QuadTree implementation. This set
/// of flags, which is merged with [`GeomCalc`] in the QuadTrees drives the selection of the
/// distance algorithm.
///
/// The following options are avilable:
/// - **None:** A null operator that errors all possible distance calcs.
/// - **Euclidean:** Uses standard planar euclidean geometry.
/// - **Spherical:** Uses the haversine formula for great circle distances (useful for geographic
///   applications)
///
/// Euclidean will always output distances in the same units as the inputs, whereas Spherical
/// requires radian inputs and always produces radian outputs. To get distances in length units,
/// multiply by the sphere's diameter.
#[derive(Debug, Clone, Copy)]
pub enum CalcMethod {
    None,
    Euclidean,
    Spherical,
}

/// Struct that applies a specific distance algorithm to the reference. Provides `dist_geom` and
/// `dist_bbox` methods to calculate distances between an arbitrary geometry and a rectangular
/// bounding box respectively.
pub struct GeomCalc<'a, T>
where
    T: GeoNum,
{
    geom: GeometryRef<'a, T>,
    method: CalcMethod,
}

impl<T> GeomCalc<'_, T>
where
    T: QtFloat,
{
    /// Calculate the distance between the contained geometry and another arbitrary geometry `geom`
    /// useing the coordinate system contained in the [`GeomCalc`] struct.
    pub fn dist_geom(&self, geom: &GeometryRef<T>) -> Result<T, crate::Error> {
        match self.method {
            CalcMethod::None => Err(Error::CalcMethodNotSet),
            CalcMethod::Euclidean => Ok(self.geom.dist_euclidean(geom)),
            CalcMethod::Spherical => self.geom.dist_haversine(geom),
        }
    }

    /// Calculate the distance between the contained geometry and the passed [`Rect`] bounding box
    /// using the coordinate system contained in the [`GeomCalc`] struct.
    pub fn dist_bbox(&self, bbox: &Rect<T>) -> Result<T, crate::Error> {
        match self.method {
            CalcMethod::None => Err(Error::CalcMethodNotSet),
            CalcMethod::Euclidean => Ok(bbox.dist_euclidean(&self.geom)),
            CalcMethod::Spherical => bbox.dist_haversine(&self.geom),
        }
    }
}

/// The main constraint for QuadTree input data. Both inserted items and test comparators must
/// implement [`AsGeom`] to be used in any [`QuadTree`] implementation. It provides a methos that
/// converts an arbitrary geometry, or any custom type containing some form of geometry into a
/// [`GeometryRef`], enabling polymorphic distance calculations in the quadtree.
///
/// the trait comes pre-implemented on a variety of geo-types (see [`GeometryRef`] for a list, plus
/// [`Geometry`] and [`GeometryRef`] enums, allowing them to be used directly as data or
/// comparators.
pub trait AsGeom<T>
where
    T: GeoNum,
{
    /// Convert this geometry/datum to a [`GeometryRef`]. This is used in QuadTree implementations
    /// to provide poymorphic distance calculations.
    fn as_geom(&self) -> GeometryRef<T>;

    fn with_calc(&self, method: CalcMethod) -> GeomCalc<'_, T> {
        self.as_geom().into_calc(method)
    }
}

impl<T> AsGeom<T> for Point<T>
where
    T: GeoNum,
{
    fn as_geom(&self) -> GeometryRef<T> {
        GeometryRef::Point(self)
    }
}

impl<T> AsGeom<T> for Line<T>
where
    T: GeoNum,
{
    fn as_geom(&self) -> GeometryRef<T> {
        GeometryRef::Line(self)
    }
}

impl<T> AsGeom<T> for LineString<T>
where
    T: GeoNum,
{
    fn as_geom(&self) -> GeometryRef<T> {
        GeometryRef::LineString(self)
    }
}

impl<T> AsGeom<T> for Polygon<T>
where
    T: GeoNum,
{
    fn as_geom(&self) -> GeometryRef<T> {
        GeometryRef::Polygon(self)
    }
}

impl<T> AsGeom<T> for Rect<T>
where
    T: GeoNum,
{
    fn as_geom(&self) -> GeometryRef<T> {
        GeometryRef::Rect(self)
    }
}
