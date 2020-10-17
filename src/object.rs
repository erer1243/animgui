/*
This file contains the Object struct, representing one renderable 3D mesh along with attributes
such as position, rotation, scale, color, etc. Contains all buffers and uniforms required for
rendering the object.

TODO:
    * Getters and setters for rendering attributes that will cause model_mat to be generated
      only when needed, rather than for every frame
    * Keyframe animating. Maybe this should go in the object struct?
*/
use crate::mesh::Mesh;
use nalgebra_glm as glm;

pub struct Object {
    // Rendering attributes
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
    pub color: [f32; 3],

    // Mesh data
    pub mesh: Mesh,
}

impl Object {
    pub fn new<T: ToString>(mesh: Mesh, indices: &[u16]) -> Object {
        Object {
            position: glm::zero(),
            rotation: glm::zero(),
            scale: glm::vec3(1., 1., 1.),
            color: [1.; 3],
            mesh,
        }
    }

    pub fn model_mat(&self) -> glm::Mat4 {
        let mut matrix = glm::identity();
        matrix = glm::translate(&matrix, &self.position);
        matrix = glm::rotate_x(&matrix, self.rotation.x);
        matrix = glm::rotate_y(&matrix, self.rotation.y);
        matrix = glm::rotate_z(&matrix, self.rotation.z);
        matrix = glm::scale(&matrix, &self.scale);
        matrix
    }
}
