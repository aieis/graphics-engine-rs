#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec3 frag_color;

const vec3 VERTICES[4] = {
    vec3(-0.5, -0.5, 0),
    vec3( 0.5, -0.5, 0),
    vec3( 0.5,  0.5, 0),
    vec3(-0.5,  0.5, 0)
};

int INDICES[6] = { 0, 2, 1, 2, 3, 0 };

void main() {

    gl_Position = vec4(VERTICES[INDICES[gl_VertexIndex % 6]], 1.0);
    frag_color = vec3(1.0, 1.0, 1.0);

}
