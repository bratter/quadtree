/*
 * TODO: Should this have both bounded and point versions?
 *       If yes, maybe do them as separate objects with a trait?
 *       What about an integer-with-power-2-bounds
 * 
 * TODO: Find method, customizable finder function as a trait or closure
 */

const DEFAULT_MAX_CHILDREN: usize = 4;
const DEFAULT_MAX_DEPTH: u8 = 4;

enum SubNode {
    TopLeft = 0,
    TopRight = 1,
    BottomRight = 2,
    BottomLeft = 3,
}

pub trait Point {
    fn coords(&self) -> (f64, f64);

    fn in_bounds(&self, bounds: &Bounds) -> bool {
        let x1 = bounds.x;
        let x2 = x1 + bounds.width;
        let y1 = bounds.y;
        let y2 = y1 + bounds.height;

        let (x, y) = self.coords();

        x >= x1 && x <= x2 && y >= y1 && x <= y2
    }
}

#[derive(Debug, Clone)]
pub struct Bounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Bounds {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }
}

#[derive(Debug)]
// Here we maintain a count for size
// Could also calculate this each time, but it only saves usize memory
pub struct QuadTree<T> {
    root: Node<T>,
    size: usize,
}

// TODO: When generalizing behavior...
// Out of bounds insertion and retrieval behavior is up to the specific
// implementation, and could even panic if required
impl <T: Point> QuadTree<T> {
    // Private constructor
    fn private_new(bounds: Bounds, max_depth: Option<u8>, max_children:Option<usize>) -> Self {
        let max_depth = max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
        let max_children = max_children.unwrap_or(DEFAULT_MAX_CHILDREN);

        QuadTree {
            root: Node::new(bounds, 0, max_depth, max_children),
            size: 0,
        }
    }

    /// Create a new Quadtree.
    pub fn new(bounds: Bounds, max_depth: u8, max_children: usize) -> Self {
        QuadTree::private_new(bounds, Some(max_depth), Some(max_children))
    }

    /// Create a new QuadTree using default values for max_depth and max_children.
    pub fn new_def(bounds: Bounds) -> Self {
        QuadTree::private_new(bounds, None, None)
    }

    pub fn size(&self) -> usize {
        self.size
    }

    // TODO: Should we use Error semantics on insert? Rust requires errors to be handled
    // Here we assume that `root.insert` always succeeds so we can increment
    // count. This should work if the pt is in bounds
    pub fn insert(&mut self, pt: T) {
        if pt.in_bounds(&self.root.bounds) {
            self.root.insert(pt);
            self.size += 1;
        }
    }

    pub fn retrieve(&self, pt: &T) -> Option<&Vec<T>> {
        // Bounds check first - capturing out of bounds here
        // This trusts the Node implementation to act correctly
        if pt.in_bounds(&self.root.bounds) {
            self.root.retrieve(pt)
        } else {
            None
        }
    }

    pub fn iter(&self) -> QuadTreeIter<'_, T> {
        QuadTreeIter::new(&self.root)
    }

    /// Find the closest item in the quadtree to the passed point
    pub fn find(&self, pt: T) -> Option<&T> {
        // TODO: Implement find with a customizable distance calc

        let mut stack = vec![&self.root];
        let mut min_dist = f64::INFINITY;
        let mut min_item: Option<&T> = None;

        while let Some(node) = stack.pop() {
            // First look at the current node and check if it should be
            // processed - skip processing if the edge of the node is further
            // than the current minDist
            // TODO: Point to node distance trait method?
            //       Which one to put this on? Challenge is that we need to unify the type of the bounding box and the type of the incoming points
            //       The corners of a node have to be points and the edges lines/line segments
            //       These have to be in the same coordinate system as the added points
            //       Let's just start with the cartesian example to get it working
            let (px, py) = pt.coords();
            let (x1, y1) = (node.bounds.x, node.bounds.y);
            let (x2, y2) = (x1 + node.bounds.width, y1 + node.bounds.height);

            let x_cmp = if px < x1 { x1 } else if px > x2 { x2 } else { px };
            let y_cmp = if py < y1 { y1 } else if py > y2 { y2 } else { py };

            // TODO: Do this instead, with the appropriate bounding line in each if block
            let d = if px < x1 { 4 }
                else if px > x2 { 3 }
                else if py < y1 { 2 }
                else if py > y2 { 1 }
                else { 0 };

            // Also using squared distance here
            // TODO: This is not the best generalized algorithm
            //       Maybe there is a point to BB that can come for free, or maybe just have to find the closest line first
            let d = (px - x_cmp).powi(2) + (py - y_cmp).powi(2);
            if d >= min_dist { continue; }

            // Loop through all the children of the current node, retaining
            // only the currently closest child
            for child in &node.children {
                // TODO: pt to pt distance
                //       Start with cartesian example for simplicity, then abstract
                //       Using suqre distance to avoid sqrt calc
                // TODO: Consider providing a d_squared in the trait that has an auto implementation that can be overridden
                let (px, py) = pt.coords();
                let (cx, cy) = child.coords();
                let d = (px - cx).powi(2) + (py - cy).powi(2);

                if d < min_dist {
                    min_dist = d;
                    min_item = Some(child);
                }
            }
        }

        min_item
    }
}

impl <T> std::fmt::Display for QuadTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Quadtree Root:")?;
        write!(f, "{}", self.root)
    }
}

impl <'a, T: Point> IntoIterator for &'a QuadTree<T> {
    type Item = &'a T;
    type IntoIter = QuadTreeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct QuadTreeIter<'a, T> {
    stack: Vec<&'a Node<T>>,
    child_iter: Option<std::slice::Iter<'a, T>>,
}

impl <'a, T> QuadTreeIter<'a, T> {
    fn new(root: &'a Node<T>) -> Self {
        QuadTreeIter { stack: vec![root], child_iter: None }
    }
}

impl <'a, T> Iterator for QuadTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // If we are currently working through a child iterator,
        // keep going if there are still results left
        let next = self.child_iter.as_mut().and_then(|x| x.next());
        if next.is_some() { return next; }

        while !self.stack.is_empty() {
            let cur_node = self.stack.pop()?;

            match &cur_node.nodes {
                Some(sub_nodes) => {
                    // When we have sub-nodes, push onto the stack in reverse order
                    for i in 0..4 {
                        self.stack.push(&sub_nodes[3 - i]);
                    }
                    continue;
                }
                None => {
                    // When there are no sub-nodes, grab an iterator for the children
                    let mut child_iter = cur_node.children.iter();

                    match child_iter.next() {
                        Some(item) => {
                            self.child_iter = Some(child_iter);
                            return Some(item);
                        }
                        None => { continue; }
                    }
                }
            }
        }

        // Finally return None if nothing left
        None
    }
}

#[derive(Debug)]
struct Node<T> {
    bounds: Bounds,
    depth: u8,
    max_depth: u8,
    max_children: usize,
    children: Vec<T>,
    nodes: Option<Box<[Node<T>; 4]>>,
}

impl <T> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = " ".repeat(self.depth as usize * 4);
        let count = self.children.len();
        let children = if count == 0 {
            "".to_owned()
        } else if count == 1 {
            " 1 child".to_owned()
        } else {
            format!(" {count} children")
        };

        writeln!(f, "{indent}({:.2}, {:.2}):{children}", self.bounds.x, self.bounds.y)?;
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                write!(f, "{node}")?;
            }
        };
        write!(f, "")
    }
}

impl <T: Point> Node<T> {
    fn new (bounds: Bounds, depth: u8, max_depth: u8, max_children: usize) -> Node<T> {
        Node {
            bounds,
            depth,
            max_depth,
            max_children,
            children: Vec::new(),
            nodes: None,
        }
    }

    fn insert(&mut self, pt: T) {
        match self.nodes {
            // If we have sub-nodes already, pass down the tree
            Some(_) => {
                let sub_node_idx = self.find_sub_node(&pt);
                let sub_nodes = self.nodes.as_mut().unwrap();

                sub_nodes[sub_node_idx as usize].insert(pt);
            },
            // If there is no room left, subdivide and push all children down
            // Subdivision does not happen if we've exceeded the max depth,
            // which takes priority over the children length
            None if self.children.len() >= self.max_children && !(self.depth >= self.max_depth)  => {
                self.subdivide();

                // Replace the old children with a new empty vector
                // and push the new point on last to preserve ordering
                let mut children = std::mem::replace(&mut self.children, Vec::new());
                children.push(pt);

                // Now consume the original children vector
                for pt in children { self.insert(pt); }
            }
            // Otherwise can simply push the point
            None => {
                self.children.push(pt);
            }
        }
    }

    // Pulls all children within the node that would contain the passed point
    fn retrieve(&self, pt: &T) -> Option<&Vec<T>> {
        match &self.nodes {
            Some(nodes) => {
                nodes[self.find_sub_node(pt) as usize].retrieve(pt)
            },
            None => {
                if self.children.len() == 0 {
                    None
                } else {
                    Some(&self.children)
                }
            },
        }
    }

    fn find_sub_node(&self, pt: &T) -> SubNode {
        let b = &self.bounds;
        let (x, y) = pt.coords();
        let left = x <= b.x + b.width / 2.0;
        let top = y <= b.y + b.height / 2.0;

        if left && top { SubNode::TopLeft }
        else if !left && top { SubNode::TopRight }
        else if left && !top { SubNode::BottomLeft }
        else { SubNode::BottomRight }
    }

    fn subdivide(&mut self) {
        let depth = self.depth + 1;
        let md = self.max_depth;
        let mc = self.max_children;
        
        let wh = self.bounds.width / 2.0;
        let hh = self.bounds.height / 2.0;
        
        let x1 = self.bounds.x;
        let y1 = self.bounds.y;
        let x2 = x1 + wh;
        let y2 = y1 + hh;

        // Fixed order of iteration tl, tr, br, bl
        self.nodes = Some(Box::new([
            Node::new(Bounds::new(x1, y1, wh, hh), depth, md, mc),
            Node::new(Bounds::new(x1, y2, wh, hh), depth, md, mc),
            Node::new(Bounds::new(x2, y2, wh, hh), depth, md, mc),
            Node::new(Bounds::new(x2, y1, wh, hh), depth, md, mc),
        ]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Making Pt Copy for this test
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Pt(f64, f64);

    impl Point for Pt {
        fn coords(&self) -> (f64, f64) {
            (self.0, self.1)
        }
    }

    #[test]
    fn subdivide_occurs_at_max_children() {
        let mut qt = QuadTree::new_def(Bounds::new(0.0, 0.0, 1.0, 1.0));
        
        let pt1 = Pt(0.1, 0.1);
        let pt2 = Pt(0.2, 0.2);
        let pt3 = Pt(0.1, 0.8);
        
        // Initially will be no sub-nodes, no children
        let root = &qt.root;
        assert_eq!(root.depth, 0);
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
        assert_eq!(n0.depth, 1);
        assert_eq!(n0.nodes.is_none(), true);
        assert_eq!(n0.children.len(), 4);

        // n3 takes 1 child and the others are empty
        assert_eq!(nodes[1].children.len(), 0);
        assert_eq!(nodes[2].children.len(), 0);
        assert_eq!(nodes[3].children.len(), 1);
    }

    #[test]
    fn can_change_max_depth_and_max_children_and_subdivide_stops_at_max_depth() {
        let mut qt = QuadTree::new(
            Bounds::new(0.0, 0.0, 1.0, 1.0),
            2,
            2,
        );

        let pt1 = Pt(0.1, 0.1);
        
        qt.insert(pt1);
        qt.insert(pt1);
        qt.insert(pt1);
        
        // All points the same value should immediately max out the depth
        // With the value putting them all in the top left
        let node = &qt.root
            .nodes.as_ref().unwrap()[0]
            .nodes.as_ref().unwrap()[0];
        assert_eq!(qt.size(), 3);
        assert_eq!(node.depth, 2);
        assert_eq!(node.children.len(), 3);
    }
}
