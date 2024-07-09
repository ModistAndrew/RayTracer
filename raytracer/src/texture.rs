use crate::canvas::Canvas;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::noise::Noise;

#[derive(Clone, Copy, Default)]
pub struct UV {
    pub u: f64,
    pub v: f64,
}

impl UV {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }
}

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
    fn value(&self, hit_record: &HitRecord) -> Color {
        let p = hit_record.get_hit().position;
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
    fn value(&self, hit_record: &HitRecord) -> Color {
        self.image.read_uv(hit_record.get_hit().uv)
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
    fn value(&self, hit_record: &HitRecord) -> Color {
        let p = hit_record.get_hit().position;
        Color::WHITE
            .lighten(0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turbulence(p, 7)).sin()))
    }
}

pub struct TexturedMaterial<T: Texture, M: Material> {
    texture: T,
    material: M,
}

impl<T: Texture, M: Material> TexturedMaterial<T, M> {
    pub fn new(texture: T, material: M) -> Self {
        Self { texture, material }
    }
}

impl<T: Texture, M: Material> Material for TexturedMaterial<T, M> {
    fn scatter(&self, hit_record: &mut HitRecord) -> bool {
        if !self.material.scatter(hit_record) {
            return false;
        }
        hit_record.get_scatter_mut().attenuation = self.texture.value(hit_record);
        true
    }
}

pub struct Emissive<T: Texture> {
    texture: T,
}

impl<T: Texture> Emissive<T> {
    pub fn new(texture: T) -> Self {
        Self { texture }
    }
}

impl<T: Texture> Material for Emissive<T> {
    fn scatter(&self, hit_record: &mut HitRecord) -> bool {
        hit_record.get_scatter_mut().emission = self.texture.value(hit_record);
        false
    }
}
