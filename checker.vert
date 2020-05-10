#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoords;

out VS_OUTPUT {
    vec3 Normal;
    vec2 TexCoords;
} OUT;

uniform mat4 VP;
uniform vec3 u_position;

void main() {
    gl_Position =  VP * vec4(Position + u_position, 1.0);
    OUT.Normal = Normal;
    OUT.TexCoords = TexCoords;
}