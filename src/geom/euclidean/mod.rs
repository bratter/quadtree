use core::ops::Deref;
use std::marker::PhantomData;
use geo::{Rect, GeoNum, GeoFloat};
use num_traits::{FloatConst, Signed};
use rstar::RTreeNum;
use crate::{Distance, Datum, Geometry, Error};
pub use dist::DistEuclidean;

mod dist;

/// Module containing Euclidean coordinate math
pub mod math;

/// Convenience function to wrap a `Test` item with a Euclidean geometry wrapper.
pub fn eucl<X, T>(test: X) -> Euclidean<X, T>
where
    X: Datum<T>,
    T: GeoNum,
{
    Euclidean(test, PhantomData)
}

/// Geometry wrapper type that implements Euclidean distance formulas.
#[derive(Debug)]
pub struct Euclidean<X, T> (X, PhantomData<T>)
where
    X: Datum<T>,
    T: GeoNum;

impl<X, T> Euclidean<X, T>
where
    X: Datum<T>,
    T: GeoNum,
{
    /// Wrap a `Test` item with a Euclidean geometry wrapper.
    pub fn new(t: X) -> Self {
        Self(t, PhantomData)
    }
}

impl<X, T> Distance<T> for Euclidean<X, T>
where
    X: Datum<T>,
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_geom(&self, geom: &Geometry<T>) -> Result<T, Error> {
        let test_geom = self.0.geometry();

        Ok(test_geom.dist_euclidean(&geom))
    }

    fn dist_bbox(&self, bbox: &Rect<T>) -> Result<T, Error> {
        let test_geom = self.0.geometry();

        Ok(bbox.dist_euclidean(&test_geom))
    }
}

impl<X, T> Deref for Euclidean<X, T>
where
    X: Datum<T>,
    T: GeoNum,
{
    type Target = X;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}