#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("../bindings/bindings.rs");

use anyhow::{Result, anyhow};

pub struct CodePoint {
    pub bitmap: Vec<u8>,
    pub w     : usize,
    pub h     : usize
}

pub struct Glyph {
    pub bitmap: Vec<u8>,
    pub info: GlyphInfo
}


#[derive(Clone, Debug)]
pub struct GlyphInfo {
	pub x       : i32,
	pub y       : i32,
    pub advance : i32,

    pub w       : usize,
    pub h       : usize,
}


#[derive(Debug, Clone)]
pub struct FontAtlasInfo {
    pub chars_per_row: u32,
    pub chars_per_col: u32,
    pub char_width: u32,
    pub char_height: u32
}



pub struct FontInfo {
    pub data: stbtt_fontinfo
}

pub fn InitFont(font_buffer: &[u8]) -> FontInfo {

    let mut font_data = std::mem::MaybeUninit::<stbtt_fontinfo>::uninit();

    unsafe {
        let font = font_data.as_mut_ptr();
        stbtt_InitFont(font, font_buffer.as_ptr(), 0);
        FontInfo { data: font_data.assume_init() }
    }
}


pub fn GetCodepointBitmap(font: &FontInfo, c: char, char_height: f32) -> CodePoint {
    unsafe {
        let font = &font.data as *const stbtt_fontinfo;

        let c = c as i32;
        let mut w = 0;
        let mut h = 0;

        // TODO: Find alternative method (perhaps offload the free and carry the pointer around)

        let scale: f32 = stbtt_ScaleForPixelHeight(font, char_height);
        let bitmap_ptr = stbtt_GetCodepointBitmap(font, 0.0, scale, c, &mut w, &mut h, std::ptr::null_mut(), std::ptr::null_mut());

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

pub fn ScaleFontForPixelHeight(font: &FontInfo, pixel_height: f32) -> f32 {
    unsafe {
        let font = &font.data as *const stbtt_fontinfo;
        stbtt_ScaleForPixelHeight(font, pixel_height)
    }
}

pub fn GetFontVMetrics(font: &FontInfo) -> i32 {
    unsafe {

        let font = &font.data as *const stbtt_fontinfo;
		let mut ascent = 0;
		let mut descent = 0;
		let mut line_gap = 0;
		stbtt_GetFontVMetrics(font, &mut ascent, &mut descent, &mut line_gap);

		ascent
	}

}


pub fn GetCodepointBitmapBoxSubpixel(font: &FontInfo, c: char, scale_x: f32, scale_y: f32, shift_x: f32, shift_y: f32) -> (i32, i32, i32, i32) {
    unsafe {
        let font = &font.data as *const stbtt_fontinfo;
        let c = c as i32;

        let mut x0 = 0;
        let mut y0 = 0;
        let mut x1 = 0;
        let mut y1 = 0;

        stbtt_GetCodepointBitmapBoxSubpixel(font, c, scale_x, scale_y, shift_x, shift_y, &mut x0, &mut y0, &mut x1,&mut y1);

        (x0, y0, x1, y1)
    }
}



pub fn GetGlyph(font_info: &FontInfo, c: char, scale: f32) -> Glyph {
    unsafe {
        let font = &font_info.data as *const stbtt_fontinfo;

        let c = c as i32;

        let mut advance = 0;
        let mut lsb = 0;

        let x_shift = 0.0;

        stbtt_GetCodepointHMetrics(font, c, &mut advance, &mut lsb);
        let (x0, y0, x1, y1) = GetCodepointBitmapBoxSubpixel(font_info, c as u8 as char, scale, scale, x_shift, 0.0);

        let w = x1 - x0;
        let h = y1 - y0;
        let mut bitmap = vec![0; (w * h) as usize];
        stbtt_MakeCodepointBitmapSubpixel(font, bitmap.as_mut_ptr() as _, w, h, w, scale, scale, x_shift, 0.0, c);

        let info = GlyphInfo {
            x: x0 as i32,
            y: y0 as i32,
            w: w as usize,
            h: h as usize,
            advance,
        };

        Glyph {
            bitmap,
            info,
        }
    }
}


pub fn GetCodepointKernAdvance(font: &FontInfo, c: char, c_next: char) -> i32 {
    unsafe {
        let font = &font.data as *const stbtt_fontinfo;
        stbtt_GetCodepointKernAdvance(font, c as i32, c_next as i32)
    }
}

pub fn IsGlyphEmpty(font: &FontInfo, c: char) -> bool {
    println!("Checking if char is empty : {}", c as i32);

    unsafe {
        let font = &font.data as *const stbtt_fontinfo;
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


pub fn get_font_atlas_path(pfx: &str, font_atlas_info: &FontAtlasInfo) -> String {
    format!("{}_{}x{}_{}x{}_atlas.ff", pfx, font_atlas_info.chars_per_row, font_atlas_info.chars_per_col, font_atlas_info.char_width, font_atlas_info.char_height)
}

pub fn get_font_atlas_desc_path(pfx: &str, font_atlas_info: &FontAtlasInfo) -> String {
    format!("{}_{}x{}_{}x{}_atlas_desc.bin", pfx, font_atlas_info.chars_per_row, font_atlas_info.chars_per_col, font_atlas_info.char_width, font_atlas_info.char_height)
}

pub fn get_font_atlas_desc_from_path(path: &str) -> String {
	format!("{}_desc.bin", &path[0..path.len()-3])
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
            chars_per_row: 32,
            chars_per_col: 8,
            char_width: 8,
            char_height: 8,
        };

        let path = get_font_atlas_path("sample_font", &info);
        assert!(&path == "sample_font_32x8_8x8_atlas.ff");
    }

    #[test]
    pub fn test_atlas_info_extractor() {
        let path = "sample_font_32x8_8x9_atlas.ff";
        let info = parse_font_atlas_info(path).expect("Failed to parse");

		assert!(info.chars_per_row == 32);
		assert!(info.chars_per_col == 8);
		assert!(info.char_width == 8);
		assert!(info.char_height == 9);
    }

	#[test]
	pub fn test_atlas_get_desc_from_ff() {
        let path = "sample_font_32x8_8x9_atlas.ff";
        let desc_path = get_font_atlas_desc_from_path(path);

		assert!(desc_path == "sample_font_32x8_8x9_atlas_desc.bin");

	}

}
