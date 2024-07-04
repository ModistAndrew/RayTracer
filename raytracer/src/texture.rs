use crate::canvas::Canvas;
use crate::color::{BlendMode, Color};
use crate::hittable::HitRecord;
use crate::material::Material;
use crate::perlin::Perlin;

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

pub trait Texture {
    type Inner: Material;
    fn value(&self, hit_record: &HitRecord) -> Color;
    fn get_inner(&self) -> &Self::Inner;
}

impl<T: Texture> Material for T {
    fn scatter(&self, hit_record: &mut HitRecord) {
        self.get_inner().scatter(hit_record);
        let color = self.value(hit_record);
        hit_record
            .get_scatter_mut()
            .color
            .blend_assign(color, BlendMode::Mul);
    }
}

pub struct SolidColor<T: Material> {
    color: Color,
    inner: T,
}

impl<T: Material> SolidColor<T> {
    pub fn new(color: Color, inner: T) -> Self {
        Self { color, inner }
    }
}

impl<T: Material> Texture for SolidColor<T> {
    type Inner = T;
    fn value(&self, _hit_record: &HitRecord) -> Color {
        self.color
    }
    fn get_inner(&self) -> &T {
        &self.inner
    }
}

pub struct CheckerTexture<T: Material> {
    even: Color,
    odd: Color,
    inv_scale: f64,
    inner: T,
}

impl<T: Material> CheckerTexture<T> {
    pub fn new(even: Color, odd: Color, scale: f64, inner: T) -> Self {
        Self {
            even,
            odd,
            inv_scale: 1.0 / scale,
            inner,
        }
    }
}

impl<T: Material> Texture for CheckerTexture<T> {
    type Inner = T;
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
    fn get_inner(&self) -> &T {
        &self.inner
    }
}

pub struct ImageTexture<T: Material> {
    image: Canvas,
    inner: T,
}

impl<T: Material> ImageTexture<T> {
    pub fn new(path: &str, inner: T) -> Self {
        Self {
            image: Canvas::from_path(path),
            inner,
        }
    }
}

impl<T: Material> Texture for ImageTexture<T> {
    type Inner = T;
    fn value(&self, hit_record: &HitRecord) -> Color {
        self.image.read_uv(hit_record.get_hit().uv)
    }
    fn get_inner(&self) -> &T {
        &self.inner
    }
}

pub struct NoiseTexture<T: Material> {
    perlin: Perlin,
    scale: f64,
    inner: T,
}

impl<T: Material> NoiseTexture<T> {
    pub fn new(perlin: Perlin, scale: f64, inner: T) -> Self {
        Self {
            perlin,
            scale,
            inner,
        }
    }
}

impl<T: Material> Texture for NoiseTexture<T> {
    type Inner = T;
    fn value(&self, hit_record: &HitRecord) -> Color {
        let p = hit_record.get_hit().position;
        Color::WHITE.lighten(0.5 * (1.0 + self.perlin.noise(p * self.scale)))
    }
    fn get_inner(&self) -> &T {
        &self.inner
    }
}
