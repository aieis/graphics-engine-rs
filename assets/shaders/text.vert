#version 450
#extension GL_ARB_separate_shader_objects : enable


layout(location = 0) out vec2 TexCoord;

layout(set = 0, binding = 0) uniform sampler2D FontAtlas;

layout(set = 0, binding = 1) uniform uint Chars[];
layout(set = 0, binding = 2) uniform TextData {
    float Scale;
    vec3 CharDims;
    vec3 Position;
    vec3 Colour;
} T;

static const vec2 OFFSETS[4] = {
    vec2(-1, -1),
    vec2(-1,  1),
    vec2( 1,  1),
    vec2( 1, -1)
};


void main() {


    uint char_index = gl_VertexID / 4;
    uint c = T.Chars[char_index];

    float cy = T.Position.y + char_index*T.CharDims.y;
    float cx = 0;

    vec2 dims = CharDims.xy;

    vec2 coord = vec2(cx, cy) + (OFFSETS[gl_VertexID % 4] + vec2(1, 1)) / 2 * dims;
    
    
    vec2 pos =  T.Position * char_index*T.Ch + OFFSETS[gl_VertexID % 4] * T.Scale;

    gl_Position = vec4(pos, 0.0, 1.0);
    TexCoord = coord;
}
