use quadtree::*;
use quadtree::geom::*;

const EPSILON: f64 = 1e-6;

#[test]
fn create_empty_retrieve_inside_bounds_returns_empty_vec() {
    let origin = Point::<Euclidean>::new(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let qt = PointQuadTree::default(bounds);
    let pt1 = Point::new(0.1, 0.1);

    assert_eq!(qt.size(), 0);
    assert_eq!(qt.retrieve(&pt1).len(), 0);
}

#[test]
fn create_and_retrieve_single_point_returns_vec_of_point() {
    let origin = Point::<Euclidean>::new(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = PointQuadTree::default(bounds);
    let pt1 = Point::new(0.1, 0.1);

    qt.insert(pt1.clone());

    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt1), vec![&pt1]);
}

#[test]
fn insert_out_of_bounds_doesnt_add_and_retrieve_out_of_bounds_yields_none() {
    let origin = Point::<Euclidean>::new(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = PointQuadTree::default(bounds);
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
    let mut qt = PointQuadTree::default(bounds);
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

    // Test right length and in preorder
    let vec = qt.into_iter().collect::<Vec<_>>();
    assert_eq!(vec.len(), 6);
    assert_eq!(vec[0], &pt1);
    assert_eq!(vec[1], &pt1);
    assert_eq!(vec[2], &pt1);
    assert_eq!(vec[3], &pt2);
    assert_eq!(vec[4], &pt2);
    assert_eq!(vec[5], &pt3);

    // We can re-iterate as its non-consumptive
    let vec = qt.into_iter().collect::<Vec<_>>();
    assert_eq!(vec.len(), 6);
}

#[test]
fn find_returns_closest_in_eucildean_for_point_qt() {
    let origin = Euclidean::point(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = PointQuadTree::new(bounds, 2, 2);

    let p1 = Euclidean::point(0.4, 0.2);
    let p2 = Euclidean::point(0.2, 0.4);
    let p3 = Euclidean::point(0.1, 0.1);
    let p4 = Euclidean::point(0.8, 0.8);

    qt.insert(p1.clone());
    qt.insert(p2.clone());
    qt.insert(p3.clone());
    qt.insert(p4.clone());
    qt.insert(p4.clone());

    // Make this slightly closer to the x axis
    let cmp = Euclidean::point(0.4, 0.39);
    assert_eq!(qt.find(&cmp).unwrap(), (&p1, 0.19));
}

#[test]
fn find_returns_closest_in_spherical_for_point_qt() {
    let origin = Spherical::point(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = PointQuadTree::new(bounds, 2, 2);

    let p1 = Spherical::point(0.4, 0.2);
    let p2 = Spherical::point(0.2, 0.4);
    let p3 = Spherical::point(0.1, 0.1);
    let p4 = Spherical::point(0.8, 0.8);

    qt.insert(p1.clone());
    qt.insert(p2.clone());
    qt.insert(p3.clone());
    qt.insert(p4.clone());
    qt.insert(p4.clone());

    // Make this slightly closer to the x axis
    // Then in spherical the distance is closer to the other point
    let cmp = Spherical::point(0.4, 0.39);
    let (p, d) = qt.find(&cmp).unwrap();
    assert_eq!(p, &p2);
    assert!((d - cmp.dist(&p2)).abs() < EPSILON);
}

// Make a segment into a bounds datum
#[derive(Debug, Clone, PartialEq)]
struct SegE(Segment<Euclidean>);
impl SegE {
    fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        SegE(Euclidean::segment(Euclidean::point(x1, y1), Euclidean::point(x2, y2)))
    }
}

impl Datum<Euclidean> for SegE {
    fn point(&self) -> Point<Euclidean> {
        self.0.a()
    }
}

impl BoundsDatum<Euclidean> for SegE {
    fn bounds(&self) -> Bounds<Euclidean> {
        Bounds::from_points(self.0.a(), self.0.b())
    }
}

impl Distance<SegE> for Point<Euclidean> {
    fn dist(&self, cmp: &SegE) -> f64 {
        self.dist(&cmp.0)
    }
}

#[test]
fn find_returns_closest_in_euclidean_for_segments_in_bounds_qt() {
    let origin = Euclidean::point(0.0, 0.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = BoundsQuadTree::new(bounds, 2, 2);

    // Will be stuck in TL at the second level
    let d1 = SegE::new(0.3, 0.0, 0.0, 0.4);
    let d2 = SegE::new(0.0, 0.0, 0.0, 0.4);
    let d3 = SegE::new(0.0, 0.0, 0.3, 0.0);
    // Will be stuck in root
    let d4 = SegE::new(0.6, 0.0, 0.0, 0.8);
    // In the TR
    let d5 = SegE::new(0.9, 0.8, 0.9, 0.9);

    qt.insert(d1.clone());
    qt.insert(d2.clone());
    qt.insert(d3.clone());
    qt.insert(d4.clone());
    qt.insert(d5.clone());

    // Closer to the y-axis
    let cmp = Euclidean::point(0.05, 0.1);
    assert_eq!(qt.find(&cmp).unwrap(), (&d2, 0.05));

    // Closer to the diagonal
    let cmp = Euclidean::point(0.1, 0.2);
    let cmp_dist = euclidean::math::dist_pt_line((0.1, 0.2), (0.3, 0.0), (0.0, 0.4));
    let (datum, dist) = qt.find(&cmp).unwrap();
    assert!((dist - cmp_dist).abs() < EPSILON);
    assert_eq!(datum, &d1);

    
    // Closer to the random line
    let cmp = Euclidean::point(0.8, 0.8);
    let (datum, dist) = qt.find(&cmp).unwrap();
    assert!((dist - 0.1).abs() < EPSILON);
    assert_eq!(datum, &d5);
}

#[test]
fn find_returns_closest_in_speherical_in_bounds_qt() {
    // Work with point cmp here which already implements the right Dist
    let origin = Spherical::point(-1.0, -1.0);
    let bounds = Bounds::new(origin, 1.0, 1.0);
    let mut qt = BoundsQuadTree::new(bounds, 2, 2);

    // Both in the TL
    let d1 = Segment::new(Spherical::point(-0.4, 0.0), Spherical::point(-0.4, -0.4));
    let d2 = Segment::new(Spherical::point(0.0, -0.4), Spherical::point(-0.4, -0.4));

    qt.insert(d1.clone());
    qt.insert(d2.clone());

    // Should be closer to the vertical line due to curvature
    let cmp = Spherical::point(-0.2, -0.2);
    let dist_cmp = spherical::math::dist_pt_pt((-0.2, -0.2), (-0.4, -0.2));
    let (datum, dist) = qt.find(&cmp).unwrap();
    assert!((dist - dist_cmp).abs() < EPSILON);
    assert_eq!(datum, &d1);
}

#[test]
fn knn_on_point_qt_returns_k_nodes_in_dist_order() {
    let origin = Euclidean::point(0.0, 0.0);
    let bounds = Bounds::new(origin, 8.0, 8.0);
    let mut qt = PointQuadTree::new(bounds, 2, 2);

    let p1 = Euclidean::point(2.0, 2.0);
    let p2 = Euclidean::point(3.0, 3.0);
    let p3 = Euclidean::point(6.0, 6.0);

    qt.insert(p1.clone());
    qt.insert(p1.clone());
    qt.insert(p2.clone());
    qt.insert(p3.clone());

    let cmp = Euclidean::point(6.0, 5.0);
    let res = qt.knn(&cmp, 3, f64::INFINITY);

    assert_eq!(res.len(), 3);
    assert_eq!(res[0].0.as_tuple(), p3.as_tuple());
    assert_eq!(res[0].1, 1.0);
    assert_eq!(res[1].0.as_tuple(), p2.as_tuple());
    assert!((res[1].1 - 13.0f64.sqrt()).abs() < EPSILON);
    assert_eq!(res[2].0.as_tuple(), p1.as_tuple());
    assert!((res[2].1 - 5.0).abs() < EPSILON);
}

#[test]
fn knn_on_point_qt_stops_at_r() {
    let origin = Euclidean::point(0.0, 0.0);
    let bounds = Bounds::new(origin, 8.0, 8.0);
    let mut qt = PointQuadTree::new(bounds, 2, 2);

    let p1 = Euclidean::point(2.0, 2.0);
    let p2 = Euclidean::point(3.0, 3.0);
    let p3 = Euclidean::point(6.0, 6.0);

    qt.insert(p1.clone());
    qt.insert(p1.clone());
    qt.insert(p2.clone());
    qt.insert(p3.clone());

    let cmp = Euclidean::point(6.0, 5.0);
    let res = qt.knn(&cmp, 3, 4.0);
    
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].0.as_tuple(), p3.as_tuple());
    assert_eq!(res[0].1, 1.0);
    assert_eq!(res[1].0.as_tuple(), p2.as_tuple());
    assert!((res[1].1 - 13.0f64.sqrt()).abs() < EPSILON);
}