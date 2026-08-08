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
    vec2(-1, -1),
    vec2( 1, -1),
    vec2( 1,  1),
    vec2(-1, -1),
    vec2( 1,  1),
    vec2(-1,  1),
};


#define CHARS_LEN 13
const uint CHAR_TEST[CHARS_LEN] = {
	    72, 69, 76, 76, 79, 44, 32, 87, 79, 82, 76, 68, 33
	// 	 H   E   L   L   O   ,       W   O   R   L   D   !
};

const uint CHARS_PER_ROW = 32;
const uint CHARS_PER_COL = 8;


void main() {


    uint char_index = (gl_VertexIndex  / 6);
    uint c = Chars.Data[char_index];

	c = CHAR_TEST[char_index % CHARS_LEN];

    vec2 dims = vec2(7, 8); //T.CharDims.xy;

    uint c_pos_x = c % CHARS_PER_ROW;
    uint c_pos_y = c / CHARS_PER_ROW;

    float cy = c_pos_y*dims.y;
    float cx = c_pos_x*dims.x;

    float v_x_offset = (OFFSETS[gl_VertexIndex % 6].x + 1.0) * dims.x;
    float coord_cx_v = (cx + v_x_offset) / (CHARS_PER_ROW * dims.x);

    float v_y_offset = (OFFSETS[gl_VertexIndex % 6].y + 1.0) / 2.0 * dims.y;
    float coord_cy_v = (cy + v_y_offset) / (CHARS_PER_COL * dims.y);



    vec2 coord = vec2(coord_cx_v, coord_cy_v);

    // vec2 pos =  T.Position.xy + vec2(char_index*T.CharDims.x, 0) + OFFSETS[gl_VertexIndex % 4] * T.Scale;

	float cw = 0.05;
	float cw_scale = cw / dims.x;

    float p_x_offset = OFFSETS[gl_VertexIndex % 6].x * cw;
    float p_y_offset = OFFSETS[gl_VertexIndex % 6].y * cw;


	vec2 pos =  vec2(-1 + cw, -1 + cw) + vec2(char_index*cw + p_x_offset, p_y_offset);

    gl_Position = vec4(pos, 0.0, 1.0);
    TexCoord = coord;
}
