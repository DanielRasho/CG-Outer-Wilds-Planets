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
use internal::shader::{crater_shader, earth_shader, hypnos_shader, pluto_shader, saturn_ring_shader, saturn_shader, simple_shader, sun_shader, vortex_shader};


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
        Vec3::new(0.0, 10.0, 60.0),
        Vec3::new(0.0, 0.0, -1.0), 
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        15.0,
        Vec3::new(-5.0, 110.0, -10.0), 
        Vec3::new(0.0, 0.0, 0.0),
    );
    
    let space_ship_obj = Obj::load("./assets/mesh/spaceShip2.obj").expect("Failed to load obj");
    let planet_obj = Obj::load("./assets/mesh/sphere.obj").expect("Failed to load obj");
    let rings_obj = Obj::load("./assets/mesh/rings.obj").expect("Failed to load obj");
    
    let space_ship_vertices = Arc::new(space_ship_obj.get_vertex_array());
    let planet_vertices = Arc::new(planet_obj.get_vertex_array());
    let rings_vertices = Arc::new(rings_obj.get_vertex_array());
    
    // Create a list of models with one inline-defined SimpleModel
    let mut models: Vec<Box<dyn Model>> = vec![
        Box::new(SimpleModel {
            vertex_array: space_ship_vertices.clone(), // Clone the Arc
            shader: simple_shader,
            position: Vec3::new(0.0, 10.0, 55.0),
            scale: 1.0,
            rotation: Vec3::new(0.0, 0.0, 0.0),
            collision_radius: 5.0,
        }),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            10.0,
            sun_shader,
            1.0,
            0.0,
            0.0,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            0
        )),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            1.0,
            crater_shader,
            15.0,
            0.0,
            0.0008,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            40
        )),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            2.0,
            earth_shader,
            25.0,
            0.0,
            0.0004,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            40
        )),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            2.0,
            saturn_shader,
            30.0,
            0.0,
            0.0001,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            40
        )),
        Box::new(Planet::new(
            rings_vertices.clone(), // Clone the Arc
            2.0,
            saturn_ring_shader,
            30.0,
            0.0,
            0.0001,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            40
        )),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            4.0,
            vortex_shader,
            38.0,
            0.0,
            0.0002,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            40
        )),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            4.0,
            hypnos_shader,
            45.0,
            0.0,
            0.0001,
            3.0,
            Vec3::new(0.0, 0.0, 0.0),
            40
        )),
        Box::new(Planet::new(
            planet_vertices.clone(), // Clone the Arc
            4.0,
            pluto_shader,
            60.0,
            0.0,
            0.00015,
            3.0,
            Vec3::new(10.0, 0.0, 5.0),
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
    const LIGHT_ROTATION: f32 = PI / 20.0; // Light rotation angle for subject

    // camera orbit controls
    if window.is_key_down(Key::B) {
        camera.toogle_bird_view();
    }

    // camera orbit controls
    if window.is_key_down(Key::D) {
        camera.orbit(-ROTATION_SPEED, 0.0);
    }
    if window.is_key_down(Key::A) {
        camera.orbit(ROTATION_SPEED, 0.0);
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

    if camera.is_bird_view{
        return
    }

    let mut subject_position = subject.get_position();
    let mut subject_rotation = subject.get_rotation(); // Assuming `get_rotation` returns the current rotation

    // Model controls with rotation
    if window.is_key_down(Key::J) { // Move left
        subject_position.x -= TRANSLATE_STEP;
        subject_rotation.z = LIGHT_ROTATION; // Rotate counter-clockwise around Y-axis
        subject.set_position(subject_position);
        subject.set_rotation(subject_rotation);
        camera.change_center(subject_position);
    }
    if window.is_key_down(Key::L) { // Move right
        subject_position.x += TRANSLATE_STEP;
        subject_rotation.z = -LIGHT_ROTATION; // Rotate clockwise around Y-axis
        subject.set_position(subject_position);
        subject.set_rotation(subject_rotation);
        camera.change_center(subject_position);
    }
    if window.is_key_down(Key::K) { // Move back
        subject_position.z += TRANSLATE_STEP;
        subject_rotation.x = LIGHT_ROTATION; // Rotate upwards around X-axis
        subject.set_position(subject_position);
        subject.set_rotation(subject_rotation);
        camera.change_center(subject_position);
    }
    if window.is_key_down(Key::I) { // Move forward
        subject_position.z -= TRANSLATE_STEP;
        subject_rotation.x = -LIGHT_ROTATION; // Rotate downwards around X-axis
        subject.set_position(subject_position);
        subject.set_rotation(subject_rotation);
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
    // Model controls with rotation
    if window.is_key_released(Key::J) ||
    window.is_key_released(Key::L) ||
    window.is_key_released(Key::I) ||
    window.is_key_released(Key::K)
    { // Move left
        subject.set_rotation(Vec3::zeros());// Rotate counter-clockwise around Y-axis
    }
}

