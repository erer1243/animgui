/*
This file contains the Vertex struct used as a vertex when rendering through glium. For each Vertex,
the vertex shader is run once.
Currently the struct only contains the position of a vertex.

TODO:
    * Lighting data (see shaders.rs too)
    * Texturing data (see shaders.rs too)
*/
use glium::implement_vertex;

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
