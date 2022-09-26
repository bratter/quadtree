use crate::*;

mod node;
use node::*;

#[derive(Debug)]
pub struct PointQuadTree<T: Datum<Geom>, Geom: System<Geometry = Geom>> {
    root: PointNode<T, Geom>,
    // Maintain a count for size
    // Could calculate this each time, but it only saves usize memory
    size: usize,
}

impl <T: Datum<Geom>, Geom: System<Geometry = Geom>> PointQuadTree<T, Geom> {
    // Private constructor
    fn private_new(bounds: Bounds<Geom>, max_depth: Option<u8>, max_children:Option<usize>) -> Self {
        let max_depth = max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
        let max_children = max_children.unwrap_or(DEFAULT_MAX_CHILDREN);

        Self {
            root: PointNode::new(bounds, 0, max_depth, max_children),
            size: 0,
        }
    }
}

// TODO: When generalizing behavior...
// Out of bounds insertion and retrieval behavior is up to the specific
// implementation, and could even panic if required
impl <T: Datum<Geom>, Geom: System<Geometry = Geom>> QuadTree<T, Geom> for PointQuadTree<T, Geom> {
    fn new(bounds: Bounds<Geom>, max_depth: u8, max_children: usize) -> Self {
        PointQuadTree::private_new(bounds, Some(max_depth), Some(max_children))
    }

    fn new_def(bounds: Bounds<Geom>) -> Self {
        PointQuadTree::private_new(bounds, None, None)
    }

    fn size(&self) -> usize {
        self.size
    }

    // TODO: Should we use Error semantics on insert? Rust requires errors to be handled
    // Here we assume that `root.insert` always succeeds so we can increment
    // count. This should work if the pt is in bounds
    fn insert(&mut self, pt: T) {
        if self.root.bounds().contains(pt.point()) {
            self.root.insert(pt);
            self.size += 1;
        }
    }

    fn retrieve(&self, pt: &T) -> Vec<&T> {
        // Bounds check first - capturing out of bounds here
        // This trusts the Node implementation to act correctly
        if self.root.bounds().contains(pt.point()) {
            self.root.retrieve(pt)
        } else {
            vec![]
        }
    }

    // fn iter(&self) -> QuadTreeIter<'_, T, Geom> {
        // TODO: Fix this
        // todo!()
        // QuadTreeIter::new(&self.root)
    // }

    fn find(&self, cmp: &Point<Geom>) -> Option<&T> {
        let mut stack = vec![&self.root];
        let mut min_dist = f64::INFINITY;
        let mut min_item: Option<&T> = None;

        while let Some(node) = stack.pop() {
            // First look at the current node and check if it should be
            // processed - skip processing if the edge of the node is further
            // than the current 
            let bounds_dist = node.bounds().dist(cmp);
            if bounds_dist >= min_dist { continue; }

            // Loop through all the children of the current node, retaining
            // only the currently closest child
            for child in &node.children {
                let child_dist = child.point().dist(cmp);
                if child_dist < min_dist {
                    min_dist = child_dist;
                    min_item = Some(child);
                }
            }
        }

        min_item
    }
}

impl <T: Datum<Geom>, Geom: System<Geometry = Geom>> std::fmt::Display for PointQuadTree<T, Geom> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Quadtree Root:")?;
        write!(f, "{}", self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // We can use Point directly, or make our own wrapper
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MyData(f64, f64);

    impl Datum<Euclidean> for MyData {
        fn point(&self) -> Point<Euclidean> {
            Point::new(self.0, self.1)
        }
    }

    #[test]
    fn subdivide_occurs_at_max_children() {
        let origin = Point::new(0.0, 0.0);
        let mut qt = PointQuadTree::new_def(Bounds::new(origin, 1.0, 1.0));
        
        // Using a data wrapper here
        let pt1 = MyData(0.1, 0.1);
        let pt2 = MyData(0.2, 0.2);
        let pt3 = MyData(0.1, 0.8);
        
        // Initially will be no sub-nodes, no children
        let root = &qt.root;
        assert_eq!(root.depth(), 0);
        assert_eq!(root.nodes.is_none(), true);
        assert_eq!(root.children.len(), 0);

        qt.insert(pt1);
        qt.insert(pt1);
        qt.insert(pt2);
        qt.insert(pt3);
        
        // Insert four points, still no sub-nodes, but now four children
        let root = &qt.root;
        assert_eq!(root.nodes.is_none(), true);
        assert_eq!(root.children.len(), 4);

        qt.insert(pt2);

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
        let origin = Point::new(0.0, 0.0);
        let mut qt = PointQuadTree::new(
            Bounds::new(origin, 1.0, 1.0),
            2,
            2,
        );

        let pt1 = MyData(0.1, 0.1);
        
        qt.insert(pt1);
        qt.insert(pt1);
        qt.insert(pt1);
        
        // All points the same value should immediately max out the depth
        // With the value putting them all in the top left
        let node = &qt.root
            .nodes.as_ref().unwrap()[0]
            .nodes.as_ref().unwrap()[0];
        assert_eq!(qt.size(), 3);
        assert_eq!(node.depth(), 2);
        assert_eq!(node.children.len(), 3);
    }
}