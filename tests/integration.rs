use quadtree::*;

#[derive(Debug, Clone, PartialEq)]
struct Pt(f64, f64);

impl Point for Pt {
    fn coords(&self) -> (f64, f64) {
        (self.0, self.1)
    }
}

#[test]
fn create_empty_retrieve_inside_bounds_returns_empty_vec() {
    let bounds = Bounds::new(0.0, 0.0, 1.0, 1.0);
    let qt = QuadTree::new_def(bounds);
    let pt1 = Pt(0.1, 0.1);

    assert_eq!(qt.size(), 0);
    assert_eq!(qt.retrieve(&pt1), None);
}

#[test]
fn create_and_retrieve_single_point_returns_vec_of_point() {
    let bounds = Bounds::new(0.0, 0.0, 1.0, 1.0);
    let mut qt = QuadTree::new_def(bounds);
    let pt1 = Pt(0.1, 0.1);

    qt.insert(pt1.clone());

    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt1).unwrap(), &vec![pt1]);
}

#[test]
fn insert_out_of_bounds_doesnt_add_and_retrieve_out_of_bounds_yields_none() {
    let bounds = Bounds::new(0.0, 0.0, 1.0, 1.0);
    let mut qt = QuadTree::new_def(bounds);
    let pt1 = Pt(0.1, 0.1);
    let pt2 = Pt(2.0, 2.0);

    qt.insert(pt1.clone());
    qt.insert(pt2.clone());

    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt2), None);
}

#[test]
fn iterator_runs_preorder() {
    let bounds = Bounds::new(0.0, 0.0, 1.0, 1.0);
    let mut qt = QuadTree::new_def(bounds);
    let pt1 = Pt(0.1, 0.1);
    let pt2 = Pt(0.2, 0.2);
    let pt3 = Pt(0.1, 0.8);

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

    // Test right length and in preorder
    let vec = qt.iter().collect::<Vec<&Pt>>();
    assert_eq!(vec.len(), 6);
    assert_eq!(vec[0], &pt1);
    assert_eq!(vec[1], &pt1);
    assert_eq!(vec[2], &pt1);
    assert_eq!(vec[3], &pt2);
    assert_eq!(vec[4], &pt2);
    assert_eq!(vec[5], &pt3);

    // We can re-iterate as its non-consumptive
    let vec = qt.iter().collect::<Vec<&Pt>>();
    assert_eq!(vec.len(), 6);
}