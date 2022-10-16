use geo::{GeoFloat, Point, Rect, Line, EuclideanDistance, LineString, Polygon};
use num_traits::{FloatConst, Signed};
use rstar::RTreeNum;
use crate::Geometry;

use super::math::dist_rect_rect;

pub trait DistEuclidean<T, Rhs = Self> {
    fn dist_euclidean(&self, rhs: &Rhs) -> T;
}

impl<T> DistEuclidean<T, Geometry<T>> for Point<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(d) => self.euclidean_distance(d),
            Geometry::Line(d) => self.euclidean_distance(d),
            Geometry::LineString(d) => self.euclidean_distance(d),
            Geometry::Polygon(d) => self.euclidean_distance(d),
            Geometry::Rect(d) => self.euclidean_distance(&d.to_polygon()),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for Line<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(d) => self.euclidean_distance(d),
            Geometry::Line(d) => self.euclidean_distance(d),
            Geometry::LineString(d) => self.euclidean_distance(d),
            Geometry::Polygon(d) => self.euclidean_distance(d),
            Geometry::Rect(d) => self.euclidean_distance(&d.to_polygon()),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for LineString<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(d) => self.euclidean_distance(d),
            Geometry::Line(d) => self.euclidean_distance(d),
            Geometry::LineString(d) => self.euclidean_distance(d),
            Geometry::Polygon(d) => self.euclidean_distance(d),
            Geometry::Rect(d) => self.euclidean_distance(&d.to_polygon()),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for Polygon<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(d) => self.euclidean_distance(d),
            Geometry::Line(d) => self.euclidean_distance(d),
            Geometry::LineString(d) => self.euclidean_distance(d),
            Geometry::Polygon(d) => self.euclidean_distance(d),
            Geometry::Rect(d) => self.euclidean_distance(&d.to_polygon()),
        }
    }
}

impl<T> DistEuclidean<T, Geometry<T>> for Rect<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(d) => d.euclidean_distance(&self.to_polygon()),
            Geometry::Line(d) => d.euclidean_distance(&self.to_polygon()),
            Geometry::LineString(d) => d.euclidean_distance(&self.to_polygon()),
            Geometry::Polygon(d) => d.euclidean_distance(&self.to_polygon()),
            Geometry::Rect(d) => dist_rect_rect(self, d),
        }
    }
}

// TODO: This can fail for spherical, so should probably return an error if it can't be done?
impl<T> DistEuclidean<T, Geometry<T>> for Geometry<T>
where
    T: GeoFloat + FloatConst + Signed + RTreeNum,
{
    fn dist_euclidean(&self, rhs: &Geometry<T>) -> T {
        match self {
            Geometry::Point(d) => d.dist_euclidean(rhs),
            Geometry::Line(d) => d.dist_euclidean(rhs),
            Geometry::LineString(d) => d.dist_euclidean(rhs),
            Geometry::Polygon(d) => d.dist_euclidean(rhs),
            Geometry::Rect(d) => d.dist_euclidean(rhs),
        }
    }
}