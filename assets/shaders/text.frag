#version 450

#extension GL_ARB_separate_shader_objects : enable


layout(location = 0) in vec2 TexCoord;

layout(binding = 0) uniform sampler2D Texture;

layout(location = 0) out vec4 FragColor;

void main()
{
    vec4 color = texture(Texture, TexCoord);

    FragColor = vec4(0, 0, 0, color.a);
}
