use crate::color::Color;
use crate::hittable::HitRecord;
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
        let mut scatter_direction = hit_record.get_hit().normal + Vec3d::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.get_hit().normal;
        }
        hit_record.set_scatter(scatter_direction, self.albedo);
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &mut HitRecord) {
        let mut reflected = hit_record
            .ray
            .direction
            .reflect(hit_record.get_hit().normal);
        reflected = reflected.normalize() + Vec3d::random_unit_vector() * self.fuzz;
        hit_record.set_scatter(
            reflected,
            if reflected.dot(hit_record.get_hit().normal) > 0.0 {
                self.albedo
            } else {
                Color::new(0.0, 0.0, 0.0)
            },
        );
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &mut HitRecord) {
        let refraction_ratio = if hit_record.get_hit().front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        hit_record.set_scatter(
            hit_record
                .ray
                .direction
                .normalize()
                .refract(hit_record.get_hit().normal, refraction_ratio),
            Color::new(1.0, 1.0, 1.0),
        );
    }
}
