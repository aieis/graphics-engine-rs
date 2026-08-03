#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) in vec3 proj;
layout (location = 1) in float dist;

layout (location = 0) out vec3 vs_color;

void main()
{
    float norm = dist / 1000;
    gl_Position = vec4(proj.x * norm, proj.y * norm, norm, 1.0);
    if (abs(norm) < 0.01) {
        gl_PointSize = 0;
    } else {
        gl_PointSize =  0.5 / norm;
    }
    vs_color = vec3(1.0, 0.0, 0.0);
}
