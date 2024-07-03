use crate::aabb::AABB;
use crate::bvh::BVHNode;
use crate::color::Color;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::vec3::Vec3;

pub struct Hit {
    pub t: f64,
    pub position: Vec3,   // the hit position
    pub normal: Vec3,     // always normalized and points opposite to the ray
    pub front_face: bool, // whether outside the object
    pub u: f64,
    pub v: f64, // texture coordinate
}

pub struct HitRecord {
    pub ray: Ray, // the original ray
    pub hit: Option<Hit>,
    pub scatter: Option<Ray>,
}

impl HitRecord {
    // outward_normal should be normalized
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            hit: None,
            scatter: None,
        }
    }

    pub fn set_hit(&mut self, t: f64, outward_normal: Vec3) {
        let front_face = self.ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        let position = self.ray.at(t);
        self.hit = Some(Hit {
            t,
            position,
            normal,
            front_face,
            u: 0.0,
            v: 0.0,
        });
        self.ray.interval.limit_max(t)
    }

    pub fn set_scatter(&mut self, direction: Vec3, blender: Color) {
        self.scatter = Some(Ray::new(
            self.get_hit().position,
            direction,
            blender.blend(self.ray.color, crate::color::BlendMode::Mul),
            self.ray.time,
            Interval::POSITIVE,
        ));
    }

    pub fn does_hit(&self) -> bool {
        self.hit.is_some()
    }

    pub fn get_hit(&self) -> &Hit {
        self.hit.as_ref().unwrap()
    }

    pub fn get_scatter(&self) -> &Ray {
        self.scatter.as_ref().unwrap()
    }
}

pub trait Hittable: Sync + Send {
    // hit_record.ray is the original ray.
    // if hit, update hit_record.hit and scatter and return true
    fn hit(&self, hit_record: &mut HitRecord) -> bool;

    // return the bounding box for hit testing
    fn aabb(&self) -> AABB;
}

pub struct Object {
    pub shape: Box<dyn Shape>,
    pub material: Box<dyn Material>,
}

impl Object {
    pub fn new(shape: Box<dyn Shape>, material: Box<dyn Material>) -> Self {
        Self { shape, material }
    }
}

impl Hittable for Object {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        self.shape.hit(hit_record) && {
            self.material.scatter(hit_record);
            true
        }
    }

    fn aabb(&self) -> AABB {
        self.shape.aabb()
    }
}

// a simple wrapper for the BVH tree
pub struct HittableList {
    bvh_node: BVHNode,
}

impl HittableList {
    pub fn new(hittable_list: Vec<Box<dyn Hittable>>) -> Self {
        Self {
            bvh_node: BVHNode::new(hittable_list),
        }
    }
}

#[derive(Default)]
pub struct Empty;

impl Hittable for Empty {
    fn hit(&self, _hit_record: &mut HitRecord) -> bool {
        false
    }

    fn aabb(&self) -> AABB {
        AABB::default()
    }
}

impl Hittable for HittableList {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        self.bvh_node.hit(hit_record)
    }

    fn aabb(&self) -> AABB {
        self.bvh_node.aabb()
    }
}
