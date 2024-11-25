use std::sync::Arc;
use std::any::Any;
use nalgebra_glm::{Vec2, Vec3,Vec4};

use super::entity::vertex::Vertex;
use super::entity::fragment::Fragment;
use super::render::Uniforms;
use super::entity::color::Color;

// Trait definition
pub trait Model {
    fn get_vertex_array(&self) -> Arc<Vec<Vertex>>;
    fn get_shader(&self) -> fn(&Fragment, &Uniforms) -> Color;
    fn get_position(&self) -> Vec3;
    fn set_position(&mut self, position: Vec3);
    fn get_scale(&self) -> f32;
    fn get_rotation(&self) -> Vec3;
    fn set_rotation(&mut self, rotation: Vec3);
    fn get_colision_radius(&self) -> f32;
    fn as_any(&self) -> &dyn Any; // Add this method
    fn as_any_mut(&mut self) -> &mut dyn Any; // Add this method for mutable access
}

// SimpleModel struct
pub struct SimpleModel {
    pub vertex_array: Arc<Vec<Vertex>>, // Change to Arc<Vec<Vertex>>
    pub shader: fn(&Fragment, &Uniforms) -> Color,
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3,
    pub collision_radius: f32,
}

// Implement the Model trait for SimpleModel
impl Model for SimpleModel {
    fn get_vertex_array(&self) -> Arc<Vec<Vertex>> {
        Arc::clone(&self.vertex_array) // Clone the Arc to return a reference-counted version
    }

    fn get_shader(&self) -> fn(&Fragment, &Uniforms) -> Color {
        self.shader
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, position: Vec3){
        self.position = position;
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    fn set_rotation(&mut self, rotation: Vec3){
        self.rotation = rotation;
    }

    fn get_colision_radius(&self) -> f32 {
        self.collision_radius
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Planet struct
pub struct Planet {
    pub vertex_array: Arc<Vec<Vertex>>, // Change to Arc<Vec<Vertex>>
    pub shader: fn(&Fragment, &Uniforms) -> Color,
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3,
    pub collision_radius: f32,

    pub center: Vec3,
    pub orbit_angle: f32,
    pub orbit_speed: f32,
    pub orbit_radius: f32,
    pub orbit_segments: Vec<Vertex>
}

// Implement the Model trait for Planet
impl Model for Planet {
    fn get_vertex_array(&self) -> Arc<Vec<Vertex>> {
        Arc::clone(&self.vertex_array) // Clone the Arc to return a reference-counted version
    }

    fn get_shader(&self) -> fn(&Fragment, &Uniforms) -> Color {
        self.shader
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, position: Vec3){
        self.position = position;
    }

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_rotation(&self) -> Vec3 {
        self.rotation
    }

    fn set_rotation(&mut self, rotation: Vec3){
        self.rotation = rotation;
    }

    fn get_colision_radius(&self) -> f32 {
        self.collision_radius
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

// Planet implementation with new and translate methods
impl Planet {
    
    pub fn new(
        vertex_array: Arc<Vec<Vertex>>,
        scale: f32,
        shader: fn(&Fragment, &Uniforms) -> Color,
        orbit_radius: f32,
        orbit_angle: f32,
        orbit_speed: f32,
        collision_radius: f32,
        center: Vec3, // Center of the orbit
        orbit_segments: usize, // Number of segments for the orbit
    ) -> Self {
        // Calculate initial position based on orbit parameters
        let x = center.x + orbit_radius * orbit_angle.cos();
        let z = center.z + orbit_radius * orbit_angle.sin();
        let position = Vec3::new(x, center.y, z);

        let rotation = Vec3::new(0.0, 0.0, 0.0);

        // Generate orbit vertices
        let orbit_vertices = create_orbit(orbit_radius, center, orbit_segments);

        Planet {
            vertex_array,
            shader,
            position,
            scale,
            rotation,
            collision_radius,
            center,
            orbit_angle,
            orbit_speed,
            orbit_radius,
            orbit_segments: orbit_vertices, // Initialize the orbit vertices
        }
    }

    pub fn translate(&mut self, delta_time: f32) {
        // Update orbit angle based on orbit speed and time step
        self.orbit_angle += self.orbit_speed * delta_time;

        // Ensure the angle stays within the range [0, 2π] to prevent overflow
        self.orbit_angle %= std::f32::consts::TAU;

        // Recalculate position based on the updated orbit angle
        let x = self.center.x + self.orbit_radius * self.orbit_angle.cos();
        let z = self.center.z + self.orbit_radius * self.orbit_angle.sin();
        self.position = Vec3::new(x, self.position.y, z);
    }

}

fn create_orbit(radius: f32, center: Vec3, segments: usize) -> Vec<Vertex> {
    let mut orbit_vertices = Vec::with_capacity(segments);

    for i in 0..segments {
        let angle = 2.0 * std::f32::consts::PI * (i as f32 / segments as f32);
        let x = center.x + radius * angle.cos();
        let z = center.z + radius * angle.sin();
        let position = Vec3::new(x, center.y, z);

        orbit_vertices.push(Vertex::new(
            position,
            Vec3::new(0.0, 1.0, 0.0), // Normal vector for orbit vertices
            Vec2::zeros(),      // Placeholder UV coordinates
            Vec4::zeros(), // Placeholder transformed position
        ));
    }

    orbit_vertices
}
