use crate::Point;

// TODO: Plan for abstraction
//       1. Put the basic distance calcs as pure functions in geom::euclidean and geom::spherical modules
//       2. QuadTree needs to be turned into a trait (eventually + with multiple implementations)
//       3. Put Distance, etc. in a geom module
//       4. Custom type needs to implement Distance for itself covering both point and line segments. Should they have their own types? 
//       5. Do we need some sort of guard unit type for the coordinate system
//       6. Bounds needs to be beefed up to produce valid line segments
//       7. Provide data wrappers to enable easy use of the traits

// Bounds and the Point/Line constructs need to know what geom they are working in
// This is the tricky thing - need to block other coord systems without making it clunky
// Quadtree and Node probably shouldn't care - they don't use anything directly
// Probably use a unit struct in the distance trait
// Perhaps some way in a generic to make it such that both sides have to implement a Geom trait (or just the distance one) AND the generic has to be the same
// Perhaps dynamic dispatch onto the right implementation?

// Type alias for a point - this is easier in these calcs than requiring a Point
type PT = (f64, f64);

// TODO: This is a scratch pad function for Euclidean point to line-segment distance
// This is just to get the math working, traits, generics, interface, etc. still tbd
// Base version from this SO answer: https://stackoverflow.com/a/6853926
pub fn line_dist(p: PT, p1: PT, p2: PT) -> f64 {
    let (x, y) = p;
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let (a, b, c, d) = (x - x1, y - y1, x2 - x1, y2 - y1);

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;
    // Wrap in an `if` to account for a zero line length
    // Just has to be <0 to work so we pick distance to p1
    let param = if len_sq == 0.0 { -1.0 } else { dot / len_sq };

    let (xx, yy) = if param < 0.0 {
        // Closest to p1, so reduces to pt-pt
        (x1, y1)
    } else if param > 1.0 {
        // Closest to p2, so pt-pt again
        (x2, y2)
    } else {
        // Here we project onto the segment
        (x1 + param * c, y1 + param * d)
    };

    // Now calculate the distance
    // TODO: Should we return squared here?
    //       In general all geom functions should return squared then get unsquared from free from the trait?
    ((x - xx).powi(2) + (y - yy).powi(2)).sqrt()
}

pub fn point_dist(p: PT, p1: PT) -> f64 {
    let (x, y) = p;
    let (x1, y1) = p1;

    ((x - x1).powi(2) + (y - y1).powi(2)).sqrt()
}

struct Pt(f64, f64);
impl Point for Pt {
    fn coords(&self) -> (f64, f64) {
        (self.0, self.1)
    }
}

struct Ln((f64, f64), (f64, f64));

enum Geo {
    Euclid,
    Sphere,
}

// TODO: Note the generic traits allowing Distance to be implemented on different types
//       To use the trait as a constraint, do need to specify the A though
//       Can make the implementations generic if the implementing type is generic
trait Distance<A> {
    fn distance(&self, cmp: &A) -> f64;
}

impl Distance<Pt> for Pt {
    fn distance(&self, cmp: &Pt) -> f64 {
        point_dist(self.coords(), cmp.coords())
    }
}

impl Distance<Ln> for Pt {
    fn distance(&self, cmp: &Ln) -> f64 {
        line_dist(self.coords(), cmp.0, cmp.1)
    }
}

// Note the inverse implementation to the above which just delegates
impl Distance<Pt> for Ln {
    fn distance(&self, cmp: &Pt) -> f64 {
        cmp.distance(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let p1 = Pt(0.0, 0.0);
        let p2 = Pt(3.0, 4.0);
        let l = Ln((0.0, 0.0), (0.0, 6.0));

        assert_eq!(p1.distance(&p2), 5.0);
        assert_eq!(p2.distance(&l), 3.0);
        assert_eq!(l.distance(&p1), 0.0);
    }
}