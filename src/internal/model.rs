use std::sync::Arc;
use std::any::Any;
use nalgebra_glm::Vec3;

use super::entity::vertex::Vertex;
use super::entity::fragment::Fragment;
use super::render::Uniforms;
use super::entity::color::Color;

// Trait definition
pub trait Model {
    fn get_vertex_array(&self) -> Arc<Vec<Vertex>>;
    fn get_shader(&self) -> fn(&Fragment, &Uniforms) -> Color;
    fn get_position(&self) -> Vec3;
    fn get_scale(&self) -> f32;
    fn get_rotation(&self) -> Vec3;
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

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_rotation(&self) -> Vec3 {
        self.rotation
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

    pub orbit_angle: f32,
    pub orbit_speed: f32,
    pub orbit_radius: f32,
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

    fn get_scale(&self) -> f32 {
        self.scale
    }

    fn get_rotation(&self) -> Vec3 {
        self.rotation
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
        vertex_array: Arc<Vec<Vertex>>, // Use Arc here
        scale: f32,
        shader: fn(&Fragment, &Uniforms) -> Color,
        orbit_radius: f32,
        orbit_angle: f32,
        orbit_speed: f32,
        collision_radius: f32,
    ) -> Self {
        // Calculate initial position based on orbit parameters
        let x = orbit_radius * orbit_angle.cos();
        let z = orbit_radius * orbit_angle.sin();
        let position = Vec3::new(x, 0.0, z);

        let rotation = Vec3::new(0.0, 0.0, 0.0);

        Planet {
            vertex_array,
            shader,
            position,
            scale,
            rotation,
            collision_radius,
            orbit_angle,
            orbit_speed,
            orbit_radius,
        }
    }
    pub fn translate(&mut self, delta_time: u32) {
        // Update orbit angle based on orbit speed and time step
        self.orbit_angle += self.orbit_speed * delta_time as f32;

        // Ensure the angle stays within the range [0, 2π] to prevent overflow
        self.orbit_angle %= std::f32::consts::TAU;

        // Recalculate position based on the updated orbit angle
        let x = self.orbit_radius * self.orbit_angle.cos();
        let z = self.orbit_radius * self.orbit_angle.sin();
        self.position = Vec3::new(x, self.position.y, z);
    }
}
