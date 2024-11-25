use nalgebra_glm::{look_at, perspective, Mat4, Vec2, Vec3, Vec4};
use std::f32::consts::PI;
use std::sync::Arc;

use super::camera::Camera;
use super::entity::vertex::Vertex;
use super::framebuffer::Framebuffer;
use super::shader::vertex_shader;
use super::line::{line, triangle_flat_shade};
use super::entity::fragment::Fragment;
use super::entity::color::Color;

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub perspective_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: u32
}

pub fn render(framebuffer: &mut Framebuffer, 
    uniforms: &Uniforms,
    vertex_array: Arc<Vec<Vertex>>,
    camera: &Camera, 
    shader: fn(&Fragment, &Uniforms) -> Color) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    let tranformation_matrix = uniforms.perspective_matrix * uniforms.view_matrix * uniforms.model_matrix;
    for vertex in vertex_array.iter() {
        let transformed = vertex_shader(vertex, &tranformation_matrix, &uniforms);
        transformed_vertices.push(transformed);
    }
    // println!("{}", uniforms.model_matrix);
    
    // println!("a: {}, b:{}", vertex_array[1].position, transformed_vertices[1].position);

    // Primitive Assembly Stage
    let triangles = assembly(&transformed_vertices, true);

    // Rasterization Stage
    let mut fragments = Vec::new();
    let camera_view_dir = (camera.center - camera.eye).normalize();
    for tri in &triangles {
        fragments.extend(triangle_flat_shade(&tri[0], &tri[1], &tri[2], camera_view_dir));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let color = shader(&fragment, uniforms);
            framebuffer.set_current_color(color);
            framebuffer.draw_point(x, y, fragment.depth);
        }
    }
}

fn assembly(vertices: &[Vertex], should_optimize: bool) -> Vec<&[Vertex]> {
    let triangles = vertices.chunks(3);

    if should_optimize {
        triangles
            .filter(|triangle_vertices| {
                let range = -1.0..1.0;
                let a = &triangle_vertices[0];
                let b = &triangle_vertices[1];
                let c = &triangle_vertices[2];
                let a_in_range = range.contains(&a.frustrum_position.x)
                    && range.contains(&a.frustrum_position.y)
                    && range.contains(&a.frustrum_position.z);
                let b_in_range = range.contains(&b.frustrum_position.x)
                    && range.contains(&b.frustrum_position.y)
                    && range.contains(&b.frustrum_position.z);
                let c_in_range = range.contains(&c.frustrum_position.x)
                    && range.contains(&c.frustrum_position.y)
                    && range.contains(&c.frustrum_position.z);

                a_in_range || b_in_range || c_in_range
            })
            .collect()
    } else {
        triangles.collect()
    }
}

pub fn draw_orbit(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    segments: &[Vertex],
    camera: &Camera,
    orbit_color: Color,
) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(segments.len());

    let modified_uniforms = &Uniforms { 
        model_matrix: create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.0, Vec3::new(0.0, 0.0, 0.0)),
        view_matrix: uniforms.view_matrix,
        perspective_matrix: uniforms.perspective_matrix, 
        viewport_matrix: uniforms.viewport_matrix,
        time: 0 };

    let transformation_matrix = modified_uniforms.perspective_matrix * modified_uniforms.view_matrix * modified_uniforms.model_matrix;


    for vertex in segments {
        let transformed = vertex_shader(vertex, &transformation_matrix, modified_uniforms);
        transformed_vertices.push(transformed);
    }

    // Line Assembly Stage
    let mut lines = Vec::new();
    for i in 0..segments.len() {
        let start = &transformed_vertices[i];
        let end = &transformed_vertices[(i + 1) % segments.len()]; // Wrap around for closed orbit

        // Only keep lines where at least one endpoint is within the clip space range
        let range = -1.0..1.0;
        let start_in_range = range.contains(&start.frustrum_position.x)
            && range.contains(&start.frustrum_position.y)
            && range.contains(&start.frustrum_position.z);
        let end_in_range = range.contains(&end.frustrum_position.x)
            && range.contains(&end.frustrum_position.y)
            && range.contains(&end.frustrum_position.z);

        if start_in_range || end_in_range {
            lines.push((start.clone(), end.clone()));
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    let camera_view_dir = (camera.center - camera.eye).normalize();

    for (start, end) in lines {
        fragments.extend(line(&start, &end)); // Assume `line` rasterizes a line into fragments
    }

    // Fragment Processing Stage
    framebuffer.set_current_color(orbit_color);
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            framebuffer.draw_point(x, y, fragment.depth);
        }
    }
}

pub fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

pub fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

pub fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.5;
    let far = 150.0;

    perspective(fov, aspect_ratio, near, far)
}

pub fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}