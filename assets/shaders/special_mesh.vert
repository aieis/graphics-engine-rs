#version 450
#extension GL_ARB_separate_shader_objects : enable

#include "utils/camera.glsl"
#include "utils/common.glsl"

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;
layout(location = 2) in vec3 normals;

layout(set = 1, binding = 0) uniform Shared
{
    float Time;
    float Aspect;
    float GlobalCamera;
} S;


vec3 light = vec3(-1, 0, -1);

#define TARGET_COL_COLOUR  0
#define TARGET_COL_NORMAL  1
#define TARGET_COL_DEPTH   2

#define TARGET 0


#define PI 3.141592653589793


void main() {

    float time   = S.Time;
    float period = radians(180) / 10;
    float ST = sin(time*period/2);
    float CT = cos(time*period/2);

    float STO = 1 - ST*ST;
    vec3  camera_pos = vec3(0, 0, 7);
    vec3  camera_dir = normalize(vec3(ST, 0, CT));
    vec3  camera_up  = vec3(0, 1, 0);

    if (S.GlobalCamera > 0) {
        camera_pos = G.CamPos;
        camera_dir = G.CamDir;
        camera_up  = G.CamUp;
    }

    float FOV   = PI / 4;

    mat4 view = create_view_matrix(camera_pos, camera_dir, camera_up);

    vec4 world_pos = view * vec4(pos, 1.0);
    world_pos.z *= -1;


    mat4 proj = create_projection_matrix(FOV, S.Aspect);
    vec4 proj_pos = proj * world_pos;

    gl_Position = proj_pos;

    if (TARGET == TARGET_COL_COLOUR) {
        float light_cos = dot(normals,light);
        float alpha = ((light_cos * -1) + 1) / 2;
        float dark_factor = 0.8 * alpha;

        frag_color = 0.2 * vec3(col.x, col.y, col.z) * ( 1 - dark_factor);

    } else if (TARGET == TARGET_COL_DEPTH) {
        float dz_col = (world_pos.z - camera_pos.z - 1) / 8.0;
        frag_color = vec3(dz_col, dz_col, dz_col);

    } else {
        frag_color = normals;
    }
}
