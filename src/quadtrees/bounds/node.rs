use std::marker::PhantomData;

use geo::{Rect, Coordinate, Intersects, BoundingRect, GeoNum};
use crate::*;

#[derive(Debug)]
pub struct BoundsNode<D, T>
where
    D: Datum<T>,
    T: GeoNum,
{
    bounds: Rect<T>,
    depth: u8,
    max_depth: u8,
    max_children: usize,
    pub children: Vec<D>,
    pub stuck_children: Vec<D>,
    pub nodes: Option<Box<[BoundsNode<D, T>; 4]>>,
    _num_type: PhantomData<T>,
}

impl<D, T> Node<D, T> for BoundsNode<D, T>
where
    D: Datum<T>,
    T: GeoNum,
{
    fn new(bounds: Rect<T>, depth: u8, max_depth: u8, max_children: usize) -> Self {
        Self {
            bounds,
            depth,
            max_depth,
            max_children,
            children: Vec::new(),
            stuck_children: Vec::new(),
            nodes: None,
            _num_type: PhantomData,
        }
    }

    fn datum_position(datum: &D) -> Coordinate<T> {
        // SAFETY: Unwrap here is Ok because bbox generation is required to
        // insert, and we ensure it returns before this is called on insertion
        let bbox = datum.geometry().bounding_rect().unwrap();
        let (x, y) = bbox.min().x_y();
        let two = T::one() + T::one();
        
        Coordinate { x: x + bbox.width() / two, y: y + bbox.height() / two }
    }

    // Getters
    fn bounds(&self) -> &Rect<T> { &self.bounds }
    fn depth(&self) -> u8 { self.depth }
    fn max_depth(&self) -> u8 { self.max_depth }
    fn max_children(&self) -> usize { self.max_children }
    fn children(&self) -> Vec<&D> {
        self.children.iter().chain(&self.stuck_children).collect()
    }
    fn nodes(&self) -> &Option<Box<[Self; 4]>> { &self.nodes }

    // Setters
    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>) { self.nodes = nodes; }

    fn insert(&mut self, datum: D) -> Result<(), Error> {
        // See notes in the PointQuadTree implementation on take
        match self.nodes.take() {
            // If we have sub-nodes already, pass down the tree
            // Also works for stuck nodes, will be pushed down as far as they can go 
            Some(mut sub_nodes) => {
                // Generate the bounding box for the geometry, which may fail
                // SAFTEY: Do this before find_sub_node to ensure bbox fails
                // are captured and the unwrap in find_sub_ndoe won't trigger
                let bbox = datum
                    .geometry()
                    .bounding_rect()
                    .ok_or(Error::CannotMakeBbox)?;

                // Get the index of the datum - will be based on the datum's
                // top-left point from its bounds
                let sub_node_idx = self.find_sub_node(&datum);
                let sub_node = &mut sub_nodes[sub_node_idx as usize];

                // Check if the datum is totally contained by the sub-node
                // If not, it is a stuck child, noting that contains includes
                // bordering, see notes in rect_in_rect for why
                if rect_in_rect(sub_node.bounds(), &bbox) {
                    sub_node.insert(datum)?
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
                for pt in children { self.insert(pt)?; }
            }
            // Otherwise can simply push the point
            None => self.children.push(datum)
        }

        Ok(())
    }

    fn retrieve(&self, datum: &D) -> Vec<&D> {
        let mut children = match &self.nodes {
            // Where there are nodes, processes them
            Some(nodes) => {
                let sub_node = &nodes[self.find_sub_node(datum) as usize];

                // TODO: See note in mod about what to do re. errors here
                let bbox = datum.geometry().bounding_rect().unwrap();
                if rect_in_rect(sub_node.bounds(), &bbox) {
                    sub_node.retrieve(datum)
                } else {
                    let mut inner = vec![];
                    // Return the entire contents of any overlapping node
                    // Same semantics as https://github.com/mikechambers/ExamplesByMesh/blob/master/JavaScript/QuadTree/src/QuadTree.js
                    for sub_node in &**nodes {
                        if sub_node.bounds().intersects(&bbox) {
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

impl<D, T> std::fmt::Display for BoundsNode<D, T>
where
    D: Datum<T>,
    T: GeoNum + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}