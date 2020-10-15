#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod shaders;
mod object;

use glium::{glutin, program, uniform, Surface};
use glutin::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        ElementState::*,
        Event,
        MouseButton::Right,
        WindowEvent::{CloseRequested, CursorMoved, MouseInput},
    },
    event_loop::ControlFlow,
};
use nalgebra_glm as glm;
use std::{f32::consts::PI, time::Instant};
use object::{Object, Vertex};

/*
Todo list
    * Mouse speed adjustment
    * Add an axis marker at origin
    * Add click to select model functionality
*/

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
    let triangle = Object::new(
        &display,
        &vertices!(-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0),
        &[0, 1, 2],
    );

    let mut cube = Object::new(
        &display,
        &vertices!(
            -1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0,
            -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0
        ),
        &[
            0, 1, 2, 2, 3, 0, 1, 5, 6, 6, 2, 1, 7, 6, 5, 5, 4, 7, 4, 0, 3, 3, 7, 4, 4, 5, 1, 1, 0,
            4, 3, 2, 6, 6, 7, 3,
        ],
    );

    display.gl_window().window().set_visible(true);

    // Main loop state
    // For imgui to know how long between frames
    let mut last_frame = Instant::now();

    // For fun shapes
    // let start = Instant::now();

    // For camera control
    let mut mouse = CameraMouse::default();

    // For rendering from camera perspective
    let mut camera = Camera::new();

    // Main loop
    // =============================================================================================
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            // Do imgui drawing
            let mut ui = imgui.frame();
            draw_ui(&mut ui);
            platform.prepare_render(&ui, display.gl_window().window());

            // Clear frame buffer
            let mut target = display.draw();
            target.clear_color_and_depth((0.0, 0.2, 0.2, 1.0), 1.);

            let matrix = camera.camera_mat() * triangle.model_mat();

            let uniforms = uniform! {
                color: triangle.color,
                matrix: mat4_to_array(&matrix),
            };

            target
                .draw(
                    &triangle.vb,
                    &triangle.ib,
                    &program,
                    &uniforms,
                    &glium::draw_parameters::DrawParameters {
                        // Specify depth buffer functionality
                        depth: glium::Depth {
                            test: glium::DepthTest::IfLess,
                            write: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )
                .unwrap();

            // Draw imgui ui
            renderer.render(&mut target, ui.render()).unwrap();

            // Swap frames
            target.finish().unwrap();
        }

        // Handle right click press (to move camera)
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
                    mouse.grab();
                    window.set_cursor_grab(true).unwrap();
                    window.set_cursor_visible(false);
                }

                Released => {
                    // Show and release mouse cursor
                    mouse.release();
                    window.set_cursor_grab(false).unwrap();
                    window.set_cursor_visible(true);
                }
            }
        }

        // Handle right click drag (to move camera)
        Event::WindowEvent {
            event: CursorMoved { position, .. },
            ..
        } if mouse.right_held => {
            // Get distance mouse moved
            let (dx, dy) = mouse.moved(position.x, position.y);

            // Move camera
            camera.add_angle(dx / 10., dy / 10.);

            // Move mouse back to center of screen
            mouse.moved(500., 500.);
            display
                .gl_window()
                .window()
                .set_cursor_position(PhysicalPosition::new(500., 500.))
                .unwrap();
        }

        // Misc event handling for imgui
        Event::NewEvents(_) => {
            let now = Instant::now();
            imgui.io_mut().update_delta_time(now - last_frame);
            last_frame = now;
        }

        // Misc event handling for imgui
        Event::MainEventsCleared => {
            platform
                .prepare_frame(imgui.io_mut(), &display.gl_window().window())
                .unwrap();
            display.gl_window().window().request_redraw();
        }

        // Handle exiting the program
        Event::WindowEvent {
            event: CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,

        // Pass events to imgui if not controlling camera
        ev if !mouse.right_held => {
            platform.handle_event(imgui.io_mut(), display.gl_window().window(), &ev)
        }

        _ => (),
    });
}

fn mat4_to_array(m: &glm::Mat4) -> [[f32; 4]; 4] {
    [
        [m[0], m[1], m[2], m[3]],
        [m[4], m[5], m[6], m[7]],
        [m[8], m[9], m[10], m[11]],
        [m[12], m[13], m[14], m[15]],
    ]
}


#[derive(Default)]
struct CameraMouse {
    // Whether RMB is being held at the moment
    right_held: bool,

    // If the position of the mouse has been captured since RMB started being held
    have_pos: bool,

    // The current position of the mouse
    pos: Option<(f64, f64)>,
}

impl CameraMouse {
    fn grab(&mut self) {
        self.right_held = true;
    }

    fn release(&mut self) {
        self.right_held = false;
        self.pos = None;
    }

    fn moved(&mut self, xpos: f64, ypos: f64) -> (f32, f32) {
        match self.pos {
            Some((prev_x, prev_y)) => {
                let dx = xpos - prev_x;
                let dy = ypos - prev_y;
                self.pos = Some((xpos, ypos));
                (dx as f32, dy as f32)
            }

            None => {
                self.pos = Some((xpos, ypos));
                (0., 0.)
            }
        }
    }
}

#[derive(Default)]
struct Camera {
    // Camera position
    pos: glm::Vec3,

    // Angles that the camera is facing
    pitch: f32,
    yaw: f32,

    // Position to look at, just in front of the camera
    // calculated from pitch and yaw
    front: glm::Vec3,

    // Whether to update mats when camera_mat() is called
    update_mats: bool,

    // Rendering matrices
    // proj_mat is projection matrix
    // camera_mat is proj_mat * view_matrix (calculated in camera_mat())
    camera_mat: glm::Mat4,
    proj_mat: glm::Mat4,
}

impl Camera {
    fn new() -> Camera {
        let proj_mat = glm::perspective(
            1.,                 // Aspect ratio
            65f32.to_radians(), // Y axis fov
            0.1,                // Z near
            100.,               // Z far
        );

        let mut cam = Camera::default();
        cam.proj_mat = proj_mat;
        cam.add_position(2., 0., -1.);
        cam.add_angle(-208., 0.);
        cam
    }

    fn camera_mat(&mut self) -> &glm::Mat4 {
        if self.update_mats {
            self.update_mats = false;
            self.camera_mat = self.proj_mat
                * glm::look_at(&self.pos, &(self.pos + self.front), &glm::vec3(0., 1., 0.));
        }

        &self.camera_mat
    }

    fn add_position(&mut self, dx: f32, dy: f32, dz: f32) {
        self.update_mats = true;
        self.pos += glm::vec3(dx, dy, dz);
    }

    fn add_angle(&mut self, yaw: f32, pitch: f32) {
        self.update_mats = true;
        self.yaw += yaw;
        self.pitch -= pitch;

        if self.pitch > 89. {
            self.pitch = 89.;
        } else if self.pitch < -89. {
            self.pitch = -89.;
        }

        // Reference: https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/7.3.camera_mouse_zoom/camera_mouse_zoom.cpp
        // Go to function mouse_callback ^^^
        let yr = self.yaw * PI / 180.;
        let pr = self.pitch * PI / 180.;
        let (ys, yc) = (yr.sin(), yr.cos());
        let (ps, pc) = (pr.sin(), pr.cos());

        self.front = glm::vec3(yc * pc, ps, ys * pc).normalize();
    }
}

fn draw_ui(ui: &mut imgui::Ui) {
    use imgui::*;

    Window::new(im_str!("Hello, window!"))
        .position([0., 0.], Condition::Appearing)
        .size([150., 80.], Condition::Appearing)
        .build(&ui, || {});
}
