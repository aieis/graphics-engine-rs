#version 450

#extension GL_ARB_separate_shader_objects : enable


layout(location = 0) in vec2 TexCoord;
layout(location = 1) in vec3 TextColor;

layout(set = 0, binding = 0) uniform sampler2D FontAtlas;

layout(location = 0) out vec4 FragColor;

void main()
{
    vec4 color = texture(FontAtlas, TexCoord);

    float v = color.a;

	// There is no transparency yet

	vec3 bg_col = vec3(0, 0, 0); //vec3(0.3, 0.3, 0.3);

	vec3 col = color.a * TextColor + (1.0 - color.a) * bg_col;

	FragColor = vec4(col, 1.0);
}
