use crate::{color::Color, texture::Texture};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub zbuffer: Vec<f32>, // Asegúrate de incluir el Z-buffer
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![Color::new(0, 0, 0); width * height];
        let zbuffer = vec![f32::INFINITY; width * height]; // Z-buffer inicializado en infinito
        Self {
            buffer,
            zbuffer, // Inicializa el Z-buffer
            width,
            height,
            background_color: Color::new(0, 0, 0),
            current_color: Color::new(255, 255, 255),
        }
    }

    pub fn point(&mut self, x: usize, y: usize, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if depth < self.zbuffer[index] {
                self.buffer[index] = self.current_color;
                self.zbuffer[index] = depth; // Actualiza el Z-buffer
            }
        }
    }

    pub fn point_with_color(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.background_color = color.into();
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = Color::from_hex(color);
    }

    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) {
        for i in 0..width {
            for j in 0..height {
                self.point_with_color(x + i, y + j, color.clone());
            }
        }
    }

    pub fn clear(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.background_color.clone();
        }
        for depth in &mut self.zbuffer {
            *depth = f32::INFINITY; // Restablecer el Z-buffer
        }
    }

    pub fn is_point_set(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] == Color::from_hex(0xFFFFFF)
        } else {
            false
        }
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|color| color.to_hex()).collect()
    }

    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let mut x0 = x0 as isize;
        let mut y0 = y0 as isize;
        let x1 = x1 as isize;
        let y1 = y1 as isize;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = if dx > dy { dx } else { -dy } / 2;

        loop {
            if x0 >= 0 && x0 < self.width as isize && y0 >= 0 && y0 < self.height as isize {
                self.point_with_color(x0 as usize, y0 as usize, Color::from_hex(color));
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if e2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_skybox(&mut self, texture: &Texture) {
        for y in 0..self.height {
            for x in 0..self.width {
                // Mapear las coordenadas del framebuffer a las coordenadas de la textura
                let u = x as f32 / self.width as f32;
                let v = y as f32 / self.height as f32;

                // Obtener el color de la textura
                let color = texture.sample(u, v);

                // Dibujar el color en el buffer
                self.buffer[y * self.width + x] = color;
            }
        }
    }
}
