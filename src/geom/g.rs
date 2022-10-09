// TODO: Using the geo crate
use core::ops::Deref;
use core::marker::PhantomData;
use geo::{self, EuclideanDistance, HaversineDistance};

use super::Distance;

enum Euclidean {}
enum Spherical {}

struct Point<T> {
    point: geo::Point<f64>,
    _geometry: PhantomData<T>,
}

impl<T> Deref for Point<T> {
    type Target = geo::Point<f64>;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}

// TODO: Probably move all the distance implementations into a module with the trait
impl Distance<Point<Euclidean>> for Point<Euclidean> {
    fn dist(&self, cmp: &Point<Euclidean>) -> f64 {
        self.euclidean_distance(&**cmp)
    }
}

// TODO: Should use my implementations instead as can use any radius
impl Distance<Point<Spherical>> for Point<Spherical> {
    fn dist(&self, cmp: &Point<Spherical>) -> f64 {
        self.haversine_distance(&**cmp)
    }
}

// TODO: Move to macros module
macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::<Euclidean> { point: geo::Point::new($x, $y), _geometry: PhantomData }
    };
    ($x:expr, $y:expr, $geom:ty) => {
        P2::<$geom> { point: geo::Point::new($x, $y), _geometry: PhantomData }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x() {
        let origin = point!(0.0, 0.0);
    }
}