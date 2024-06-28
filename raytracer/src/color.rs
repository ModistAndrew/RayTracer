use crate::interval::Interval;
use image::Rgb;

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

    pub fn blend(colors: &[Color]) -> Self {
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
}

impl From<Color> for Rgb<u8> {
    fn from(color: Color) -> Self {
        let r_byte = (Interval::UNIT.clamp(color.r) * 256.0) as u8;
        let g_byte = (Interval::UNIT.clamp(color.g) * 256.0) as u8;
        let b_byte = (Interval::UNIT.clamp(color.b) * 256.0) as u8;
        Rgb([r_byte, g_byte, b_byte])
    }
}
