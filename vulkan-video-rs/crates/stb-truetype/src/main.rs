
use stb_truetype::*;

fn main() {
    let mut info = std::mem::MaybeUninit::<stbtt_fontinfo>::uninit();
    unsafe {
        let info_ptr = info.as_mut_ptr();
        stbtt_InitFont(info_ptr, std::ptr::null::<u8>(), 0);

    }

    println!("Hello, World!");


}
