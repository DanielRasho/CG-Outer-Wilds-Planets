// fragment.rs

use nalgebra_glm::{Vec2, Vec3};
use super::color::Color;

pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
    pub intensity: f32,
    pub vertex_position: Vec3,
}

impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32, intensity: f32, vertex_position: Vec3) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
            intensity,
            vertex_position
        }
    }
}