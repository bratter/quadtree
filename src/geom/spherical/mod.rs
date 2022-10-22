use core::ops::Deref;
use std::marker::PhantomData;
use geo::{Rect, GeoNum, GeoFloat};
use num_traits::FromPrimitive;
use crate::{Distance, Datum, Geometry, Error};
pub use dist::DistHaversine;

mod dist;

pub mod math;
pub mod to_radians;

/// Mean radius of Earth in meters
/// This is the value recommended by the IUGG:
/// Moritz, H. (2000). Geodetic Reference System 1980. Journal of Geodesy, 74(1), 128–133. doi:10.1007/s001900050278
/// "Derived Geometric Constants: mean radius" (p133)
/// https://link.springer.com/article/10.1007%2Fs001900050278
/// https://sci-hub.se/https://doi.org/10.1007/s001900050278
/// https://en.wikipedia.org/wiki/Earth_radius#Mean_radius
/// Extracted as-is from geo-rust source
pub const MEAN_EARTH_RADIUS: f64 = 6371008.8;

/// Convenience function to wrap a `Test` item with a Spherical geometry wrapper.
pub fn sphere<X, T>(test: X) -> Spherical<X, T>
where
    X: Datum<T>,
    T: GeoNum,
{
    Spherical(test, PhantomData)
}

/// Geometry wrapper type that implements Haversine distance formulas.
#[derive(Debug)]
pub struct Spherical<X, T> (X, PhantomData<T>)
where
    X: Datum<T>,
    T: GeoNum;

impl<X, T> Spherical<X, T>
where
    X: Datum<T>,
    T: GeoNum,
{
    /// Wrap a `Test` item with a Spherical geometry wrapper.
    pub fn new(t: X) -> Self {
        Self(t, PhantomData)
    }
}

impl<X, T> Distance<T> for Spherical<X, T>
where
    X: Datum<T>,
    T: GeoFloat + FromPrimitive,
{
    fn dist_geom(&self, geom: &Geometry<T>) -> Result<T, Error> {
        let test_geom = self.0.geometry();

        test_geom.dist_haversine(&geom)
    }

    fn dist_bbox(&self, bbox: &Rect<T>) -> Result<T, Error> {
        let test_geom = self.0.geometry();

        bbox.dist_haversine(&test_geom)
    }
}

impl<X, T> Deref for Spherical<X, T>
where
    X: Datum<T>,
    T: GeoNum,
{
    type Target = X;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}