use glium::{
    implement_vertex, index::PrimitiveType::TrianglesList, Display, IndexBuffer, VertexBuffer,
};
use nalgebra_glm as glm;
use std::f32::consts::PI;

implement_vertex!(Vertex, position);
#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
}

#[macro_export]
macro_rules! vertices {
    ($($n1:expr, $n2:expr, $n3:expr),+) => {
        [
            $(Vertex { position: [$n1, $n2, $n3 ] }),+
        ]
    };
}

// Drawable object
pub struct Object {
    pub position: glm::Vec3,
    pub rotation: glm::Vec3,
    pub scale: glm::Vec3,
    pub color: [f32; 3],
    pub vb: VertexBuffer<Vertex>,
    pub ib: IndexBuffer<u16>,
}

impl Object {
    pub fn new(display: &Display, data: &[Vertex], indices: &[u16]) -> Object {
        let vb = VertexBuffer::new(display, data).unwrap();
        let ib = IndexBuffer::new(display, TrianglesList, indices).unwrap();

        Object {
            position: glm::zero(),
            rotation: glm::zero(),
            scale: glm::vec3(1., 1., 1.),
            color: [1.; 3],
            vb,
            ib,
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
