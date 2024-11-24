use nalgebra_glm::{Vec3, Vec4, Mat4};
use super::entity::vertex::Vertex;
use super::entity::fragment::Fragment;
use super::render::Uniforms;
use super::entity::color::Color;

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
  let ndc_position = if w != 0.0 {
    Vec4::new(
    clip_pos.x / w,
    clip_pos.y / w,
    clip_pos.z / w,
    1.0
  )} else {
    Vec4::new(
    clip_pos.x,
    clip_pos.y,
    clip_pos.z,
    1.0)
  };

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
    transformed_position,
    transformed_normal: transformed_normal,
  }
}

pub fn simple_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  fragment.color * fragment.intensity
}