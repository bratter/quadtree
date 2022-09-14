pub mod euclidean;
pub mod spherical;

mod trait_test;

pub trait Distance<A> {
    fn distance(&self, cmp: &A) -> f64;

    // TODO: A rel distance method also - that is guaranteed to cmp fine when compared to another of the same type
}

pub trait Point<S: System> {
    fn coords(&self) -> (f64, f64);
}

// impl Point<Euclid> for (f64, f64) {
//     // TODO: Return a borrow or do a clone?
//     fn coords(&self) -> (f64, f64) {
//         *self
//     }
// }


pub trait System {
    // fn system() -> Sys;
}



struct Euclid;
struct Sphere;

impl System for Euclid {
    // fn system() -> Sys {
    //     Sys::Abc
    // }
}

impl System for Sphere {}

mod euclid {
    pub struct Point(pub f64, pub f64);
    impl super::Point<super::Euclid> for Point {
        fn coords(&self) -> (f64, f64) {
            (self.0, self.1)
        }
    }
}

struct EuclidPoint(f64, f64);

struct SpherePoint(f64, f64);

impl Point<Euclid> for EuclidPoint {
    fn coords(&self) -> (f64, f64) {
        // This seems to work if in separate modules to have type overloading
        euclid::Point(1.0, 1.0);
        (self.0, self.1)
    }
}

impl Point<Sphere> for SpherePoint {
    fn coords(&self) -> (f64, f64) {
        (self.0, self.1)    
    }
}

// TODO: Not super ergonomic, but it will work fine, can assume raw tuples are euclidean?
impl Distance<EuclidPoint> for EuclidPoint {
    fn distance(&self, cmp: &EuclidPoint) -> f64 {
        euclidean::dist_to_pt(self.coords(), cmp.coords())
    }
}


// struct Bounds<T: Point> {
//     top_left: T,
//     bottom_right: T,
// }

// impl<T: Point> Bounds<T> {
//     fn new(p1: T, p2: T) -> Self {
//         // TODO: This can be any opposite corners, need teo pull out the smallest for top left
//         Bounds { top_left: p1, bottom_right: p2, }
//     }

//     fn from_points() {
//         todo!("From points")
//     }

//     fn from_origin() {
//         todo!("from origin and w/h")
//     }
// }


fn play() {
    // (1.0, 1.0).coords()
    // TODO: This is not great, want to stick Euclid in somewhere and have anything Euclid work
    // let bounds = Bounds::new(EuclidPoint(1.0, 1.0), EuclidPoint(2.0, 2.0));


}
