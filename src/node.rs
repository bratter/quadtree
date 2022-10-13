use geo::{Rect, coord};
use super::*;

pub enum SubNode {
    TopLeft = 0,
    TopRight = 1,
    BottomRight = 2,
    BottomLeft = 3,
}

pub trait Node <T>
where
    T: Datum,
    Self: Sized
{
    fn new(bounds: Rect, depth: u8, max_depth: u8, max_children: usize) -> Self;

    fn bounds(&self) -> &Rect;

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
        let (x, y) = datum.point().x_y();
        let center = self.bounds().center();
        let left = x <= center.x;
        let top = y <= center.y;

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
        
        let (x1, y1) = bounds.min().x_y();
        let (x2, y2) = (x1 + wh, y1 + hh);
        let (x3, y3) = bounds.max().x_y();

        // Fixed order of iteration tl, tr, br, bl
        self.set_nodes(Some(Box::new([
            Self::new(Rect::new(coord! {x: x1, y: y1}, coord! {x: x2, y: y2}), depth, md, mc),
            Self::new(Rect::new(coord! {x: x2, y: y1}, coord! {x: x3, y: y2}), depth, md, mc),
            Self::new(Rect::new(coord! {x: x2, y: y2}, coord! {x: x3, y: y3}), depth, md, mc),
            Self::new(Rect::new(coord! {x: x1, y: y2}, coord! {x: x2, y: y3}), depth, md, mc),
        ])));
    }

    fn display(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = " ".repeat(self.depth() as usize * 4);
        let min = self.bounds().min();
        let count = self.children().len();
        let children = if count == 0 {
            "".to_owned()
        } else if count == 1 {
            " 1 child".to_owned()
        } else {
            format!(" {count} children")
        };

        writeln!(f, "{indent}({:.2}, {:.2}):{children}", min.x, min.y)?;

        if let Some(nodes) = &self.nodes() {
            for node in &**nodes {
                node.display(f)?
            }
        };
        write!(f, "")
    }
}

// TODO: Better to implement as iter on Node?
pub fn get_all_children<N, T>(node: &N) -> Vec<&T>
where
    N: Node<T>,
    T: Datum,
{
    let mut children = node.children();

    if let Some(nodes) = node.nodes() {
        for sub_node in &**nodes {
            children.extend(get_all_children(sub_node));
        }
    }

    children
}