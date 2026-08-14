#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) out vec2 TexCoord;
layout(location = 1) out vec3 TextColor;

layout(set = 0, binding = 0) uniform sampler2D FontAtlas;

// For some reason uint gets padded so that they take up 16 bytes each (12 bytes of padding)
layout(set = 0, binding = 1) uniform CharsDataArray { uvec4 Data[16]; } Chars;

layout(set = 0, binding = 2) uniform PositionalInfo { ivec4 Data[32]; } CharPositions;

layout(set = 0, binding = 3) uniform TextData {
    vec3 CharDims;
    vec3 Position;
    vec3 Colour;
	vec3 CharPacking;
} T;


layout(set = 0, binding = 4) uniform GlyphInfo { uvec4 Data[96 / 2]; } Glyphs;

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

void main() {

    uint char_index = (gl_VertexIndex  / 6);
    uint c = Chars.Data[char_index / 4][char_index % 4] - 32;
	c = CHAR_TEST[char_index % CHARS_LEN];

    uvec4 glyph_group = Glyphs.Data[c / 2];
    vec2 glyph_dims;
    if (c % 2 == 0 ) {
        glyph_dims = vec2(glyph_group.x, glyph_group.y);
    } else {
        glyph_dims = vec2(glyph_group.z, glyph_group.w);
    }

    ivec4 glyph_position_s = CharPositions.Data[char_index / 2];
    vec2 glyph_position;
    if (char_index % 2 == 0 ) {
        glyph_position = vec2(glyph_position_s.x, glyph_position_s.y);
    } else {
        glyph_position = vec2(glyph_position_s.z, glyph_position_s.w);
    }

    vec2 dims = vec2(T.CharDims.x, T.CharDims.y);
	
	uint chars_per_row = uint(T.CharPacking.x);
	uint chars_per_col = uint(T.CharPacking.y);

    uint c_pos_x = c % chars_per_row;
    uint c_pos_y = c / chars_per_row;

    float cy = c_pos_y*dims.y;
    float cx = c_pos_x*dims.x;

    float v_x_offset = (OFFSETS[gl_VertexIndex % 6].x + 1.0) / 2.0 * glyph_dims.x;
    float coord_cx_v = (cx + v_x_offset) / (chars_per_row * dims.x);

    float v_y_offset = (OFFSETS[gl_VertexIndex % 6].y + 1.0) / 2.0 * glyph_dims.y + glyph_dims.y;
    float coord_cy_v = (cy + v_y_offset) / (chars_per_col * dims.y);

    vec2 coord = vec2(coord_cx_v, coord_cy_v);

	float cw = 0.03;
	float cw_scale = cw / dims.x;

    float p_x_offset = OFFSETS[gl_VertexIndex % 6].x * glyph_dims.x;
    float p_y_offset = OFFSETS[gl_VertexIndex % 6].y * glyph_dims.y;

	// glyph_position = vec2(char_index * dims.x, 0) * 5;
	glyph_position.y = 0;

    vec2 pos =  T.Position.xy + vec2(glyph_position.x, glyph_position.y) * cw_scale + vec2(p_x_offset, p_y_offset) * cw_scale;

    gl_Position = vec4(pos, 0.0, 1.0);
    TexCoord = coord;
	TextColor = T.Colour;
}
