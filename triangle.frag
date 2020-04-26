#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;

out vec4 Color;
uniform float luma;

//
void main()
{
    Color = vec4(IN.Color*luma, 1.0f);
}