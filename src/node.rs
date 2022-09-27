use super::*;

pub trait Node <T: Datum<Geom>, Geom: System<Geometry = Geom>>
where Self: Sized {
    fn new(bounds: Bounds<Geom>, depth: u8, max_depth: u8, max_children: usize) -> Self;

    fn bounds(&self) -> &Bounds<Geom>;

    fn depth(&self) -> u8;

    fn max_depth(&self) -> u8;

    fn max_children(&self) -> usize;

    /// Return a vector with references to all children of the current node.
    /// This includes direct children and any stuck children if that concept
    /// exists for this QuadTree type. This means that any node, not just leaf
    /// nodes, may have children.
    fn children(&self) -> Vec<&T>;

    fn nodes(&self) -> &Option<Box<[Self; 4]>>;

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

    fn subdivide(&mut self) where Self: Sized {
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

    fn display(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = " ".repeat(self.depth() as usize * 4);
        let count = self.children().len();
        let children = if count == 0 {
            "".to_owned()
        } else if count == 1 {
            " 1 child".to_owned()
        } else {
            format!(" {count} children")
        };

        writeln!(f, "{indent}({:.2}, {:.2}):{children}", self.bounds().x_min(), self.bounds().y_min())?;

        if let Some(nodes) = &self.nodes() {
            for node in &**nodes {
                node.display(f)?
            }
        };
        write!(f, "")
    }
}

// TODO: Better to implement as iter on Node?
pub fn get_all_children<N: Node<T, Geom>, T: Datum<Geom>, Geom: System<Geometry = Geom>>(node: &N) -> Vec<&T> {
    let mut children = node.children();

    if let Some(nodes) = node.nodes() {
        for sub_node in &**nodes {
            children.extend(get_all_children(sub_node));
        }
    }

    children
}