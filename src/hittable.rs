use crate::aabb::Aabb;
use crate::bvh_wrapper::{AabbProvider, BoundedTree, BoundedTreeBuilder};
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

impl AabbProvider for Box<dyn Hittable> {
    fn aabb(&self) -> Aabb {
        self.bounding_box().pad()
    }
}

pub type HittableTree = BoundedTree<Box<dyn Hittable>>;
pub type HittableTreeBuilder = BoundedTreeBuilder<Box<dyn Hittable>>;

impl Hittable for HittableTree {
    // implement Shape for ShapeTree. you can use ShapeTree as a Shape.
    fn hit(&self, hit_record: &mut HitRecord) -> bool {
        let mut hit = false;
        self.traverse(&hit_record.get_ray().ray3())
            .into_iter()
            .for_each(|shape| {
                hit |= shape.inner.hit(hit_record);
            });
        hit
    }

    fn bounding_box(&self) -> Aabb {
        self.aabb()
    }
}

impl HittableTreeBuilder {
    pub fn add_hittable<T: Hittable + 'static>(&mut self, shape: T) {
        self.add(Box::new(shape));
    }
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
    objects: HittableTreeBuilder,
    light_pdf: ShapePDF,
    background: Option<Color>,
}

impl WorldBuilder {
    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.add_hittable(object);
    }

    pub fn add_object<S: Shape + 'static, M: Material + 'static>(
        &mut self,
        shape: S,
        material: M,
        atlas: Atlas,
    ) {
        self.objects.add_hittable(Object {
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
            objects: self.objects.build(),
            light_pdf: self.light_pdf,
            background: self.background.unwrap_or(Color::BLACK),
        }
    }
}
