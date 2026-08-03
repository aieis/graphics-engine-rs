#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;
layout(location = 2) in vec3 normals;

void main() {

    vec3  camera_pos = vec3(0, 0, 1.5);

    float dz         = (pos - camera_pos).z;

    if (dz == 0) {
        dz = 0.1;
    }

    // // x' = x * focal_length / dz
    float focal_length = 1;

    float f = - focal_length / dz;

    mat3 proj = mat3 ( f,  0,  0,
                       0,  f,  0,
                       0,  0,  1 );

    vec3  proj_pos   = proj * pos;

    gl_Position = vec4(proj_pos, 1.0);

    frag_color = col;
}
