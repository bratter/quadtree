// Iterator implementation for a quadtree

use super::*;

impl <'a, T: Datum<Geom>, Geom: System<Geometry = Geom>> IntoIterator for &'a QuadTree<T, Geom> {
    type Item = &'a T;
    type IntoIter = QuadTreeIter<'a, T, Geom>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
pub struct QuadTreeIter<'a, T: Datum<Geom>, Geom: System<Geometry = Geom>> {
    stack: Vec<&'a Node<T, Geom>>,
    child_iter: Option<std::slice::Iter<'a, T>>,
}

impl <'a, T: Datum<Geom>, Geom: System<Geometry = Geom>> QuadTreeIter<'a, T, Geom> {
    pub fn new(root: &'a Node<T, Geom>) -> Self {
        QuadTreeIter { stack: vec![root], child_iter: None }
    }
}

impl <'a, T: Datum<Geom>, Geom: System<Geometry = Geom>> Iterator for QuadTreeIter<'a, T, Geom> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // If we are currently working through a child iterator,
        // keep going if there are still results left
        let next = self.child_iter.as_mut().and_then(|x| x.next());
        if next.is_some() { return next; }

        while !self.stack.is_empty() {
            let cur_node = self.stack.pop()?;

            match &cur_node.nodes {
                Some(sub_nodes) => {
                    // When we have sub-nodes, push onto the stack in reverse order
                    for i in 0..4 {
                        self.stack.push(&sub_nodes[3 - i]);
                    }
                    continue;
                }
                None => {
                    // When there are no sub-nodes, grab an iterator for the children
                    let mut child_iter = cur_node.children.iter();

                    match child_iter.next() {
                        Some(item) => {
                            self.child_iter = Some(child_iter);
                            return Some(item);
                        }
                        None => { continue; }
                    }
                }
            }
        }

        // Finally return None if nothing left
        None
    }
}