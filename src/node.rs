use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use geo::{coord, Coord, GeoNum, Rect};

use crate::iter::{DatumIter, DescendantIter};
use crate::Error;

/// Sub-node indicies.
///
/// Naming scheme based on clockwise rotation with a top-left origin. Or
/// counterclockeise with a bottom-left origin.
#[derive(Debug, Clone, Copy)]
pub enum SubNode {
    TopLeft = 0,
    TopRight = 1,
    BottomRight = 2,
    BottomLeft = 3,
}

/// Internal enum to track whether an element in a Node iterator or stack is a
/// child datum or a sub-node.
#[derive(Debug, Clone)]
pub enum NodeType<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    Node(&'a N),
    Child(&'a D),
    _NumType(PhantomData<T>),
}

/// Trait for a QuadTree node. Nodes should not be visible to the consumer.
pub trait Node<D, T>
where
    Self: Sized,
    T: GeoNum,
{
    /// Create a new Node with the passed structure.
    fn new(bounds: Rect<T>, depth: u8, max_depth: u8, max_children: usize) -> Self;

    /// Get a single [`Coordinate`] position of the datum in a manner suitable
    /// for the constraints of the implementation.
    fn datum_position(datum: &D) -> Option<Coord<T>>;

    /// Get the bounding rect for the Node.
    fn bounds(&self) -> &Rect<T>;

    /// Get the depth of the current Node in the QuadTree.
    fn depth(&self) -> u8;

    /// Get the maximum depth permitted in the QuadTree.
    fn max_depth(&self) -> u8;

    /// Get the maxmimum number of children allowed per node before subdividing.
    /// Note that max depth has precedence, so children may stack arbitrarily at
    /// the deepest level.
    fn max_children(&self) -> usize;

    /// Returns the Node's sub-nodes. Is an Option because these won;t exist
    /// for leaf Nodes.
    fn nodes(&self) -> &Option<Box<[Self; 4]>>;

    /// Return an iterator with references to all children of the current node.
    /// This includes direct children and any stuck children if that concept
    /// exists for this QuadTree type. This means that any node, not just leaf
    /// nodes, may have children.
    fn children(&self) -> DatumIter<'_, Self, D, T>;

    /// Return all descendant data of this node in preorder. The iterator first
    /// emits the children of the current node, then recurses into the
    /// sub-nodes if they exist.
    fn descendants(&self) -> DatumIter<'_, Self, D, T> {
        if let Some(nodes) = self.nodes() {
            DatumIter::Descendant(DescendantIter::new(self.children(), nodes.iter()))
        } else {
            self.children()
        }
    }

    /// Set the sub-nodes for this Node. Required as a separate method to
    /// enable the sub-node logic to live in the trait.
    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>);

    /// Insert a child into this Node, or delegate to a sub-node where
    /// appropriate.
    fn insert(&mut self, datum: D) -> Result<(), Error>;

    /// Retrieve from this Node. Will return children and also delegate to the
    /// appropriate sub-nodes based on the implementation.
    fn retrieve(&self, datum: &D) -> DatumIter<'_, Self, D, T>;

    /// Find the index of the appropriate sub-node to delegate an insert or
    /// retrieve operation if required.
    fn find_sub_node(&self, datum: &D) -> Option<SubNode> {
        let (x, y) = Self::datum_position(datum)?.x_y();
        let b = self.bounds();
        let two = T::one() + T::one();
        let left = x <= b.min().x + b.width() / two;
        let top = y <= b.min().y + b.height() / two;

        let sn = if left && top {
            SubNode::TopLeft
        } else if !left && top {
            SubNode::TopRight
        } else if left && !top {
            SubNode::BottomLeft
        } else {
            SubNode::BottomRight
        };

        Some(sn)
    }

    /// Subdivide the current Node into four sub-nodes at the next-depth level
    /// and set them in the current Node.
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
            Self::new(
                Rect::new(coord! {x: x1, y: y1}, coord! {x: x2, y: y2}),
                depth,
                md,
                mc,
            ),
            Self::new(
                Rect::new(coord! {x: x2, y: y1}, coord! {x: x3, y: y2}),
                depth,
                md,
                mc,
            ),
            Self::new(
                Rect::new(coord! {x: x2, y: y2}, coord! {x: x3, y: y3}),
                depth,
                md,
                mc,
            ),
            Self::new(
                Rect::new(coord! {x: x1, y: y2}, coord! {x: x2, y: y3}),
                depth,
                md,
                mc,
            ),
        ])));
    }

    /// Heavy-lifting for custom Node display.
    fn display(&self, f: &mut Formatter) -> std::fmt::Result
    where
        T: Display,
    {
        let indent = " ".repeat(self.depth() as usize * 4);
        let min = self.bounds().min();
        let count = self.children().count();
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
