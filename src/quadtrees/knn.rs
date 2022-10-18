use std::marker::PhantomData;
use geo::GeoFloat;

use crate::*;

/// Internal knn enum to track whether an element is a child datum or a node.
enum KnnType<'a, NodeType, D, T>
where
    NodeType: Node<D, T>,
    T: GeoFloat,
{
    Node(&'a NodeType),
    Child(&'a D),
    _NumType(PhantomData<T>),
}

/// Private, general, knn function implementation that takes an explcit node
/// This gets around forcing Node to be object safe and doing priv-in-pub to
/// get access to the root node, as root is just passed here.
/// QT implementations can simply delegate to this function.
/// Note that unlike find, knn will not return Err for empty trees as it is not
/// necessary (find would return an Err anyway).
pub(crate) fn knn<'a, D, N, X, T>(root: &'a N, cmp: &X, k: usize, r: T) -> Result<Vec<(&'a D, T)>, Error>
where
    N: Node<D, T>,
    D: Datum<T>,
    X: Distance<T>,
    T: GeoFloat,
{
    // Error early on invalid inputs
    if cmp.dist_bbox(root.bounds()) != T::zero() {
        return Err(Error::OutOfBounds);
    }

    // We work on a tuple that contains the distance plus an enum
    // containing either a child or a node, and start by seeding the root
    let root_d = cmp.dist_bbox(root.bounds());
    let mut work_stack = vec![(KnnType::Node(root), root_d)];
    let mut results = vec![];

    // Traverse the work stack in distance sorted order
    loop {
        // 1. Sort the stack in distance-descending order
        work_stack.sort_unstable_by(
            |(_, d1), (_, d2)|
            // TODO: Replace this expect, or should knn error if any single partial_cmp fails?
            d2.partial_cmp(d1).expect("Distances contain no NaN values.")
        );

        // 2. Process any Children at the top of the stack
        //    Done in an inner loop to prevent re-sorting if multiple
        //    children are on top
        while let Some(&(KnnType::Child(child), d)) = work_stack.last() {
            // If the distance is > r, we are done completely
            if d > r { return Ok(results); }

            // Pop the stack once we know its a child
            work_stack.pop();

            // Push the result, returning if we've reached k results
            results.push((child, d));
            if results.len() >= k { return Ok(results); }
        }

        // 3. Push sub Nodes and Children onto the stack if inside the radius
        //    We know that the top of the stack is either None or Some(Node(...))
        if let Some((KnnType::Node(node), d)) = work_stack.pop() {
            if d > r { return Ok(results); }

            let children = node.children()
                .into_iter()
                .map(|child| (KnnType::Child(child), cmp.dist_geom(&child.geometry())));
            work_stack.extend(children);

            if let Some(nodes) = node.nodes() {
                work_stack.extend(
                    nodes.iter().map(
                        |node| (KnnType::Node(node), cmp.dist_bbox(node.bounds()))
                    )
                );
            }
        } else {
            // If we don't match here, then we are done with the loop
            return Ok(results);
        }
    }
}