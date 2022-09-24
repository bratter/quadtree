use super::*;

pub trait Node <T: Datum<Geom>, Geom: System<Geometry = Geom>> where Self: Sized {
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

// TODO: Make nodes and children private?
#[derive(Debug)]
pub struct PointNode<T: Datum<Geom>, Geom: System<Geometry = Geom>> {
    bounds: Bounds<Geom>,
    depth: u8,
    max_depth: u8,
    max_children: usize,
    pub children: Vec<T>,
    pub nodes: Option<Box<[PointNode<T, Geom>; 4]>>,
}

impl <T: Datum<Geom>, Geom: System<Geometry = Geom>> Node<T, Geom> for PointNode<T, Geom> {
    fn new(bounds: Bounds<Geom>, depth: u8, max_depth: u8, max_children: usize) -> Self {
        Self {
            bounds,
            depth,
            max_depth,
            max_children,
            children: Vec::new(),
            nodes: None,
        }
    }

    // Getters
    fn bounds(&self) -> &Bounds<Geom> { &self.bounds }
    fn depth(&self) -> u8 { self.depth }
    fn max_depth(&self) -> u8 { self.max_depth }
    fn max_children(&self) -> usize { self.max_children }

    // Setters
    fn set_nodes(&mut self, nodes: Option<Box<[Self; 4]>>) { self.nodes = nodes; }

    fn insert(&mut self, datum: T) {
        // Take ownership of the sub-nodes before matching to enable the insertion
        // This, apparently, is a very common pattern
        // Works here because we replace the nodes at the end, and the None branch
        // is unaffected. Overall a more ergonomic solution than the alterantive
        // `let sub_nodes = self.nodes.as_mut().unwrap()`
        match self.nodes.take() {
            // If we have sub-nodes already, pass down the tree
            Some(mut sub_nodes) => {
                let sub_node_idx = self.find_sub_node(&datum);
                sub_nodes[sub_node_idx as usize].insert(datum);
                
                // Make sure to replace the nodes
                self.nodes = Some(sub_nodes);
            },
            // If there is no room left, subdivide and push all children down
            // Subdivision does not happen if we've exceeded the max depth,
            // which takes priority over the children length
            None if self.children.len() >= self.max_children && !(self.depth >= self.max_depth)  => {
                self.subdivide();

                // Replace the old children with a new empty vector
                // and push the new point on last to preserve ordering
                let mut children = std::mem::replace(&mut self.children, Vec::new());
                children.push(datum);

                // Now consume the original children vector
                for pt in children { self.insert(pt); }
            }
            // Otherwise can simply push the point
            None => {
                self.children.push(datum);
            }
        }
    }

    // Pulls all children within the node that would contain the passed point
    fn retrieve(&self, datum: &T) -> Vec<&T> {
        match &self.nodes {
            Some(nodes) => {
                nodes[self.find_sub_node(datum) as usize].retrieve(datum)
            },
            None => {
                // Convert &Vec<T> to Vec<&T>
                self.children.iter().collect()
            }
        }
    }
}

// TODO: Can this be made universal
impl <T: Datum<Geom>, Geom: System<Geometry = Geom>> std::fmt::Display for PointNode<T, Geom> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent = " ".repeat(self.depth as usize * 4);
        let count = self.children.len();
        let children = if count == 0 {
            "".to_owned()
        } else if count == 1 {
            " 1 child".to_owned()
        } else {
            format!(" {count} children")
        };

        writeln!(f, "{indent}({:.2}, {:.2}):{children}", self.bounds.x_min(), self.bounds.y_min())?;
        if let Some(nodes) = &self.nodes {
            for node in nodes.iter() {
                write!(f, "{node}")?;
            }
        };
        write!(f, "")
    }
}