use console::style;
use image::{ImageBuffer, RgbImage};
use std::fs;
use std::path::PathBuf;

use crate::color::Color;
use crate::texture::UV;

pub struct Canvas {
    image: RgbImage,
}

impl Canvas {
    pub fn from_path(path: &str) -> Self {
        let image = image::open(path)
            .expect("Cannot open the image file")
            .to_rgb8();
        Self { image }
    }

    pub fn empty(width: u32, height: u32) -> Self {
        Self {
            image: ImageBuffer::new(width, height),
        }
    }

    pub fn read(&self, x: u32, y: u32) -> Color {
        let pixel = self.image.get_pixel(x, y);
        (*pixel).into()
    }

    pub fn read_uv(&self, uv: UV) -> Color {
        self.read(
            (uv.u * self.width() as f64) as u32,
            ((1.0 - uv.v) * self.height() as f64) as u32, // flip y axis
        )
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

    fn rename_if_exists(mut path: PathBuf) -> PathBuf {
        while path.exists() {
            println!(
                "File \"{}\" already exists. Renaming...",
                style(path.to_str().unwrap()).yellow()
            );
            let extension = path
                .extension()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("");
            let filename = path
                .file_stem()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("");
            path = path.with_file_name(format!("{}_{}.{}", filename, 1, extension));
        }
        path
    }

    pub fn save(&self, path: &str) {
        let path = std::path::Path::new(path);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix).expect("Cannot create all the parents");
        let path_buf = Self::rename_if_exists(path.to_path_buf());
        let path = path_buf.as_path();
        println!(
            "Output image as \"{}\"",
            style(path.to_str().unwrap()).green()
        );
        self.image
            .save(path)
            .expect("Cannot save the image to the file");
    }
}
