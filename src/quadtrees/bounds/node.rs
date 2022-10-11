use geo::{Rect, Contains, Intersects};
use crate::*;

#[derive(Debug)]
pub struct BoundsNode<T: BoundsDatum> {
    bounds: Rect,
    depth: u8,
    max_depth: u8,
    max_children: usize,
    pub children: Vec<T>,
    pub stuck_children: Vec<T>,
    pub nodes: Option<Box<[BoundsNode<T>; 4]>>,
}

impl<T: BoundsDatum> Node<T> for BoundsNode<T> {
    fn new(bounds: Rect, depth: u8, max_depth: u8, max_children: usize) -> Self {
        Self {
            bounds,
            depth,
            max_depth,
            max_children,
            children: Vec::new(),
            stuck_children: Vec::new(),
            nodes: None,
        }
    }

    // Getters
    fn bounds(&self) -> &Rect { &self.bounds }
    fn depth(&self) -> u8 { self.depth }
    fn max_depth(&self) -> u8 { self.max_depth }
    fn max_children(&self) -> usize { self.max_children }
    fn children(&self) -> Vec<&T> {
        self.children.iter().chain(&self.stuck_children).collect()
    }
    fn nodes(&self) -> &Option<Box<[Self; 4]>> { &self.nodes }

    // Setters
    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>) { self.nodes = nodes; }

    fn insert(&mut self, datum: T) {
        // See notes in the PointQuadTree implementation on take
        match self.nodes.take() {
            // If we have sub-nodes already, pass down the tree
            // Also works for stuck nodes, will be pushed down as far as they can go 
            Some(mut sub_nodes) => {
                // Get the index of the datum - will be based on the datum's
                // top-left point from its bounds
                let sub_node_idx = self.find_sub_node(&datum);
                let sub_node = &mut sub_nodes[sub_node_idx as usize];

                // Now check if the datum is totally contained by the sub-node
                // If not, it is a stuck child
                if sub_node.bounds().contains(&datum.bounds()) {
                    sub_node.insert(datum)
                } else {
                    self.stuck_children.push(datum);
                }

                // Make sure to replace the nodes
                self.nodes = Some(sub_nodes);
            }
            // If no room left, subdivide
            // See notes in PointQuadTree implementation
            None if self.children.len() >= self.max_children && !(self.depth >= self.max_depth) => {
                self.subdivide();

                let mut children = std::mem::replace(&mut self.children, Vec::new());
                children.push(datum);

                // Re-insert all children
                for pt in children { self.insert(pt); }
            }
            // Otherwise can simply push the point
            None => self.children.push(datum)
        }
    }

    fn retrieve(&self, datum: &T) -> Vec<&T> {
        let mut children = match &self.nodes {
            // Where there are nodes, processes them
            Some(nodes) => {
                let sub_node = &nodes[self.find_sub_node(datum) as usize];

                if sub_node.bounds().contains(&datum.bounds()) {
                    sub_node.retrieve(datum)
                } else {
                    let mut inner = vec![];
                    // Return the entire contents of any overlapping node
                    // Same semantics as https://github.com/mikechambers/ExamplesByMesh/blob/master/JavaScript/QuadTree/src/QuadTree.js
                    for sub_node in &**nodes {
                        if sub_node.bounds().intersects(&datum.bounds()) {
                            inner.extend(get_all_children(sub_node));
                        }
                    }
                    inner
                }
            }
            // Where there are no nodes, return the children
            // There will be no children where there are child nodes
            None => self.children.iter().collect()
        };

        // Always add the stuck children, this happens after the recursion
        children.extend(&self.stuck_children);
        children
    }
}

impl<T: BoundsDatum> std::fmt::Display for BoundsNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}