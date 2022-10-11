use core::ops::Deref;
use core::marker::PhantomData;
use geo;

use crate::Point;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment<Geom> {
    line: geo::Line<f64>,
    _geometry: PhantomData<Geom>,
}

impl <Geom> Segment<Geom>
where
    Geom: Copy
{
    pub fn new(start: Point<Geom>, end: Point<Geom>) -> Segment<Geom> {
        Segment {
            line: geo::Line::new(start.x_y(), end.x_y()),
            _geometry: PhantomData,
        }
    }

    pub fn a(&self) -> Point<Geom> { Point::new(self.start.x, self.start.y) }
    pub fn b(&self) -> Point<Geom> { Point::new(self.end.x, self.end.y) }

    pub fn start_point(&self) -> Point<Geom> { Point::new(self.start.x, self.start.y) }
    pub fn end_point(&self) -> Point<Geom> { Point::new(self.end.x, self.end.y) }
}

impl<Geom> Deref for Segment<Geom> {
    type Target = geo::Line<f64>;

    fn deref(&self) -> &Self::Target {
        &self.line
    }
}

// TODO: Work out what syntax to use here, then fix the errors
#[macro_export]
macro_rules! segment {
    ($x:expr, $y:expr) => {
        Segment::<Euclidean>::new($x, $y)
    };
    ($x:expr, $y:expr, $geom:ty) => {
        Segment::<$geom>::new($x, $y)
    };
}
pub use segment;