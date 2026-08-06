#version 450

#extension GL_ARB_separate_shader_objects : enable


layout(location = 0) in vec2 TexCoord;

layout(binding = 0) uniform sampler2D Texture;

layout(location = 0) out vec4 FragColor;

void main()
{
    vec4 colour = texture(Texture, TexCoord);

	FragColor = colour; //vec4(colour.a, colour.a, colour.a, 1.0);

}
