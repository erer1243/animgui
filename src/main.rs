/*
This file contains window and imgui initialization code along with the main event loop
used to render the UI and models, control camera position and angle, etc. There should be zero
glium rendering code outside of this file. All mesh rendering is done here.

TODO:
    * Adjustable mouse speed
    * Improve mouse input in general
    * Adjustable camera move speed
    * Add axis markers, like in blender
*/
// #![allow(dead_code)]
// #![allow(unused_mut)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

mod camera;
mod controls;
mod mesh;
mod object;
mod project;
mod shaders;
mod vertex;

use camera::Camera;
use controls::CameraControls;
use glium::{glutin, program, uniform, Surface};
use glutin::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        ElementState::*,
        Event,
        MouseButton::Right,
        WindowEvent::{CloseRequested, CursorMoved, KeyboardInput, MouseInput},
    },
    event_loop::ControlFlow,
};
use nalgebra_glm as glm;
use object::Object;
use project::Project;
use std::time::Instant;
use vertex::Vertex;

fn main() {
    // Make window
    // =============================================================================================
    let event_loop = glutin::event_loop::EventLoop::new();
    let display = {
        let context = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_depth_buffer(24);
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Anim")
            .with_inner_size(PhysicalSize {
                width: 1000,
                height: 1000,
            })
            .with_resizable(false)
            .with_visible(false);

        glium::Display::new(window_builder, context, &event_loop)
            .expect("Failed to create glium display")
    };

    // Make imgui
    // =============================================================================================
    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display)
        .expect("Failed to create imgui renderer");

    imgui.set_ini_filename(None);
    platform.attach_window(
        imgui.io_mut(),
        &display.gl_window().window(),
        imgui_winit_support::HiDpiMode::Locked(1.0),
    );

    // Load rendering stuff
    // =============================================================================================
    // Make shader program
    let program = program!(&display,
        330 => {
            vertex: shaders::VERT_SHADER,
            fragment: shaders::FRAG_SHADER
        }
    )
    .expect("Failed to compile shaders");

    // Make triangle
    // let triangle = Object::new(
    //     &display,
    //     Some("Triangle"),
    //     &vertices!(-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0),
    //     &[0, 1, 2],
    // );

    display.gl_window().window().set_visible(true);

    // Main loop state
    // For imgui to know how long between frames
    let mut last_frame = Instant::now();

    // For camera control
    let mut controls = CameraControls::default();

    // For rendering from camera perspective
    let mut camera = Camera::new();

    // For keeping track of project data
    let project = Project::default();
    // project
    //     .load_obj_from_file(&display, "cube.obj")
    //     .expect("Loading cube.obj");

    // Main loop
    // =============================================================================================
    event_loop.run(move |event, _, control_flow| match event {
        // Draw frame
        // =========================================================================================
        Event::RedrawRequested(_) => {
            // Move camera based on WASD Shift and Space inputs
            if controls.rmb_held() {
                camera.move_up(controls.up_movement() / 100.);
                camera.move_right(controls.right_movement() / 100.);
                camera.move_forward(controls.forward_movement() / 100.);
            }

            // Do imgui drawing
            let mut ui = imgui.frame();
            draw_ui(&mut ui);
            platform.prepare_render(&ui, display.gl_window().window());

            // Clear frame buffer
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.2, 0.2, 1.0), 1.);

            let draw_params = glium::draw_parameters::DrawParameters {
                // Specify depth buffer functionality
                depth: glium::Depth {
                    test: glium::DepthTest::IfLess,
                    write: true,
                    ..Default::default()
                },
                ..Default::default()
            };

            // Draw objects in project
            for obj in project.objs.iter() {
                let uniforms = uniform! {
                    color: obj.color,
                    matrix: mat4_to_array(&(camera.camera_mat() * obj.model_mat()))
                };

                let mesh = &project.meshes[&obj.mesh_id];

                target
                    .draw(&mesh.vb, &mesh.ib, &program, &uniforms, &draw_params)
                    .unwrap();
            }

            // Draw imgui ui
            renderer.render(&mut target, ui.render()).unwrap();

            // Swap frames
            target.finish().unwrap();
        }

        // Handle inputs for camera movement
        // =========================================================================================
        // Handle pressing/release RMB
        Event::WindowEvent {
            event:
                MouseInput {
                    button: Right,
                    state,
                    ..
                },
            ..
        } => {
            let gl_window = display.gl_window();
            let window = gl_window.window();

            match state {
                Pressed => {
                    // Hide and grab mouse cursor
                    controls.grab();
                    window.set_cursor_grab(true).unwrap();
                    window.set_cursor_visible(false);
                }

                Released => {
                    // Show and release mouse cursor
                    controls.release();
                    window.set_cursor_grab(false).unwrap();
                    window.set_cursor_visible(true);
                }
            }
        }

        // Handle right click drag to move camera
        Event::WindowEvent {
            event: CursorMoved { position, .. },
            ..
        } if controls.rmb_held() => {
            // Get distance mouse moved
            let (dx, dy) = controls.mouse_moved(position.x, position.y);

            // Move camera
            camera.add_angle(dx / 50., dy / 50.);

            // Move mouse back to center of screen
            controls.mouse_moved(500., 500.);
            display
                .gl_window()
                .window()
                .set_cursor_position(PhysicalPosition::new(500., 500.))
                .unwrap();
        }

        // Handle pressing WASD to move camera
        Event::WindowEvent {
            event: KeyboardInput { input, .. },
            ..
        } => {
            if controls.rmb_held() {
                let is_pressed = input.state == Pressed;

                match input.scancode {
                    17 => controls.w_input(is_pressed),
                    30 => controls.a_input(is_pressed),
                    31 => controls.s_input(is_pressed),
                    32 => controls.d_input(is_pressed),
                    42 => controls.shift_input(is_pressed),
                    57 => controls.space_input(is_pressed),
                    _ => (),
                }
            }
        }

        // Handle exiting the program
        // =========================================================================================
        Event::WindowEvent {
            event: CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,

        // Misc event handling for imgui
        // =========================================================================================
        // Update imgui internal frame time
        Event::NewEvents(_) => {
            let now = Instant::now();
            imgui.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }

        // Pre-frame imgui stuff, and request new frame be drawn (Winit does not automatically
        // request a new frame every time vsync could take one. Must be manually requested).
        Event::MainEventsCleared => {
            platform
                .prepare_frame(imgui.io_mut(), &display.gl_window().window())
                .unwrap();
            display.gl_window().window().request_redraw();
        }

        // Pass events to imgui if the user is not controlling camera
        ev if !controls.rmb_held() => {
            platform.handle_event(imgui.io_mut(), display.gl_window().window(), &ev)
        }

        _ => (),
    });
}

fn draw_ui(ui: &mut imgui::Ui) {
    use imgui::*;

    Window::new(im_str!("Hello, window!"))
        .position([0., 0.], Condition::Appearing)
        .size([150., 80.], Condition::Appearing)
        .build(&ui, || {});
}

fn mat4_to_array(m: &glm::Mat4) -> [[f32; 4]; 4] {
    [
        [m[0], m[1], m[2], m[3]],
        [m[4], m[5], m[6], m[7]],
        [m[8], m[9], m[10], m[11]],
        [m[12], m[13], m[14], m[15]],
    ]
}
