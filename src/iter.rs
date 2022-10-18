use std::marker::PhantomData;

use geo::GeoNum;

use super::*;

/// Preorder iterator intermediate type for a QuadTree. Iterates through the
/// QuadTree in preorder, which in this context will drill depth first into
/// each sub node starting with the lowest x/y combo, then proceeding
/// counterclockwise as observed on a standard Euclidean plane.
pub struct PreorderIter<'a, D, N, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    stack: Vec<&'a N>,
    children: Option<Vec<&'a D>>,
    _num_type: PhantomData<T>,
}

impl<'a, D, N, T> PreorderIter<'a, D, N, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    pub fn new(root: &'a N) -> Self {
        Self { stack: vec![root], children: None, _num_type: PhantomData }
    }
}

impl<'a, D, N, T> Iterator for PreorderIter<'a, D, N, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    type Item = &'a D;

    fn next(&mut self) -> Option<Self::Item> {
        // If we are currently working through children,
        // keep going if there are still results left
        if let Some(children) = &mut self.children {
            if children.len() > 0 {
                return children.pop();
            }
        }
        
        while !self.stack.is_empty() {
            let cur_node = self.stack.pop()?;
            
            // When we have sub-nodes, push onto the stack in reverse order
            if let Some(sub_nodes) = &cur_node.nodes() {
                for i in 0..4 {
                    self.stack.push(&sub_nodes[3 - i]);
                }
            }
            
            // Now grab the children and save them for iteration
            // Have to do for all nodes, as children may not only be at leaves
            let mut children = cur_node.children();
            match children.len() {
                0 => { continue; },
                1 => { return children.pop(); },
                _ => {
                    let child = children.pop();
                    self.children = Some(children);
                    return child;
                },
            }
        }

        // Finally return None if nothing left
        None
    }
}