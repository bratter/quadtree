mod node;

use geo::{BoundingRect, GeoFloat, GeoNum, Rect};
use std::vec;

use super::{
    knn::knn,
    sorted::{sorted, SortIter},
};
use crate::*;
pub use datum::*;
use node::*;

/// A [`QuadTree`] implementation for bounded items (i.e. those with a finite
/// width and/or height).
/// 
/// This implementation requires the datum to simply be a [`Datum`], which
/// enables the conversion of the datum to a [`Geometry`] - an enum that wraps
/// valid underlying geo-types. Datum comes pre-implemented for all valid
/// geo-types.
/// 
/// Users can implement [`PointDatum`] on any custom type they wish to use as
/// a datum. See more detailed instructions in the [`PointDatum`] docs.
#[derive(Debug)]
pub struct BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoNum,
{
    root: BoundsNode<D, T>,
    size: usize,
}

impl<D, T> BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoNum,
{
    /// Create a new Bounds QuadTree.
    pub fn new(bounds: Rect<T>, max_depth: u8, max_children: usize) -> Self {
        BoundsQuadTree::private_new(bounds, Some(max_depth), Some(max_children))
    }

    /// Create a new Bounds QuadTree using default values for max_depth and
    /// max_children.
    pub fn from_bounds(bounds: Rect<T>) -> Self {
        BoundsQuadTree::private_new(bounds, None, None)
    }

    // Private constructor
    fn private_new(bounds: Rect<T>, max_depth: Option<u8>, max_children: Option<usize>) -> Self {
        let max_depth = max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
        let max_children = max_children.unwrap_or(DEFAULT_MAX_CHILDREN);

        Self {
            root: BoundsNode::new(bounds, 0, max_depth, max_children),
            size: 0,
        }
    }
}

impl<D, T> QuadTree<D, T> for BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoNum,
{
    type Node = BoundsNode<D, T>;

    fn size(&self) -> usize {
        self.size
    }

    fn insert(&mut self, datum: D) -> Result<(), Error> {
        // Bounds check - discard nodes that are not completely contained
        let qb = self.root.bounds();
        let db = &datum
            .geometry()
            .bounding_rect()
            .ok_or(Error::CannotMakeBbox)?;

        // Cannot use Rect::contains here, see notes on rect_in_rect for why
        if rect_in_rect(qb, db) {
            self.root.insert(datum)?;
            self.size += 1;
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }

    fn retrieve(&self, datum: &D) -> DatumIter<'_, Self::Node, D, T> {
        // Squash errors and return an empty iterator if we can't get the bbox
        if let Some(bbox) = datum.geometry().bounding_rect() {
            // Cannot use Rect::contains here, see notes on rect_in_rect for why
            if rect_in_rect(self.root.bounds(), &bbox) {
                self.root.retrieve(datum)
            } else {
                DatumIter::Empty
            }
        } else {
            DatumIter::Empty
        }
    }
}

impl<D, T> QuadTreeSearch<D, T> for BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoFloat,
{
    type Node = BoundsNode<D, T>;

    fn find_r<X>(&self, cmp: &X, r: T) -> Result<(&D, T), Error>
    where
        X: Distance<T>,
    {
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
            // No need to check the children if the bounds are too far,
            // checking bounds is cheaper then checking each child
            let bounds_dist = cmp.dist_bbox(node.bounds())?;
            if bounds_dist >= min_dist {
                continue;
            }

            // Loop through all the children of the current node, retaining
            // only the currently closest child, stuck or otherwise
            // Children will iterate through all children, stuck or otherwise
            for child in node.children() {
                // Shortcut the potentially complex distance calc by using the
                // bounds. This optimization may not always be faster, but if
                // the bbox is expensive to calculate then the distance likely
                // is also.
                let bbox = child
                    .geometry()
                    .bounding_rect()
                    .ok_or(Error::CannotMakeBbox)?;

                if cmp.dist_bbox(&bbox)? > min_dist {
                    continue;
                }

                let child_dist = cmp.dist_geom(&child.geometry())?;
                // See notes in point about <= usage
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
        X: Distance<T>,
    {
        knn(&self.root, cmp, k, r)
    }

    fn sorted<'a, X>(&'a self, cmp: &'a X) -> SortIter<'a, Self::Node, D, X, T>
    where
        X: Distance<T>,
    {
        sorted(&self.root, cmp)
    }
}

impl<'a, D, T> IntoIterator for &'a BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoNum,
{
    type Item = &'a D;
    type IntoIter = DatumIter<'a, BoundsNode<D, T>, D, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.root.descendants()
    }
}

impl<D, T> std::fmt::Display for BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoNum + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bounds Quadtree Root:")?;
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::{coord, Point, Rect};

    // helper function for bounds datum creation
    fn b(x: f64, y: f64, w: f64, h: f64) -> Rect {
        Rect::new(coord! {x: x, y: y}, coord! {x: x + w, y: y + h})
    }

    #[test]
    #[allow(unused_variables)]
    fn retrieve_grabs_all_in_overlapping_bounds() {
        let origin = Point::new(0.0, 0.0);
        let bounds = Rect::new(origin.0, coord! {x: 8.0, y: 8.0});
        let mut qt = BoundsQuadTree::new(bounds, 2, 2);

        // In root[TL][TL]
        let b1 = b(1.0, 1.0, 0.0, 0.0);
        // In root[TL][BR]
        let b2 = b(3.0, 3.0, 1.0, 1.0);
        // Stuck in root[TL]
        let b3 = b(1.0, 1.0, 2.0, 2.0);
        // Stuck in the root node
        let b4 = b(6.0, 2.0, 1.0, 4.0);
        // In root[TR]
        let b5 = b(6.0, 1.0, 1.0, 1.0);
        // In root[BR]
        let b6 = b(6.0, 5.0, 1.0, 1.0);

        qt.insert(b1.clone()).unwrap();
        qt.insert(b1.clone()).unwrap();
        qt.insert(b2.clone()).unwrap();
        qt.insert(b3.clone()).unwrap();
        qt.insert(b4.clone()).unwrap();
        qt.insert(b5.clone()).unwrap();
        qt.insert(b6.clone()).unwrap();

        // Dropping into an empty node returns only the one stuck on root
        let cmp = b(1.0, 5.0, 1.0, 1.0);
        assert_eq!(qt.retrieve(&cmp).collect::<Vec<_>>(), vec![&b4]);

        // Dropping into a node with a child returns both the child and the stuck ones above
        // Stuck children happens before the recursion, so will be at the start
        let cmp = b(5.0, 5.0, 0.5, 0.5);
        assert_eq!(qt.retrieve(&cmp).collect::<Vec<_>>(), vec![&b4, &b6]);

        // Straddling two node returns from both and all stuck
        let cmp = b(5.0, 3.0, 2.0, 2.0);
        assert_eq!(qt.retrieve(&cmp).collect::<Vec<_>>(), vec![&b4, &b5, &b6]);

        // Straddling two nodes returns everything from all sub-nodes of both, and all stuck
        // This includes if the sub node is not directly covered
        let cmp = b(1.0, 1.0, 1.0, 2.0);
        assert_eq!(
            qt.retrieve(&cmp).collect::<Vec<_>>(),
            vec![&b4, &b3, &b1, &b1, &b2]
        );
    }
}
