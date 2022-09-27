use std::marker::PhantomData;
use super::*;

// Iterator implementation for a quadtree
pub struct QuadTreeIter<'a, D, N, Geom>
where
    D: Datum<Geom>,
    N: Node<D, Geom>,
    Geom: System<Geometry = Geom>,
{
    stack: Vec<&'a N>,
    children: Option<Vec<&'a D>>,
    geom: PhantomData<Geom>,
}

impl <'a, D, N, Geom> QuadTreeIter<'a, D, N, Geom>
where
    D: Datum<Geom>,
    N: Node<D, Geom>,
    Geom: System<Geometry = Geom>,
{
    pub fn new(root: &'a N) -> Self {
        Self { stack: vec![root], children: None, geom: PhantomData }
    }
}

impl <'a, D, N, Geom> Iterator for QuadTreeIter<'a, D, N, Geom>
where
    D: Datum<Geom>,
    N: Node<D, Geom>,
    Geom: System<Geometry = Geom>,
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