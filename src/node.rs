use geo::{Rect, Coordinate, coord, GeoNum};

use crate::Error;

/// Sub-node indicies.
pub enum SubNode {
    TopLeft = 0,
    TopRight = 1,
    BottomRight = 2,
    BottomLeft = 3,
}

/// Trait for a QuadTree node. This should not be visible to the consumer.
pub trait Node<D, T>
where
    Self: Sized,
    T: GeoNum,
{
    fn new(bounds: Rect<T>, depth: u8, max_depth: u8, max_children: usize) -> Self;

    /// Get a single Coordinate position of the datum in a manner suitable for
    /// the constraints of the implementation.
    fn datum_position(datum: &D) -> Coordinate<T>;

    fn bounds(&self) -> &Rect<T>;

    fn depth(&self) -> u8;

    fn max_depth(&self) -> u8;

    fn max_children(&self) -> usize;

    /// Return a vector with references to all children of the current node.
    /// This includes direct children and any stuck children if that concept
    /// exists for this QuadTree type. This means that any node, not just leaf
    /// nodes, may have children.
    fn children(&self) -> Vec<&D>;

    fn nodes(&self) -> &Option<Box<[Self; 4]>>;

    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>);

    fn insert(&mut self, datum: D) -> Result<(), Error>;

    fn retrieve(&self, datum: &D) -> Vec<&D>;

    fn find_sub_node(&self, datum: &D) -> SubNode {
        let (x, y) = Self::datum_position(datum).x_y();
        let two = T::one() + T::one();
        let left = x <= self.bounds().width() / two;
        let top = y <= self.bounds().height() / two;

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
        
        let two = T::one() + T::one();
        let wh = bounds.width() / two;
        let hh = bounds.height() / two;
        
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

    fn display(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    where
        T: std::fmt::Display,
    {
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
pub(crate) fn get_all_children<N, D, T>(node: &N) -> Vec<&D>
where
    N: Node<D, T>,
    T: GeoNum,
{
    let mut children = node.children();

    if let Some(nodes) = node.nodes() {
        for sub_node in &**nodes {
            children.extend(get_all_children(sub_node));
        }
    }

    children
}