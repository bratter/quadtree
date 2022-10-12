// use core::ops::Deref;
use geo::*;
use num_traits::{FromPrimitive, FloatConst, Signed};
use rstar::RTreeNum;

// TODO: Better than this is to have our own EuclideanDistance and HaversineDistance traits that we implement
// pub struct Rect<T = f64>(geo::Rect<T>)
// where
//     T: geo::CoordNum;

// impl<T> Deref for Rect<T>
// where
//     T: geo::CoordNum,
// {
//     type Target = geo::Rect<T>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> geo::EuclideanDistance<T, geo::Point<T>> for Rect<T>
// where
//     T: geo::GeoFloat,
// {
//     fn euclidean_distance(&self, rhs: &geo::Point<T>) -> T {
//         // self.to_polygon().euclidean_distance(rhs)
//         todo!("Use the method in bounds instead.")
//     }
// }

// impl<T> geo::EuclideanDistance<T, Rect<T>> for Rect<T>
// where
//     T: geo::GeoFloat,
// {
//     fn euclidean_distance(&self, rhs: &Rect<T>) -> T {
//         todo!("Use the method in bounds.")
//     }
// }

//
// Euclidean implementations
//

pub trait DistEuclidean<T, Rhs = Self> {
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


impl<T> DistEuclidean<T, Rect<T>> for Point<T>
where
    T: GeoFloat,
{
    // TODO: Use math from bounds
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
        // TODO: Use math from bounds
        todo!()
    }
}

impl<T> DistEuclidean<T, Line<T>> for Rect<T>
where
    T: GeoFloat
{
    fn dist_euclidean(&self, rhs: &Line<T>) -> T {
        // TODO: Use math from bounds
        todo!()
    }
}

impl<T> DistEuclidean<T, Line<T>> for Line<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum
{
    fn dist_euclidean(&self, rhs: &Line<T>) -> T {
        self.euclidean_distance(rhs)
    }
}

//
// Haversine implementations
//

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