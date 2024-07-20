use crate::aabb::Aabb;
use crate::hit_record::HitRecord;
use crate::hittable::Hittable;
use crate::shape::Shape;
use crate::texture::Atlas;

pub struct EmptyHittable;

impl Hittable for EmptyHittable {
    fn hit(&self, _hit_record: &mut HitRecord) -> bool {
        false
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::default()
    }
}

pub struct HittableTree {
    // left and right are the two children of the node.
    // left isn't necessarily smaller than right, but each child is concentrated on one side of the parent for aabb pruning
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    // the bounding box of the node. cached for performance
    aabb: Aabb,
}

impl HittableTree {
    // take a list of AabbProvider and build a BVH tree
    pub fn new(mut aabb_provider_list: Vec<Box<dyn Hittable>>) -> Self {
        let aabb = aabb_provider_list
            .iter()
            .fold(Aabb::default(), |acc, aabb_provider| {
                acc.union(aabb_provider.bounding_box())
            });
        if aabb_provider_list.len() <= 2 {
            return HittableTree {
                aabb,
                left: aabb_provider_list.pop().unwrap_or(Box::new(EmptyHittable)),
                right: aabb_provider_list.pop().unwrap_or(Box::new(EmptyHittable)),
            };
        }
        let axis = aabb.longest_axis();
        aabb_provider_list.sort_by(|a, b| a.bounding_box()[axis].min.total_cmp(&b.bounding_box()[axis].min));
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
        if !self.aabb.hit(hit_record.get_ray(), hit_record.get_interval()) {
            return false;
        }
        // note that we don't short-circuit here, because both children need to be hit
        self.left.hit(hit_record) | self.right.hit(hit_record)
    }

    fn bounding_box(&self) -> Aabb {
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

    fn bounding_box(&self) -> Aabb {
        let mut aabb = Aabb::default();
        for shape in &self.hittable_list {
            aabb = aabb.union(shape.bounding_box());
        }
        aabb
    }
}

pub struct EmptyShape;

impl Shape for EmptyShape {
    fn hit(&self, _hit_record: &mut HitRecord, _atlas: &Atlas) -> bool {
        false
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::default()
    }
}

pub struct ShapeTree {
    left: Box<dyn Shape>,
    right: Box<dyn Shape>,
    aabb: Aabb,
}

impl ShapeTree {
    pub fn new(mut aabb_provider_list: Vec<Box<dyn Shape>>) -> Self {
        let aabb = aabb_provider_list
            .iter()
            .fold(Aabb::default(), |acc, aabb_provider| {
                acc.union(aabb_provider.bounding_box())
            });
        if aabb_provider_list.len() <= 2 {
            return ShapeTree {
                aabb,
                left: aabb_provider_list.pop().unwrap_or(Box::new(EmptyShape)),
                right: aabb_provider_list.pop().unwrap_or(Box::new(EmptyShape)),
            };
        }
        let axis = aabb.longest_axis();
        aabb_provider_list.sort_by(|a, b| a.bounding_box()[axis].min.total_cmp(&b.bounding_box()[axis].min));
        let mid = aabb_provider_list.len() / 2;
        Self {
            aabb,
            left: Box::new(ShapeTree::new(aabb_provider_list.split_off(mid))),
            right: Box::new(ShapeTree::new(aabb_provider_list)),
        }
    }
}

impl Shape for ShapeTree {
    fn hit(&self, hit_record: &mut HitRecord, atlas: &Atlas) -> bool {
        if !self.aabb.hit(hit_record.get_ray(), hit_record.get_interval()) {
            return false;
        }
        // note that we don't short-circuit here, because both children need to be hit
        self.left.hit(hit_record, atlas) | self.right.hit(hit_record, atlas)
    }

    fn bounding_box(&self) -> Aabb {
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
    fn hit(&self, hit_record: &mut HitRecord, atlas: &Atlas) -> bool {
        let mut hit = false;
        for shape in &self.shape_list {
            hit |= shape.hit(hit_record, atlas);
        }
        hit
    }

    fn bounding_box(&self) -> Aabb {
        let mut aabb = Aabb::default();
        for shape in &self.shape_list {
            aabb = aabb.union(shape.bounding_box());
        }
        aabb
    }
}