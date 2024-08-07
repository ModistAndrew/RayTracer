use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::pdf::{CosineHemisphere, UniformSphere};
use crate::texture::Atlas;
use crate::vec3::Vec3;

pub trait Material: Sync + Send {
    // hit_record.ray and hit_record.hit are the original ray and hit info
    // should set hit_record.scatter to three possible values (Absorb by default)
    // may decorate emission and attenuation
    fn scatter(&self, hit_record: &mut HitRecord, atlas: &Atlas);
}

pub struct Lambertian;

impl Material for Lambertian {
    fn scatter(&self, hit_record: &mut HitRecord, atlas: &Atlas) {
        hit_record.set_scatter_pdf(CosineHemisphere::new(hit_record.get_hit().normal));
        hit_record.get_hit_mut().attenuation = atlas.get_attenuation(hit_record.get_hit());
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
    fn scatter(&self, hit_record: &mut HitRecord, atlas: &Atlas) {
        let reflected = hit_record
            .get_ray()
            .direction
            .reflect(hit_record.get_hit().normal);
        let reflected = reflected.normalize() + Vec3::random_unit_vector() * self.fuzz;
        if reflected.dot(hit_record.get_hit().normal) > 0.0 {
            hit_record.set_scatter_ray(reflected);
            hit_record.get_hit_mut().attenuation = atlas.get_attenuation(hit_record.get_hit());
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
    fn scatter(&self, hit_record: &mut HitRecord, _atlas: &Atlas) {
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

pub struct Isotropic {
    glow: f64,
}

impl Isotropic {
    pub fn new(glow: f64) -> Self {
        Self { glow }
    }
}

impl Material for Isotropic {
    fn scatter(&self, hit_record: &mut HitRecord, atlas: &Atlas) {
        if rand::random::<f64>() > self.glow {
            hit_record.set_scatter_pdf(UniformSphere);
            hit_record.get_hit_mut().attenuation = atlas.get_attenuation(hit_record.get_hit());
        } else {
            hit_record.set_scatter_absorb();
            hit_record.get_hit_mut().emission = atlas.get_attenuation(hit_record.get_hit());
        }
    }
}

pub struct Translucent {
    refraction_index: f64,
}

impl Translucent {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Translucent {
    fn scatter(&self, hit_record: &mut HitRecord, _atlas: &Atlas) {
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
        if refraction_ratio * sin_theta <= 1.0 {
            let reflect_color = Color::new(1.0, sin_theta, 0.0);
            if rand::random::<bool>() {
                let r_out_perp = (unit_direction + normal * cos_theta) * refraction_ratio;
                let r_out_parallel = -normal * (1.0 - r_out_perp.length_squared()).sqrt();
                direction = r_out_perp + r_out_parallel;
                hit_record.get_hit_mut().attenuation = (Color::WHITE - reflect_color) * 2.0;
            } else {
                hit_record.get_hit_mut().attenuation = reflect_color * 2.0;
            }
        };
        hit_record.set_scatter_ray(direction);
    }
}

pub struct Emissive {
    ratio: f64,
}

impl Emissive {
    pub fn new(ratio: f64) -> Self {
        Self { ratio }
    }
}

impl Material for Emissive {
    fn scatter(&self, hit_record: &mut HitRecord, atlas: &Atlas) {
        if hit_record.get_hit().front_face {
            hit_record.get_hit_mut().emission = atlas.get_emission(hit_record.get_hit()) * self.ratio;
        }
    }
}
