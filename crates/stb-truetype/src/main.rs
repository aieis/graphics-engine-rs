use stb_truetype::*;


const FONT_BUFFER: &[u8] = include_bytes!("../../../assets/fonts/Iosevka-Regular.ttf");

fn main() {
    let font = InitFont(FONT_BUFFER);
    let code_point = GetCodepointBitmap(&font, 'a', 20.0);
    let arr: Vec<_> = " .:ioVM@".to_string().chars().collect();

    for j in 0..code_point.h {
        for i in 0..code_point.w {
            print!("{}", arr[(code_point.bitmap[j*code_point.w+i]>>5) as usize]);
        }
        println!();
    }
}
