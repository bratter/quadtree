use core::ops::Deref;
use geo::{GeoFloat, Point, Rect, Line, EuclideanDistance};
use num_traits::{FloatConst, Signed};
use rstar::RTreeNum;
use super::math::dist_rect_rect;

pub trait DistEuclidean<T, Rhs = Self> where Rhs: ?Sized {
    fn dist_euclidean(&self, rhs: &Rhs) -> T;
}

impl<T> DistEuclidean<T, Point<T>> for Point<T>
where
    T: GeoFloat,
{
    fn dist_euclidean(&self, rhs: &Point<T>) -> T {
        self.euclidean_distance(rhs)
    }
}

// TODO: Does this mean that anything that derefs to something with an impl here can be used?
//       So any datum that implements deref to point, rect, line, etc. should work out of the box?
//       And it looks like the type can also be a deref... so might be able to make it that as long as
//       both T and D are either geotypes or deref to geotypes it'll just work
//       Although prob no point requiring the T (which I think is the for... here) to do deref stuff as 
//       the user controls it and can just pass the right thing
// TODO: Should we make Datum just deref to something that implements the right thing? Maybe too complex?
impl<T> DistEuclidean<T, dyn Deref<Target = Point<T>>> for Point<T>
where
    T: GeoFloat,
{
    fn dist_euclidean(&self, rhs: &dyn Deref<Target = Point<T>>) -> T {
        self.euclidean_distance(rhs.deref())
    }
}

impl<T> DistEuclidean<T, Rect<T>> for Point<T>
where
    T: GeoFloat,
{
    fn dist_euclidean(&self, rhs: &Rect<T>) -> T {
        self.euclidean_distance(&rhs.to_polygon())
    }
}

impl<T> DistEuclidean<T, Line<T>> for Point<T>
where
    T: GeoFloat,
{
    fn dist_euclidean(&self, rhs: &Line<T>) -> T {
        self.euclidean_distance(rhs)
    }
}

impl<T> DistEuclidean<T, Point<T>> for Rect<T>
where
    T: GeoFloat,
{
    fn dist_euclidean(&self, rhs: &Point<T>) -> T {
        rhs.dist_euclidean(self)
    }
}

impl<T> DistEuclidean<T, Rect<T>> for Rect<T>
where
    T: GeoFloat,
{
    fn dist_euclidean(&self, rhs: &Rect<T>) -> T {
        dist_rect_rect(self, rhs)
    }
}

impl<T> DistEuclidean<T, Line<T>> for Rect<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_euclidean(&self, rhs: &Line<T>) -> T {
        self.to_polygon().euclidean_distance(rhs)
    }
}

impl<T> DistEuclidean<T, Line<T>> for Line<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_euclidean(&self, rhs: &Line<T>) -> T {
        self.euclidean_distance(rhs)
    }
}