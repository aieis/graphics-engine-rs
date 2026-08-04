#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("../bindings/bindings.rs");

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

        println!("Check complete : {} ({})", c as i32, res);

        res
    }
}


const BAD_GLYPH_ARR: [bool; 256] = make_bad_glyph_arr();

pub fn IsGlyphBad(c: char) -> bool {

    return BAD_GLYPH_ARR[c as usize];

}


const fn make_bad_glyph_arr() -> [bool; 256]{
    let mut map = [false; 256];

    map[13] = true; // Cariage-return
    map[32] = true; // Space
    map[160] = true; // TODO: FIX this is 'p'
    map
}
