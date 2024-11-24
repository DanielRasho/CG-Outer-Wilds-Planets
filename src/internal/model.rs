use super::entity::vertex::Vertex;
use super::entity::fragment::Fragment;
use super::render::Uniforms;
use super::entity::color::Color;

use nalgebra_glm::Vec3;

// Trait definition with explicit lifetime
pub trait Model<'a> {
    fn get_vertex_array(&self) -> &'a [Vertex];
    fn get_shader(&self) -> fn(&Fragment, &Uniforms) -> Color;
    fn get_position(&self) -> Vec3;
    fn get_scale(&self) -> f32;
    fn get_rotation(&self) -> Vec3;
    fn get_colision_radius(&self) -> f32;
}

// SimpleModel struct
pub struct SimpleModel<'a> {
    pub vertex_array: &'a [Vertex],
    pub shader: fn(&Fragment, &Uniforms) -> Color,
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3,
    pub collision_radius: f32,
}

// Implement the Model trait for SimpleModel
impl<'a> Model<'a> for SimpleModel<'a> {
    fn get_vertex_array(&self) -> &'a [Vertex] {
        self.vertex_array
    }

    fn get_shader(&self) -> fn(&Fragment, &Uniforms) -> Color {
        self.shader
    }
    
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn get_scale(&self) -> f32{
        self.scale
    }

    fn get_rotation(&self) -> Vec3{
        self.rotation
    }

    fn get_colision_radius(&self) -> f32 {
        self.collision_radius        
    }
}

// Planet struct
pub struct Planet<'a> {
    pub vertex_array: &'a [Vertex],
    pub shader: fn(&Fragment, &Uniforms) -> Color,
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3,
    pub collision_radius: f32,

    pub orbit_offset: f32,
    pub orbit_angle: f32,
    pub orbit_speed: f32,
    pub orbit_radius: f32
}

// Implement the Model trait for Planet
impl<'a> Model<'a> for Planet<'a> {
    fn get_vertex_array(&self) -> &'a [Vertex] {
        self.vertex_array
    }

    fn get_shader(&self) -> fn(&Fragment, &Uniforms) -> Color {
        self.shader
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn get_scale(&self) -> f32{
        self.scale
    }

    fn get_rotation(&self) -> Vec3{
        self.rotation
    }

    fn get_colision_radius(&self) -> f32 {
        self.collision_radius        
    }
}