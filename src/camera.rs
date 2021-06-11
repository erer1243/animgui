/*
This file contains the Camera struct, used to hold and update all information related to the
imaginary camera used to render 3D structures.

TODO:
    * Zoom functionality (fov update?)
*/
use nalgebra_glm as glm;
use std::f32::consts::PI;

#[derive(Default)]
pub struct Camera {
    // Camera position
    pos: glm::Vec3,

    // Angles that the camera is facing
    pitch: f32,
    yaw: f32,

    // Aspect ratio, used for calculating projection matrix, window's width/height
    aspect_ratio: f32,

    // Position to look at, just in front of the camera
    // calculated from pitch and yaw
    front: glm::Vec3,

    // Position just to the right of the camera
    // Used for moving left/right with keyboard
    right: glm::Vec3,

    // Whether to update respective matrices when camera_mat() is called
    update_view_mat: bool,
    update_proj_mat: bool,

    // Rendering matrices
    // camera_mat is projection_matrix * view_matrix (calculated in camera_mat())
    camera_matrix: glm::Mat4,
    view_matrix: glm::Mat4,
    projection_matrix: glm::Mat4,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pos: glm::vec3(2., 0., -1.),
            yaw: 140.,
            aspect_ratio: 1.,
            update_view_mat: true,
            update_proj_mat: true,
            ..Camera::default()
        }
    }

    // Generate (if needed) and return matrix that adjusts for camera position and projection
    pub fn camera_mat(&mut self) -> &glm::Mat4 {
        if self.update_view_mat {
            // Reference: https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/7.3.camera_mouse_zoom/camera_mouse_zoom.cpp
            // Go to function mouse_callback in above link
            let yr = self.yaw * PI / 180.;
            let pr = self.pitch * PI / 180.;
            let (ys, yc) = (yr.sin(), yr.cos());
            let (ps, pc) = (pr.sin(), pr.cos());

            self.front = glm::vec3(yc * pc, ps, ys * pc).normalize();
            self.right = self.front.cross(&up()).normalize();

            self.view_matrix = glm::look_at(
                &self.pos,                // Position of camera
                &(self.pos + self.front), // Look-at target
                &up(),                    // Univeral up
            );
        }

        if self.update_proj_mat {
            self.projection_matrix = glm::perspective(
                self.aspect_ratio,   // Aspect ratio
                65_f32.to_radians(), // Y axis fov
                0.1,                 // Z near
                100.,                // Z far
            );
        }

        if self.update_view_mat || self.update_proj_mat {
            self.camera_matrix = self.projection_matrix * self.view_matrix;
            self.update_view_mat = false;
            self.update_proj_mat = false;
        }

        &self.camera_matrix
    }

    // Set the aspect ratio, width / height of window
    pub fn update_aspect_ratio(&mut self, n: f32) {
        self.update_proj_mat = true;
        self.aspect_ratio = n;
    }

    // Move the camera the given amount left/right
    pub fn move_right(&mut self, n: f32) {
        self.update_view_mat = true;
        self.pos += n * self.right;
    }

    // Move the camera the given amount up/down (not relative to camera angle)
    pub fn move_up(&mut self, n: f32) {
        self.update_view_mat = true;
        self.pos += n * up();
    }

    // Move the camera the given amount forward/backward
    pub fn move_forward(&mut self, n: f32) {
        self.update_view_mat = true;
        self.pos += n * self.front;
    }

    // Rotates the camera by the given amounts
    pub fn add_angle(&mut self, yaw: f32, pitch: f32) {
        self.update_view_mat = true;
        self.yaw += yaw;
        self.pitch -= pitch;

        if self.pitch > 89. {
            self.pitch = 89.;
        } else if self.pitch < -89. {
            self.pitch = -89.;
        }
    }
}

// Clarity alias for camera up direction
fn up() -> glm::Vec3 {
    glm::vec3(0., 1., 0.)
}
