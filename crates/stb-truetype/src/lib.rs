#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("../bindings/bindings.rs");

use anyhow::{Result, anyhow};

pub fn InitFont(font_buffer: &[u8]) -> stbtt_fontinfo {

    let mut font_data = std::mem::MaybeUninit::<stbtt_fontinfo>::uninit();

    unsafe {
        let font = font_data.as_mut_ptr();
        stbtt_InitFont(font, font_buffer.as_ptr(), 0);
        font_data.assume_init()
    }
}

pub struct CodePoint {
    pub bitmap: Vec<u8>,
    pub w     : usize,
    pub h     : usize
}

pub fn GetCodepointBitmap(font: &stbtt_fontinfo, c: char, char_height: f32) -> CodePoint {
    unsafe {
        let font = font as *const stbtt_fontinfo;

        let c = c as i32;
        let mut w = 0;
        let mut h = 0;

        // TODO: Find alternative method (perhaps offload the free and carry the pointer around)

        let bitmap_ptr = stbtt_GetCodepointBitmap(font, 0.0, stbtt_ScaleForPixelHeight(font, char_height), c, &mut w, &mut h, std::ptr::null_mut(), std::ptr::null_mut());
        let slice  = std::ptr::slice_from_raw_parts(bitmap_ptr, (w*h) as usize);
        let bitmap = (*slice).to_vec();

        stbtt_FreeBitmap(bitmap_ptr, std::ptr::null_mut());

        CodePoint {
            bitmap,
            w: w as usize,
            h: h as usize
        }
    }
}

pub fn IsGlyphEmpty(font: &stbtt_fontinfo, c: char) -> bool {
    println!("Checking if char is empty : {}", c as i32);

    unsafe {
        let font = font as *const stbtt_fontinfo;
        let c = c as i32;

        let res = stbtt_IsGlyphEmpty(font, c) != 0;

        println!("Check complete : {} ({})", c, res);

        res
    }
}


const BAD_GLYPH_ARR: [bool; 256] = make_bad_glyph_arr();

pub fn IsGlyphBad(c: char) -> bool {

    BAD_GLYPH_ARR[c as usize]

}


const fn make_bad_glyph_arr() -> [bool; 256]{
    let mut map = [false; 256];

    map[13] = true; // Cariage-return
    map[32] = true; // Space
    map[160] = true; // TODO: FIX this is 'p'
    map
}

pub struct FontAtlasInfo {
    pub chars_per_col: u32,
    pub chars_per_row: u32,
    pub char_width: u32,
    pub char_height: u32
}

pub fn get_font_atlas_path(pfx: &str, font_atlas_info: &FontAtlasInfo) -> String {
    format!("{}_{}x{}_{}x{}_atlas.ff", pfx, font_atlas_info.chars_per_col, font_atlas_info.chars_per_row, font_atlas_info.char_width, font_atlas_info.char_height)
}



pub fn parse_font_atlas_info(path: &str) -> Result<FontAtlasInfo> {

    let info: Vec<_> = path.split('_').collect::<Vec<_>>();

    let n  = info.len();
    if  n < 4 {
        return Err(anyhow!("Atlas path is bad."));
    }

    let info = [info[n-3], info[n-2]];

    let [cols_x_rows, width_x_height] = info;

    let cols_x_rows: Vec<_>    = cols_x_rows.split('x').collect::<Vec<_>>();
    let width_x_height: Vec<_> = width_x_height.split('x').collect::<Vec<_>>();

	if cols_x_rows.len() != 2 || width_x_height.len() != 2 {
		return Err(anyhow!("Atlas format is incorrect."));
	}

    let cols_x_rows    = [cols_x_rows[0], cols_x_rows[1]];
    let width_x_height = [width_x_height[0], width_x_height[1]];

	let [cols, rows] = cols_x_rows;
	let [width, height] = width_x_height;

	let chars_per_row: u32 = cols.parse()?;
	let chars_per_col: u32 = rows.parse()?;

	let char_width: u32 = width.parse()?;
	let char_height: u32 = height.parse()?;

	Ok(FontAtlasInfo {
		chars_per_col,
		chars_per_row,
		char_width,
		char_height,
	})
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_path_maker() {
        let info = FontAtlasInfo {
            chars_per_col: 32,
            chars_per_row: 8,
            char_width: 8,
            char_height: 8,
        };

        let path = get_font_atlas_path("sample_font", &info);
        assert!(&path == "sample_font_32x8_8x8_atlas.ff");
    }

    #[test]
    pub fn test_atlas_info_extractor() {
        let path = "sample_font_32x8_8x8_atlas.ff";
        let info = parse_font_atlas_info(path).expect("Failed to parse");

		assert!(info.chars_per_row == 32);
		assert!(info.chars_per_col == 8);
		assert!(info.char_height == 8);
		assert!(info.char_width == 8);
    }

}
