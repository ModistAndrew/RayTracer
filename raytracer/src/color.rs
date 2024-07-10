use crate::interval::Interval;
use image::Rgb;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
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

    pub fn blend(self, other: Color, mode: BlendMode) -> Color {
        let r = mode.apply(self.r, other.r);
        let g = mode.apply(self.g, other.g);
        let b = mode.apply(self.b, other.b);
        Self::new(r, g, b)
    }

    pub fn blend_assign(&mut self, other: Color, mode: BlendMode) {
        self.r = mode.apply(self.r, other.r);
        self.g = mode.apply(self.g, other.g);
        self.b = mode.apply(self.b, other.b);
    }

    pub fn lighten(self, factor: f64) -> Color {
        Self::new(self.r * factor, self.g * factor, self.b * factor)
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            linear_component.sqrt()
        } else {
            0.0
        }
    }

    fn gamma_to_linear(gamma_component: f64) -> f64 {
        gamma_component * gamma_component
    }

    pub fn random(min: f64, max: f64) -> Color {
        let mut rng = rand::thread_rng();
        Self::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    // abandon nan and inf
    pub fn fix(self) -> Color {
        let r = if self.r.is_normal() { self.r } else { 0.0 };
        let g = if self.g.is_normal() { self.g } else { 0.0 };
        let b = if self.b.is_normal() { self.b } else { 0.0 };
        Self::new(r, g, b)
    }
}

impl From<Color> for Rgb<u8> {
    // Convert Color to Rgb<u8> by gamma-correcting the color components.
    // Note that the color components would be clamped to [0, 1] before conversion.
    fn from(color: Color) -> Self {
        let r_byte = (Color::linear_to_gamma(Interval::UNIT.clamp(color.r)) * 256.0) as u8;
        let g_byte = (Color::linear_to_gamma(Interval::UNIT.clamp(color.g)) * 256.0) as u8;
        let b_byte = (Color::linear_to_gamma(Interval::UNIT.clamp(color.b)) * 256.0) as u8;
        Rgb([r_byte, g_byte, b_byte])
    }
}

impl From<Rgb<u8>> for Color {
    fn from(val: Rgb<u8>) -> Self {
        let r = Color::gamma_to_linear(val.0[0] as f64 / 256.0);
        let g = Color::gamma_to_linear(val.0[1] as f64 / 256.0);
        let b = Color::gamma_to_linear(val.0[2] as f64 / 256.0);
        Color::new(r, g, b)
    }
}
