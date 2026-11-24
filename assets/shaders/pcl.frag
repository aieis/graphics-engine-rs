#version 450

#extension GL_ARB_separate_shader_objects : enable

const uint COLOURS_LEN = 16;

const vec3 COLOURS[COLOURS_LEN] = {
    vec3(1.0, 1.0, 1.0),
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0),
    vec3(1.0, 1.0, 0.0),
    vec3(1.0, 0.0, 1.0),
    vec3(0.0, 1.0, 1.0),
    vec3(1.0, 0.5, 0.0),
    vec3(0.5, 0.0, 1.0),
    vec3(0.0, 1.0, 0.5),
    vec3(1.0, 0.0, 0.5),
    vec3(0.5, 1.0, 0.0),
    vec3(0.0, 0.5, 1.0),
    vec3(0.5, 0.25, 0.0),
    vec3(0.5, 0.5, 0.5),
    vec3(0.0, 0.5, 0.5)
};

layout(location = 0) flat in uint vs_col;

layout(location = 0) out vec4 out_color;

void main() {
    uint i = vs_col == 0u ? 0u : 1u + (vs_col - 1u) % (COLOURS_LEN - 1u);
    out_color = vec4(COLOURS[i], 0.8);
}
