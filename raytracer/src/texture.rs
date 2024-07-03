use crate::color::Color;
use crate::hittable::HitRecord;

pub trait Texture: Sync + Send {
    fn value(&self, hit_record: &HitRecord) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _hit_record: &HitRecord) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    inv_scale: f64,
}

impl CheckerTexture {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>, scale: f64) -> Self {
        Self {
            odd,
            even,
            inv_scale: 1.0 / scale,
        }
    }

    pub fn from_color(color1: Color, color2: Color, scale: f64) -> Self {
        Self::new(
            Box::new(SolidColor::new(color1)),
            Box::new(SolidColor::new(color2)),
            scale,
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, hit_record: &HitRecord) -> Color {
        let p = hit_record.get_hit().position;
        let x = (p.x * self.inv_scale).floor() as i32;
        let y = (p.y * self.inv_scale).floor() as i32;
        let z = (p.z * self.inv_scale).floor() as i32;
        if (x + y + z) % 2 == 0 {
            self.even.value(hit_record)
        } else {
            self.odd.value(hit_record)
        }
    }
}
