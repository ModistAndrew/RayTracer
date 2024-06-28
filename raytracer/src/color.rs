use image::Rgb;

use crate::interval::Interval;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
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

    pub fn darken(&self, factor: f64) -> Self {
        Self::new(self.r * factor, self.g * factor, self.b * factor)
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
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Self {
        let r_byte = (Interval::UNIT.clamp(color.r_gamma()) * 256.0) as u8;
        let g_byte = (Interval::UNIT.clamp(color.g_gamma()) * 256.0) as u8;
        let b_byte = (Interval::UNIT.clamp(color.b_gamma()) * 256.0) as u8;
        Rgb([r_byte, g_byte, b_byte])
    }
}
