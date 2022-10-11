use core::ops::Deref;
use core::marker::PhantomData;
use geo;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point<Geom> {
    point: geo::Point<f64>,
    _geometry: PhantomData<Geom>,
}

impl<Geom> Point<Geom> {
    pub fn new(x: f64, y: f64) -> Point<Geom> {
        Point {
            point: geo::Point::new(x, y),
            _geometry: PhantomData,
        }
    }
    
    pub fn x_y(&self) -> (f64, f64) {
        self.point.x_y()
    }
}

impl<Geom> Deref for Point<Geom> {
    type Target = geo::Point<f64>;

    fn deref(&self) -> &Self::Target {
        &self.point
    }
}

impl<Geom> From<geo::Coordinate> for Point<Geom> {
    fn from(coord: geo::Coordinate) -> Self {
        Point { point: geo::Point::from(coord), _geometry: PhantomData }
    }
}

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        Point::<Euclidean>::new($x, $y)
    };
    ($x:expr, $y:expr, $geom:ty) => {
        Point::<$geom>::new($x, $y)
    };
    ($coord:expr, $geom:ty) => {
        Point::<$geom>::
    }
}
pub use point;