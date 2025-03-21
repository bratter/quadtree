mod node;

use super::knn::knn;
use super::sorted::{SortIter, sorted};
use crate::*;
use geo::{Coord, GeoNum, Point, Rect};
use node::PointNode;

/// Trait required for an item to be useable in a [`crate::PointQuadTree`].
/// It simply requires that the item can produce a [`Point`] for comparisons.
///
/// This trait comes implemented for [`Coordinate`] and [`Point`], so
/// coords and points can be used in quadtrees directly.
///
/// We only constrain by [`GeoNum`] here as floats are not required if we are not
/// using distance-based search methods.
///
/// We do not provide implementations for non-point shapes. It is up for the
/// user to decide how to generate a point from a polygon, etc. if they wish to
/// use them as points.
pub trait AsPoint<T = f64>
where
    T: GeoNum,
{
    fn as_point(&self) -> Point<T>;
}

// We turn a Point into a datum so it can be used in the qt directly
impl<T> AsPoint<T> for Coord<T>
where
    T: GeoNum,
{
    fn as_point(&self) -> Point<T> {
        Point::from(*self)
    }
}

impl<T> AsPoint<T> for Point<T>
where
    T: GeoNum,
{
    fn as_point(&self) -> Point<T> {
        *self
    }
}

/// A [`QuadTree`] implementation for point-like geometries.
///
/// This implementation requires the datum be a [`AsPoint`], which enables
/// conversion of the datum to a [`geo::Point`]. [`AsPoint`] comes
/// pre-implemented for [`geo::Point`] and [`geo::Coordinate`], but not for
/// other geo-types, as there is no single way to convert non-point geometries
/// to a point.
///
/// Users can implement [`AsPoint`] on any custom type they wish to use as
/// a datum.
///
/// Note that [`AsGeom`] is also required on datum in order to access the [`QuadTreeSearch`]
/// functionality per the trait bound.
#[derive(Debug)]
pub struct PointQuadTree<D, T>
where
    D: AsPoint<T>,
    T: GeoNum,
{
    root: PointNode<D, T>,

    // Maintain a count for size
    // Could calculate this each time, but it only saves usize memory
    size: usize,

    calc_method: CalcMethod,
}

impl<D, T> PointQuadTree<D, T>
where
    D: AsPoint<T>,
    T: GeoNum,
{
    /// Create a new Point QuadTree.
    pub fn new(
        bounds: Rect<T>,
        calc_method: CalcMethod,
        max_depth: u8,
        max_children: usize,
    ) -> Self {
        PointQuadTree::private_new(bounds, calc_method, Some(max_depth), Some(max_children))
    }

    /// Create a new Point QuadTree using default values for max_depth and
    /// max_children.
    pub fn from_bounds(bounds: Rect<T>, calc_method: CalcMethod) -> Self {
        PointQuadTree::private_new(bounds, calc_method, None, None)
    }

    // Private constructor
    fn private_new(
        bounds: Rect<T>,
        calc_method: CalcMethod,
        max_depth: Option<u8>,
        max_children: Option<usize>,
    ) -> Self {
        let max_depth = max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
        let max_children = max_children.unwrap_or(DEFAULT_MAX_CHILDREN);

        Self {
            root: PointNode::new(bounds, 0, max_depth, max_children),
            size: 0,
            calc_method,
        }
    }
}

impl<D, T> QuadTree<D, T> for PointQuadTree<D, T>
where
    D: AsPoint<T>,
    T: GeoNum,
{
    type Node = PointNode<D, T>;

    fn size(&self) -> usize {
        self.size
    }

    fn insert(&mut self, pt: D) -> Result<(), Error> {
        // Cannot use Rect::contains here, see notes on pt_in_rect for why
        if pt_in_rect(&self.root.bounds(), &pt.as_point()) {
            self.root.insert(pt)?;
            self.size += 1;
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    fn retrieve(&self, pt: &D) -> DatumIter<'_, Self::Node, D, T> {
        // Bounds check first - capturing out of bounds here
        // This trusts the Node implementation to act correctly
        // Cannot use Rect::contains here, see notes on pt_in_rect for why
        if pt_in_rect(&self.root.bounds(), &pt.as_point()) {
            self.root.retrieve(pt)
        } else {
            DatumIter::Empty
        }
    }
}

impl<D, T> QuadTreeSearch<D, T> for PointQuadTree<D, T>
where
    D: AsGeom<T> + AsPoint<T>,
    T: QtFloat,
{
    type Node = PointNode<D, T>;

    fn calc_method(&self) -> CalcMethod {
        self.calc_method
    }

    fn find_r<X>(&self, cmp: &X, r: T) -> Result<(&D, T), Error>
    where
        X: AsGeom<T>,
    {
        // Convert the comparison geometry to something we can work with internally
        let cmp = cmp.with_calc(self.calc_method());

        // Error early if invalid
        if cmp.dist_bbox(self.root.bounds())? != T::zero() {
            return Err(Error::OutOfBounds);
        }
        if self.size == 0 {
            return Err(Error::Empty);
        }

        let mut stack = vec![&self.root];
        let mut min_dist = r;
        let mut min_item = Err(Error::NoneInRadius);

        while let Some(node) = stack.pop() {
            // First look at the current node and check if it should be
            // processed - skip processing if the edge of the node is further
            // than the current
            let bounds_dist = cmp.dist_bbox(node.bounds())?;
            if bounds_dist >= min_dist {
                continue;
            }

            // Loop through all the children of the current node, retaining
            // only the currently closest child
            for child in node.children() {
                let child_dist = cmp.dist_geom(&child.as_geom())?;
                // Using <= here so points at a distance equal to r will be
                // returned, but this also slightly changes which Datum will
                // be returned if they are equal distances away. This is fine
                // as we only promise to return an arbitrary closest Datum
                if child_dist <= min_dist {
                    min_dist = child_dist;
                    min_item = Ok(child);
                }
            }

            // Push nodes onto the stack in reverse order
            if let Some(sub_nodes) = node.nodes() {
                for i in 0..4 {
                    stack.push(&sub_nodes[3 - i]);
                }
            }
        }

        min_item.map(|item| (item, min_dist))
    }

    fn knn_r<X>(&self, cmp: &X, k: usize, r: T) -> Result<Vec<(&D, T)>, Error>
    where
        X: AsGeom<T>,
    {
        knn(&self.root, cmp.with_calc(self.calc_method()), k, r)
    }

    fn sorted<'a, X>(&'a self, cmp: &'a X) -> SortIter<'a, Self::Node, D, T>
    where
        X: AsGeom<T> + 'a,
    {
        sorted(&self.root, cmp.with_calc(self.calc_method()))
    }
}

impl<'a, D, T> IntoIterator for &'a PointQuadTree<D, T>
where
    D: AsPoint<T>,
    T: GeoNum,
{
    type Item = &'a D;
    type IntoIter = DatumIter<'a, PointNode<D, T>, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.root.descendants()
    }
}

impl<D, T> std::fmt::Display for PointQuadTree<D, T>
where
    D: AsPoint<T>,
    T: GeoNum + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Point Quadtree Root:")?;
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod tests {
    use geo::{Point, coord};

    use super::*;

    // We can use Point directly, or make our own wrapper
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MyData(f64, f64);

    impl AsPoint for MyData {
        fn as_point(&self) -> Point {
            Point::new(self.0, self.1)
        }
    }

    #[test]
    fn subdivide_occurs_at_max_children() {
        let origin: Point = Point::new(0.0, 0.0);
        let bounds = Rect::new(origin.0, coord!(x: 1.0, y: 1.0));
        let mut qt = PointQuadTree::from_bounds(bounds, CalcMethod::Euclidean);

        // Using a data wrapper here
        let pt1 = MyData(0.1, 0.1);
        let pt2 = MyData(0.2, 0.2);
        let pt3 = MyData(0.1, 0.8);

        // Initially will be no sub-nodes, no children
        let root = &qt.root;
        assert_eq!(root.depth(), 0);
        assert_eq!(root.nodes.is_none(), true);
        assert_eq!(root.children.len(), 0);

        qt.insert(pt1).unwrap();
        qt.insert(pt1).unwrap();
        qt.insert(pt2).unwrap();
        qt.insert(pt3).unwrap();

        // Insert four points, still no sub-nodes, but now four children
        let root = &qt.root;
        assert_eq!(root.nodes.is_none(), true);
        assert_eq!(root.children.len(), 4);

        qt.insert(pt2).unwrap();

        // Fifth point, now subdivided, with four in the first sub-node
        let root = &qt.root;
        let nodes = root.nodes.as_ref().unwrap();
        let n0 = &nodes[0];
        assert_eq!(root.children.len(), 0);
        assert_eq!(nodes.len(), 4);

        // n0 takes 4 children
        assert_eq!(n0.depth(), 1);
        assert_eq!(n0.nodes.is_none(), true);
        assert_eq!(n0.children.len(), 4);

        // n3 takes 1 child and the others are empty
        assert_eq!(nodes[1].children.len(), 0);
        assert_eq!(nodes[2].children.len(), 0);
        assert_eq!(nodes[3].children.len(), 1);
    }

    #[test]
    fn can_change_max_depth_and_max_children_and_subdivide_stops_at_max_depth() {
        let origin: Point = Point::new(0.0, 0.0);
        let mut qt = PointQuadTree::new(
            Rect::new(origin.0, coord! { x: 1.0, y: 1.0 }),
            CalcMethod::Euclidean,
            2,
            2,
        );

        let pt1 = MyData(0.1, 0.1);

        qt.insert(pt1).unwrap();
        qt.insert(pt1).unwrap();
        qt.insert(pt1).unwrap();

        // All points the same value should immediately max out the depth
        // With the value putting them all in the top left
        let node = &qt.root.nodes.as_ref().unwrap()[0].nodes.as_ref().unwrap()[0];
        assert_eq!(qt.size(), 3);
        assert_eq!(node.depth(), 2);
        assert_eq!(node.children.len(), 3);
    }
}
