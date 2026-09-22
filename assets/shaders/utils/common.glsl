layout(std140, set = 0, binding = 0) uniform GlobalParameters
{
    vec3 CamPos;
    vec3 CamDir;
    vec3 CamUp;
    mat4 View;
    mat4 Projection;
} G;
