use crate::aabb::AABB;
use crate::bvh::HittableTree;
use crate::color::Color;
use crate::hittable::Scatter::{Absorb, ScatterPDF, ScatterRay};
use crate::material::Material;
use crate::pdf::{ShapePDF, PDF};
use crate::ray::Ray;
use crate::shape::{Shape, ShapePDFProvider};
use crate::texture::UV;
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
            emission: Color::BLACK,
            attenuation: Color::WHITE,
            scatter: Absorb,
        });
        self.ray.interval.limit_max(t)
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
    pub fn generate_scatter(&self, shape_pdf: &ShapePDF) -> (Ray, f64, f64) {
        let scatter_pdf = self.get_hit().scatter.pdf();
        let origin = self.get_hit().position;
        let v = if shape_pdf.empty() {
            scatter_pdf.generate()
        } else {
            shape_pdf.generate(origin)
        };
        let value = if shape_pdf.empty() {
            scatter_pdf.prob(v)
        } else {
            shape_pdf.prob(v, origin)
        };
        (
            self.ray.new_ray(self.get_hit().position, v),
            value,
            scatter_pdf.prob(v),
        )
    }
}

pub trait Hittable: Sync + Send {
    // hit_record.ray is the original ray.
    // if hit, update hit_record.hit and scatter and return true
    fn hit(&self, hit_record: &mut HitRecord) -> bool;

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

pub struct World {
    pub objects: HittableTree,
    pub pdf: ShapePDF,
    pub background: Color,
}

#[derive(Default)]
pub struct WorldBuilder {
    objects: Vec<Box<dyn Hittable>>,
    pdf: ShapePDF,
    background: Option<Color>,
}

impl WorldBuilder {
    pub fn add_object<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn add_light<T: ShapePDFProvider + 'static>(&mut self, shape: T) {
        self.pdf.push(shape);
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = Some(color);
    }

    pub fn build(self) -> World {
        World {
            objects: HittableTree::new(self.objects),
            pdf: self.pdf,
            background: self.background.unwrap_or(Color::BLACK),
        }
    }
}
