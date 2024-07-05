use crate::aabb::AABB;
use crate::bvh::BVHNode;
use crate::color::Color;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::texture::UV;
use crate::vec3::Vec3;

pub struct Hit {
    pub position: Vec3,   // the hit position
    pub normal: Vec3,     // always normalized and points opposite to the ray
    pub front_face: bool, // whether outside the object
    pub uv: UV,
}

pub struct Scatter {
    pub emission: Color,
    pub attenuation: Color,
    pub scatter: Ray,
}

pub struct HitRecord {
    pub ray: Ray, // the original ray
    pub hit: Option<Hit>,
    pub scatter: Option<Scatter>,
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

    pub fn set_hit(&mut self, t: f64, position: Vec3, outward_normal: Vec3, uv: UV) {
        let front_face = self.ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        self.hit = Some(Hit {
            position,
            normal,
            front_face,
            uv,
        });
        self.ray.interval.limit_max(t)
    }

    pub fn set_scatter(&mut self, direction: Vec3) {
        self.scatter = Some(Scatter {
            emission: Color::BLACK,
            attenuation: Color::WHITE,
            scatter: Ray::new(
                self.get_hit().position,
                direction,
                self.ray.time,
                Interval::POSITIVE,
            ),
        });
    }

    pub fn does_hit(&self) -> bool {
        self.hit.is_some()
    }

    // for decoration
    pub fn get_hit_mut(&mut self) -> &mut Hit {
        self.hit.as_mut().unwrap()
    }

    // for decoration
    pub fn get_scatter_mut(&mut self) -> &mut Scatter {
        self.scatter.as_mut().unwrap()
    }

    pub fn get_hit(&self) -> &Hit {
        self.hit.as_ref().unwrap()
    }

    pub fn get_scatter(&self) -> &Scatter {
        self.scatter.as_ref().unwrap()
    }

    pub fn get_output(self) -> Ray {
        self.scatter.unwrap().scatter
    }
}

#[derive(Clone, Copy)]
pub enum HitResult {
    Miss = 0,
    Absorb = 1,
    Scatter = 2,
}

impl HitResult {
    pub fn max(self, other: Self) -> Self {
        if self as i8 > other as i8 {
            self
        } else {
            other
        }
    }
}

pub trait Hittable: Sync + Send {
    // hit_record.ray is the original ray.
    // if hit, update hit_record.hit and scatter and return true
    fn hit(&self, hit_record: &mut HitRecord) -> HitResult;

    // return the bounding box for hit testing
    fn aabb(&self) -> AABB;
}

pub struct Object<S: Shape + Sync + Send, M: Material + Sync + Send> {
    pub shape: S,
    pub material: M,
}

impl<S: Shape + Sync + Send, M: Material + Sync + Send> Object<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self { shape, material }
    }
}

impl<S: Shape + Sync + Send, M: Material + Sync + Send> Hittable for Object<S, M> {
    fn hit(&self, hit_record: &mut HitRecord) -> HitResult {
        if !self.shape.hit(hit_record) {
            HitResult::Miss
        } else if !self.material.scatter(hit_record) {
            HitResult::Absorb
        } else {
            HitResult::Scatter
        }
    }

    fn aabb(&self) -> AABB {
        self.shape.aabb()
    }
}

// a simple builder for the BVH tree
#[derive(Default)]
pub struct HittableList {
    hittable_list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push<T: Hittable + 'static>(&mut self, hittable: T) {
        self.hittable_list.push(Box::new(hittable));
    }

    pub fn build(self) -> BVHNode {
        BVHNode::new(self.hittable_list)
    }
}

#[derive(Default)]
pub struct Empty;

impl Hittable for Empty {
    fn hit(&self, _hit_record: &mut HitRecord) -> HitResult {
        HitResult::Miss
    }

    fn aabb(&self) -> AABB {
        AABB::default()
    }
}
