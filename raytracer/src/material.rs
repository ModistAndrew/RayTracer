use crate::color::{BlendMode, Color};
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3d::Vec3d;

pub trait Material {
    // hit_record.ray and hit_record.hit are the original ray and hit record. may contain the former scattered ray. update hit_record.scatter
    fn scatter(&self, hit_record: &mut HitRecord);
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, hit_record: &mut HitRecord) {
        let scatter_direction = hit_record.get_hit().normal + Vec3d::random_unit_vector();
        hit_record.set_scatter(Ray::new(hit_record.get_hit().position, scatter_direction,
                                        hit_record.ray.color.blend(self.albedo, BlendMode::Mul)));
    }
}