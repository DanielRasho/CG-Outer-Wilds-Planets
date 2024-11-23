use nalgebra_glm::Vec3;
use super::color::Color;
use super::super::framebuffer::Framebuffer;
use super::super::render::Uniforms;

use std::f32::consts::PI;
use rand::Rng;

pub struct Skybox {
    pub stars: Vec<Vec3>,       // Positions of stars in 3D space
    pub stars_color: Color,     // Color of the stars
    pub space_color: Color,     // Background color of space
}

impl Skybox {
    /// Creates a new Skybox with randomly generated stars at a fixed distance.
    ///
    /// - `num_stars`: Number of stars to generate.
    /// - `distance`: The fixed distance of all stars from the origin.
    /// - `stars_color`: The color of the stars.
    /// - `space_color`: The background color of space.
    pub fn new(num_stars: usize, distance: f32, stars_color: Color, space_color: Color) -> Self {
        let mut rng = rand::thread_rng(); // Initialize random number generator
        let mut stars = Vec::with_capacity(num_stars);

        for _ in 0..num_stars {
            // Generate random directions for the stars
            let theta = rng.gen_range(0.0..PI * 2.0); // Random angle for x-y plane
            let phi = rng.gen_range(0.0..PI);         // Random angle for z-axis

            // Convert spherical coordinates to Cartesian
            let x = distance * phi.sin() * theta.cos();
            let y = distance * phi.sin() * theta.sin();
            let z = distance * phi.cos();

            stars.push(Vec3::new(x, y, z));
        }

        Skybox {
            stars,
            stars_color,
            space_color,
        }
    }

    /// Renders the skybox onto the framebuffer.
    /// - `framebuffer`: A mutable slice representing the framebuffer.
    /// - `width`: Width of the framebuffer.
    /// - `height`: Height of the framebuffer.
    pub fn render(&self, framebuffer: &mut Framebuffer, uniforms: &Uniforms) {
        let background_color = self.space_color.to_hex();
        let star_color = self.stars_color.to_hex();
        // Fill the background with space_color
        for pixel in framebuffer.buffer.iter_mut() {
            *pixel = background_color;
        }
        // Combine view and projection matrices
        let vp_matrix = uniforms.perspective_matrix * uniforms.view_matrix;

        // Render each star
        for star in &self.stars {
            // Transform star position to camera's view space
            let star_position = vp_matrix * star.push(1.0);

            // Perspective divide to get normalized device coordinates (NDC)
            if star_position.w > 0.0 {
                let ndc_x = star_position.x / star_position.w;
                let ndc_y = star_position.y / star_position.w;

                // Convert NDC to screen coordinates
                let screen_x = ((ndc_x + 1.0) * 0.5 * framebuffer.width as f32) as isize;
                let screen_y = ((1.0 - ndc_y) * 0.5 * framebuffer.height as f32) as isize;

                // Ensure the star is within the framebuffer bounds
                if screen_x >= 0 && screen_x < framebuffer.width as isize && screen_y >= 0 && screen_y < framebuffer.height as isize {
                    let index = screen_y as usize * framebuffer.width + screen_x as usize;
                    framebuffer.buffer[index] = star_color;
                }
            }
        }
    }
}