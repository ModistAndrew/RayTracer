use crate::aabb::AABB;
use crate::hittable::{Empty, HitRecord, Hittable};

pub struct BVHNode {
    // left and right are the two children of the node.
    // left isn't necessarily smaller than right, but each child is concentrated on one side of the parent for aabb pruning
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    // the bounding box of the node. cached for performance
    aabb: AABB,
}

impl BVHNode {
    // take a list of hittable and build a BVH tree
    pub fn new(mut hittable_list: Vec<Box<dyn Hittable>>) -> Self {
        let aabb = hittable_list
            .iter()
            .fold(AABB::default(), |acc, hittable| acc.union(hittable.aabb()));
        if hittable_list.len() <= 2 {
            return Self {
                aabb,
                left: hittable_list.pop().unwrap_or(Box::<Empty>::default()),
                right: hittable_list.pop().unwrap_or(Box::<Empty>::default()),
            };
        }
        let axis = aabb.longest_axis();
        hittable_list.sort_by(|a, b| a.aabb()[axis].min.total_cmp(&b.aabb()[axis].min));
        let mid = hittable_list.len() / 2;
        Self {
            aabb,
            left: Box::new(BVHNode::new(hittable_list.split_off(mid))),
            right: Box::new(BVHNode::new(hittable_list)),
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        if !self.aabb.hit(&hit_record.ray) {
            return false;
        }
        // note that the | operator is used instead of || because we want to call both hit functions
        self.left.hit(hit_record) | self.right.hit(hit_record)
    }

    fn aabb(&self) -> AABB {
        self.aabb
    }
}
