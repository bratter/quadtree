
/*
 * TODO: Should this have both bounded and point versions?
 * If yes, maybe do them as separate objects with a trait?
 */

const MAX_CHILDREN: usize = 4;

// TODO: Implement generics on data - point needs to borrow or box a generic
// TODO: Hide the existence of points, and it shouldn't need clone
//       May have to take a closure that accepts T and returns (x, y)
//       Or just implement a GetCoords trait with fn get_coords on the type T
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    data: usize,
}

impl Point {
    pub fn new(x: f64, y: f64, data: usize) -> Self {
        Self { x, y, data }
    }

    fn in_bounds(&self, bounds: &Bounds) -> bool {
        let x1 = bounds.x;
        let x2 = x1 + bounds.width;
        let y1 = bounds.y;
        let y2 = y1 + bounds.height;

        let Point { x, y, .. } = *self;

        x >= x1 && x <= x2 && y >= y1 && x <= y2
    }
}

#[derive(Debug)]
struct Bounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

#[derive(Debug)]
// Here we maintain a count for size
// Could also calculate this each time, but it only saves usize memory
pub struct QuadTree {
    root: Node,
    size: usize,
}

// Out of bounds insertion and retrieval behavior is up to the specific
// implementation, however insertshould not panic
impl QuadTree {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        QuadTree {
            root: Node::new(x, y, width, height, 0),
            size: 0
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    // TODO: Should we use Error semantics on insert? Rust requires errors to be handled
    // Here we assume that `root.insert` always succeeds so we can increment
    // count. This should work if the pt is in bounds
    pub fn insert(&mut self, pt: Point) {
        if pt.in_bounds(&self.root.bounds) {
            self.root.insert(pt);
            self.size += 1;
        }
    }

    pub fn retrieve(&self, pt: &Point) -> Option<&Vec<Point>> {
        // Bounds check first - capturing out of ounds here
        // This trusts the Node implementation to act correctly
        if pt.in_bounds(&self.root.bounds) {
            self.root.retrieve(pt)
        } else {
            None
        }

    }
}

impl std::fmt::Display for QuadTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Quadtree Root:")?;
        write!(f, "{}", self.root)
    }
}

#[derive(Debug)]
struct Node {
    bounds: Bounds,
    depth: u8,
    children: Vec<Point>,
    nodes: Option<Box<[Node; 4]>>,
}

impl std::fmt::Display for Node {
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

impl Node {
    fn new (x: f64, y: f64, width: f64, height: f64, depth: u8) -> Node {
        Node {
            bounds: Bounds { x, y, width, height },
            depth,
            children: Vec::new(),
            nodes: None
        }
    }

    fn insert(&mut self, pt: Point) {
        match self.nodes {
            // If we have sub-nodes already, pass down the tree
            // TODO: Want to grab the nodes array in the match
            // But there appears to be no way to make it wortk without an error
            // Leading to the ugly as_mut().unwrap()
            Some(_) => {
                let sub_node = self.find_sub_node(&pt);
                self.nodes.as_mut().unwrap()[sub_node].insert(pt);
            },
            // Otherwise add to this node when there is room left at this depth
            // TODO: Implement some sort of max depth / max children logic other than the fixed value
            None if self.children.len() < MAX_CHILDREN => {
                self.children.push(pt);
            }
            // If not subdivide and push all children down
            None => {
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
        }
    }

    // Pulls all children within the node that would contain the passed point
    fn retrieve(&self, pt: &Point) -> Option<&Vec<Point>> {
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

    fn find_sub_node(&self, pt: &Point) -> usize {
        let b = &self.bounds;
        let left = pt.x <= b.x + b.width / 2.0;
        let top = pt.y <= b.y + b.height / 2.0;

        if left && top { 0 }
        else if !left && top { 1 }
        else if left && !top { 3 }
        else { 2 }
    }

    fn subdivide(&mut self) {
        let depth = self.depth + 1;
        
        let wh = self.bounds.width / 2.0;
        let hh = self.bounds.height / 2.0;
        
        let x1 = self.bounds.x;
        let y1 = self.bounds.y;
        let x2 = x1 + wh;
        let y2 = y1 + hh;

        // Fixed order of iteration tl, tr, br, bl
        self.nodes = Some(Box::new([
            Node::new(x1, y1, wh, hh, depth),
            Node::new(x1, y2, wh, hh, depth),
            Node::new(x2, y2, wh, hh, depth),
            Node::new(x2, y1, wh, hh, depth),
        ]));
    }
}

// TODO: Update this test suite
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut qt = QuadTree::new(0.0, 0.0, 1.0, 1.0);
        
        let pt1 = Point { x: 0.5, y: 0.5, data: 42 };
        let pt2 = Point { x: 0.3, y: 0.5, data: 42 };
        let pt3 = Point { x: 0.5, y: 0.3, data: 42 };
        
        qt.insert(pt1.clone());
        qt.insert(pt1.clone());
        qt.insert(pt1.clone());
        qt.insert(pt2.clone());
        qt.insert(pt3.clone());

        println!("{}", qt);
        println!("{:?}", qt.retrieve(&pt1));
    }
}
