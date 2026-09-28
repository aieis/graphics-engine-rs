#version 450
#extension GL_ARB_separate_shader_objects : enable

#include "utils/camera.glsl"
#include "utils/common.glsl"

layout(location = 0) out vec3 frag_color;

// Light spans point A and point B
layout(push_constant) uniform Params
{
    vec3 PointA;
    vec3 PointB;
    vec3 Colour;
} P;


#define PI 3.141592653589793

#define Radius 1

#define N 8

const int   V = N + 2;
const float DeltaTheta = 2 * PI / N;

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


const float CircleTris = N * 3;


vec3 get_v_pos_circle_offset(int tri_idx, int tri_v_idx, bool rotate) {
    if (tri_v_idx == 0) {
        return vec3(0, 0, 0);
    }

    int v = tri_idx + tri_v_idx - 1;
    if (!rotate) {
        float x = Radius * cos(2 * PI - v * DeltaTheta);
        float y = Radius * sin(2 * PI - v * DeltaTheta);
        return vec3(x, y, 0);
    } else {
        float x = Radius * cos(v * DeltaTheta);
        float y = Radius * sin(v * DeltaTheta);
        return vec3(x, y, 0);
    }

}

void main() {

    vec3 line_axis  = normalize(P.PointB - P.PointA);

    vec3 minor_axis = vec3(-1, 0, 0);

    vec3 UP = vec3(0, 1, 0);

    // This is for re-arranging the vertices to match the light-cone

    int v = gl_VertexIndex;

    vec4 pos = vec4(0, 0, 0, 1.0);
    if (v < N * 3) {
        // Circle 1
        int tri_idx   = (v / 3);
        int tri_v_idx = (v % 3);

        bool rotate = true;
        vec3 v_pos = get_v_pos_circle_offset(tri_idx, tri_v_idx, rotate);

        mat4 fake_view_matrix = mat4(1.0);
        if (1.0 - abs(dot(line_axis, UP)) > 1.0e-3) {
            fake_view_matrix = create_view_matrix(P.PointA, line_axis, UP);
        }

        pos = fake_view_matrix * vec4(v_pos, 1.0);

    } else if (v < N * 3 * 2) {
        // Circle 2

        int tri_idx   = ((v - (N * 3)) / 3);
        int tri_v_idx = (v % 3);

        bool rotate = false;
        vec3 v_pos = get_v_pos_circle_offset(tri_idx, tri_v_idx, rotate);

        mat4 fake_view_matrix = mat4(1.0);
        if (1.0 - abs(dot(line_axis, UP)) > 1.0e-3) {
            fake_view_matrix = create_view_matrix(P.PointB, line_axis, UP);
        }

        pos = fake_view_matrix * vec4(v_pos, 1.0);

    } else {
        // Rectangle

    }

    vec4 world_pos = G.View * pos;
    vec4 proj_pos = G.Projection * world_pos;
    gl_Position = proj_pos;
    frag_color  = P.Colour;
}
