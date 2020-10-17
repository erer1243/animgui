/*
This file contains the Project struct which contains state related to what is currently being
animated and how.

TODO:
*/
use crate::{mesh::Mesh, Object, Vertex};
use glium::Display;
use std::{fs::File, io::BufReader, path::Path};

// Project state
#[derive(Default)]
pub struct Project {
    pub meshes: Vec<Mesh>,
    pub objs: Vec<Object>,
}

impl Project {
    pub fn load_mesh_from_file<P>(&mut self, display: &Display, path: P) -> Result<(), String>
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

        // Create name for mesh
        let name = format!(
            "{} [{:?}]",
            obj.name.as_deref().unwrap_or("Nameless Mesh"),
            path.as_ref()
        );

        // Create mesh and object and add to project
        let _mesh = Mesh::new(display, name, &vertices, &obj.indices);

        Ok(())
    }
}
