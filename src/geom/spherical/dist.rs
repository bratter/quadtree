use geo::{CoordFloat, Point, Rect, Line, HaversineDistance};
use num_traits::FromPrimitive;

pub trait DistHaversine<T, Rhs = Self> {
    fn dist_haversine(&self, rhs: &Rhs) -> T;
}

impl<T> DistHaversine<T, Point<T>> for Point<T>
where
    T: CoordFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Point<T>) -> T {
        self.haversine_distance(rhs)
    }
}

impl<T> DistHaversine<T, Rect<T>> for Point<T>
where
    T: CoordFloat
{
    fn dist_haversine(&self, rhs: &Rect<T>) -> T {
        // TODO: Use math from spherical, but check correctness later
        todo!()
    }
}

impl<T> DistHaversine<T, Line<T>> for Point<T>
where
    T: CoordFloat
{
    fn dist_haversine(&self, rhs: &Line<T>) -> T {
        // TODO: As above
        todo!()
    }
}

impl<T> DistHaversine<T, Point<T>> for Rect<T>
where
    T: CoordFloat
{
    fn dist_haversine(&self, rhs: &Point<T>) -> T {
        // TODO: As above
        todo!()
    }
}

impl<T> DistHaversine<T, Rect<T>> for Rect<T>
where
    T: CoordFloat
{
    fn dist_haversine(&self, rhs: &Rect<T>) -> T {
        // TODO: As above
        todo!()
    }
}

impl<T> DistHaversine<T, Line<T>> for Rect<T>
where
    T: CoordFloat
{
    fn dist_haversine(&self, rhs: &Line<T>) -> T {
        // TODO: As above
        todo!()
    }
}

impl<T> DistHaversine<T, Line<T>> for Line<T>
where
    T: CoordFloat
{
    fn dist_haversine(&self, rhs: &Line<T>) -> T {
        // TODO: As above
        todo!()
    }
}