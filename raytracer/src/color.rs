use image::Rgb;
use rand::Rng;

use crate::interval::Interval;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub enum BlendMode {
    Add,
    Mul,
}

impl BlendMode {
    pub fn apply(&self, a: f64, b: f64) -> f64 {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

impl Color {
    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    pub const WHITE: Self = Self {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    };
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn blend(&self, other: Color, mode: BlendMode) -> Color {
        let r = mode.apply(self.r, other.r);
        let g = mode.apply(self.g, other.g);
        let b = mode.apply(self.b, other.b);
        Self::new(r, g, b)
    }

    pub fn mix(colors: &[Color]) -> Self {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        for color in colors {
            r += color.r;
            g += color.g;
            b += color.b;
        }
        let n = colors.len() as f64;
        Self::new(r / n, g / n, b / n)
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    fn r_gamma(&self) -> f64 {
        Self::linear_to_gamma(self.r)
    }

    fn g_gamma(&self) -> f64 {
        Self::linear_to_gamma(self.g)
    }

    fn b_gamma(&self) -> f64 {
        Self::linear_to_gamma(self.b)
    }

    pub fn is_black(&self) -> bool {
        const S: f64 = 1e-8;
        self.r < S && self.g < S && self.b < S
    }

    pub fn random(min: f64, max: f64) -> Color {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Self {
        let r_byte = (Interval::UNIT.clamp(color.r_gamma()) * 256.0) as u8;
        let g_byte = (Interval::UNIT.clamp(color.g_gamma()) * 256.0) as u8;
        let b_byte = (Interval::UNIT.clamp(color.b_gamma()) * 256.0) as u8;
        Rgb([r_byte, g_byte, b_byte])
    }
}
