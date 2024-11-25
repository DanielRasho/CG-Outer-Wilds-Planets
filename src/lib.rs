mod internal;

use internal::camera::Camera;
use internal::entity::skybox::Skybox;
use internal::entity::vertex::{self, Vertex};
use internal::object::Obj;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::{Mat4, Vec3};

use std::sync::Arc;
use std::time::Duration;
use std::f32::consts::PI;

use internal::framebuffer::Framebuffer;
use internal::render::{create_model_matrix, create_perspective_matrix, create_view_matrix, create_viewport_matrix, draw_orbit, render, Uniforms};
use internal::entity::color::Color;
use internal::model::{Model, SimpleModel, Planet};
use internal::shader::{earth_shader, simple_shader, sun_shader};


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
    
    let skybox = Skybox::new(200, 200.0, Color::new(255, 255, 255), Color::new(0, 0, 20));

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 4.0), 
        Vec3::new(0.0, 0.0, -1.0), 
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        10.0
    );
    
    let space_ship_obj = Obj::load("./assets/mesh/spaceShip.obj").expect("Failed to load obj");
    let planet_obj = Obj::load("./assets/mesh/planet.obj").expect("Failed to load obj");
    
    let space_ship_vertices = Arc::new(space_ship_obj.get_vertex_array());
    let planet_vertices = Arc::new(planet_obj.get_vertex_array());
    
    // Create a list of models with one inline-defined SimpleModel
    let mut models: Vec<Box<dyn Model>> = vec![
        Box::new(SimpleModel {
            vertex_array: planet_vertices.clone(), // Clone the Arc
            shader: earth_shader,
            position: Vec3::new(0.0, 0.0, 1.0),
            scale: 0.5,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            collision_radius: 5.0,
        }),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            1.0,
            simple_shader,
            20.0,
            0.0,
            0.00001,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            40
        )),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            4.0,
            simple_shader,
            50.0,
            0.0,
            0.0001,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            40
        )),
    ];
    // let vertex_array = obj.get_vertex_array();
    // let vertex_array : Vec<Vertex> = vec![];

    let mut view_matrix : Mat4;
    let perspective_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
    
    let mut time : f32 = 0.0;
    
    // RENDER LOOP
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        
        time += 0.1;

        let subject = models.get_mut(0).expect("Subject not found."); // OR SOMETHING LIKE THAT

        handle_input(&window, &mut camera, &mut **subject); // MODIFY THE CAMERA AND SUBJECT POSITION
        
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(255, 255, 255));

        view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);

        skybox.render(&mut framebuffer, &perspective_matrix, &view_matrix);
        
        /* 
            draw_orbit(&mut framebuffer, 
                Vec3::new(0.0, 0.0, 0.0),
                15.0, 
                &perspective_matrix, 
                &view_matrix, 
                &viewport_matrix,
                20, 
                Color::new(255, 200, 255)
            );
        */

        for model in &mut models{
            
            let model_matrix = create_model_matrix(model.get_position(), model.get_scale(), model.get_rotation());

            let uniforms = Uniforms{ model_matrix , view_matrix, perspective_matrix, viewport_matrix, time};
            
            if let Some(planet) = model.as_any_mut().downcast_mut::<Planet>() {
                draw_orbit(
                    &mut framebuffer,
                    &uniforms,
                    &planet.orbit_segments,
                    &camera,
                    Color::new(255, 255, 255)
                );
                planet.translate(time);
            }
            
            render(&mut framebuffer, &uniforms, model.get_vertex_array(), &camera, model.get_shader());
        }


        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();


        std::thread::sleep(frame_delay)
    }
}

fn handle_input(window: &Window, camera: &mut Camera, subject: &mut dyn Model) {

    const ROTATION_SPEED : f32 = PI /20.0;
    const ZOOM_SPEED : f32 = 1.0;
    const TRANSLATE_STEP : f32 = 0.3;

    // camera orbit controls
    if window.is_key_down(Key::D) {
        camera.orbit(ROTATION_SPEED, 0.0);
    }
    if window.is_key_down(Key::A) {
        camera.orbit(-ROTATION_SPEED, 0.0);
    }
    if window.is_key_down(Key::S) {
        camera.orbit(0.0, -ROTATION_SPEED);
    }
    if window.is_key_down(Key::W) {
        camera.orbit(0.0, ROTATION_SPEED);
    }

    // camera zoom
    if window.is_key_down(Key::Q) {
        camera.zoom(ZOOM_SPEED);
    }
    if window.is_key_down(Key::E) {
        camera.zoom(-ZOOM_SPEED);
    }

    let mut subject_position = subject.get_position();
    // Model controls
    if window.is_key_down(Key::J) {
        subject_position.x += TRANSLATE_STEP;
        subject.set_position(subject_position);
        camera.change_center(subject_position);
    }
    if window.is_key_down(Key::L) {
        subject_position.x -= TRANSLATE_STEP;
        subject.set_position(subject_position);
        camera.change_center(subject_position);
    }
    if window.is_key_down(Key::K) {
        subject_position.z += TRANSLATE_STEP;
        subject.set_position(subject_position);
        camera.change_center(subject_position);
    }
    if window.is_key_down(Key::I) {
        subject_position.z -= TRANSLATE_STEP;
        subject.set_position(subject_position);
        camera.change_center(subject_position);
    }

    if window.is_key_down(Key::U) {
        subject_position.y -= TRANSLATE_STEP;
        subject.set_position(subject_position);
        camera.change_center(subject_position);
    }
    if window.is_key_down(Key::O) {
        subject_position.y += TRANSLATE_STEP;
        subject.set_position(subject_position);
        camera.change_center(subject_position);
    }

    
    // print!("eye {} center {} up {}", camera.eye, camera.center, camera.up);
    
}

