use crate::color::Color;
use crate::material::{Material, Object};
use crate::pdf::ShapePDF;
use crate::shape::{Shape, ShapePDFProvider, ShapeTree, ShapeTreeBuilder};

pub struct World {
    pub objects: ShapeTree,
    pub light_pdf: ShapePDF,
    pub background: Color,
}

#[derive(Default)]
pub struct WorldBuilder {
    objects: ShapeTreeBuilder,
    light_pdf: ShapePDF,
    background: Option<Color>,
}

impl WorldBuilder {
    pub fn add<T: Shape + 'static>(&mut self, object: T) {
        self.objects.add(Box::new(object));
    }

    pub fn add_object<S: Shape + 'static, M: Material + 'static>(&mut self, shape: S, material: M) {
        self.add(Object::new(shape, material));
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
