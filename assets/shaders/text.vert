#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec2 TexCoord;

layout(set = 0, binding = 0) uniform sampler2D FontAtlas;

layout(set = 0, binding = 1) uniform CharsDataArray { uint Data[64]; } Chars;

layout(set = 0, binding = 2) uniform TextData {
    float Scale;
    vec3 CharDims;
    vec3 Position;
    vec3 Colour;
} T;

const vec2 OFFSETS[6] = {
    vec2(-1,  1),
    vec2(-1, -1),
    vec2( 1,  1),
    vec2( 1, -1),
    vec2( 1,  1),
    vec2(-1, -1),
};


const uint CHAR_TEST[5] = {
	110, 101, 108, 108, 111
	// 72, 101, 108, 108, 111
};


void main() {


    uint char_index = (gl_VertexIndex  / 6);
    uint c = Chars.Data[char_index];

	char_index = char_index % 5;
	c = CHAR_TEST[char_index];

    vec2 dims = vec2(15, 33); //T.CharDims.xy;
    float cy = c*dims.y;
    float cx = 0;

    vec2 coord = (vec2(cx, cy) + (OFFSETS[gl_VertexIndex % 6] + vec2(1, 1)) / 2 * dims) / vec2(15, 8448);

    // vec2 pos =  T.Position.xy + vec2(char_index*T.CharDims.x, 0) + OFFSETS[gl_VertexIndex % 4] * T.Scale;

	float cw = 0.05;
	float cw_scale = cw / dims.x;

	vec2 pos =  vec2(0, 0) + (vec2(char_index*dims.x, 0) + OFFSETS[gl_VertexIndex % 6] * dims) * cw_scale;

    gl_Position = vec4(pos, 0.0, 1.0);
    TexCoord = coord;
}
