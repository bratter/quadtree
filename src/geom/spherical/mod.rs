use core::ops::Deref;
use std::marker::PhantomData;
use geo::{Rect, GeoNum, GeoFloat};
use num_traits::FromPrimitive;
use crate::{Distance, Datum, Geometry};
pub use dist::DistHaversine;

mod dist;

/// Module containing spherical coordinate math
pub mod math;

/// Mean radius of Earth in meters
/// This is the value recommended by the IUGG:
/// Moritz, H. (2000). Geodetic Reference System 1980. Journal of Geodesy, 74(1), 128–133. doi:10.1007/s001900050278
/// "Derived Geometric Constants: mean radius" (p133)
/// https://link.springer.com/article/10.1007%2Fs001900050278
/// https://sci-hub.se/https://doi.org/10.1007/s001900050278
/// https://en.wikipedia.org/wiki/Earth_radius#Mean_radius
/// Extracted as-is from geo-rust source
pub const MEAN_EARTH_RADIUS: f64 = 6371008.8;

// TODO: Replace Test with X throughout
/// Convenience function to wrap a `Test` item with a Spherical geometry wrapper.
pub fn sphere<Test, T>(test: Test) -> Spherical<Test, T>
where
    Test: Datum<T>,
    T: GeoNum,
{
    Spherical(test, PhantomData)
}

/// Geometry wrapper type that implements Haversine distance formulas. 
pub struct Spherical<Test, T> (Test, PhantomData<T>)
where
    Test: Datum<T>,
    T: GeoNum;

impl<Test, T> Spherical<Test, T>
where
    Test: Datum<T>,
    T: GeoNum,
{
    /// Wrap a `Test` item with a Spherical geometry wrapper.
    pub fn new(t: Test) -> Self {
        Self(t, PhantomData)
    }
}

impl<Test, T> Distance<T> for Spherical<Test, T>
where
    Test: Datum<T>,
    T: GeoFloat + FromPrimitive,
{
    fn dist_datum(&self, datum: &dyn Datum<T>) -> T {
        let test_geom = self.0.geometry();
        let datum_geom = datum.geometry();

        test_geom.dist_haversine(&datum_geom)
    }

    fn dist_geom(&self, geom: &Geometry<T>) -> T {
        let test_geom = self.0.geometry();

        test_geom.dist_haversine(&geom)
    }

    fn dist_bbox(&self, bbox: &Rect<T>) -> T {
        let test_geom = self.0.geometry();

        bbox.dist_haversine(&test_geom)
    }
}

impl<Test, T> Deref for Spherical<Test, T>
where
    Test: Datum<T>,
    T: GeoNum,
{
    type Target = Test;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}