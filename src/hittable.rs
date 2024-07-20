use crate::aabb::Aabb;
use crate::bvh::{HittableTree, HittableList};
use crate::color::Color;
use crate::hit_record::HitRecord;
use crate::material::Material;
use crate::pdf::ShapePDF;
use crate::shape::{Shape, ShapePDFProvider};
use crate::texture::Atlas;

pub trait Hittable: Sync + Send {
    // similar to shape but needn't be supplied with atlas
    fn hit(&self, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}

pub struct Object<S: Shape, M: Material> {
    pub shape: S,
    pub material: M,
    pub atlas: Atlas,
}

impl<S: Shape, M: Material> Hittable for Object<S, M> {
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        self.shape.hit(hit_record, &self.atlas) && {
            self.material.scatter(hit_record, &self.atlas);
            true
        }
    }
    fn bounding_box(&self) -> Aabb {
        self.shape.bounding_box()
    }
}

pub struct World {
    pub objects: HittableTree,
    pub light_pdf: ShapePDF,
    pub background: Color,
}

#[derive(Default)]
pub struct WorldBuilder {
    objects: HittableList,
    light_pdf: ShapePDF,
    background: Option<Color>,
}

impl WorldBuilder {
    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(object);
    }

    pub fn add_object<S: Shape + 'static, M: Material + 'static>(
        &mut self,
        shape: S,
        material: M,
        atlas: Atlas,
    ) {
        self.objects.push(Object {
            shape,
            material,
            atlas,
        });
    }

    pub fn add_light<T: ShapePDFProvider + 'static>(&mut self, shape: T) {
        self.light_pdf.push(shape);
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = Some(color);
    }

    pub fn build(self) -> World {
        World {
            objects: self.objects.tree(),
            light_pdf: self.light_pdf,
            background: self.background.unwrap_or(Color::BLACK),
        }
    }
}
