use nalgebra_glm::{look_at, perspective, Mat4, Vec2, Vec3, Vec4};
use std::f32::consts::PI;

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

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], camera: &Camera, shader: fn(&Fragment, &Uniforms) -> Color) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }
    // println!("{}", uniforms.model_matrix);
    
    // println!("a: {}, b:{}", vertex_array[1].position, transformed_vertices[1].position);

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

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

pub fn draw_orbit(
    framebuffer: &mut Framebuffer,
    center: Vec3, 
    radius: f32, 
    perspective_matrix: &Mat4, 
    view_matrix: &Mat4, 
    viewport_matrix: &Mat4,
    segments: usize,
    color: Color,
) {
        framebuffer.set_current_color(color);

        // Create a combined view-projection matrix
        let vp_matrix = perspective_matrix * view_matrix;

        // Create a vector to store the vertices
        let mut orbit_vertices = Vec::with_capacity(segments);
    
        // Generate vertices for the circle in 3D space and store them in `orbit_vertices`
        for i in 0..segments {
            let angle = 2.0 * std::f32::consts::PI * (i as f32 / segments as f32);
            let x = center.x + radius * angle.cos();
            let y = center.y; // You can adjust y if you want the circle to tilt
            let z = center.z + radius * angle.sin();
    
            let position = Vec3::new(x, y, z);
    
            // Create a new Vertex with the position
            let mut vertex = Vertex::new(position, Vec3::new(0.0, 1.0, 0.0), Vec2::new(0.0, 0.0));
            
            // Apply the view-projection matrix to the position
            let transformed_position = vp_matrix * position.push(1.0); // Homogeneous transformation
            vertex.set_transformed(transformed_position.xyz(), Vec3::new(0.0, 0.0, 0.0)); // Set the transformed position
    
            orbit_vertices.push(vertex);
        }
        
        
        let uniforms = Uniforms{ 
            model_matrix: create_model_matrix(Vec3::new(0.0, 0.0, 0.0) , 1.0, Vec3::new(0.0, 0.0, 0.0)) , 
            view_matrix: *view_matrix, 
            perspective_matrix: *perspective_matrix,
            viewport_matrix: *viewport_matrix,
            time: 0
        };

        // Vertex Shader Stage
        let mut transformed_vertices = Vec::with_capacity(orbit_vertices.len());
        for vertex in orbit_vertices {
            let transformed = vertex_shader(&vertex, &uniforms);
            transformed_vertices.push(transformed);
        }
    
    
        let mut fragments = Vec::new();
        // Now we have all the vertices, and we can render the orbit by connecting them with lines
        for i in 0..segments {
            let start = &transformed_vertices[i];
            let end = &transformed_vertices[(i + 1) % segments]; // Loop back to the start for a closed circle
            fragments.extend(line(start, end));
    }
        
    // Fragment Processing Stage
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
    let near = 0.1;
    let far = 100.0;

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