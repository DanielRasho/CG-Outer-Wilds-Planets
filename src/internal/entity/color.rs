use std::ops::Add;
use std::ops::Mul;
use std::fmt;

#[derive (Debug, Copy, Clone, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8; // fixed this line
        Color { r, g, b }
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn black() -> Color {
        Color {r: 0, g: 0, b: 0}
    }
    
    // Linear interpolation for Color
    pub fn lerp(&self, other: &Color, t: f32) -> Color {
        let r = self.r as f32 + (other.r as f32 - self.r as f32) * t;
        let g = self.g as f32 + (other.g as f32 - self.g as f32) * t;
        let b = self.b as f32 + (other.b as f32 - self.b as f32) * t;
        Color {
            r: r.clamp(0.0, 255.0) as u8,
            g: g.clamp(0.0, 255.0) as u8,
            b: b.clamp(0.0, 255.0) as u8,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = self.r.saturating_add(other.r);
        let g = self.g.saturating_add(other.g);
        let b = self.b.saturating_add(other.b);
        Color { r, g, b }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    
    fn mul(self, factor: f32) -> Color{
        let r = (self.r as f32 * factor).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * factor).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * factor).clamp(0.0, 255.0) as u8;
        Color { r, g, b }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}