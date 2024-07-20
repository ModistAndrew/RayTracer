use std::ops::{Add, Mul, Sub};

use crate::canvas::Canvas;
use crate::color::Color;
use crate::hit_record::HitInfo;
use crate::noise::Noise;

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
    transparency: Option<Box<dyn Texture>>,
    attenuation: Option<Box<dyn Texture>>,
    emission: Option<Box<dyn Texture>>,
}

impl Atlas {
    pub fn set_transparency<T: Texture + 'static>(mut self, texture: T) -> Self {
        self.transparency = Some(Box::new(texture));
        self
    }

    pub fn set_attenuation<T: Texture + 'static>(mut self, texture: T) -> Self {
        self.attenuation = Some(Box::new(texture));
        self
    }

    pub fn set_emission<T: Texture + 'static>(mut self, texture: T) -> Self {
        self.emission = Some(Box::new(texture));
        self
    }

    pub fn should_render(&self, hit_info: &HitInfo) -> bool {
        self.transparency
            .as_ref()
            .map_or(true, |t| t.value(hit_info).r < 0.5)
    }

    pub fn get_attenuation(&self, hit_info: &HitInfo) -> Color {
        self.attenuation
            .as_ref()
            .map_or(Color::WHITE, |t| t.value(hit_info))
    }

    pub fn get_emission(&self, hit_info: &HitInfo) -> Color {
        self.emission
            .as_ref()
            .map_or(Color::BLACK, |t| t.value(hit_info))
    }
}
