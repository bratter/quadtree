use approx::assert_abs_diff_eq;
use geo::{coord, EuclideanDistance, Line, Point, Rect};
use quadtree::spherical::math::dist_pt_pt;
use quadtree::*;

#[test]
fn create_empty_retrieve_inside_bounds_returns_empty_vec() {
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord! {x: 1.0, y: 1.0});
    let qt = PointQuadTree::from_bounds(bounds);
    let pt1 = Point::new(0.1, 0.1);

    assert_eq!(qt.size(), 0);
    assert_eq!(qt.retrieve(&pt1).count(), 0);
}

#[test]
fn create_and_retrieve_single_point_returns_vec_of_point() {
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord! {x: 1.0, y: 1.0});
    let mut qt = PointQuadTree::from_bounds(bounds);
    let pt1 = Point::new(0.1, 0.1);

    qt.insert(pt1.clone()).unwrap();

    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt1).collect::<Vec<_>>(), vec![&pt1]);
}

#[test]
fn insert_out_of_bounds_doesnt_add_and_retrieve_out_of_bounds_yields_error() {
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord! {x: 1.0, y: 1.0});
    let mut qt = PointQuadTree::from_bounds(bounds);
    let pt1 = Point::new(0.1, 0.1);
    let pt2 = Point::new(2.0, 2.0);

    qt.insert(pt1.clone()).unwrap();
    let res = qt.insert(pt2.clone());

    assert_eq!(res, Err(Error::OutOfBounds));
    assert_eq!(qt.size(), 1);
    assert_eq!(qt.retrieve(&pt2).count(), 0);
}

#[test]
fn iterator_runs_preorder() {
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord! {x: 1.0, y: 1.0});
    let mut qt = PointQuadTree::from_bounds(bounds);
    let pt1 = Point::new(0.1, 0.1);
    let pt2 = Point::new(0.2, 0.2);
    let pt3 = Point::new(0.1, 0.8);

    // Inserting in a random order
    qt.insert(pt3.clone()).unwrap();
    qt.insert(pt1.clone()).unwrap();
    qt.insert(pt2.clone()).unwrap();
    qt.insert(pt1.clone()).unwrap();
    qt.insert(pt2.clone()).unwrap();
    qt.insert(pt1.clone()).unwrap();

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
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord!(x: 1.0, y: 1.0));
    let mut qt = PointQuadTree::new(bounds, 2, 2);

    let p1 = Point::new(0.4, 0.2);
    let p2 = Point::new(0.2, 0.4);
    let p3 = Point::new(0.1, 0.1);
    let p4 = Point::new(0.8, 0.8);

    qt.insert(p1.clone()).unwrap();
    qt.insert(p2.clone()).unwrap();
    qt.insert(p3.clone()).unwrap();
    qt.insert(p4.clone()).unwrap();
    qt.insert(p4.clone()).unwrap();

    let cmp = eucl(Point::new(0.4, 0.39));
    assert_eq!(qt.find(&cmp).unwrap(), (&p1, 0.19));
}

#[test]
fn find_returns_closest_in_spherical_for_point_qt() {
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord!(x: 1.0, y: 1.0));
    let mut qt = PointQuadTree::new(bounds, 2, 2);

    let p1 = Point::new(0.4, 0.2);
    let p2 = Point::new(0.2, 0.4);
    let p3 = Point::new(0.1, 0.1);
    let p4 = Point::new(0.8, 0.8);

    qt.insert(p1.clone()).unwrap();
    qt.insert(p2.clone()).unwrap();
    qt.insert(p3.clone()).unwrap();
    qt.insert(p4.clone()).unwrap();
    qt.insert(p4.clone()).unwrap();

    // Make this slightly closer to the x axis
    // Then in spherical the distance is closer to the other point
    let cmp = sphere(Point::new(0.4, 0.39));
    let (p, d) = qt.find(&cmp).unwrap();
    assert_eq!(p, &p2);
    assert_abs_diff_eq!(d, dist_pt_pt(&cmp, &p2));
}

/// Helper function to construct a line segment.
fn line(x1: f64, y1: f64, x2: f64, y2: f64) -> Line<f64> {
    Line::new(coord! {x: x1, y: y1}, coord! {x:x2, y: y2})
}

#[test]
fn find_returns_closest_in_euclidean_for_segments_in_bounds_qt() {
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord! {x: 1.0, y: 1.0});
    let mut qt = BoundsQuadTree::new(bounds, 2, 2);

    // Will be stuck in TL at the second level
    let d1 = line(0.3, 0.0, 0.0, 0.4);
    let d2 = line(0.0, 0.0, 0.0, 0.4);
    let d3 = line(0.0, 0.0, 0.3, 0.0);
    // Will be stuck in root
    let d4 = line(0.6, 0.0, 0.0, 0.8);
    // In the TR
    let d5 = line(0.9, 0.8, 0.9, 0.9);

    qt.insert(d1.clone()).unwrap();
    qt.insert(d2.clone()).unwrap();
    qt.insert(d3.clone()).unwrap();
    qt.insert(d4.clone()).unwrap();
    qt.insert(d5.clone()).unwrap();

    // Closer to the y-axis
    let cmp = Euclidean::new(Point::new(0.05, 0.1));
    assert_eq!(qt.find(&cmp).unwrap(), (&d2, 0.05));

    // Closer to the diagonal
    let cmp = Euclidean::new(Point::new(0.1, 0.2));
    let cmp_dist = Point::new(0.1, 0.2).euclidean_distance(&line(0.3, 0.0, 0.0, 0.4));
    let (datum, dist) = qt.find(&cmp).unwrap();
    assert_abs_diff_eq!(dist, cmp_dist);
    assert_eq!(datum, &d1);

    // Closer to the random line
    let cmp = Euclidean::new(Point::new(0.8, 0.8));
    let (datum, dist) = qt.find(&cmp).unwrap();
    assert_abs_diff_eq!(dist, 0.1);
    assert_eq!(datum, &d5);
}

#[test]
fn find_returns_closest_in_speherical_in_bounds_qt() {
    // Work with point cmp here which already implements the right Dist
    let origin = Point::new(-1.0, -1.0);
    let bounds = Rect::new(origin.0, coord! {x: 1.0, y: 1.0});
    let mut qt = BoundsQuadTree::new(bounds, 2, 2);

    // Both in the TL
    let d1 = Line::new(coord!(x: -0.4, y: 0.0), coord!(x: -0.4, y: -0.4));
    let d2 = Line::new(coord!(x: 0.0, y: -0.4), coord!(x: -0.4, y: -0.4));

    qt.insert(d1.clone()).unwrap();
    qt.insert(d2.clone()).unwrap();

    // Should be closer to the vertical line due to curvature
    let cmp = sphere(Point::new(-0.2, -0.2));
    let dist_cmp = dist_pt_pt(&Point::new(-0.2, -0.2), &Point::new(-0.4, -0.2));
    let (datum, dist) = qt.find(&cmp).unwrap();

    assert_abs_diff_eq!(dist, dist_cmp);
    assert_eq!(datum, &d1);
}

#[test]
fn knn_on_point_qt_returns_k_nodes_in_dist_order() {
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord! {x: 8.0, y: 8.0});
    let mut qt = PointQuadTree::new(bounds, 2, 2);

    let p1 = Point::new(2.0, 2.0);
    let p2 = Point::new(3.0, 3.0);
    let p3 = Point::new(6.0, 6.0);

    qt.insert(p1.clone()).unwrap();
    qt.insert(p1.clone()).unwrap();
    qt.insert(p2.clone()).unwrap();
    qt.insert(p3.clone()).unwrap();

    let cmp = Euclidean::new(Point::new(6.0, 5.0));
    let res = qt.knn_r(&cmp, 3, f64::INFINITY).unwrap();

    assert_eq!(res.len(), 3);
    assert_eq!(res[0].0.x_y(), p3.x_y());
    assert_eq!(res[0].1, 1.0);
    assert_eq!(res[1].0.x_y(), p2.x_y());
    assert_abs_diff_eq!(res[1].1, 13.0f64.sqrt());
    assert_eq!(res[2].0.x_y(), p1.x_y());
    assert_abs_diff_eq!(res[2].1, 5.0);
}

#[test]
fn knn_on_point_qt_stops_at_r() {
    let origin = Point::new(0.0, 0.0);
    let bounds = Rect::new(origin.0, coord! {x: 8.0, y: 8.0});
    let mut qt = PointQuadTree::new(bounds, 2, 2);

    let p1 = Point::new(2.0, 2.0);
    let p2 = Point::new(3.0, 3.0);
    let p3 = Point::new(6.0, 6.0);

    qt.insert(p1.clone()).unwrap();
    qt.insert(p1.clone()).unwrap();
    qt.insert(p2.clone()).unwrap();
    qt.insert(p3.clone()).unwrap();

    let cmp = Euclidean::new(Point::new(6.0, 5.0));
    let res = qt.knn_r(&cmp, 3, 4.0).unwrap();

    assert_eq!(res.len(), 2);
    assert_eq!(res[0].0.x_y(), p3.x_y());
    assert_eq!(res[0].1, 1.0);
    assert_eq!(res[1].0.x_y(), p2.x_y());
    assert_abs_diff_eq!(res[1].1, 13.0f64.sqrt());
}
