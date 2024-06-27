use crate::vec3d::Vec3d;
use image::Rgb;

#[derive(Debug)]
pub struct Color {
    rgb: Vec3d,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            rgb: Vec3d::new(r, g, b),
        }
    }

    pub fn r(&self) -> f64 {
        self.rgb.x
    }

    pub fn g(&self) -> f64 {
        self.rgb.y
    }

    pub fn b(&self) -> f64 {
        self.rgb.z
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Self {
        let r = (color.r().clamp(0.0, 1.0) * 255.0) as u8;
        let g = (color.g().clamp(0.0, 1.0) * 255.0) as u8;
        let b = (color.b().clamp(0.0, 1.0) * 255.0) as u8;
        Rgb([r, g, b])
    }
}
