/*
This file contains source code for GLSL shaders used for rendering.
Currently they only provide solid-color rendering.

TODO:
    * Lighting (see vertex.rs too)
    * Texturing (see vertex.rs too)
*/
pub const VERT_SHADER: &str = r#"
#version 330 core

layout (location = 0) in vec3 position;
uniform mat4 matrix;
uniform vec3 color;
out vec3 color_;

void main() { color_ = (position+1)/2;
              gl_Position = matrix * vec4(position, 1.0); }
"#;

pub const FRAG_SHADER: &str = r#"
#version 330 core

in vec3 color_;
out vec4 color;

void main() { color = vec4(color_, 1.0); }
"#;
