use crate::color::Color;
use crate::hittable::HitRecord;
use crate::pdf::{CosineHemisphere, UniformSphere};
use crate::vec3::Vec3;

pub trait Material: Sync + Send {
    // hit_record.ray and hit_record.hit are the original ray and hit info
    // should set hit_record.scatter to three possible values (Absorb by default)
    // may decorate emission and attenuation
    fn scatter(&self, hit_record: &mut HitRecord);
}

pub struct Lambertian;

impl Material for Lambertian {
    fn scatter(&self, hit_record: &mut HitRecord) {
        hit_record.set_scatter_pdf(CosineHemisphere::new(hit_record.get_hit().normal));
    }
}

pub struct Metal {
    fuzz: f64,
}

impl Metal {
    pub fn new(fuzz: f64) -> Self {
        Self { fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, hit_record: &mut HitRecord) {
        let reflected = hit_record
            .get_ray()
            .direction
            .reflect(hit_record.get_hit().normal);
        let reflected = reflected.normalize() + Vec3::random_unit_vector() * self.fuzz;
        if reflected.dot(hit_record.get_hit().normal) > 0.0 {
            hit_record.set_scatter_ray(reflected);
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, hit_record: &mut HitRecord) {
        let refraction_ratio = if hit_record.get_hit().front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = hit_record.get_ray().direction.normalize();
        let normal = hit_record.get_hit().normal;
        let cos_theta = (-unit_direction).dot(normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let direction = if refraction_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > rand::random()
        {
            unit_direction.reflect(normal)
        } else {
            let r_out_perp = (unit_direction + normal * cos_theta) * refraction_ratio;
            let r_out_parallel = -normal * (1.0 - r_out_perp.length_squared()).sqrt();
            r_out_perp + r_out_parallel
        };
        hit_record.set_scatter_ray(direction);
    }
}

pub struct Isotropic;

impl Material for Isotropic {
    fn scatter(&self, hit_record: &mut HitRecord) {
        hit_record.set_scatter_pdf(UniformSphere);
    }
}

pub struct Translucent {
    refraction_index: f64,
}

impl Translucent {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index,
        }
    }
}

impl Material for Translucent {
    fn scatter(&self, hit_record: &mut HitRecord) {
        let refraction_ratio = if hit_record.get_hit().front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = hit_record.get_ray().direction.normalize();
        let normal = hit_record.get_hit().normal;
        let cos_theta = (-unit_direction).dot(normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let mut direction = unit_direction.reflect(normal);
        if refraction_ratio * sin_theta <= 1.0
        {
            let color = Color::new(1.0, sin_theta, 0.0);
            if rand::random::<bool>() {
                let r_out_perp = (unit_direction + normal * cos_theta) * refraction_ratio;
                let r_out_parallel = -normal * (1.0 - r_out_perp.length_squared()).sqrt();
                direction = r_out_perp + r_out_parallel;
                hit_record.get_hit_mut().attenuation = (Color::WHITE - color) * 2.0;
            } else {
                hit_record.get_hit_mut().attenuation = color * 2.0;
            }
        };
        hit_record.set_scatter_ray(direction);
    }
}