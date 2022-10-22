use geo::GeoFloat;

use crate::*;

/// Iterator to output QuadTree data in distance-sorted order.
///
/// Due to the additional requirement for supporting arbitrary test types, this
/// is not unifed with [`DatumIter`].
pub struct SortIter<'a, N, D, X, T>
where
    N: Node<D, T>,
    D: Datum<T>,
    X: Distance<T>,
    T: GeoFloat,
{
    // We work on a tuple of a node/child enum and its distance to the comparator
    stack: Vec<(NodeType<'a, N, D, T>, T)>,
    cmp: &'a X,
    is_sorted: bool,
}

impl<'a, N, D, X, T> Iterator for SortIter<'a, N, D, X, T>
where
    N: Node<D, T>,
    D: Datum<T>,
    X: Distance<T>,
    T: GeoFloat,
{
    type Item = (&'a D, T);

    fn next(&mut self) -> Option<Self::Item> {
        // 1. Sort the stack in distance-descending order
        //    Only do this if the is_sorted flag is false
        if !self.is_sorted {
            self.stack.sort_unstable_by(|(_, d1), (_, d2)| {
                d2.partial_cmp(d1)
                    .expect("Unreachable, NaN distances already removed.")
            });
        }

        // 2. Return early if the stack is empty
        let (item, d) = self.stack.pop()?;

        // Now process the rest in a match
        match item {
            // 2. Process any children at the top of the stack
            //    No need to re-sort as no pushing
            NodeType::Child(child) => Some((child, d)),

            // 3. Push sub-nodes and children onto the stack
            //    Set the flag to re-sort because new items hve been added
            //    Then recurse because we have not returned anything
            NodeType::Node(node) => {
                for child in node.children() {
                    if let Some(d) = self
                        .cmp
                        .dist_geom(&child.geometry())
                        .ok()
                        .and_then(|d| d.is_finite().then_some(d))
                    {
                        self.stack.push((NodeType::Child(child), d));
                    }
                }

                if let Some(nodes) = node.nodes() {
                    for sub_node in nodes.iter() {
                        if let Some(d) = self
                            .cmp
                            .dist_bbox(sub_node.bounds())
                            .ok()
                            .and_then(|d| d.is_finite().then_some(d))
                        {
                            self.stack.push((NodeType::Node(sub_node), d));
                        }
                    }
                }

                self.is_sorted = false;
                self.next()
            }

            // This is PhantomData
            NodeType::_NumType(_) => unreachable!(),
        }
    }
}

/// Private, general, implementation that returns an iterator that produces
/// QuadTree data in distance sorted order starting at the passed root node.
/// This gets around forcing Node to be object safe and doing priv-in-pub to
/// get access to the root node, as root is just passed here.
/// QT implementations can simply delegate to this function.
/// Note that unlike find and knn, this method tries to not error, skipping over
/// items it cannot process.
pub(crate) fn sorted<'a, D, N, X, T>(root: &'a N, cmp: &'a X) -> SortIter<'a, N, D, X, T>
where
    N: Node<D, T>,
    D: Datum<T>,
    X: Distance<T>,
    T: GeoFloat,
{
    // Simply return an empty iterator if the bbox is out of bounds or the
    // distance calc fails
    let root_d = cmp
        .dist_bbox(root.bounds())
        .ok()
        .and_then(|d| (d == T::zero()).then_some(d));

    match root_d {
        Some(d) => SortIter {
            stack: vec![(NodeType::Node(root), d)],
            cmp,
            is_sorted: false,
        },
        None => SortIter {
            stack: vec![],
            cmp,
            is_sorted: true,
        },
    }
}
