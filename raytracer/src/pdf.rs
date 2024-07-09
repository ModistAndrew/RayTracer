use crate::onb::ONB;
use crate::vec3::Vec3;
use std::f64::consts::PI;

pub trait PDF {
    // return the probability density function value for a given Vec3
    fn value(&self, direction: Vec3) -> f64;
    // generate a random Vec3 according to the probability density function
    fn generate(&self) -> Vec3;
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(normal: Vec3) -> Self {
        Self {
            uvw: ONB::normal(normal),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: Vec3) -> f64 {
        let cosine = direction.normalize().dot(self.uvw.w);
        (cosine / PI).max(0.0)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(Vec3::random_cosine_direction())
    }
}

pub struct SpherePDF;

impl PDF for SpherePDF {
    fn value(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }
    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}
