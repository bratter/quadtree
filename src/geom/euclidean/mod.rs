use core::ops::Deref;
use geo::Rect;
use crate::Distance;
pub use dist::DistEuclidean;

mod dist;

/// Module containing Euclidean coordinate math
pub mod math;

/// Convenience function to wrap a `Test` item with a Euclidean geometry wrapper.
pub fn eucl<Test>(test: Test) -> Euclidean<Test> {
    Euclidean(test)
}

/// Geometry wrapper type that implements Euclidean distance formulas. 
pub struct Euclidean<Test> (Test);

impl<Test> Euclidean<Test> {
    /// Wrap a `Test` item with a Euclidean geometry wrapper.
    pub fn new(t: Test) -> Self {
        Self(t)
    }
}

impl<Test, Datum> Distance<Datum> for Euclidean<Test>
where
    Test: DistEuclidean<f64, Datum> + DistEuclidean<f64, Rect>,
{
    fn dist_datum(&self, datum: &Datum) -> f64 {
        self.dist_euclidean(datum)
    }

    fn dist_bbox(&self, bbox: &Rect) -> f64 {
        self.dist_euclidean(bbox)
    }
}

impl<Test> Deref for Euclidean<Test> {
    type Target = Test;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}