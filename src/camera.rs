/*
This file contains the Camera struct, used to hold and update all information related to the
imaginary camera used to render 3D structures.

TODO:
    * Zoom functionality
    * Ability to update FOV in projection matrix
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

    // Position to look at, just in front of the camera
    // calculated from pitch and yaw
    front: glm::Vec3,

    // Position just to the right of the camera
    // Used for moving left/right with keyboard
    right: glm::Vec3,

    // Whether to update mats when camera_mat() is called
    update_mats: bool,

    // Rendering matrices
    // proj_mat is projection matrix
    // camera_mat is proj_mat * view_matrix (calculated in camera_mat())
    camera_mat: glm::Mat4,
    proj_mat: glm::Mat4,
}

impl Camera {
    pub fn new() -> Camera {
        let proj_mat = glm::perspective(
            1.,                 // Aspect ratio
            65f32.to_radians(), // Y axis fov
            0.1,                // Z near
            100.,               // Z far
        );

        let mut cam = Camera::default();
        cam.proj_mat = proj_mat;
        cam.pos = glm::vec3(2., 0., -1.);
        cam.add_angle(-208., 0.);
        cam
    }

    // Generate (if needed) and return matrix that adjusts for camera position and projection
    pub fn camera_mat(&mut self) -> &glm::Mat4 {
        if self.update_mats {
            self.update_mats = false;
            self.camera_mat =
                self.proj_mat * glm::look_at(&self.pos, &(self.pos + self.front), &up());
        }

        &self.camera_mat
    }

    // Move the camera the given amount left/right
    pub fn move_right(&mut self, n: f32) {
        self.update_mats = true;
        self.pos += n * self.right;
    }

    // Move the camera the given amount up/down (not relative to camera angle)
    pub fn move_up(&mut self, n: f32) {
        self.update_mats = true;
        self.pos += n * up();
    }

    // Move the camera the given amount forward/backward
    pub fn move_forward(&mut self, n: f32) {
        self.update_mats = true;
        self.pos += n * self.front;
    }

    // Rotates the camera by the given amounts
    pub fn add_angle(&mut self, yaw: f32, pitch: f32) {
        self.update_mats = true;
        self.yaw += yaw;
        self.pitch -= pitch;

        if self.pitch > 89. {
            self.pitch = 89.;
        } else if self.pitch < -89. {
            self.pitch = -89.;
        }

        // Reference: https://learnopengl.com/code_viewer_gh.php?code=src/1.getting_started/7.3.camera_mouse_zoom/camera_mouse_zoom.cpp
        // Go to function mouse_callback in above link
        let yr = self.yaw * PI / 180.;
        let pr = self.pitch * PI / 180.;
        let (ys, yc) = (yr.sin(), yr.cos());
        let (ps, pc) = (pr.sin(), pr.cos());

        self.front = glm::vec3(yc * pc, ps, ys * pc).normalize();
        self.right = self.front.cross(&up()).normalize();
    }
}

// Clarity alias for camera up direction
fn up() -> glm::Vec3 {
    glm::vec3(0., 1., 0.)
}
