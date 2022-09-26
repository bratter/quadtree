use super::*;

pub trait Node <T: Datum<Geom>, Geom: System<Geometry = Geom>>
where Self: Sized {
    fn new(bounds: Bounds<Geom>, depth: u8, max_depth: u8, max_children: usize) -> Self;

    fn bounds(&self) -> &Bounds<Geom>;

    fn depth(&self) -> u8;

    fn max_depth(&self) -> u8;

    fn max_children(&self) -> usize;

    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>);

    fn insert(&mut self, datum: T);

    fn retrieve(&self, datum: &T) -> Vec<&T>;

    fn find_sub_node(&self, datum: &T) -> SubNode {
        let (x, y) = datum.point().as_tuple();
        let b = &self.bounds();
        let left = x <= b.x_min() + b.width() / 2.0;
        let top = y <= b.y_min() + b.height() / 2.0;

        if left && top { SubNode::TopLeft }
        else if !left && top { SubNode::TopRight }
        else if left && !top { SubNode::BottomLeft }
        else { SubNode::BottomRight }
    }

    fn subdivide(&mut self) {
        let bounds = self.bounds();
        let depth = self.depth() + 1;
        let md = self.max_depth();
        let mc = self.max_children();
        
        let wh = bounds.width() / 2.0;
        let hh = bounds.height() / 2.0;
        
        let (x1, y1) = (bounds.x_min(), bounds.y_min());
        let (x2, y2) = (x1 + wh, y1 + hh);

        // Fixed order of iteration tl, tr, br, bl
        self.set_nodes(Some(Box::new([
            Self::new(Bounds::new(Point::new(x1, y1), wh, hh), depth, md, mc),
            Self::new(Bounds::new(Point::new(x2, y1), wh, hh), depth, md, mc),
            Self::new(Bounds::new(Point::new(x2, y2), wh, hh), depth, md, mc),
            Self::new(Bounds::new(Point::new(x1, y2), wh, hh), depth, md, mc),
        ])));
    }
}