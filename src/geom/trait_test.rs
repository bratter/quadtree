use super::euclidean::*;

// A marker trait
trait Euclid {}

trait Point {
    fn coords(&self) -> (f64, f64);
}

struct Pt1(f64, f64);
impl Euclid for Pt1 {}
impl Point for Pt1 {
    fn coords(&self) -> (f64, f64) {
        (self.0, self.1)
    }
}

struct Pt2(f64, f64);
impl Point for Pt2 {
    fn coords(&self) -> (f64, f64) {
        (self.0, self.1)
    }
}

struct Pt3 {
    x: f64,
    y: f64,
}
impl Euclid for Pt3 {}
impl Point for Pt3 {
    fn coords(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

trait Dist<T> {
    fn dist(&self, cmp: &T) -> f64;
}

impl<T: Point + Euclid> Dist<T> for Pt1 {
    fn dist(&self, cmp: &T) -> f64 {
        dist_sq_to_pt(self.coords(), cmp.coords())
    }
}

fn test() {
    let p1 = Pt1(1.0, 1.0);
    let p2 = Pt2(2.0, 2.0);
    let p3 = Pt3 { x: 3.0, y: 3.0 };

    // Doesn't work because of the marker trait
    // p1.dist(&p2);
    p1.dist(&p3);
}
