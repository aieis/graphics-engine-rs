use stb_truetype::*;


const FONT_BUFFER: &[u8] = include_bytes!("../../../assets/fonts/Iosevka-Regular.ttf");

fn main() {
    let font = InitFont(FONT_BUFFER);
    let arr: Vec<_> = " .:ioVM@".to_string().chars().collect();

	let chars_lc: Vec<_> = ('a' as u8..='z' as u8).map(|c| { c as char }).collect();
	let chars_uc: Vec<_> = ('A' as u8..='Z' as u8).map(|c| { c as char }).collect();
	let chars_d: Vec<_>  = ('0' as u8..='9' as u8).map(|c| { c as char }).collect();
	let chars_p = vec![',', ';', '-', '=', '+'];

	let chars = vec![chars_lc, chars_uc, chars_d, chars_p].concat();

	for c in chars {

		let code_point = GetCodepointBitmap(&font, c as char, 20.0);

		for j in 0..code_point.h {
			for i in 0..code_point.w {
				print!("{}", arr[(code_point.bitmap[j*code_point.w+i]>>5) as usize]);
			}
			println!();
		}
	}
}
