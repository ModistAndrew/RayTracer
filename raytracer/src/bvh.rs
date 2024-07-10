use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::shape::Shape;
use crate::transform::Transform;

pub struct EmptyHittable;

impl Hittable for EmptyHittable {
    fn hit(&self, _hit_record: &mut HitRecord) -> bool {
        false
    }

    fn aabb(&self) -> AABB {
        AABB::default()
    }
}

pub struct HittableTree {
    // left and right are the two children of the node.
    // left isn't necessarily smaller than right, but each child is concentrated on one side of the parent for aabb pruning
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    // the bounding box of the node. cached for performance
    aabb: AABB,
}

impl HittableTree {
    // take a list of AABBProvider and build a BVH tree
    pub fn new(mut aabb_provider_list: Vec<Box<dyn Hittable>>) -> Self {
        let aabb = aabb_provider_list
            .iter()
            .fold(AABB::default(), |acc, aabb_provider| {
                acc.union(aabb_provider.aabb())
            });
        if aabb_provider_list.len() <= 2 {
            return Self {
                aabb,
                left: aabb_provider_list.pop().unwrap_or(Box::new(EmptyHittable)),
                right: aabb_provider_list.pop().unwrap_or(Box::new(EmptyHittable)),
            };
        }
        let axis = aabb.longest_axis();
        aabb_provider_list.sort_by(|a, b| a.aabb()[axis].min.total_cmp(&b.aabb()[axis].min));
        let mid = aabb_provider_list.len() / 2;
        Self {
            aabb,
            left: Box::new(HittableTree::new(aabb_provider_list.split_off(mid))),
            right: Box::new(HittableTree::new(aabb_provider_list)),
        }
    }
}

impl Hittable for HittableTree {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        if !self.aabb.hit(&hit_record.ray) {
            return false;
        }
        // note that we don't short-circuit here, because both children need to be hit
        self.left.hit(hit_record) | self.right.hit(hit_record)
    }

    fn aabb(&self) -> AABB {
        self.aabb
    }
}

#[derive(Default)]
pub struct HittableList {
    hittable_list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push<T: Hittable + 'static>(&mut self, hittable: T) {
        self.hittable_list.push(Box::new(hittable));
    }

    pub fn tree(self) -> HittableTree {
        HittableTree::new(self.hittable_list)
    }
}

impl Hittable for HittableList {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let mut hit_result = false;
        for hittable in &self.hittable_list {
            hit_result |= hittable.hit(hit_record);
        }
        hit_result
    }

    fn aabb(&self) -> AABB {
        let mut aabb = AABB::default();
        for shape in &self.hittable_list {
            aabb = aabb.union(shape.aabb());
        }
        aabb
    }
}

pub struct EmptyShape;

impl Shape for EmptyShape {
    fn hit(&self, _hit_record: &mut HitRecord) -> bool {
        false
    }

    fn transform(&mut self, _matrix: Transform) {}

    fn aabb(&self) -> AABB {
        AABB::default()
    }
}

pub struct ShapeTree {
    left: Box<dyn Shape>,
    right: Box<dyn Shape>,
    aabb: AABB,
}

impl ShapeTree {
    pub fn new(mut shape_list: Vec<Box<dyn Shape>>) -> Self {
        let aabb = shape_list
            .iter()
            .fold(AABB::default(), |acc, shape| acc.union(shape.aabb()));
        if shape_list.len() <= 2 {
            return Self {
                aabb,
                left: shape_list.pop().unwrap_or(Box::new(EmptyShape)),
                right: shape_list.pop().unwrap_or(Box::new(EmptyShape)),
            };
        }
        let axis = aabb.longest_axis();
        shape_list.sort_by(|a, b| a.aabb()[axis].min.total_cmp(&b.aabb()[axis].min));
        let mid = shape_list.len() / 2;
        Self {
            aabb,
            left: Box::new(ShapeTree::new(shape_list.split_off(mid))),
            right: Box::new(ShapeTree::new(shape_list)),
        }
    }
}

impl Shape for ShapeTree {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        if !self.aabb.hit(&hit_record.ray) {
            return false;
        }
        // note that we don't short-circuit here, because both children need to be hit
        self.left.hit(hit_record) | self.right.hit(hit_record)
    }

    fn transform(&mut self, matrix: Transform) {
        self.left.transform(matrix);
        self.right.transform(matrix);
        self.aabb = self.left.aabb().union(self.right.aabb());
    }

    fn aabb(&self) -> AABB {
        self.aabb
    }
}

#[derive(Default)]
pub struct ShapeList {
    shape_list: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn push<T: Shape + 'static>(&mut self, shape: T) {
        self.shape_list.push(Box::new(shape));
    }

    pub fn tree(self) -> ShapeTree {
        ShapeTree::new(self.shape_list)
    }
}

impl Shape for ShapeList {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let mut hit = false;
        for shape in &self.shape_list {
            hit |= shape.hit(hit_record);
        }
        hit
    }

    fn transform(&mut self, matrix: Transform) {
        for shape in &mut self.shape_list {
            shape.transform(matrix);
        }
    }

    fn aabb(&self) -> AABB {
        let mut aabb = AABB::default();
        for shape in &self.shape_list {
            aabb = aabb.union(shape.aabb());
        }
        aabb
    }
}
