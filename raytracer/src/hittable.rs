use crate::aabb::AABB;
use crate::color::Color;
use crate::material::Material;
use crate::pdf::{SpherePDF, PDF};
use crate::ray::Ray;
use crate::shape::Shape;
use crate::texture::UV;
use crate::vec3::Vec3;

pub struct HitInfo {
    pub t: f64,
    pub position: Vec3,   // the hit position
    pub normal: Vec3,     // always normalized and points opposite to the ray
    pub front_face: bool, // whether outside the object
    pub uv: UV,
    pub scatter_info: ScatterInfo,
}

pub struct ScatterInfo {
    pub emission: Color,
    pub attenuation: Color,
    pub scatter: Result<Box<dyn PDF>, Ray>,
}

impl Default for ScatterInfo {
    fn default() -> Self {
        Self {
            emission: Color::BLACK,
            attenuation: Color::WHITE,
            scatter: Ok(Box::new(SpherePDF)),
        }
    }
}

pub struct HitRecord {
    pub ray: Ray, // the original ray
    pub hit_info: Option<HitInfo>,
}

impl HitRecord {
    // outward_normal should be normalized
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            hit_info: None,
        }
    }

    pub fn set_hit(&mut self, t: f64, outward_normal: Vec3, uv: UV) {
        let position = self.ray.at(t);
        let front_face = self.ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        self.hit_info = Some(HitInfo {
            t,
            position,
            normal,
            front_face,
            uv,
            scatter_info: ScatterInfo::default(),
        });
        self.ray.interval.limit_max(t)
    }

    pub fn set_scatter(&mut self, direction: Vec3) {
        self.get_scatter_mut().scatter = Err(self.ray.new_ray(self.get_hit().position, direction))
    }

    pub fn does_hit(&self) -> bool {
        self.hit_info.is_some()
    }

    pub fn get_hit(&self) -> &HitInfo {
        self.hit_info.as_ref().unwrap()
    }

    // for decoration
    pub fn get_hit_mut(&mut self) -> &mut HitInfo {
        self.hit_info.as_mut().unwrap()
    }

    pub fn get_scatter(&self) -> &ScatterInfo {
        &self.get_hit().scatter_info
    }

    // for decoration
    pub fn get_scatter_mut(&mut self) -> &mut ScatterInfo {
        &mut self.get_hit_mut().scatter_info
    }

    pub fn get_output(self) -> Ray {
        self.hit_info.unwrap().scatter_info.scatter.err().unwrap()
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

pub struct Object<S: Shape, M: Material> {
    pub shape: S,
    pub material: M,
}

impl<S: Shape, M: Material> Object<S, M> {
    pub fn new(shape: S, material: M) -> Self {
        Self { shape, material }
    }
}

impl<S: Shape, M: Material> Hittable for Object<S, M> {
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
