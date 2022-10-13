use geo::{Rect, Coordinate};
use crate::*;

#[derive(Debug)]
pub struct PointNode<T: PointDatum> {
    bounds: Rect,
    depth: u8,
    max_depth: u8,
    max_children: usize,
    pub children: Vec<T>,
    pub nodes: Option<Box<[PointNode<T>; 4]>>,
}

impl<D: PointDatum> Node<D> for PointNode<D> {
    fn new(bounds: Rect, depth: u8, max_depth: u8, max_children: usize) -> Self {
        Self {
            bounds,
            depth,
            max_depth,
            max_children,
            children: Vec::new(),
            nodes: None,
        }
    }

    fn datum_position(datum: &D) -> Coordinate {
        datum.point().0
    }

    // Getters
    fn bounds(&self) -> &Rect { &self.bounds }
    fn depth(&self) -> u8 { self.depth }
    fn max_depth(&self) -> u8 { self.max_depth }
    fn max_children(&self) -> usize { self.max_children }
    fn children(&self) -> Vec<&D> { self.children.iter().collect() }
    fn nodes(&self) -> &Option<Box<[PointNode<D>; 4]>> { &self.nodes }

    // Setters
    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>) { self.nodes = nodes; }

    fn insert(&mut self, datum: D) {
        // Take ownership of the sub-nodes before matching to enable the insertion
        // This, apparently, is a very common pattern
        // Works here because we replace the nodes at the end, and the None branch
        // is unaffected. Overall a more ergonomic solution than the alterantive
        // `let sub_nodes = self.nodes.as_mut().unwrap()`
        match self.nodes.take() {
            // If we have sub-nodes already, pass down the tree
            Some(mut sub_nodes) => {
                let sub_node_idx = self.find_sub_node(&datum);
                sub_nodes[sub_node_idx as usize].insert(datum);
                
                // Make sure to replace the nodes
                self.nodes = Some(sub_nodes);
            },
            // If there is no room left, subdivide and push all children down
            // Subdivision does not happen if we've exceeded the max depth,
            // which takes priority over the children length
            None if self.children.len() >= self.max_children && !(self.depth >= self.max_depth)  => {
                self.subdivide();

                // Replace the old children with a new empty vector
                // and push the new point on last to preserve ordering
                let mut children = std::mem::replace(&mut self.children, Vec::new());
                children.push(datum);

                // Now consume the original children vector
                for pt in children { self.insert(pt); }
            }
            // Otherwise can simply push the point
            None => {
                self.children.push(datum);
            }
        }
    }

    // Pulls all children within the node that would contain the passed point
    fn retrieve(&self, datum: &D) -> Vec<&D> {
        match &self.nodes {
            Some(nodes) => {
                nodes[self.find_sub_node(datum) as usize].retrieve(datum)
            },
            None => {
                // Convert &Vec<T> to Vec<&T>
                self.children.iter().collect()
            }
        }
    }
}

impl<D: PointDatum> std::fmt::Display for PointNode<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}