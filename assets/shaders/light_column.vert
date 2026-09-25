#version 450
#extension GL_ARB_separate_shader_objects : enable

#include "utils/camera.glsl"
#include "utils/common.glsl"

layout(location = 0) out vec3 frag_color;

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 col;
layout(location = 2) in vec3 normals;

// Light spans point A and point B
layout(push_constant) uniform Params
{
    vec3 PointA;
    vec3 PointB;
} P;


#define PI 3.141592653589793

#define Radius 1

#define N 8


/// Simple two points + offset each point
/// Needs no culling
/// Needs to semi circles
/// Vertex inputs:
///     [Circle 1] [Circle 2] [4 Points]

/// Circle spec where N is the number of segments :
///     N Triangles N * 3 Points (N triangles because there will be a point in the center)
///     N+2 Vertices per circle

/// Rectangle Spec:
///     2 Triangles 6 Points


void main() {

    vec3 line_axis  = normalize(P.PointA - P.PointB);

    vec3 minor_axis = vec3(-1, 0, 0);

    vec3 UP = vec3(0, 1, 0);

    if (1.0 - abs(dot(line_axis - UP)) > 1.0e-3) {
        minor_axis = normalize(cross(line_axis, UP));
    }

    int v = gl_VertexIndex;

    if (v < N+2) {
        // Circle 1

        


    } else if (v < (N+2) * 2) {
        // Circle 2

    } else {
        // Rectangle

    }

    mat4 view = G.View;

    vec4 world_pos = view * vec4(pos, 1.0);

    mat4 proj = G.Projection;
    vec4 proj_pos = proj * world_pos;

    gl_Position = proj_pos;

}
