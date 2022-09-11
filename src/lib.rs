/*
 * TODO: Should this have both bounded and point versions?
 * If yes, maybe do them as separate objects with a trait?
 */

const DEFAULT_MAX_CHILDREN: usize = 4;
const DEFAULT_MAX_DEPTH: u8 = 4;

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
    /// Create a new Quadtree.
    pub fn new(bounds: Bounds, max_depth: Option<u8>, max_children:Option<usize>) -> Self {
        let max_depth = max_depth.unwrap_or(DEFAULT_MAX_DEPTH);
        let max_children = max_children.unwrap_or(DEFAULT_MAX_CHILDREN);

        QuadTree {
            root: Node::new(bounds, 0, max_depth, max_children),
            size: 0,
        }
    }

    /// Create a new QuadTree using default values for max_depth and max_children.
    pub fn new_def(bounds: Bounds) -> Self {
        QuadTree::new(bounds, None, None)
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
        // Bounds check first - capturing out of ounds here
        // This trusts the Node implementation to act correctly
        if pt.in_bounds(&self.root.bounds) {
            self.root.retrieve(pt)
        } else {
            None
        }

    }
}

impl <T> std::fmt::Display for QuadTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Quadtree Root:")?;
        write!(f, "{}", self.root)
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
            // TODO: Want to grab the nodes array in the match
            //       But there appears to be no way to make it work without an error
            //       Leading to the ugly as_mut().unwrap()
            Some(_) => {
                let sub_node = self.find_sub_node(&pt);
                self.nodes.as_mut().unwrap()[sub_node].insert(pt);
            },
            // If there is no room left, subdivide and push all children down
            // Subdivision does not happen if we've exceeded the max depth,
            // which takes priority over the children length
            None if self.children.len() >= self.max_children && !(self.depth >= self.max_depth)  => {
                self.subdivide();

                // Replace the old children with a new empty vector
                let children = std::mem::replace(&mut self.children, Vec::new());

                // Now we can consume the original children vector
                for pt in children {
                    self.insert(pt);
                }

                // Retry the insert the new point last to preserve ordering
                self.insert(pt);
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
                nodes[self.find_sub_node(pt)].retrieve(pt)
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

    fn find_sub_node(&self, pt: &T) -> usize {
        let b = &self.bounds;
        let (x, y) = pt.coords();
        let left = x <= b.x + b.width / 2.0;
        let top = y <= b.y + b.height / 2.0;

        if left && top { 0 }
        else if !left && top { 1 }
        else if left && !top { 3 }
        else { 2 }
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

// TODO: Update this test suite
//       At least include testing max depth/size etc.
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Pt(f64, f64);

    impl Point for Pt {
        fn coords(&self) -> (f64, f64) {
            (self.0, self.1)
        }
    }

    #[test]
    fn it_works() {
        let mut qt = QuadTree::new_def(Bounds::new(0.0, 0.0, 1.0, 1.0));
        
        let pt1 = Pt(0.5, 0.5);
        let pt2 = Pt(0.3, 0.5);
        let pt3 = Pt(0.5, 0.3);
        
        qt.insert(pt1.clone());
        qt.insert(pt1.clone());
        qt.insert(pt1.clone());
        qt.insert(pt2.clone());
        qt.insert(pt3.clone());

        println!("{}", qt);
        println!("{:?}", qt.retrieve(&pt1));
    }
}
