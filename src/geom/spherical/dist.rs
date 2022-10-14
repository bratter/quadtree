use geo::{CoordFloat, Point, Rect, Line};
use num_traits::FromPrimitive;
use super::math::{dist_pt_pt, dist_pt_line, dist_rect_rect, dist_pt_rect};

// TODO: Document this, note that we require all inputs in radians and produce all outputs in radians
//       Mention this is different from the geo crate
//       Provide examples and convenience methods for conversion
pub trait DistHaversine<T, Rhs = Self> {
    fn dist_haversine(&self, rhs: &Rhs) -> T;
}

impl<T> DistHaversine<T, Point<T>> for Point<T>
where
    T: CoordFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Point<T>) -> T {
        dist_pt_pt(self, rhs)
    }
}

impl<T> DistHaversine<T, Rect<T>> for Point<T>
where
    T: CoordFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Rect<T>) -> T {
        dist_pt_rect(self, rhs)
    }
}

impl<T> DistHaversine<T, Line<T>> for Point<T>
where
    T: CoordFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Line<T>) -> T {
        dist_pt_line(self, rhs)
    }
}

impl<T> DistHaversine<T, Point<T>> for Rect<T>
where
    T: CoordFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Point<T>) -> T {
        dist_pt_rect(rhs, self)
    }
}

impl<T> DistHaversine<T, Rect<T>> for Rect<T>
where
    T: CoordFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Rect<T>) -> T {
        dist_rect_rect(self, rhs)
    }
}

// TODO: Consider implementing this and the reverse
// impl<T> DistHaversine<T, Line<T>> for Rect<T>
// where
//     T: CoordFloat,
// {
//     fn dist_haversine(&self, rhs: &Line<T>) -> T {
//         todo!("Haversine Line Rect combos")
//     }
// }

// TODO: Consider implementing this
// impl<T> DistHaversine<T, Line<T>> for Line<T>
// where
//     T: CoordFloat,
// {
//     fn dist_haversine(&self, rhs: &Line<T>) -> T {
//         todo!("Haversine Line for Line")
//     }
// }