use console::style;
use image::{ImageBuffer, RgbImage};

use crate::color::Color;

pub struct Canvas {
    image: RgbImage,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: ImageBuffer::new(width, height),
        }
    }

    pub fn write(&mut self, x: u32, y: u32, color: Color) {
        let pixel = self.image.get_pixel_mut(x, y);
        *pixel = color.into();
    }

    pub fn width(&self) -> u32 {
        self.image.width()
    }

    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn save(&self, path: &str) {
        let path = std::path::Path::new(path);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
        println!(
            "Output image as \"{}\"",
            style(path.to_str().unwrap()).yellow()
        );
        self.image
            .save(path)
            .unwrap_or_else(|e| eprintln!("Outputting image failed: {}", e));
    }
}
