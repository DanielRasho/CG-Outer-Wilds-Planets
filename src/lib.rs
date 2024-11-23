mod internal;

use internal::camera::Camera;
use internal::entity::skybox::Skybox;
use internal::entity::vertex::{self, Vertex};
use internal::object::Obj;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec3;

use std::time::Duration;
use std::f32::consts::PI;

use internal::framebuffer::Framebuffer;
use internal::render::{create_model_matrix, create_perspective_matrix, create_view_matrix, create_viewport_matrix, render, Uniforms};
use internal::entity::color::Color;


pub fn start() {
    // Window Size configuration
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width =  window_width;
    let framebuffer_height = window_height;
    
    // Frame Rate
    let frame_delay = Duration::from_millis(16);
  
    // Window Objects initialization
    let mut framebuffer = Framebuffer::new(window_width, window_height, Color::new(0, 0, 0));
    let mut window = Window::new(
      "Minecraft Diorama",
      window_width,
      window_height,
      WindowOptions::default()
    ).unwrap();
    
    let skybox = Skybox::new(120, 10.0, Color::new(255, 255, 255), Color::new(0, 0, 50));

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 100.0), 
        Vec3::new(0.0, 0.0, 0.0), 
        Vec3::new(0.0, 1.0, 0.0)
    );
    
    let translation = Vec3::new(0.0, 0.0, 0.0);
    let rotation = Vec3::new(0.0, 0.0, 0.0);
    let scale = 1.0f32;
    
    let obj = Obj::load("./assets/mesh/spaceShip.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();
    // let vertex_array : Vec<Vertex> = vec![];

    let model_matrix = create_model_matrix(translation, scale, rotation);
    let mut view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
    let perspective_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    
    // RENDER LOOP
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut camera);
        if camera.check_if_changed(){
            view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
            framebuffer.clear();
            framebuffer.set_current_color_hex(0xFFFFFF);
            let uniforms = Uniforms{ model_matrix , view_matrix, perspective_matrix, viewport_matrix};
            skybox.render(&mut framebuffer, &uniforms);
            render(&mut framebuffer, &uniforms, &vertex_array, &camera);
        }

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        std::thread::sleep(frame_delay)
    }
}

fn handle_input(window: &Window, camera: &mut Camera) {

    const ROTATION_SPEED : f32 = PI /20.0;
    const ZOOM_SPEED : f32 = 1.0;

    // camera orbit controls
    if window.is_key_down(Key::Right) {
        camera.orbit(ROTATION_SPEED, 0.0);
    }
    if window.is_key_down(Key::Left) {
        camera.orbit(-ROTATION_SPEED, 0.0);
    }
    if window.is_key_down(Key::Down) {
        camera.orbit(0.0, -ROTATION_SPEED);
    }
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0, ROTATION_SPEED);
    }

    // camera zoom
    if window.is_key_down(Key::J) {
        camera.zoom(ZOOM_SPEED);
    }
    if window.is_key_down(Key::K) {
        camera.zoom(-ZOOM_SPEED);
    }
    
    // print!("eye {} center {} up {}", camera.eye, camera.center, camera.up);
    // print!("==========================");
    
}

