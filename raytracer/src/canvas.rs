use std::fs;
use std::path::PathBuf;
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

    fn rename_if_exists(mut path: PathBuf) -> PathBuf {
        let mut counter = 1;
        while path.exists() {
            println!(
                "File \"{}\" already exists. Renaming...",
                style(path.to_str().unwrap()).yellow()
            );
            let extension = path.extension().and_then(|os_str| os_str.to_str()).unwrap_or("");
            let filename = path.file_stem().and_then(|os_str| os_str.to_str()).unwrap_or("");
            path = path.with_file_name(format!("{}_{}.{}", filename, counter, extension));
            counter += 1;
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
