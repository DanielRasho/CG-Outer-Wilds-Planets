use once_cell::sync::Lazy;
use nalgebra_glm::{Vec2, Vec3, Vec4, Mat4};
use std::sync::Mutex;

use super::entity::vertex::Vertex;
use super::entity::fragment::Fragment;
use super::render::Uniforms;
use super::entity::color::Color;

use fastnoise_lite::{FastNoiseLite, NoiseType};

static NOISE_GENERATOR: Lazy<Mutex<FastNoiseLite>> = Lazy::new(|| {
  let mut noise = FastNoiseLite::new();
  noise.set_noise_type(Some(NoiseType::Cellular)); // Use Cellular noise for texture-like patterns
  noise.set_frequency(Some(10.0)); // Decrease frequency for bigger cells (larger scale)
  Mutex::new(noise) // Wrap the noise generator in a Mutex
});

pub fn vertex_shader(vertex: &Vertex, transformation_matrix: &Mat4, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );

  let clip_pos = transformation_matrix * position;

  // Perform perspective division
  let w = clip_pos.w;
  let ndc_position = Vec4::new(
    clip_pos.x / w,
    clip_pos.y / w,
    clip_pos.z / w,
    1.0);

  // Transform normal
  let screen_position = uniforms.viewport_matrix * ndc_position;
  let transformed_position = Vec3::new(screen_position.x, screen_position.y, screen_position.z);

  // Transform normal
  let vertex_normal = Vec4::new(vertex.normal.x, vertex.normal.y, vertex.normal.z, 1.0);
  let normal_matrix = uniforms.model_matrix
      .try_inverse()
      .unwrap_or(Mat4::identity())
      .transpose();
  let transformed_normal = normal_matrix * vertex_normal;
  let w = transformed_normal.w;
  let transformed_normal = Vec3::new(
      transformed_normal.x / w,
      transformed_normal.y / w,
      transformed_normal.z / w,
  );


  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    frustrum_position: ndc_position,
    transformed_position,
    transformed_normal: transformed_normal,
  }
}

pub fn simple_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  fragment.color * fragment.intensity
}

pub fn sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
      // Lock the Mutex to get a mutable reference to the noise generator
      let mut noise = NOISE_GENERATOR.lock().unwrap();

      // Slow down the passage of time by scaling the time value
      let time_factor = (uniforms.time as f32) / 2.0; // Slow down time progression
      
      // Instead of resetting the seed every time, displace it around the current noise
      let displacement = time_factor * 0.05; // Small displacement factor to smooth the noise evolution
  
      // Displace the noise coordinates slightly over time
      let noise_x = noise.get_noise_2d(fragment.vertex_position.x + displacement, fragment.vertex_position.y + displacement);
      let noise_y = noise.get_noise_2d(fragment.vertex_position.x + displacement + 0.5, fragment.vertex_position.y + displacement + 0.5);
      
      // Combine noise for more variation
      let noise_factor = (noise_x + noise_y) * 0.5;
      
      // Compute intensity based only on the noise factor
      let intensity = 1.0 + noise_factor * 0.3; // Increased noise factor to make the pattern more pronounced
      
      // Define the yellow, orange, and white colors for blending
      let yellow = Color::new(255, 186, 3); // Bright yellow
      let orange = Color::new(200, 50, 0); // Darker orange
      let white = Color::new(255, 255, 255); // White for the lighter parts
      
      // Blend between yellow/orange and white, depending on the intensity
      // If intensity is high, blend to white, else blend to orange
      let color = if intensity > 1.0 {
          yellow.lerp(&white, intensity - 1.0) // Blend from yellow to white
      } else {
          orange.lerp(&white, 1.0 - intensity) // Blend from orange to white
      };
  
      color
}