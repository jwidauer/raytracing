use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::color::Color;
use crate::settings::ImageSettings;

#[derive(Debug, Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0., 0., 0.); width * height],
        }
    }

    pub fn from_settings(settings: ImageSettings) -> Self {
        Self::new(settings.image_width, settings.image_height)
    }

    pub fn write_ppm(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::new(File::create(filename)?);
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.width, self.height)?;
        writeln!(writer, "255")?;

        for j in (0..self.height).rev() {
            for i in 0..self.width {
                let index = j * self.width + i;
                let pixel = self.pixels[index];
                writeln!(writer, "{}", pixel)?;
            }
        }

        writer.flush()?;

        Ok(())
    }
}
