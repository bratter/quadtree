use geo::{GeoFloat, Point, Rect, Line, LineString, Polygon};
use crate::Geometry;

use super::math::{dist_pt_pt, dist_pt_line, dist_rect_rect, dist_pt_rect};

// TODO: Add more Haversine implementations, or make distance produce an error instead
// TODO: Document this, note that we require all inputs in radians and produce all outputs in radians
//       Mention this is different from the geo crate
//       Provide examples and convenience methods for conversion
pub trait DistHaversine<T, Rhs = Self> {
    fn dist_haversine(&self, rhs: &Rhs) -> T;
}

impl<T> DistHaversine<T, Geometry<T>> for Point<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(d) => dist_pt_pt(self, d),
            Geometry::Line(d) => dist_pt_line(self, d),
            Geometry::LineString(_) => todo!(),
            Geometry::Polygon(_) => todo!(),
            Geometry::Rect(d) => dist_pt_rect(self, d),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Line<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(d) => dist_pt_line(d, self),
            Geometry::Line(_) => todo!(),
            Geometry::LineString(_) => todo!(),
            Geometry::Polygon(_) => todo!(),
            Geometry::Rect(_) => todo!(),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for LineString<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(_) => todo!(),
            Geometry::Line(_) => todo!(),
            Geometry::LineString(_) => todo!(),
            Geometry::Polygon(_) => todo!(),
            Geometry::Rect(_) => todo!(),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Polygon<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(_) => todo!(),
            Geometry::Line(_) => todo!(),
            Geometry::LineString(_) => todo!(),
            Geometry::Polygon(_) => todo!(),
            Geometry::Rect(_) => todo!(),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Rect<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match rhs {
            Geometry::Point(d) => dist_pt_rect(d, self),
            Geometry::Line(_) => todo!(),
            Geometry::LineString(_) => todo!(),
            Geometry::Polygon(_) => todo!(),
            Geometry::Rect(d) => dist_rect_rect(self, d),
        }
    }
}

impl<T> DistHaversine<T, Geometry<T>> for Geometry<T>
where
    T: GeoFloat,
{
    fn dist_haversine(&self, rhs: &Geometry<T>) -> T {
        match self {
            Geometry::Point(d) => d.dist_haversine(rhs),
            Geometry::Line(d) => d.dist_haversine(rhs),
            Geometry::LineString(d) => d.dist_haversine(rhs),
            Geometry::Polygon(d) => d.dist_haversine(rhs),
            Geometry::Rect(d) => d.dist_haversine(rhs),
        }
    }
}