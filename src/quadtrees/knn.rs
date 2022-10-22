use geo::GeoFloat;

use crate::*;

/// Private, general, knn function implementation that takes an explcit node
/// This gets around forcing Node to be object safe and doing priv-in-pub to
/// get access to the root node, as root is just passed here.
/// QT implementations can simply delegate to this function.
/// Note that unlike find, knn will not return Err for empty trees as it is not
/// necessary (find would return an Err anyway).
/// 
/// This could be implemented in terms of the `sorted` function with
/// `take_while`, but here we have more aggressive error semantics.
pub(crate) fn knn<'a, D, N, X, T>(root: &'a N, cmp: &X, k: usize, r: T) -> Result<Vec<(&'a D, T)>, Error>
where
    N: Node<D, T>,
    D: Datum<T>,
    X: Distance<T>,
    T: GeoFloat,
{
    // Error early on invalid inputs
    let root_d = cmp.dist_bbox(root.bounds());
    if root_d != T::zero() {
        return Err(Error::OutOfBounds);
    }

    // We work on a tuple that contains the distance plus an enum
    // containing either a child or a node, and start by seeding the root
    let mut work_stack = vec![(NodeType::Node(root), root_d)];
    let mut results = vec![];

    // Traverse the work stack in distance sorted order
    loop {
        // 1. Sort the stack in distance-descending order
        work_stack.sort_unstable_by(
            |(_, d1), (_, d2)|
            d2.partial_cmp(d1).expect("Unreachable, NaN distances already removed.")
        );

        // 2. Process any Children at the top of the stack
        //    Done in an inner loop to prevent re-sorting if multiple
        //    children are on top
        while let Some(&(NodeType::Child(child), d)) = work_stack.last() {
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
        if let Some((NodeType::Node(node), d)) = work_stack.pop() {
            if d > r { return Ok(results); }

            for child in node.children() {
                let d = cmp.dist_geom(&child.geometry());

                if !d.is_finite() {
                    return Err(Error::InvalidDistance);
                }

                work_stack.push((NodeType::Child(child), d))
            }

            if let Some(nodes) = node.nodes() {
                for sub_node in nodes.iter() {
                    let d = cmp.dist_bbox(sub_node.bounds());

                    if !d.is_finite() {
                        return Err(Error::InvalidDistance);
                    }

                    work_stack.push((NodeType::Node(sub_node), d));
                }
            }
        } else {
            // If we don't match here, then we are done with the loop
            return Ok(results);
        }
    }
}