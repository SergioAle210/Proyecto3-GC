extern crate image;

use crate::color::Color;
use image::{DynamicImage, GenericImageView};
use image::{ImageReader, RgbImage};

pub struct Texture {
    image: RgbImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb8();
        let width = img.width();
        let height = img.height();
        Texture {
            image: img,
            width,
            height,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let pixel = self.image.get_pixel(x as u32, y as u32);
        Color::new(pixel[0] as u8, pixel[1] as u8, pixel[2] as u8)
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        // Asegúrate de que u y v estén en el rango [0, 1]
        let u = u.clamp(0.0, 1.0);
        let v = v.clamp(0.0, 1.0);

        // Convertir u y v a coordenadas de píxel en la textura
        let x = (u * (self.width - 1) as f32).round() as u32;
        let y = (v * (self.height - 1) as f32).round() as u32;

        // Obtener el color del píxel desde el campo `image`
        let pixel = self.image.get_pixel(x, y);
        Color::new(pixel[0], pixel[1], pixel[2])
    }
}
