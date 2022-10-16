use geo::{GeoFloat, Point, Rect, Line, LineString, Polygon};
use num_traits::FromPrimitive;
use crate::Geometry;

use super::math::{dist_pt_pt, dist_pt_line, dist_rect_rect, dist_pt_rect};

// TODO: Document this, note that we require all inputs in radians and produce all outputs in radians
//       Mention this is different from the geo crate
//       Provide examples and convenience methods for conversion
pub trait DistHaversine<T, Rhs = Self> {
    fn dist_haversine(&self, rhs: &Rhs) -> T;
}

impl<T> DistHaversine<T, Geometry<T>> for Point<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            // TODO: Rewrite, must use my haversine math
            Geometry::Point(d) => todo!(),
            Geometry::Line(d) => todo!(),
            Geometry::LineString(d) => todo!(),
            Geometry::Polygon(d) => todo!(),
            Geometry::Rect(d) => todo!(),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Line<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            // TODO: Rewrite, must use my haversine math
            Geometry::Point(d) => todo!(),
            Geometry::Line(d) => todo!(),
            Geometry::LineString(d) => todo!(),
            Geometry::Polygon(d) => todo!(),
            Geometry::Rect(d) => todo!(),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for LineString<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            // TODO: Rewrite, must use my haversine math
            Geometry::Point(d) => todo!(),
            Geometry::Line(d) => todo!(),
            Geometry::LineString(d) => todo!(),
            Geometry::Polygon(d) => todo!(),
            Geometry::Rect(d) => todo!(),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Polygon<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            // TODO: Rewrite, must use my haversine math
            Geometry::Point(d) => todo!(),
            Geometry::Line(d) => todo!(),
            Geometry::LineString(d) => todo!(),
            Geometry::Polygon(d) => todo!(),
            Geometry::Rect(d) => todo!(),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Rect<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            // TODO: Rewrite, must use my haversine math
            Geometry::Point(d) => todo!(),
            Geometry::Line(d) => todo!(),
            Geometry::LineString(d) => todo!(),
            Geometry::Polygon(d) => todo!(),
            Geometry::Rect(d) => todo!(),
        }
    }
}

// TODO: This can fail for spherical, so should probably return an error if it can't be done?
impl<T> DistHaversine<T, Geometry<T>> for Geometry<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        let geo = self;

        match self {
            Geometry::Point(d) => d.dist_haversine(rhs),
            Geometry::Line(d) => d.dist_haversine(rhs),
            Geometry::LineString(d) => d.dist_haversine(rhs),
            Geometry::Polygon(d) => d.dist_haversine(rhs),
            Geometry::Rect(d) => d.dist_haversine(rhs),
        }
    }
}




// Delete these
impl<T> DistHaversine<T, Point<T>> for Point<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Point<T>) -> T {
        dist_pt_pt(self, rhs)
    }
}

impl<T> DistHaversine<T, Rect<T>> for Point<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Rect<T>) -> T {
        dist_pt_rect(self, rhs)
    }
}

impl<T> DistHaversine<T, Line<T>> for Point<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Line<T>) -> T {
        dist_pt_line(self, rhs)
    }
}

impl<T> DistHaversine<T, Point<T>> for Rect<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Point<T>) -> T {
        dist_pt_rect(rhs, self)
    }
}

impl<T> DistHaversine<T, Rect<T>> for Rect<T>
where
    T: GeoFloat + FromPrimitive,
{
    fn dist_haversine(&self, rhs: &Rect<T>) -> T {
        dist_rect_rect(self, rhs)
    }
}