use once_cell::sync::Lazy;
use nalgebra_glm::{Vec2, Vec3, Vec4, Mat4};
use std::sync::Mutex;

use super::entity::vertex::Vertex;
use super::entity::fragment::Fragment;
use super::render::Uniforms;
use super::entity::color::Color;

use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

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

static CRATER_GENERATOR: Lazy<Mutex<FastNoiseLite>> = Lazy::new(|| {
  let mut noise = FastNoiseLite::new();
  noise.set_noise_type(Some(NoiseType::OpenSimplex2S)); // Use OpenSimplex2S for smooth crater patterns
  noise.set_frequency(Some(1.0)); // Adjust frequency for crater density
  Mutex::new(noise)
});

static CHIP_PATH_GENERATOR: Lazy<Mutex<FastNoiseLite>> = Lazy::new(|| {
  let mut noise = FastNoiseLite::new();
  noise.set_noise_type(Some(NoiseType::Perlin)); // Use Perlin noise for smoother patterns
  noise.set_frequency(Some(10.0)); // Adjust frequency for smaller, detailed paths
  Mutex::new(noise)
});

static ADDITIONAL_PATH_GENERATOR: Lazy<Mutex<FastNoiseLite>> = Lazy::new(|| {
  let mut noise = FastNoiseLite::new();
  noise.set_noise_type(Some(NoiseType::OpenSimplex2S)); // OpenSimplex2S for smooth patterns
  noise.set_frequency(Some(3.0)); // Adjust frequency for detail
  noise.set_fractal_type(Some(FractalType::PingPong)); // Using the PingPong fractal
  noise.set_fractal_octaves(Some(1)); // Control the octaves for more complexity
  noise.set_fractal_ping_pong_strength(Some(2.0));
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
      let cloud_displacement = time_factor * 0.3; // Clouds move faster for more dynamic effect
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

pub fn crater_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Lock the Mutex to get a mutable reference to the Earth and crater noise generators
  let noise = EARTH_GENERATOR.lock().unwrap();
  let crater_noise = CRATER_GENERATOR.lock().unwrap();

  // Slow down the passage of time by scaling the time value
  let time_factor = (uniforms.time as f32) / 10.0; // Slow down time progression
  
  // Displacement for general surface texture (smoothly moving over time)
  let displacement = time_factor * 0.05; // Small displacement factor to smooth the noise evolution
  let surface_noise_value = noise.get_noise_2d(fragment.vertex_position.x + displacement, fragment.vertex_position.y);

  // Displacement for crater texture (to make craters move differently than the surface)
  let crater_displacement = time_factor * 0.05; // Craters move slower for a more stable look
  let crater_noise_value = crater_noise.get_noise_2d(fragment.vertex_position.x + crater_displacement, fragment.vertex_position.y);

  // The noise value can represent height, so map it to the terrain color
  let surface_level = 0.0; // Base surface level
  let crater_level = -0.3; // Crater depth level

  // Define colors for surface and craters
  let surface_color = Color::new(160, 160, 160); // Light grey for the general surface
  let crater_color = Color::new(90, 90, 90); // Dark grey for craters

  // Blend between surface and crater based on noise value (height)
  let base_color = if surface_noise_value > surface_level {
      surface_color // Surface color
  } else {
      // Blend between surface color and crater color based on crater depth
      surface_color.lerp(&crater_color, (crater_noise_value - crater_level) / (surface_level - crater_level))
  };

  // Adjust the base color by light intensity (shading)
  let shaded_color = base_color;

  // Multiply by intensity for additional lighting effects (to make the texture more dynamic)
  shaded_color * fragment.intensity.max(0.4)
}

pub fn saturn_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Define the three stripe colors
    let color1 = Color::new(210, 180, 140); // Light tan
    let color2 = Color::new(160, 82, 45);  // Reddish brown
    let color3 = Color::new(255, 228, 196); // Pale cream

    // Control the width of the stripes
    let stripe_width = 0.1; // Adjust for thinner or thicker bands

    // Use the y-position of the vertex to determine the stripe
    let y_position = fragment.vertex_position.y; // The y-coordinate determines the stripe
    let stripe_value = (y_position / stripe_width).floor() as i32;

    // Smooth interpolation factor between stripes
    let transition_factor = (y_position / stripe_width).fract().abs();

    // Get the base colors for the current and next stripe
    let current_color = match stripe_value % 3 {
        0 => color1,
        1 => color2,
        _ => color3,
    };

    let next_color = match (stripe_value + 1) % 3 {
        0 => color1,
        1 => color2,
        _ => color3,
    };

    // Interpolate between the current stripe and the next stripe
    let base_color = current_color.lerp(&next_color, transition_factor);

    // Adjust brightness based on the fragment's intensity
    base_color * fragment.intensity.max(0.4)
}

pub fn pluto_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Lock the noise generator for craters
  let noise = CRATER_GENERATOR.lock().unwrap();

  // Define the heart's center position and scale
  let heart_center = Vec2::new(0.0, -0.2); // Center near the bottom
  let heart_scale = 0.2; // Scale for a properly-sized heart

  // Use noise to create surface details (craters)
  let noise_value = noise.get_noise_2d(
      fragment.vertex_position.x,
      fragment.vertex_position.y,
  );

  // Map noise to grayscale for the base surface
  let base_surface = Color::new(
      (noise_value * 40.0 + 150.0) as u8, // Light gray
      (noise_value * 50.0 + 150.0) as u8,
      (noise_value * 30.0 + 150.0) as u8,
  );

  // Calculate the relative position for the heart
  let relative_pos = Vec2::new(
      (fragment.vertex_position.x - heart_center.x) / heart_scale,
      (fragment.vertex_position.y - heart_center.y) / heart_scale,
  );

  // Improved heart shape formula:
  // A smoother, fuller heart shape derived from polar cardioid equations
  // r = 1 - sin(theta), transformed into Cartesian coordinates
  let x = relative_pos.x;
  let y = relative_pos.y;
  let heart_value = (x * x + (5.0 * y / 4.0 - x.abs().sqrt()).powi(2)) - 1.0;

  // Adjust the heart mask to define the heart region
  let heart_mask = (1.0 - heart_value.abs().min(1.0)).max(0.0); // Clamp to create a smooth mask

  // Define the heart color
  let heart_color = Color::new(200, 80, 100); // Reddish-pink for the heart

  // Blend the heart color and base texture
  let blended_color = heart_color.lerp(&base_surface, 1.0 - heart_mask);

  // Apply the fragment's intensity
  blended_color * fragment.intensity.max(0.4)
}

pub fn vortex_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let noise = CRATER_GENERATOR.lock().unwrap();

    // Convert Cartesian coordinates to polar
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let radius = (x.powi(2) + y.powi(2)).sqrt(); // Distance from the center
    let mut angle = y.atan2(x); // Angle in radians

    // Add a time-based rotation to the angle for swirling
    angle += uniforms.time * 0.5;

    // Convert back to Cartesian coordinates for distortion
    let swirl_x = radius * angle.cos();
    let swirl_y = radius * angle.sin();

    // Generate noise based on the distorted coordinates
    let noise_value = noise.get_noise_2d(swirl_x, swirl_y);

    // Map the noise value to a color gradient
    let core_color = Color::new(255, 50, 50); // Bright red for the vortex center
    let mid_color = Color::new(120, 60, 240); // Purple for swirling areas
    let edge_color = Color::new(10, 10, 30); // Dark blue for outer regions

    let color = if radius < 0.5 {
        core_color.lerp(&mid_color, noise_value * 0.5 + 0.5) // Blend from red to purple
    } else {
        mid_color.lerp(&edge_color, (radius - 0.5).min(1.0)) // Blend from purple to blue
    };

    color * fragment.intensity
}

pub fn hypnos_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let noise = ADDITIONAL_PATH_GENERATOR.lock().unwrap();

  // Slow down the passage of time to create a more subtle animation effect
  let time_factor = (uniforms.time as f32) / 10.0;

  // Get the PingPong fractal noise based on the fragment's position
  let noise_value = noise.get_noise_2d(fragment.vertex_position.x * 5.0, fragment.vertex_position.y * 5.0 + time_factor);

  // Normalize the noise value to the range [0, 1]
  let normalized_noise_value = (noise_value + 1.0) * 0.5;

  // Define the base planet color (dark base color for the planet)
  let planet_color = Color::new(0, 40, 0); // Dark green color for the planet

  // Define the color for the fractal noise pattern (lighter colors for the swirling effect)
  let fractal_color = Color::new(255, 255, 0); // Yellow for the swirling effect

  // Blend the planet color with the fractal pattern based on the noise value
  let final_color = planet_color.lerp(&fractal_color, normalized_noise_value);

  // Apply fragment intensity to control brightness
  final_color * fragment.intensity.min(0.8)
}