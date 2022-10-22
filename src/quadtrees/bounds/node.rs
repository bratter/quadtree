use std::marker::PhantomData;

use crate::*;
use geo::{BoundingRect, Coordinate, GeoNum, Intersects, Rect};

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

    fn datum_position(datum: &D) -> Option<Coordinate<T>> {
        let bbox = datum.geometry().bounding_rect()?;
        let (x, y) = bbox.min().x_y();
        let two = T::one() + T::one();

        Some(Coordinate {
            x: x + bbox.width() / two,
            y: y + bbox.height() / two,
        })
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
    fn nodes(&self) -> &Option<Box<[Self; 4]>> {
        &self.nodes
    }

    fn children(&self) -> DatumIter<Self, D, T> {
        DatumIter::ChainSlice(ChainSliceIter::new(
            self.children.iter().chain(&self.stuck_children),
        ))
    }

    // Setters
    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>) {
        self.nodes = nodes;
    }

    fn insert(&mut self, datum: D) -> Result<(), Error> {
        // See notes in the PointQuadTree implementation on take
        match self.nodes.take() {
            // If we have sub-nodes already, pass down the tree
            // Also works for stuck nodes, will be pushed down as far as they can go
            Some(mut sub_nodes) => {
                // Generate the bounding box for the geometry, which may fail
                let bbox = datum
                    .geometry()
                    .bounding_rect()
                    .ok_or(Error::CannotMakeBbox)?;

                // Get the index of the datum - will be based on the datum's
                // top-left point from its bounds
                let sub_node_idx = self.find_sub_node(&datum).ok_or(Error::CannotFindSubNode)?;
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
                for pt in children {
                    self.insert(pt)?;
                }
            }
            // Otherwise can simply push the point
            None => self.children.push(datum),
        }

        Ok(())
    }

    fn retrieve(&self, datum: &D) -> DatumIter<'_, BoundsNode<D, T>, D, T> {
        // Process all three functions that produce options in one hit
        // Descendants overall will produce an iterator of children in all nodes
        // that intersect with the passed node
        let descendants = if let Some(((nodes, sn_index), bbox)) = self
            .nodes.as_ref()
            .zip(self.find_sub_node(datum))
            .zip(datum.geometry().bounding_rect())
        {
            let sub_node = &nodes[sn_index as usize];
            if rect_in_rect(sub_node.bounds(), &bbox) {
                sub_node.retrieve(datum)
            } else {
                let mut inner = DatumIter::Empty;
                // Return the entire contents of any overlapping node
                // Same semantics as https://github.com/mikechambers/ExamplesByMesh/blob/master/JavaScript/QuadTree/src/QuadTree.js
                for sub_node in &**nodes {
                    if sub_node.bounds().intersects(&bbox) {
                        inner = DatumIter::ChainSelf(ChainSelfIter::new(
                            inner,
                            sub_node.descendants(),
                        ));
                    }
                }
                inner
            }
        } else {
            DatumIter::Empty
        };

        // Start with the immediate children, which may include stuck children
        // Then chain in descendants
        DatumIter::ChainSelf(ChainSelfIter::new(self.children(), descendants))
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
