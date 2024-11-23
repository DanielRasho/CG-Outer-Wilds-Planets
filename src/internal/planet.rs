use super::object::Obj;
use super::entity::vertex::Vertex;

use nalgebra_glm::Vec3;

pub enum PlanetShader {
    Rocky,
    Gaseous,
    Frozen,
    Earth,
    Oceanic,
    Ufo,
    Gargantua,
    Wormhole,
}

pub struct Planet<'a> {
    pub vertex_array: &'a [Vertex],
    pub shader: PlanetShader,
    pub position: Vec3,
    pub scale: f32,
    pub rotation: Vec3,
    pub rotation_speed: Vec3,
    pub collision_radius: f32, // Radio de colisión
    pub orbit_angle: f32, // Nuevo campo para almacenar el ángulo de órbita
    pub orbit_speed: f32, // Nuevo campo para la velocidad de órbita
    pub orbit_radius: f32, // Nuevo campo para almacenar el radio de la órbita
}