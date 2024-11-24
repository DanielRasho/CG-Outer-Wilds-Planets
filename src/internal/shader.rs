use nalgebra_glm::{Vec3, Vec4};
use super::entity::vertex::Vertex;
use super::entity::fragment::Fragment;
use super::render::Uniforms;
use super::entity::color::Color;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.viewport_matrix * uniforms.perspective_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let transformed_position = Vec3::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w
  );

  // Transform normal

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: vertex.normal,
  }
}

pub fn simple_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  fragment.color
}