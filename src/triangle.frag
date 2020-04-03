#version 330 core

in VS_OUTPUT {
    vec3 Color;
} IN;

out vec4 Color;

uniform float offset;

void main()
{
    Color = vec4(IN.Color*offset*.01, 1.0f);
}