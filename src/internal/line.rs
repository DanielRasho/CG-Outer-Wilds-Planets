use nalgebra_glm::{dot, Vec3};

use super::entity::color::Color;
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

pub fn triangle_wireframe(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    fragments.extend(line(v1, v2));
    fragments.extend(line(v2, v3));
    fragments.extend(line(v1, v3));
    
    fragments
}

pub fn triangle_flat_shade(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();
     
     let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);
     
     let light_dir = Vec3::new(0.0, -1.0, -1.0);
     
     let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);
     
     let triangle_area =edge_function(&a, &b, &c);
     
       // Iterate over each pixel in the bounding box
  for y in min_y..=max_y {
    for x in min_x..=max_x {
      let point = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);

      // Calculate barycentric coordinates
      let (w1, w2, w3) = barycentric_coordinates(&point, &a, &b, &c, triangle_area);

      // Check if the point is inside the triangle
      if w1 >= 0.0 && w1 <= 1.0 && 
         w2 >= 0.0 && w2 <= 1.0 &&
         w3 >= 0.0 && w3 <= 1.0 {
        // Interpolate normal
        // let normal = v1.transformed_normal * w1 + v2.transformed_normal * w2 + v3.transformed_normal * w3;
        let normal = v1.normal * w1 + v2.normal * w2 + v3.normal * w3;
        let normal = normal.normalize();

        // Calculate lighting intensity
        let intensity = dot(&normal, &light_dir).max(0.0);

        // Create a gray color and apply lighting
        let base_color = Color::new(100, 100, 100); // Medium gray
        let lit_color = base_color * intensity;

        // Interpolate depth
        let depth = a.z * w1 + b.z * w2 + c.z * w3;

        fragments.push(Fragment::new(point.x as f32, point.y as f32, lit_color, depth));
      }
    }
  }

    fragments
}

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
    let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
    let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
    let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
    let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;
    
    (min_x, min_y, max_x, max_y)
}



fn barycentric_coordinates(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32 ){
    let w1 = edge_function(b, c, p) / area;
    let w2 = edge_function(c, a, p) / area;
    let w3 = edge_function(a, b, p) / area;

    
    (w1, w2, w3)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}