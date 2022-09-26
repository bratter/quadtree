// A quadtree implementation for bounded items (i.e. those with a finite width
// and/or height)
mod node;

use crate::*;
use node::*;

#[derive(Debug)]
pub struct BoundsQuadTree<T: BoundsDatum<Geom>, Geom: System<Geometry = Geom>> {
    root: BoundsNode<T, Geom>,
    size: usize,
}

impl <T: BoundsDatum<Geom>, Geom: System<Geometry = Geom>> BoundsQuadTree<T, Geom> {
    // Private constructor
    fn private_new(bounds: Bounds<Geom>, max_depth: Option<u8>, max_children:Option<usize>) -> Self {
        let max_depth = max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
        let max_children = max_children.unwrap_or(DEFAULT_MAX_CHILDREN);

        Self {
            root: BoundsNode::new(bounds, 0, max_depth, max_children),
            size: 0,
        }
    }
}

// TODO: Can quadtree be a single implementation that uses different nodes? With the exception of find
//       Can Node be an unexported trait that implements its methods differently?
//       Or at least what can be moved to the trait?
impl <T: BoundsDatum<Geom>, Geom: System<Geometry = Geom>> QuadTree<T, Geom> for BoundsQuadTree<T, Geom> {
    fn new(bounds: Bounds<Geom>, max_depth: u8, max_children: usize) -> Self {
        BoundsQuadTree::private_new(bounds, Some(max_depth), Some(max_children))
    }

    fn new_def(bounds: Bounds<Geom>) -> Self {
        BoundsQuadTree::private_new(bounds, None, None)
    }

    fn size(&self) -> usize {
        self.size
    }

    fn insert(&mut self, datum: T) {
        // Bounds check - discard nodes that are not completely contained
        let qb = self.root.bounds();
        let db = datum.bounds();

        if qb.contains_bounds(db) {
            self.root.insert(datum);
            self.size += 1;
        }
    }

    fn retrieve(&self, datum: &T) -> Vec<&T> {
        if self.root.bounds().contains_bounds(datum.bounds()) {
            self.root.retrieve(datum)
        } else {
            vec![]
        }
    }

    // fn iter(&self) -> QuadTreeIter<'_, T, Geom> {
        // QuadTreeIter::new(&self.root)
        // TODO: The iterator is struggling with the types
    //     todo!()
    // }

    fn find(&self, cmp: &Point<Geom>) -> Option<&T> {
        let mut stack = vec![&self.root];
        let mut min_dist = f64::INFINITY;
        let mut min_item: Option<&T> = None;

        while let Some(node) = stack.pop() {
            // No need to check the children if the bounds are too far,
            // checking bounds is cheaper then checking each child
            let bounds_dist = node.bounds().dist(cmp);
            if bounds_dist >= min_dist { continue; }

            // Loop through all the children of the current node, retaining
            // only the currently closest child, stuck or otherwise
            for child in node.children.iter().chain(&node.stuck_children) {
                let child_dist = child.bounds().dist(cmp);
                if child_dist < min_dist {
                    min_dist = child_dist;
                    min_item = Some(child);
                }
            }
        }

        min_item
    }
}