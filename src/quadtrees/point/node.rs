use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use crate::*;
use geo::{Coordinate, GeoNum, Rect};

/// [`Node`] implementation for [`PointQuadTree`].
#[derive(Debug)]
pub struct PointNode<D, T>
where
    D: AsPoint<T>,
    T: GeoNum,
{
    bounds: Rect<T>,
    depth: u8,
    max_depth: u8,
    max_children: usize,
    pub children: Vec<D>,
    pub nodes: Option<Box<[PointNode<D, T>; 4]>>,
    _num_type: PhantomData<T>,
}

impl<D, T> Node<D, T> for PointNode<D, T>
where
    D: AsPoint<T>,
    T: GeoNum,
{
    fn new(bounds: Rect<T>, depth: u8, max_depth: u8, max_children: usize) -> Self {
        Self {
            bounds,
            depth,
            max_depth,
            max_children,
            children: Vec::new(),
            nodes: None,
            _num_type: PhantomData,
        }
    }

    fn datum_position(datum: &D) -> Option<Coordinate<T>> {
        Some(datum.as_point().0)
    }

    // Getters
    fn bounds(&self) -> &Rect<T> {
        &self.bounds
    }

    fn depth(&self) -> u8 {
        self.depth
    }

    fn max_depth(&self) -> u8 {
        self.max_depth
    }

    fn max_children(&self) -> usize {
        self.max_children
    }

    fn nodes(&self) -> &Option<Box<[PointNode<D, T>; 4]>> {
        &self.nodes
    }

    fn children(&self) -> DatumIter<Self, D, T> {
        DatumIter::Slice(self.children.iter())
    }

    // Setters
    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>) {
        self.nodes = nodes;
    }

    fn insert(&mut self, datum: D) -> Result<(), Error> {
        // Take ownership of the sub-nodes before matching to enable the insertion
        // This, apparently, is a very common pattern
        // Works here because we replace the nodes at the end, and the None branch
        // is unaffected. Overall a more ergonomic solution than the alterantive
        // `let sub_nodes = self.nodes.as_mut().unwrap()`
        match self.nodes.take() {
            // If we have sub-nodes already, pass down the tree
            Some(mut sub_nodes) => {
                let sub_node_idx = self.find_sub_node(&datum).ok_or(Error::CannotFindSubNode)?;
                sub_nodes[sub_node_idx as usize].insert(datum)?;

                // Make sure to replace the nodes
                self.nodes = Some(sub_nodes);
            }
            // If there is no room left, subdivide and push all children down
            // Subdivision does not happen if we've exceeded the max depth,
            // which takes priority over the children length
            None if self.children.len() >= self.max_children && !(self.depth >= self.max_depth) => {
                self.subdivide();

                // Replace the old children with a new empty vector
                // and push the new point on last to preserve ordering
                let mut children = std::mem::replace(&mut self.children, Vec::new());
                children.push(datum);

                // Now consume the original children vector
                for pt in children {
                    self.insert(pt)?;
                }
            }
            // Otherwise can simply push the point
            None => {
                self.children.push(datum);
            }
        }

        Ok(())
    }

    // Pulls all children within the node that would contain the passed point
    fn retrieve(&self, datum: &D) -> DatumIter<'_, PointNode<D, T>, D, T> {
        match &self.nodes {
            Some(nodes) => {
                if let Some(sn) = self.find_sub_node(datum) {
                    nodes[sn as usize].retrieve(datum)
                } else {
                    DatumIter::Empty
                }
            }
            None => self.children(),
        }
    }
}

impl<D, T> Display for PointNode<D, T>
where
    D: AsPoint<T>,
    T: GeoNum + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}
