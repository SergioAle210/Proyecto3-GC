use crate::color::Color;
use nalgebra_glm::Vec2;

// Pueden haber otros campos en la estructura Fragment, pero estos son los mínimos requeridos.
pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
        }
    }
}