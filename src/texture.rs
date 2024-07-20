use crate::canvas::Canvas;
use crate::color::Color;
use crate::hit_record::{HitInfo, HitRecord};
use crate::material::Material;
use crate::noise::Noise;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Default, Debug)]
pub struct UV {
    pub u: f64,
    pub v: f64,
}

impl UV {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
}

impl Add for UV {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            u: self.u + other.u,
            v: self.v + other.v,
        }
    }
}

impl Sub for UV {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            u: self.u - other.u,
            v: self.v - other.v,
        }
    }
}

impl Mul<f64> for UV {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            u: self.u * rhs,
            v: self.v * rhs,
        }
    }
}

pub trait Texture: Sync + Send {
    fn value(&self, hit_info: &HitInfo) -> Color;
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
    fn value(&self, _hit_info: &HitInfo) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    even: Color,
    odd: Color,
    inv_scale: f64,
}

impl CheckerTexture {
    pub fn new(even: Color, odd: Color, scale: f64) -> Self {
        Self {
            even,
            odd,
            inv_scale: 1.0 / scale,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, hit_info: &HitInfo) -> Color {
        let p = hit_info.position;
        let x = (p.x * self.inv_scale).floor() as i32;
        let y = (p.y * self.inv_scale).floor() as i32;
        let z = (p.z * self.inv_scale).floor() as i32;
        if (x + y + z) % 2 == 0 {
            self.even
        } else {
            self.odd
        }
    }
}

pub struct ImageTexture {
    image: Canvas,
}

impl ImageTexture {
    pub fn new(path: &str) -> Self {
        Self {
            image: Canvas::from_path(path),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, hit_info: &HitInfo) -> Color {
        self.image.read_uv(hit_info.uv)
    }
}

pub struct NoiseTexture {
    noise: Noise,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(noise: Noise, scale: f64) -> Self {
        Self { noise, scale }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, hit_info: &HitInfo) -> Color {
        let p = hit_info.position;
        Color::gray(0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(p, 7)).sin()))
    }
}

// atlas for setting and reading textures. for shape and material decoration
#[derive(Default)]
pub struct Atlas {
    pub transparency: Option<Box<dyn Texture>>,
    pub attenuation: Option<Box<dyn Texture>>,
    pub emission: Option<Box<dyn Texture>>,
}

impl Atlas {
    pub fn should_render(&self, hit_info: &HitInfo) -> bool {
        self.transparency
            .as_ref()
            .map_or(true, |t| t.value(hit_info).r < 0.5)
    }

    pub fn decorate(&self, hit_record: &mut HitRecord) {
        if let Some(t) = self.attenuation.as_ref() {
            hit_record.get_hit_mut().attenuation = t.value(hit_record.get_hit());
        }
        if hit_record.get_hit().front_face {
            if let Some(t) = self.emission.as_ref() {
                hit_record.get_hit_mut().emission = t.value(hit_record.get_hit());
            }
        }
    }
}

// a simple wrapper for textured material.
// as atlas is mostly used for material, it is better to use this wrapper.
#[derive(Default)]
pub struct TexturedMaterial {
    material: Option<Box<dyn Material>>,
    pub atlas: Atlas,
}

impl TexturedMaterial {
    pub fn set_material<T: Material + 'static>(mut self, material: T) -> Self {
        self.material = Some(Box::new(material));
        self
    }

    pub fn set_transparency<T: Texture + 'static>(mut self, texture: T) -> Self {
        self.atlas.transparency = Some(Box::new(texture));
        self
    }

    pub fn set_attenuation<T: Texture + 'static>(mut self, texture: T) -> Self {
        self.atlas.attenuation = Some(Box::new(texture));
        self
    }

    pub fn set_emission<T: Texture + 'static>(mut self, texture: T) -> Self {
        self.atlas.emission = Some(Box::new(texture));
        self
    }
}

impl Material for TexturedMaterial {
    fn scatter(&self, hit_record: &mut HitRecord) {
        if let Some(m) = self.material.as_ref() {
            m.scatter(hit_record);
            self.atlas.decorate(hit_record);
        }
    }
}
