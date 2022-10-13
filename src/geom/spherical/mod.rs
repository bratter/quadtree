use core::ops::Deref;
use geo::Rect;
use crate::Distance;
pub use dist::DistHaversine;

mod dist;

/// Module containing spherical coordinate math
pub mod math;

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