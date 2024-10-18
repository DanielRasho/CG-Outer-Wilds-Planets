use super::fragment::{self, Fragment};
use super::entity::vertex::Vertex;

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let x0 = a.transformed_position.x as i32;
    let y0 = a.transformed_position.y as i32;
    let x1 = b.transformed_position.x as i32;
    let y1 = b.transformed_position.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x0;
    let mut y = y0;

    // Bresenham's algorithm loop
    while x != x1 || y != y1 {
        // Interpolate depth and color between a and b
        let t = ((x - x0) as f32 / (x1 - x0) as f32).clamp(0.0, 1.0); // Interpolation factor
        let color = a.color.lerp(&b.color, t); // Assuming you have a lerp function for Color
        let depth = a.transformed_position.z * (1.0 - t) + b.transformed_position.z * t;

        // Create a fragment at this point
        fragments.push(Fragment::new(x as f32, y as f32, color, depth));

        // Bresenham's decision
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    fragments
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v1, v3));
    
    fragments
}