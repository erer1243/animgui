/*
This file contains the MeshInternals struct, which represents a vertex buffer, and index buffer,
and an associated name. The MeshInternals struct is wrapped in an Rc in the Mesh struct to provide
cloning.

TODO: (these structs  are pretty simple, probably will not need any new features)
*/
use std::rc::Rc;
use std::ops::Deref;
use crate::vertex::Vertex;
use glium::{index::PrimitiveType::TrianglesList, Display, IndexBuffer, VertexBuffer};

#[derive(Clone)]
pub struct Mesh(Rc<MeshInternals>);

impl Mesh {
    pub fn new(display: &Display, name: String, verts: &[Vertex], inds: &[u16]) -> Mesh {
        let vb = VertexBuffer::new(display, verts).unwrap();
        let ib = IndexBuffer::new(display, TrianglesList, inds).unwrap();
        Mesh(Rc::new(MeshInternals { name, vb, ib }))
    }
}

impl Deref for Mesh {
    type Target = MeshInternals;

    fn deref(&self) -> &MeshInternals {
        &self.0
    }
}

pub struct MeshInternals {
    pub name: String,
    pub vb: VertexBuffer<Vertex>,
    pub ib: IndexBuffer<u16>,
}
