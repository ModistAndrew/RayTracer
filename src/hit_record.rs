use crate::color::Color;
use crate::hit_record::Scatter::{Absorb, ScatterPDF, ScatterRay};
use crate::interval::Interval;
use crate::pdf::{ShapePDF, PDF};
use crate::ray::Ray;
use crate::texture::{Atlas, UV};
use crate::vec3::Vec3;

pub enum Scatter {
    Absorb,
    ScatterPDF(Box<dyn PDF>),
    ScatterRay(Ray),
}

impl Scatter {
    pub fn pdf(&self) -> &dyn PDF {
        match *self {
            ScatterPDF(ref pdf) => pdf.as_ref(),
            _ => panic!("Scatter::pdf() called on non-PDF scatter"),
        }
    }

    pub fn move_pdf(self) -> Box<dyn PDF> {
        match self {
            ScatterPDF(pdf) => pdf,
            _ => panic!("Scatter::pdf() called on non-PDF scatter"),
        }
    }

    pub fn ray(&self) -> &Ray {
        match *self {
            ScatterRay(ref ray) => ray,
            _ => panic!("Scatter::ray() called on non-Ray scatter"),
        }
    }

    pub fn move_ray(self) -> Ray {
        match self {
            ScatterRay(ray) => ray,
            _ => panic!("Scatter::ray() called on non-Ray scatter"),
        }
    }
}

pub struct HitInfo {
    pub t: f64,
    pub position: Vec3,   // the hit position
    pub normal: Vec3,     // always normalized and points opposite to the ray
    pub front_face: bool, // whether outside the object
    pub uv: UV,
    pub emission: Color,
    pub attenuation: Color,
    pub scatter: Scatter,
}

pub struct HitRecord {
    ray: Ray,           // the original ray
    interval: Interval, // mutable
    hit_info: Option<HitInfo>,
}

impl HitRecord {
    // outward_normal should be normalized
    pub fn new(ray: Ray) -> Self {
        Self {
            ray,
            interval: Interval::POSITIVE,
            hit_info: None,
        }
    }

    fn generate_hit_info(&self, t: f64, outward_normal: Vec3, uv: UV) -> HitInfo {
        let position = self.ray.at(t);
        let front_face = self.ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitInfo {
            t,
            position,
            normal,
            front_face,
            uv,
            emission: Color::BLACK,
            attenuation: Color::WHITE,
            scatter: Absorb,
        }
    }

    // if normal and uv is not available, use this
    pub fn set_hit_arbitrary(&mut self, t: f64) {
        self.hit_info = Some(self.generate_hit_info(t, Vec3::default(), UV::default()));
        self.interval.limit_max(t);
    }

    // set hit info and update the interval. may fail if the predicate is false
    pub fn set_hit(&mut self, t: f64, outward_normal: Vec3, uv: UV, atlas: &Atlas) -> bool {
        let hit_info = self.generate_hit_info(t, outward_normal, uv);
        atlas.should_render(&hit_info) && {
            self.hit_info = Some(hit_info);
            self.interval.limit_max(t);
            true
        }
    }

    pub fn set_scatter_ray(&mut self, direction: Vec3) {
        self.get_hit_mut().scatter =
            ScatterRay(self.ray.new_ray(self.get_hit().position, direction))
    }

    pub fn set_scatter_pdf<T: PDF + 'static>(&mut self, pdf: T) {
        self.get_hit_mut().scatter = ScatterPDF(Box::new(pdf))
    }

    pub fn set_scatter_absorb(&mut self) {
        self.get_hit_mut().scatter = Absorb
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

    pub fn move_hit(self) -> HitInfo {
        self.hit_info.unwrap()
    }

    // generate a new ray from the shape pdf mixed with the scatter pdf
    // return (new_ray, prob_of_mixture_pdf, prob_of_scatter_pdf)
    // if shape pdf is empty, use scatter pdf only
    pub fn generate_scatter(&self, light_pdf: &ShapePDF) -> (Ray, f64, f64) {
        let scatter_pdf = self.get_hit().scatter.pdf();
        let origin = self.get_hit().position;
        let v = if light_pdf.empty() || rand::random::<f64>() < 0.5 {
            scatter_pdf.generate()
        } else {
            light_pdf.generate(origin)
        };
        let value = if light_pdf.empty() {
            scatter_pdf.prob(v)
        } else {
            0.5 * scatter_pdf.prob(v) + 0.5 * light_pdf.prob(v, origin)
        };
        (
            self.ray.new_ray(self.get_hit().position, v),
            value,
            scatter_pdf.prob(v),
        )
    }

    pub fn get_ray(&self) -> &Ray {
        &self.ray
    }

    pub fn get_ray_mut(&mut self) -> &mut Ray {
        // just for decoration. should restore the ray
        &mut self.ray
    }

    pub fn get_interval(&self) -> Interval {
        self.interval
    }

    pub fn set_interval(&mut self, interval: Interval) {
        // you may call this for restoring the interval
        self.interval = interval;
    }
}
