#version 450
#extension GL_ARB_separate_shader_objects : enable

#include "utils/camera.glsl"
#include "utils/common.glsl"
#include "utils/billboard.glsl"

layout(location = 0) out vec3 frag_color;

// Light spans point A and point B
layout(push_constant) uniform Params
{
    vec3 PointA;
    vec3 PointB;
    vec3 Colour;
} P;

#define PI 3.141592653589793

#define Radius 0.05

#define N 32

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
///     Should be two rectangles so center points can be full-colour and not cause interpolation to faded out
///     2 Triangles 6 Points


const int CircleTris = N;

const int TotalVertices = (CircleTris * 2 + 2 * 2) * 3;

vec3 get_v_pos_circle_offset(int tri_idx, int tri_v_idx, bool rotate) {
    if (tri_v_idx == 0) {
        return vec3(0, 0, 0);
    }

    int v = tri_idx + tri_v_idx - 1;
    if (rotate) {
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

    vec3 center_point = (P.PointB + P.PointA) / 2;
    vec3 line_axis  = normalize(P.PointB - P.PointA);

    vec3 UP = vec3(0, 1, 0);

    // This is for re-arranging the vertices to match the light-cone

    int v = TotalVertices - gl_VertexIndex;
    //v = gl_VertexIndex;

    vec4  pos  = vec4(0, 0, 0, 1.0);
    float dist = 1.0;
    vec3 colour = P.Colour;
    if (v <= N * 3) {
        // Circle 1
        int tri_idx   = (v / 3);
        int tri_v_idx = (v % 3);

        bool rotate = true;
        vec3 v_pos = get_v_pos_circle_offset(tri_idx, tri_v_idx, rotate);

        pos = vec4(v_pos + P.PointA, 1.0);

        dist = length(v_pos) / Radius;

        colour = vec3(0.0, 1.0, 0.0);

    } else if (v <= (N * 2) * 3 ) {
        // Circle 2

        int tri_idx   = ((v - (N * 3)) / 3);
        int tri_v_idx = (v % 3);

        bool rotate = true;
        vec3 v_pos = get_v_pos_circle_offset(tri_idx, tri_v_idx, rotate);

        pos = vec4(v_pos + P.PointB, 1.0);
        dist = length(v_pos) / Radius;

        colour = vec3(1.0, 0.0, 0.0);


    } else {
        // Rectangle
        int tri_v_idx = (v - (N*2) * 3 - 1);

        vec3  points  [2]  = {P.PointA, P.PointB}; // For indexing puposes below

        // Triangles           Tri 1      Tri 2      Tri 3       Tri 4
        int   targets [12] = { 0, 0, 1,   0, 1, 1,   0, 1,  0,    1, 1, 0}; // Target reference point above
        float offset  [12] = { 0, 1,-1,   0,-1, 0,   0, 0, -1,    0, 1,-1}; // Directed distance from that point * Radius
        //                     ---          *
        //                     |/          /|
        //                     *          ---

        vec3 position   = points[1 - targets[tri_v_idx]];
        vec3 opposition = points[targets[tri_v_idx]];
        vec3 v_pos = get_billboard_from_params(position, opposition, G.CamPos, Radius * offset[tri_v_idx]);
        pos = vec4(v_pos, 1.0);
        dist = abs(offset[tri_v_idx]);
    }

    vec4 world_pos = G.View * pos;
    vec4 proj_pos = G.Projection * world_pos;
    gl_Position = proj_pos;
    frag_color  = (1.0 - dist) * colour;
}
