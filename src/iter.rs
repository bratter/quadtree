use std::marker::PhantomData;
use std::slice::Iter;
use std::iter::{Chain, empty};

use geo::GeoNum;

use super::*;

// TODO: Docs
pub enum DatumIter<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    Empty,
    Slice(SliceIter<'a, D>),
    ChainSlice(ChainSliceIter<'a, D>),
    ChainSelf(ChainSelfIter<'a, N, D, T>),
    Descendant(DescendantIter<'a, N, D, T>),
}

impl<'a, N, D, T> Iterator for DatumIter<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    type Item = &'a D;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Empty => empty().next(),
            Self::Slice(iter) => iter.next(),
            Self::ChainSlice(iter) => iter.next(),
            Self::ChainSelf(iter) => iter.next(),
            Self::Descendant(iter) => iter.next(),
        }
    }
}

pub struct SliceIter<'a, D> {
    iter: Iter<'a, D>,
}

impl<'a, D> SliceIter<'a, D> {
    pub fn new(iter: Iter<'a, D>) -> Self {
        Self { iter }
    }
}

impl<'a, D> Iterator for SliceIter<'a, D> {
    type Item = &'a D;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct ChainSliceIter<'a, D> {
    iter: Chain<Iter<'a, D>, Iter<'a, D>>
}

impl<'a, D> ChainSliceIter<'a, D> {
    pub fn new(iter: Chain<Iter<'a, D>, Iter<'a, D>>) -> Self {
        Self { iter }
    }
}

impl<'a, D> Iterator for ChainSliceIter<'a, D> {
    type Item = &'a D;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct ChainSelfIter<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    iter: Chain<Box<DatumIter<'a, N, D, T>>, Box<DatumIter<'a, N, D, T>>>
}

impl<'a, N, D, T> ChainSelfIter<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    pub fn new(iter1: DatumIter<'a, N, D, T>, iter2: DatumIter<'a, N, D, T>) -> Self {
        Self { iter: Box::new(iter1).chain(Box::new(iter2)) }
    }
}

impl<'a, N, D, T> Iterator for ChainSelfIter<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    type Item = &'a D;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

pub struct DescendantIter<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    children_iter: Box<DatumIter<'a, N, D, T>>,
    nodes: Iter<'a, N>,
    cur_node_iter: Box<DatumIter<'a, N, D, T>>,
    _num_type: PhantomData<T>,
}

impl<'a, N, D, T> DescendantIter<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    pub fn new(
        children_iter: DatumIter<'a, N, D, T>,
        mut nodes: Iter<'a, N>,
    ) -> Self {
        let children_iter = Box::new(children_iter);
        // Unwrap will not panic because we know we always have four nodes
        let cur_node_iter = Box::new(nodes.next().unwrap().descendants());
        Self { children_iter, nodes, cur_node_iter, _num_type: PhantomData }
    }
}

impl<'a, N, D, T> Iterator for DescendantIter<'a, N, D, T>
where
    N: Node<D, T>,
    T: GeoNum,
{
    type Item = &'a D;

    // This complex implementation is an adjustment for the lack of ability to
    // return iterators simply. Would rather do:
    // `children.chain(nodes.flat_map(|n| n.descendants()))` but can't return
    // this due to inability to return impl Iterator from Traits.
    fn next(&mut self) -> Option<Self::Item> {
        // Return the item if we still have children
        if let Some(child) = self.children_iter.next() {
            return Some(child);
        }

        // Now move on to iterating through descendant children
        if let Some(child) = self.cur_node_iter.next() {
            return Some(child);
        }

        // We have run out of current children and descendants, so move on to
        // the next node, but must recurse so we return from next, otherwise
        // we are all out
        if let Some(node) = self.nodes.next() {
            self.cur_node_iter = Box::new(node.descendants());
            return self.next()
        }
        
        None
    }
}