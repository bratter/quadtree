use core::ops::Deref;
use geo::Rect;
use crate::Distance;
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

/// Convenience function to wrap a `Test` item with a Spherical geometry wrapper.
pub fn sphere<Test>(test: Test) -> Spherical<Test> {
    Spherical(test)
}

/// Geometry wrapper type that implements Haversine distance formulas. 
pub struct Spherical<Test> (Test);

impl<Test> Spherical<Test> {
    /// Wrap a `Test` item with a Spherical geometry wrapper.
    pub fn new(t: Test) -> Self {
        Self(t)
    }
}

impl<Test, Datum> Distance<Datum> for Spherical<Test>
where
    Test: DistHaversine<f64, Datum> + DistHaversine<f64, Rect>,
{
    fn dist_datum(&self, datum: &Datum) -> f64 {
        self.dist_haversine(datum)
    }

    fn dist_bbox(&self, bbox: &Rect) -> f64 {
        self.dist_haversine(bbox)
    }
}

impl<Test> Deref for Spherical<Test> {
    type Target = Test;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}