#version 330 core

in VS_OUTPUT {
    vec3 Normal;
} IN;

out vec4 Color;

//
void main()
{
    vec3 c = vec3(0.0f, 1.0f, 0.0f);
    float intensity = dot(IN.Normal,  normalize (-vec3(-6., -6., -6.)) );
    Color = vec4(c * intensity, 1.0f);
}