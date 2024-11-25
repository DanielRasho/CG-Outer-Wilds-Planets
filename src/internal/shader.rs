use once_cell::sync::Lazy;
use nalgebra_glm::{Vec2, Vec3, Vec4, Mat4};
use std::sync::Mutex;

use super::entity::vertex::Vertex;
use super::entity::fragment::Fragment;
use super::render::Uniforms;
use super::entity::color::Color;

use fastnoise_lite::{FastNoiseLite, NoiseType};

static SUN_GENERATOR: Lazy<Mutex<FastNoiseLite>> = Lazy::new(|| {
  let mut noise = FastNoiseLite::new();
  noise.set_noise_type(Some(NoiseType::Cellular)); // Use Cellular noise for texture-like patterns
  noise.set_frequency(Some(10.0)); // Decrease frequency for bigger cells (larger scale)
  Mutex::new(noise) // Wrap the noise generator in a Mutex
});

static EARTH_GENERATOR: Lazy<Mutex<FastNoiseLite>> = Lazy::new(|| {
  let mut noise = FastNoiseLite::new();
  noise.set_noise_type(Some(NoiseType::Value)); // Using Perlin noise for smooth height maps
  noise.set_frequency(Some(10.0)); // Low frequency for large terrain features
  Mutex::new(noise) // Wrap the noise generator in a Mutex
});

// New Cloud Generator
static CLOUDS_GENERATOR: Lazy<Mutex<FastNoiseLite>> = Lazy::new(|| {
  let mut noise = FastNoiseLite::new();
  noise.set_noise_type(Some(NoiseType::OpenSimplex2S)); // Use Simplex noise for smooth cloud patterns
  noise.set_frequency(Some(0.8)); // Adjust frequency for cloud density
  Mutex::new(noise)
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
      let mut noise = SUN_GENERATOR.lock().unwrap();

      // Slow down the passage of time by scaling the time value
      let time_factor = (uniforms.time as f32) / 10.0; // Slow down time progression
      
      // Instead of resetting the seed every time, displace it around the current noise
      let displacement = time_factor * 0.05; // Small displacement factor to smooth the noise evolution
  
      // Displace the noise coordinates slightly over time
      let noise_x = noise.get_noise_2d(fragment.vertex_position.x + displacement, fragment.vertex_position.y + displacement);
      let noise_y = noise.get_noise_2d(fragment.vertex_position.x + 0.5, fragment.vertex_position.y + 0.5);
      
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

pub fn earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
      // Lock the Mutex to get a mutable reference to the Earth noise generator
      let noise = EARTH_GENERATOR.lock().unwrap();

      // Lock the Mutex to get a mutable reference to the Clouds noise generator
      let clouds_noise = CLOUDS_GENERATOR.lock().unwrap();
  
      // Slow down the passage of time by scaling the time value
      let time_factor = (uniforms.time as f32) / 10.0; // Slow down time progression
      
      // Displacement for Earth texture
      let displacement = time_factor * 0.05; // Small displacement factor to smooth the noise evolution
      let noise_value = noise.get_noise_2d(fragment.vertex_position.x + displacement, fragment.vertex_position.y);
      
      // Cloud texture displacement (clouds move slightly faster than the Earth texture)
      let cloud_displacement = time_factor * 0.1; // Clouds move faster for more dynamic effect
      let cloud_noise_value = clouds_noise.get_noise_2d(fragment.vertex_position.x + cloud_displacement, fragment.vertex_position.y);
  
      // The noise value can represent height, so map it to the terrain color
      let ocean_level = 0.0; // Ocean is at noise value 0.0
      let terrain_level = 0.5; // Terrain starts at noise value 0.5 (adjustable)
    
      // Define the blue (ocean) and green (terrain) colors
      let ocean_color = Color::new(0, 0, 255); // Blue for the ocean
      let terrain_color = Color::new(34, 139, 34); // Green for the terrain
      
      // Blend between ocean and terrain based on noise value (height)
      let earth_color = if noise_value < ocean_level {
          ocean_color // Blue for ocean
      } else if noise_value < terrain_level {
          // Blend between blue and green for shallow water/shoreline
          ocean_color.lerp(&terrain_color, (noise_value - ocean_level) / (terrain_level - ocean_level))
      } else {
          terrain_color // Green for terrain
      };
  
      // Cloud effect: The higher the cloud noise value, the less visible the clouds
      let cloud_opacity = (cloud_noise_value - 0.2).abs() ; // Adjust cloud opacity based on the noise value (max 1.0)
  
      // Define the cloud color as white with some transparency
      let cloud_color = Color::new(255, 255, 255); // White for clouds
  
      // Blend the cloud color with the Earth color based on cloud opacity
      let final_color = earth_color.lerp(&cloud_color, cloud_opacity);
  
      // Multiply by intensity for lighting effects
      final_color * fragment.intensity.max(0.4)
}