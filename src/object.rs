/*
This file contains the Object struct, representing one renderable 3D mesh along with attributes
such as position, rotation, scale, color, etc. Contains all buffers and uniforms required for
rendering the object.

TODO:
    * Getters and setters for rendering attributes that will cause model_mat to be generated
      only when needed, rather than for every frame
    * Keyframe animating. Maybe this should go in the object struct?
*/
use crate::animation::{Frame, KeyframeV3};
use crate::mesh::Mesh;
use glm::Vec3;
use imgui::{ImStr, ImString};
use nalgebra_glm as glm;

pub struct Object {
    // Name (stored as ImString for imgui)
    pub name: ImString,

    // Mesh data
    pub mesh: Mesh,

    // Rendering attributes
    pub position: KeyframeV3,
    pub rotation: KeyframeV3,
    pub scale: KeyframeV3,
}

impl Object {
    pub fn new(name: ImString, mesh: Mesh) -> Object {
        Object {
            name,
            position: KeyframeV3::new(Vec3::zeros()),
            rotation: KeyframeV3::new(Vec3::zeros()),
            scale: KeyframeV3::new(Vec3::new(1., 1., 1.)),
            mesh,
        }
    }

    pub fn model_mat_at(&self, frame: Frame) -> glm::Mat4 {
        let position = self.position.at(frame);
        let rotation = self.rotation.at(frame);
        let scale = self.scale.at(frame);

        let mut matrix = glm::identity();
        matrix = glm::translate(&matrix, &position);
        matrix = glm::rotate_x(&matrix, rotation.x);
        matrix = glm::rotate_y(&matrix, rotation.y);
        matrix = glm::rotate_z(&matrix, rotation.z);
        matrix = glm::scale(&matrix, &scale);
        matrix
    }

    pub fn name_imstr(&self) -> &ImStr {
        &self.name
    }
}
