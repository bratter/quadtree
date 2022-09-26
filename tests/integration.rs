use quadtree::*;
use quadtree::geom::*;
use quadtree::quadtrees::point::*;

#[test]
fn create_empty_retrieve_inside_bounds_returns_empty_vec() {
    let origin = Point::<Euclidean>::new(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let qt = PointQuadTree::new_def(bounds);
    let pt1 = Point::new(0.1, 0.1);

    assert_eq!(qt.size(), 0);
    assert_eq!(qt.retrieve(&pt1).len(), 0);
}

#[test]
fn create_and_retrieve_single_point_returns_vec_of_point() {
    let origin = Point::<Euclidean>::new(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = PointQuadTree::new_def(bounds);
    let pt1 = Point::new(0.1, 0.1);

    qt.insert(pt1.clone());

    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt1), vec![&pt1]);
}

#[test]
fn insert_out_of_bounds_doesnt_add_and_retrieve_out_of_bounds_yields_none() {
    let origin = Point::<Euclidean>::new(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = PointQuadTree::new_def(bounds);
    let pt1 = Point::new(0.1, 0.1);
    let pt2 = Point::new(2.0, 2.0);

    qt.insert(pt1.clone());
    qt.insert(pt2.clone());

    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt2).len(), 0);
}

#[test]
fn iterator_runs_preorder() {
    let origin = Point::<Euclidean>::new(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = PointQuadTree::new_def(bounds);
    let pt1 = Point::new(0.1, 0.1);
    let pt2 = Point::new(0.2, 0.2);
    let pt3 = Point::new(0.1, 0.8);

    // Inserting in a random order
    qt.insert(pt3.clone());
    qt.insert(pt1.clone());
    qt.insert(pt2.clone());
    qt.insert(pt1.clone());
    qt.insert(pt2.clone());
    qt.insert(pt1.clone());

    assert_eq!(qt.size(), 6);

    // This won't compile because we don't implement an owned iteration
    // for pt in qt {}

    //TODO: Uncomment when iterators are fixed
    // Test right length and in preorder
    // let vec = qt.iter().collect::<Vec<&Point<Euclidean>>>();
    // assert_eq!(vec.len(), 6);
    // assert_eq!(vec[0], &pt1);
    // assert_eq!(vec[1], &pt1);
    // assert_eq!(vec[2], &pt1);
    // assert_eq!(vec[3], &pt2);
    // assert_eq!(vec[4], &pt2);
    // assert_eq!(vec[5], &pt3);

    // We can re-iterate as its non-consumptive
    // let vec = qt.iter().collect::<Vec<&Point<Euclidean>>>();
    // assert_eq!(vec.len(), 6);
}