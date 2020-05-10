#version 330 core

in VS_OUTPUT {
    vec3 Normal;
    vec2 TexCoords;
} IN;

out vec4 Color;

//
void main()
{
    vec3 c1 = vec3(0.0f, 1.0f, 0.0f);
    vec3 c2 = vec3(1.0f, 1.0f, 0.0f);

    float intensity = dot(IN.Normal,  normalize (-vec3(-6., -6., -6.)) );

    if( (int(IN.TexCoords.x*10.) + int(IN.TexCoords.y*10.)) % 2  == 0 ) {
        Color = vec4(c1 * intensity + IN.TexCoords.x, 1.0f);
    } else {
        Color = vec4(c2 * intensity + IN.TexCoords.x, 1.0f);
    }

}