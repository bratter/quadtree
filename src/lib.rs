#![allow(dead_code)]

use std::fmt::Display;

/**
 * Should this have both bounded and unbounded versions?
 * If yes, maybe do them as separate objects?
 */

const MAX_CHILDREN: usize = 4;

#[derive(Debug)]
pub struct QuadTree {
    root: Node,
}

impl QuadTree {
    fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        QuadTree { root: Node::new(x, y, width, height, 0) }
    }

    fn insert(&mut self, pt: Point) {
        self.root.insert(pt);
    }

    fn retrieve(&self, pt: &Point) {
        self.root.retrieve(pt);
    }
}

impl Display for QuadTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Quadtree Root:")?;
        write!(f, "{}", self.root)
    }
}

// TODO: Implement generics on data - point needs to borrow or box a generic
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
    data: usize,
}

#[derive(Debug)]
struct Bounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
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
    fn retrieve(&self, pt: &Point) -> &Vec<Point> {
        match &self.nodes {
            Some(nodes) => {
                &nodes[self.find_sub_node(pt)].retrieve(pt)
            },
            None => {
                &self.children
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

        // TODO: Need clear way to represent the order.
        self.nodes = Some(Box::new([
            Node::new(x1, y1, wh, hh, depth),
            Node::new(x1, y2, wh, hh, depth),
            Node::new(x2, y2, wh, hh, depth),
            Node::new(x2, y1, wh, hh, depth),
        ]));
    }
}

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
