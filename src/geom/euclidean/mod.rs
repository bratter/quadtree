use core::ops::Deref;
use std::marker::PhantomData;
use geo::{Rect, GeoNum, GeoFloat};
use num_traits::{FloatConst, Signed};
use rstar::RTreeNum;
use crate::{Distance, Datum, Geometry};
pub use dist::DistEuclidean;

mod dist;

/// Module containing Euclidean coordinate math
pub mod math;

// TODO: Rename Test as X for consistency?
/// Convenience function to wrap a `Test` item with a Euclidean geometry wrapper.
pub fn eucl<Test, T>(test: Test) -> Euclidean<Test, T>
where
    Test: Datum<T>,
    T: GeoNum,
{
    Euclidean(test, PhantomData)
}

/// Geometry wrapper type that implements Euclidean distance formulas.
#[derive(Debug)]
pub struct Euclidean<Test, T> (Test, PhantomData<T>)
where
    Test: Datum<T>,
    T: GeoNum;

impl<Test, T> Euclidean<Test, T>
where
    Test: Datum<T>,
    T: GeoNum,
{
    /// Wrap a `Test` item with a Euclidean geometry wrapper.
    pub fn new(t: Test) -> Self {
        Self(t, PhantomData)
    }
}

impl<Test, T> Distance<T> for Euclidean<Test, T>
where
    Test: Datum<T>,
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_geom(&self, geom: &Geometry<T>) -> T {
        let test_geom = self.0.geometry();

        test_geom.dist_euclidean(&geom)
    }

    fn dist_bbox(&self, bbox: &Rect<T>) -> T {
        let test_geom = self.0.geometry();

        bbox.dist_euclidean(&test_geom)
    }
}

impl<Test, T> Deref for Euclidean<Test, T>
where
    Test: Datum<T>,
    T: GeoNum,
{
    type Target = Test;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}