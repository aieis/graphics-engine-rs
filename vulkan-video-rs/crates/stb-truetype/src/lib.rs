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
        let bitmap = stbtt_GetCodepointBitmap(font, 0.0, stbtt_ScaleForPixelHeight(font, char_height), c, &mut w, &mut h, std::ptr::null_mut(), std::ptr::null_mut());
        let bitmap = Vec::from_raw_parts(bitmap, (w*h) as usize, (w*h) as usize);

        CodePoint {
            bitmap,
            w: w as usize,
            h: h as usize
        }
    }
}
