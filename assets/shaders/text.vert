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


layout(set = 0, binding = 4) uniform GlyphInfo { uvec4 Data[48]; } Glyphs;

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

#define CHAR_ATLAS_LEN 96
const uint CHAR_ATLAS_ALL[CHAR_ATLAS_LEN] = {
     32,  33,  34,  35,  36,  37,  38,  39,  40,  41,  42,  43,  44,  45,  46,  47,  48,
     49,  50,  51,  52,  53,  54,  55,  56,  57,  58,  59,  60,  61,  62,  63,  64,  65,
     66,  67,  68,  69,  70,  71,  72,  73,  74,  75,  76,  77,  78,  79,  80,  81,  82,
     83,  84,  85,  86,  87,  88,  89,  90,  91,  92,  93,  94,  95,  96,  97,  98,  99,
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
};

void main() {

    uint char_index = (gl_VertexIndex  / 6);
    uint c = Chars.Data[char_index / 4][char_index % 4] - 32;
	// c = CHAR_TEST[char_index % CHARS_LEN] - 32;
	// c = CHAR_ATLAS_ALL[char_index % CHAR_ATLAS_LEN] - 32;

    uint glyph_group_i = c / 2;
    uint glyph_group_s = c % 2;
    vec2 glyph_dims = vec2 (
        float(Glyphs.Data[glyph_group_i][glyph_group_s*2]),
        float(Glyphs.Data[glyph_group_i][glyph_group_s*2 + 1])
    );

    uint char_group_i = char_index / 2;
    uint char_group_s = char_index % 2;
    vec2 glyph_position = vec2 (
        float(CharPositions.Data[char_group_i][char_group_s*2]),
        float(CharPositions.Data[char_group_i][char_group_s*2 + 1])
    );


    vec2 dims = vec2(T.CharDims.x, T.CharDims.y);

	uint chars_per_row = uint(T.CharPacking.x);
	uint chars_per_col = uint(T.CharPacking.y);

    uint c_pos_x = c % chars_per_row;
    uint c_pos_y = c / chars_per_row;

    float cx = c_pos_x*dims.x;
    float cy = c_pos_y*dims.y;

    float v_x_offset = (OFFSETS[gl_VertexIndex % 6].x + 1.0) / 2.0 * glyph_dims.x;
    float coord_cx_v = (cx + v_x_offset) / (chars_per_row * dims.x);

    float v_y_offset = (OFFSETS[gl_VertexIndex % 6].y + 1.0) / 2.0 * glyph_dims.y + dims.y - glyph_dims.y;
    float coord_cy_v = (cy + v_y_offset) / (chars_per_col * dims.y);

    vec2 coord = vec2(coord_cx_v, coord_cy_v);


    // SET DEBUG POSITION


	float cw = 0.03;
	float cw_scale = cw / dims.x;

    // glyph_position = vec2(float(char_index) / chars_per_row, 0);
    // glyph_position.x /= (dims.x * chars_per_row);
    // glyph_position.y /= (dims.y * chars_per_col);


    /*
     *   dx
     * + -- +
     * |    | dy
     * + -- +
     */


    float p_x_offset = OFFSETS[gl_VertexIndex % 6].x * glyph_dims.x / 2;
    float p_y_offset = OFFSETS[gl_VertexIndex % 6].y * glyph_dims.y;

    vec2 pos =  T.Position.xy + glyph_position * cw_scale + vec2(p_x_offset, p_y_offset) * cw_scale;
    // vec2 pos =  glyph_position + vec2(p_x_offset, p_y_offset) * cw_scale;

    gl_Position = vec4(pos, 0.0, 1.0);
    TexCoord = coord;
	TextColor = T.Colour;
}
