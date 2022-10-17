mod datum;
mod node;

use std::vec;
use geo::{Rect, BoundingRect, GeoFloat, GeoNum};

use crate::*;
pub use datum::*;
use node::*;
use super::knn::knn;

/// A quadtree implementation for bounded items (i.e. those with a finite width
/// and/or height).
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
    fn private_new(bounds: Rect<T>, max_depth: Option<u8>, max_children:Option<usize>) -> Self {
        let max_depth = max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
        let max_children = max_children.unwrap_or(DEFAULT_MAX_CHILDREN);

        Self {
            root: BoundsNode::new(bounds, 0, max_depth, max_children),
            size: 0,
        }
    }
}

impl<D, T> QuadTree<D> for BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoNum,
{
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

    fn retrieve(&self, datum: &D) -> Vec<&D> {
        // TODO: Convert to ok_or when these can return errors... or maybe this should just return empty? 
        let bbox = datum.geometry().bounding_rect().unwrap();
        // Cannot use Rect::contains here, see notes on rect_in_rect for why
        if rect_in_rect(self.root.bounds(), &bbox) {
            self.root.retrieve(datum)
        } else {
            vec![]
        }
    }
}

impl<D, T> QuadTreeSearch<D, T> for BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoFloat,
{
    fn find_r<X>(&self, cmp: &X, r: T) -> Option<(&D, T)> 
    where
        X: Distance<T>
    {
        let mut stack = vec![&self.root];
        let mut min_dist = r;
        let mut min_item: Option<&D> = None;

        while let Some(node) = stack.pop() {
            // No need to check the children if the bounds are too far,
            // checking bounds is cheaper then checking each child
            let bounds_dist = cmp.dist_bbox(node.bounds());
            if bounds_dist >= min_dist { continue; }

            // Loop through all the children of the current node, retaining
            // only the currently closest child, stuck or otherwise
            // Children will iterate through all children, stuck or otherwise
            for child in node.children() {
                // Shortcut the potentially complex distance calc by using the
                // bounds. This optimization may not always be faster in the
                // bbox is difficult to calculate
                let bbox = child.geometry().bounding_rect()?;
                if cmp.dist_bbox(&bbox) > min_dist { continue; }

                let child_dist = cmp.dist_datum(child);
                // See notes in point about <= usage
                if child_dist <= min_dist {
                    min_dist = child_dist;
                    min_item = Some(child);
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

    fn knn_r<X>(&self, cmp: &X, k: usize, r: T) -> Vec<(&D, T)>
    where
        X: Distance<T>
    {
        knn(&self.root, cmp, k, r)
    }
}

impl<'a, D, T> IntoIterator for &'a BoundsQuadTree<D, T>
where
    D: Datum<T>,
    T: GeoNum,
{
    type Item = &'a D;
    type IntoIter = QuadTreeIter<'a, D, BoundsNode<D, T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        QuadTreeIter::new(&self.root)
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
    use geo::{Point, Rect, coord};
    use super::*;

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
        assert_eq!(qt.retrieve(&cmp), vec![&b4]);

        // Dropping into a node with a child returns both the child and the stuck ones above
        // Stuck children happens after recursion, so will be at the end, inside out
        let cmp = b(5.0, 5.0, 0.5, 0.5);
        assert_eq!(qt.retrieve(&cmp), vec![&b6, &b4]);

        // Straddling two node returns from both and all stuck
        let cmp = b(5.0, 3.0, 2.0, 2.0);
        assert_eq!(qt.retrieve(&cmp), vec![&b5, &b6, &b4]);

        // Straddling two nodes returns everything from all sub-nodes of both, and all stuck
        // This includes if the sub node is not directly covered
        let cmp = b(1.0, 1.0, 1.0, 2.0);
        assert_eq!(qt.retrieve(&cmp), vec![&b1, &b1, &b2, &b3, &b4]);
    }
}