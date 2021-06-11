/*
This file contains the MeshInternals struct, which represents a vertex buffer, and index buffer,
and an associated name. The MeshInternals struct is wrapped in an Rc in the Mesh struct to provide
cloning.

TODO: (these structs  are pretty simple, probably will not need any new features)
*/
use crate::vertex::Vertex;
use glium::{index::PrimitiveType::TrianglesList, Display, IndexBuffer, VertexBuffer};
use imgui::{ImStr, ImString};
use std::{ops::Deref, rc::Rc};

#[derive(Clone)]
pub struct Mesh(Rc<MeshInternals>);

impl Mesh {
    pub fn new(display: &Display, name: ImString, verts: &[Vertex], inds: &[u16]) -> Mesh {
        let vb = VertexBuffer::new(display, verts).unwrap();
        let ib = IndexBuffer::new(display, TrianglesList, inds).unwrap();
        Mesh(Rc::new(MeshInternals { name, vb, ib }))
    }

    pub fn name_imstr(&self) -> &ImStr {
        &self.name
    }
}

impl Deref for Mesh {
    type Target = MeshInternals;

    fn deref(&self) -> &MeshInternals {
        &self.0
    }
}

pub struct MeshInternals {
    // Name (stored as ImString for imgui rendering)
    pub name: ImString,

    // GL Buffers
    pub vb: VertexBuffer<Vertex>,
    pub ib: IndexBuffer<u16>,
}
