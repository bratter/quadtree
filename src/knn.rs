use crate::*;

enum KnnType<'a, NodeType, T>
where
    NodeType: Node<T>,
    T: Datum,
{
    Node(&'a NodeType),
    Child(&'a T),
}

// Private, general, knn function implementation that takes an explcit node
// This gets around forcing Node to be object safe and doing priv-in-pub to
// get access to the root node, as root is just passed here.
// QT implementations can simply delegate to this function.
pub fn knn<'a, T, N, X>(root: &'a N, cmp: &X, k: usize, r: f64) -> Vec<(&'a T, f64)>
where
    T: Datum,
    N: Node<T>,
    X: SearchDistance<T>,
{
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
            d2.partial_cmp(d1).expect("Distances contain no NaN values.")
        );

        // 2. Process any Children at the top of the stack
        //    Done in an inner loop to prevent re-sorting if multiple
        //    children are on top
        while let Some(&(KnnType::Child(child), d)) = work_stack.last() {
            // If the distance is > r, we are done completely
            if d > r { return results; }

            // Pop the stack once we know its a child
            work_stack.pop();

            // Push the result, returning if we've reached k results
            results.push((child, d));
            if results.len() >= k { return results; }
        }

        // 3. Push sub Nodes and Children onto the stack if inside the radius
        //    We know that the top of the stack is either None or Some(Node(...))
        if let Some((KnnType::Node(node), d)) = work_stack.pop() {
            if d > r { return results; }

            let children = node.children()
                .into_iter()
                .map(|child| (KnnType::Child(child), cmp.dist_datum(child)));
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
            return results;
        }
    }
}