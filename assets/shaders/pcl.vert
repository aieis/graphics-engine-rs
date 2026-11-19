#version 450
#extension GL_ARB_separate_shader_objects : enable

#include "utils/common.glsl"

layout (location = 0) in vec3 proj;
layout (location = 1) in vec3 col;
layout (location = 2) in float dist;

layout (location = 0) out vec3 vs_color;

void main()
{
    float norm = dist / 10000;
    vec4 pos = vec4(proj.x * norm, proj.y * norm, norm, 1.0);
    vec4 proj_pos = G.Projection * G.View * pos;

    // gl_Position = vec4(proj.x / 2, proj.y / 2, 0.0, 1.0);
    gl_Position = proj_pos;


    if (abs(norm) < 0.01) {
        gl_PointSize = 0;
    } else {
        gl_PointSize =  0.5 / norm;
    }
    vs_color = col; //vec3(1.0, 0.0, 0.0);
}
