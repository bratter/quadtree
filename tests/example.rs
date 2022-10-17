// This file contains documented examples of how to set up and run examples
// on the various quadtree implementations

use approx::assert_abs_diff_eq;
use geo::{Rect, Point, coord};
use quadtree::*;

// Some helper functions to make geo shapes
fn p(x: f64, y: f64) -> Point {
    Point::new(x, y)
}

fn r(x1: f64, y1: f64, x2: f64, y2: f64) -> Rect {
    Rect::new(coord! {x: x1, y: y1}, coord! {x: x2, y: y2})
}

#[test]
fn euclidean_point_example() {
    // Set up an arbitrary datum type to work in the quadtree by implementing
    // PointDatum. In many cases this will be a trivial passthrough to an
    // underlying geo type, but could be more complex if required.
    // Only need Clone and PartialEq for testing ergonomics, its not a
    // requirement of the qt
    #[derive(Debug, Clone, PartialEq)]
    struct MyDatum {
        id: usize,
        meta: (), // Some important data in practice
        location: Point,
    }
    impl PointDatum for MyDatum {
        fn point(&self) -> Point<f64> {
            self.location
        }
    }

    // Just a simple helper function to make it easier to construct MyDatum
    fn datum(id: usize, x: f64, y: f64) -> MyDatum {
        MyDatum { id, meta: (), location: p(x, y) }
    }

    // Construct a point qt in some bounds
    let bounds = r(0.0, 0.0, 32.0, 32.0);
    let mut qt = PointQuadTree::from_bounds(bounds);

    let data = vec![
        datum(0, 0.0, 0.0),
        datum(1, 3.0, 5.0),
        datum(2, 7.0, 2.0),
        datum(3, 2.0, 7.0),
        datum(4, 1.0, 1.0),
        datum(5, 11.0, 13.0),
        datum(6, 9.0, 8.0),
        datum(7, 5.0, 5.0),
        datum(8, 1.0, 2.0),
    ];

    // Here we loop data into the qt, these should all succeed with an empty ok
    // Borrow here so we can refer to the members later.
    for d in &data {
        // This will panic if not Ok
        qt.insert(d.clone()).unwrap();
    }
    assert_eq!(qt.size(), 9);

    // Inserting a datum that is outside the bounds produces an error
    // and doesn't increment the count
    let res = qt.insert(datum(999, -1.0, -1.0));
    assert_eq!(res, Err(Error::OutOfBounds));
    assert_eq!(qt.size(), 9);

    // Print the resulting quadtree
    // Note that Datum does not have to implement std::fmt::Display for this to print
    let str = format!("{}", qt);
    assert!(str.contains("Point Quadtree"));

    // Retrieve from the quadtree based on a passed datum
    // This is used for collision detection, etc. and will work best
    // in evenly populated quadtrees
    // TODO: Return as an iterator and test
    // qt.retrieve(datum)

    // Can filter/map etc. using the iterator to, for example filter by some metadata
    // TODO: Do this

    // The standard iterator walks the quadtree in preorder, with each node
    // being walked counter-clockwise (on a Euclidean plane) from the lowest
    // x/y values. Results with an individual nodes are non-deterministic.
    let res: Vec<_> = qt.into_iter().collect();
    let cmp = vec![&data[8], &data[4], &data[0], &data[2], &data[7], &data[3], &data[1], &data[6], &data[5]];
    assert_eq!(res, cmp);
    
    // Up to this point, we have only needed PointDatum as insertion and
    // retrieval don't require access to the entire geometry, but now we do
    // to use find and knn.
    //
    // To return a Geometry, we have to wrap the reified type in the right
    // Geometry enum to get proper polymorphism.
    impl Datum for MyDatum {
        fn geometry(&self) -> Geometry<f64> {
            Geometry::Point(self.location)
        }
    }

    // For the comparison item we can use anything that implements Datum, does
    // not have to be the same as the Datum used in the QuadTree. Datum comes
    // pre-implemented for standard geo-types, so Point, Line, etc. can be
    // dropped in directly.
    //
    // Note that we have to wrap the test item, whatever it is, in eucl to give
    // it polymorphic access to the correct distance formulas. It is possible
    // to implement Distance<T> on the type, but this only works if you will
    // definitely only use one geometry.
    let cmp = eucl(p(0.0, 0.0));
    let (res, d) = qt.find(&cmp).unwrap();
    assert_eq!(res, &data[0]);
    assert_abs_diff_eq!(d, 0.0);

    // We can of course drop a datum in directly
    let cmp = eucl(data[1].clone());
    let (res, d) = qt.find(&cmp).unwrap();
    assert_eq!(res, &data[1]);
    assert_abs_diff_eq!(d, 0.0);

    // Using the eucl() wrapper function (or Euclidean::new) does distance
    // comparisons using Euclidean math
    let cmp = Euclidean::new(p(12.0, 14.0));
    let (res, d) = qt.find(&cmp).unwrap();
    assert_eq!(res, &data[5]);
    assert_abs_diff_eq!(d, 2.0_f64.sqrt());

    // Find_r only returns if closer than the passed radius, which will be
    // Err(Error::NoneInRadius) in this case
    let res = qt.find_r(&cmp, 0.5);
    assert_eq!(res, Err(Error::NoneInRadius));

    // Knn/knn_r work the same way as find/find_r, but returns a vector of
    // results
    // TODO: Is there some way to work better with references here
    let cmp = eucl(p(0.0, 0.0));
    let res: Vec<MyDatum> = qt.knn(&cmp, 3).unwrap().iter().map(|(d, _)| (*d).clone()).collect();
    assert_eq!(res, vec![data[0].clone(), data[4].clone(), data[8].clone()]);

    // Returns an empty vec if nothing is found in the radius
    let cmp = eucl(p(14.0, 14.0));
    let res = qt.knn_r(&cmp, 3, 0.5).unwrap();
    assert_eq!(res, vec![]);
}

#[test]
fn spherical_point_example() {
    // TODO: Build out this example

    // CRITICAL NOTE: While it is irrelevant for insert/retrieve, all search
    // calcs both take and return all corrdinates in radians, not degrees!!
    // All supported geo-types provide a to_radians and a to_radians_in_place
    // method to easily convert standard degree representations to radians

    // Setup for Spherical works exactly the same way
    // Here we will just drop Points in as the Datum, noting that Point
    // already implements Datum and PointDatum
    let bounds = r(0.0, 0.0, 90.0, 90.0).to_radians();
    let mut qt = PointQuadTree::new(bounds, 4, 2);

    let data = vec![
        p(0.0, 0.0).to_radians(),
        p(0.0, 50.0).to_radians(),
        p(45.0, 45.0).to_radians(),
        p(39.0, 42.0).to_radians(),
        p(21.0, 32.0).to_radians(),
    ];

    for d in &data {
        qt.insert(d.clone()).unwrap();
    }

    // Here we wrap our comparisons in Spherical instead of Euclidean
    // Moving around y == 0 (the "equator") is simply the difference
    // in radians
    let cmp = sphere(p(22.5, 0.0).to_radians());
    let (res, d) = qt.find(&cmp).unwrap();
    assert_eq!(res, &data[0]);
    assert_abs_diff_eq!(d, std::f64::consts::FRAC_PI_8);

    // ...and same with anything on the same lng line
    // Note the radian conversions in both
    let cmp = sphere(p(45.0, 67.5).to_radians());
    let (res, d) = qt.find(&cmp).unwrap();
    assert_eq!(res, &data[2]);
    assert_abs_diff_eq!(d, std::f64::consts::FRAC_PI_8);

    println!("{}", qt);
}