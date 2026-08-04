/*
 * This little program is used to generate the font atlas to be used by the main application
 */

use farbfeld_image::{load_ff, write_ff, RgbaImage};

use stb_truetype::*;

const FONT_BUFFER: &[u8] = include_bytes!("../../../assets/fonts/Iosevka-Regular.ttf");

const FONT_ATLAS_PATH: &str = "atlas/iosevka.ff";

fn main() {

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 && args[1] == "--tight" {
        pack_atlas_tight();
    } else {
        pack_atlas_sparse();
    }
}

fn pack_atlas_sparse() {
    let font = InitFont(FONT_BUFFER);

	let chars_lc: Vec<_> = ('a' as u8..='z' as u8).map(|c| { c as char }).collect();
	let chars_uc: Vec<_> = ('A' as u8..='Z' as u8).map(|c| { c as char }).collect();
	let chars_d: Vec<_>  = ('0' as u8..='9' as u8).map(|c| { c as char }).collect();
	let chars_p = vec![',', ';', '-', '=', '+'];

	let chars = vec![chars_lc, chars_uc, chars_d, chars_p].concat();
    let code_points = chars.iter().map(|c| { GetCodepointBitmap(&font, *c, 40.0) }).collect::<Vec<_>>();

    let mut w_c = 0;
    let mut h_c = 0;

    for c in code_points.iter() {
        w_c = if c.w > w_c {c.w} else { w_c };
        h_c = if c.h > h_c {c.h} else { h_c };
    }


    let w = w_c;
    let h = h_c * 256;

    let mut image_data = vec![0; w * h * 4];

    for (i, c) in code_points.iter().enumerate() {
        let c_i = chars[i] as usize;
        let px = 0;
        let py = (c_i+1) * h_c - c.h;

		for y in 0..c.h {
			for x in 0..c.w {
                let dst = ((py + y) * w_c + (px + x)) * 4;
                image_data[dst+3] = c.bitmap[y*c.w + x];
			}
		}
	}

    let atlas = RgbaImage {
        w: w as u32,
        h: h as u32,
        data: image_data,
    };


    write_ff(FONT_ATLAS_PATH, atlas);

    match load_ff(FONT_ATLAS_PATH) {
        Ok(im) => {

            println!("Loaded Image: {}x{}", im.w, im.h);

        }


        Err(err) => {
            println!("{}", err);
        }
    };

}

fn pack_atlas_tight() {
    let font = InitFont(FONT_BUFFER);

	let chars_lc: Vec<_> = ('a' as u8..='z' as u8).map(|c| { c as char }).collect();
	let chars_uc: Vec<_> = ('A' as u8..='Z' as u8).map(|c| { c as char }).collect();
	let chars_d: Vec<_>  = ('0' as u8..='9' as u8).map(|c| { c as char }).collect();
	let chars_p = vec![',', ';', '-', '=', '+'];

	let chars = vec![chars_lc, chars_uc, chars_d, chars_p].concat();
    let code_points = chars.iter().map(|c| { GetCodepointBitmap(&font, *c, 40.0) }).collect::<Vec<_>>();

    let mut w_c = 0;
    let mut h_c = 0;

    for c in code_points.iter() {
        w_c = if c.w > w_c {c.w} else { w_c };
        h_c = if c.h > h_c {c.h} else { h_c };
    }


    let w = w_c;
    let h = h_c * code_points.len();

    let mut image_data = vec![0; w * h * 4];

    for (i, c) in code_points.iter().enumerate() {
        let px = 0;
        let py = (i+1) * h_c - c.h;

		for y in 0..c.h {
			for x in 0..c.w {
                let dst = ((py + y) * w_c + (px + x)) * 4;
                image_data[dst+3] = c.bitmap[y*c.w + x];
			}
		}
	}

    let atlas = RgbaImage {
        w: w as u32,
        h: h as u32,
        data: image_data,
    };


    write_ff(FONT_ATLAS_PATH, atlas);

    match load_ff(FONT_ATLAS_PATH) {
        Ok(im) => {

            println!("Loaded Image: {}x{}", im.w, im.h);

        }


        Err(err) => {
            println!("{}", err);
        }
    };
}
