/*
This file contains the Project struct which contains state related to the meshes loaded (vertex
data for a model) and the objects (instance of a mesh with position, rotation, scale, etc) held in
the current working project.

TODO:
*/
use crate::{mesh::Mesh, object::Object, vertex::Vertex};
use glium::Display;
use imgui::ImString;
use std::{fs::File, io::BufReader, path::Path};

#[derive(Default)]
pub struct Project {
    pub meshes: Vec<Mesh>,
    pub objs: Vec<Object>,
}

impl Project {
    pub fn load_mesh_from_file<P>(&mut self, display: &Display, path: P) -> Result<usize, String>
    where
        P: AsRef<Path>,
    {
        // Load obj from file
        let input = BufReader::new(File::open(path.as_ref()).map_err(|e| e.to_string())?);
        let obj: obj::Obj<obj::Position, u16> = obj::load_obj(input).map_err(|e| e.to_string())?;

        // Convert obj vertices into our Vertex type
        let vertices: Vec<Vertex> = obj
            .vertices
            .iter()
            .map(|p| Vertex {
                position: p.position,
            })
            .collect();

        // Create name for object and mesh
        let mesh_name_str = format!(
            "{} [{:?}]",
            obj.name.as_deref().unwrap_or("Nameless Mesh"),
            path.as_ref()
        )
        .chars()
        .map(ascii_or_qmark)
        .collect::<String>();

        let mesh_name = unsafe { ImString::from_utf8_unchecked(mesh_name_str.into()) };

        // Create mesh and object and add to project
        let mesh = Mesh::new(display, mesh_name, &vertices, &obj.indices);
        self.meshes.push(mesh);

        Ok(self.meshes.len() - 1)
    }
}

// Utility function for converting unprintable chars into '?'
fn ascii_or_qmark(c: char) -> char {
    if (c as u32) > 31 && (c as u32) < 127 {
        c
    } else {
        '?'
    }
}
