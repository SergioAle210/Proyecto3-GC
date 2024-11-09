use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor to initialize the color using r, g, b values as u8
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    fn clamp(value: i32) -> u8 {
        if value < 0 {
            0
        } else if value > 255 {
            255
        } else {
            value as u8
        }
    }

    pub fn black() -> Self {
        Color::new(0, 0, 0)
    }

    pub fn from_hex(hex: u32) -> Color {
        Color {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    // New constructor to initialize the color using r, g, b values as f32 (0.0 to 1.0)
    pub fn from_float(r: f32, g: f32, b: f32) -> Self {
        Color {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    // Linear interpolation between two colors
    pub fn lerp(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Color {
            r: (self.r as f32 + (other.r as f32 - self.r as f32) * t).round() as u8,
            g: (self.g as f32 + (other.g as f32 - self.g as f32) * t).round() as u8,
            b: (self.b as f32 + (other.b as f32 - self.b as f32) * t).round() as u8,
        }
    }

    pub fn is_black(&self) -> bool {
        self.r == 0 && self.g == 0 && self.b == 0
    }

    // New blend mode methods
    pub fn blend_normal(&self, blend: &Color) -> Color {
        if blend.is_black() {
            *self
        } else {
            *blend
        }
    }

    pub fn blend_multiply(&self, blend: &Color) -> Color {
        Color::new(
            ((self.r as f32 * blend.r as f32) / 255.0) as u8,
            ((self.g as f32 * blend.g as f32) / 255.0) as u8,
            ((self.b as f32 * blend.b as f32) / 255.0) as u8,
        )
    }

    pub fn blend_add(&self, blend: &Color) -> Color {
        Color::new(
            (self.r as u16 + blend.r as u16).min(255) as u8,
            (self.g as u16 + blend.g as u16).min(255) as u8,
            (self.b as u16 + blend.b as u16).min(255) as u8,
        )
    }

    pub fn blend_subtract(&self, blend: &Color) -> Color {
        let r = (self.r as i16 - blend.r as i16).max(0).min(255) as u8;
        let g = (self.g as i16 - blend.g as i16).max(0).min(255) as u8;
        let b = (self.b as i16 - blend.b as i16).max(0).min(255) as u8;

        Color::new(r, g, b)
    }

    pub fn blend_screen(&self, blend: &Color) -> Color {
        Color::new(
            255 - ((255 - self.r as u16) * (255 - blend.r as u16) / 255) as u8,
            255 - ((255 - self.g as u16) * (255 - blend.g as u16) / 255) as u8,
            255 - ((255 - self.b as u16) * (255 - blend.b as u16) / 255) as u8,
        )
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        Color::from_hex(hex)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            r: ((self.r as f32 * scalar).clamp(0.0, 255.0)) as u8,
            g: ((self.g as f32 * scalar).clamp(0.0, 255.0)) as u8,
            b: ((self.b as f32 * scalar).clamp(0.0, 255.0)) as u8,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}
