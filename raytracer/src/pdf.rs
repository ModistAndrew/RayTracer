use crate::onb::ONB;
use crate::shape::ShapePDFProvider;
use crate::vec3::Vec3;
use rand::prelude::IndexedRandom;
use std::f64::consts::PI;
use std::fmt::Debug;

pub trait PDF: Debug {
    // return the probability density function value for a given Vec3
    fn prob(&self, direction: Vec3) -> f64;
    // generate a random Vec3 according to the probability density function
    fn generate(&self) -> Vec3;
    fn generate_with_prob(&self) -> (Vec3, f64) {
        let v = self.generate();
        (v, self.prob(v))
    }
}

pub fn mixture_generate(p1: &dyn PDF, p2: &dyn PDF) -> Vec3 {
    if rand::random::<f64>() < 0.5 {
        p1.generate()
    } else {
        p2.generate()
    }
}

pub fn mixture_prob(p1: &dyn PDF, p2: &dyn PDF, direction: Vec3) -> f64 {
    0.5 * p1.prob(direction) + 0.5 * p2.prob(direction)
}

pub fn mixture_generate_with_prob(p1: &dyn PDF, p2: &dyn PDF) -> (Vec3, f64) {
    let v = mixture_generate(p1, p2);
    (v, mixture_prob(p1, p2, v))
}

#[derive(Debug)]
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
    fn prob(&self, direction: Vec3) -> f64 {
        let cosine = direction.normalize().dot(self.uvw.w);
        (cosine / PI).max(0.0)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local(Vec3::random_cosine_direction())
    }
}

#[derive(Debug)]
pub struct UniformPDF;

impl PDF for UniformPDF {
    fn prob(&self, _direction: Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }
    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

#[derive(Debug, Default)]
pub struct ShapePDF {
    pdfs: Vec<Box<dyn ShapePDFProvider>>, // shouldn't be empty
    origin: Vec3,
}

impl ShapePDF {
    pub fn push<T: ShapePDFProvider + 'static>(&mut self, pdf: T) {
        self.pdfs.push(Box::new(pdf));
    }

    pub fn set_origin(&mut self, origin: Vec3) {
        self.origin = origin;
    }

    pub fn empty(&self) -> bool {
        // special check for empty
        self.pdfs.is_empty()
    }
}

impl PDF for ShapePDF {
    fn prob(&self, direction: Vec3) -> f64 {
        let weight = 1.0 / self.pdfs.len() as f64;
        let mut sum = 0.0;
        for pdf in &self.pdfs {
            sum += weight * pdf.prob(self.origin, direction);
        }
        sum
    }

    fn generate(&self) -> Vec3 {
        self.pdfs
            .choose(&mut rand::thread_rng())
            .unwrap()
            .generate(self.origin)
    }
}
