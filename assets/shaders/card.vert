#version 450
#extension GL_ARB_separate_shader_objects : enable

#include "utils/camera.glsl"
#include "utils/common.glsl"

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;
layout(location = 2) in vec3 normals;

layout(push_constant) uniform Params
{
    float Aspect;
} P;


vec3 light = vec3(-1, 0, -1);

#define PI 3.141592653589793

void main() {

    float FOV   = PI / 3;

    mat4 view = create_view_matrix(G.CamPos, G.CamDir, G.CamUp);

    vec4 world_pos = view * vec4(pos, 1.0);

    mat4 proj = create_projection_matrix(FOV, P.Aspect);
    vec4 proj_pos = proj * world_pos;

    gl_Position = proj_pos;

    float light_cos = dot(normals,light);
    float alpha = ((light_cos * -1) + 1) / 2;
    float dark_factor = 0.8 * alpha;
    frag_color = 0.9 * vec3(col.x, col.y, col.z) * ( 1 - dark_factor);
}
