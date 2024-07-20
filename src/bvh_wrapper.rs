use bvh::aabb::Bounded;
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::Bvh;

use crate::aabb::{Aabb, Aabb3};
use crate::ray::Ray3;

// wrapper for bvh.
// you can use any T which impl AabbProvider
pub trait AabbProvider {
    fn aabb(&self) -> Aabb;
}

pub struct BoundedNode<T: AabbProvider> {
    pub inner: T,
    node_index: usize,
}

impl<T: AabbProvider> BoundedNode<T> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            node_index: 0,
        }
    }
}

impl<T: AabbProvider> Bounded<f64, 3> for BoundedNode<T> {
    fn aabb(&self) -> Aabb3 {
        self.inner.aabb().into()
    }
}

impl<T: AabbProvider> BHShape<f64, 3> for BoundedNode<T> {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

pub struct BoundedTree<T: AabbProvider> {
    vec: Vec<BoundedNode<T>>,
    bvh: Bvh<f64, 3>,
    aabb: Aabb, // store an AABB for the entire tree
}

impl<T: AabbProvider> BoundedTree<T> {
    pub fn traverse(&self, ray: &Ray3) -> Vec<&BoundedNode<T>> {
        self.bvh.traverse(ray, &self.vec)
    }

    pub fn aabb(&self) -> Aabb {
        self.aabb
    }
}

pub struct BoundedTreeBuilder<T: AabbProvider> {
    // a builder for BoundedTree
    vec: Vec<BoundedNode<T>>,
    aabb: Aabb,
}

impl<T: AabbProvider> Default for BoundedTreeBuilder<T> {
    fn default() -> Self {
        Self {
            vec: Vec::default(),
            aabb: Aabb::default(),
        }
    }
}

impl<T: AabbProvider> BoundedTreeBuilder<T> {
    pub fn add(&mut self, t: T) {
        self.aabb = self.aabb.union(t.aabb());
        self.vec.push(BoundedNode::new(t));
    }

    pub fn build(mut self) -> BoundedTree<T> {
        let bvh = Bvh::build(&mut self.vec);
        BoundedTree {
            vec: self.vec,
            bvh,
            aabb: self.aabb,
        }
    }
}
