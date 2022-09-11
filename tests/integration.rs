use quadtree::*;

// TODO: Which test here vs, unit tests?

#[test]
fn create_empty_retrieve_inside_bounds_returns_empty_vec() {
    let qt = QuadTree::new(0.0, 0.0, 1.0, 1.0);
    let pt1 = Point::new(0.1, 0.1, 42);

    assert_eq!(qt.size(), 0);
    assert_eq!(qt.retrieve(&pt1), None);
}

#[test]
fn create_and_retrieve_single_point_returns_vec_of_point() {
    let mut qt = QuadTree::new(0.0, 0.0, 1.0, 1.0);
    let pt1 = Point::new(0.1, 0.1, 42);

    qt.insert(pt1.clone());

    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt1).unwrap(), &vec![pt1]);
}

#[test]
fn insert_out_of_bounds_doesnt_add_and_retrieve_out_of_bounds_yields_none() {
    let mut qt = QuadTree::new(0.0, 0.0, 1.0, 1.0);
    let pt1 = Point::new(0.1, 0.1, 42);
    let pt2 = Point::new(2.0, 2.0, 2);

    qt.insert(pt1.clone());
    qt.insert(pt2.clone());

    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt2), None);
}