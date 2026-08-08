/*
 * This little program is used to generate the font atlas to be used by the main application
 */

use farbfeld_image::{load_ff, write_ff, RgbaImage};

use stb_truetype::*;

// const FONT_BUFFER: &[u8] = include_bytes!("../../../assets/fonts/Iosevka-Regular.ttf");
// const FONT_ATLAS_PATH: &str = "atlas/iosevka.ff";
const FONT_BUFFER: &[u8]    = include_bytes!("../../../assets/fonts/PixelOperator8-Bold.ttf");
const FONT_ATLAS_PATH_PFX: &str = "atlas/Atlas_Pixel_Operator";

const CHARS: [char; 65] = create_target_chars();

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

    let code_points = CHARS.iter().map(|c| { GetCodepointBitmap(&font, *c, 8.0) }).collect::<Vec<_>>();

    let mut char_width = 0;
    let mut char_height = 0;

    for c in code_points.iter() {
        char_width = if c.w > char_width {c.w} else { char_width };
        char_height = if c.h > char_height {c.h} else { char_height };
    }

    println!("Character dimensions: {}x{}", char_width, char_height);

    let chars_per_row = 32;
    let chars_per_col = 8;

    let w = char_width * chars_per_row;
    let h = char_height * chars_per_col;

    let mut image_data = vec![0; w * h * 4];

    for (i, c) in code_points.iter().enumerate() {
        let c_i = CHARS[i] as usize;

        let c_pos_x = c_i % chars_per_row;
        let c_pos_y = c_i / chars_per_row;

        let px = c_pos_x * char_width;
        let py = (c_pos_y+1) * char_height - c.h;

        for y in 0..c.h {
			for x in 0..c.w {
                let dst = ((py + y) * w + (px + x)) * 4;
                image_data[dst+3] = c.bitmap[y*c.w + x];
			}
		}
	}

    let atlas = RgbaImage {
        w: w as u32,
        h: h as u32,
        data: image_data,
    };

	let atlas_info = FontAtlasInfo {
		chars_per_col: chars_per_col as u32,
		chars_per_row: chars_per_row as u32,
		char_width: char_width as u32,
		char_height: char_height as u32,
	};

	let atlas_output_path = get_font_atlas_path(FONT_ATLAS_PATH_PFX, &atlas_info);
    write_ff(&atlas_output_path, atlas);

    match load_ff(&atlas_output_path) {
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
    let code_points = CHARS.iter().map(|c| { GetCodepointBitmap(&font, *c, 40.0) }).collect::<Vec<_>>();

    let mut char_width = 0;
    let mut char_height = 0;

    for c in code_points.iter() {
        char_width = if c.w > char_width {c.w} else { char_width };
        char_height = if c.h > char_height {c.h} else { char_height };
    }


    let w = char_width;
    let h = char_height * code_points.len();

    let mut image_data = vec![0; w * h * 4];

    for (i, c) in code_points.iter().enumerate() {
        let px = 0;
        let py = (i+1) * char_height - c.h;

		for y in 0..c.h {
			for x in 0..c.w {
                let dst = ((py + y) * char_width + (px + x)) * 4;
                image_data[dst+3] = c.bitmap[y*c.w + x];
			}
		}
	}

    let atlas = RgbaImage {
        w: w as u32,
        h: h as u32,
        data: image_data,
    };

	let atlas_info = FontAtlasInfo {
		chars_per_col: 256,
		chars_per_row: 1,
		char_width: char_width as u32,
		char_height: char_height as u32,
	};


	let atlas_output_path = get_font_atlas_path(FONT_ATLAS_PATH_PFX, &atlas_info);
    write_ff(&atlas_output_path, atlas);

    match load_ff(&atlas_output_path) {
        Ok(im) => {

            println!("Loaded Image: {}x{}", im.w, im.h);

        }


        Err(err) => {
            println!("{}", err);
        }
    };
}

const fn create_target_chars() -> [char; 65] {
	const CHARS_LC_SIZE: usize = (b'z' - b'a') as usize;
	let mut chars_lc = ['a'; CHARS_LC_SIZE];
	let mut c = b'a';
	while c < b'z' {
		chars_lc[c as usize - b'a' as usize] = c as char;
		c += 1;
	}

	const CHARS_UC_SIZE: usize = (b'z' - b'a') as usize;
	let mut chars_uc = ['a'; CHARS_UC_SIZE];
	let mut c = b'A';
	while c < b'Z' {
		chars_uc[c as usize - b'A' as usize] = c as char;
		c += 1;
	}

	const CHARS_D_SIZE: usize = (b'9' - b'0') as usize;
	let mut chars_d = ['a'; CHARS_D_SIZE];
	let mut c = b'0';
	while c < b'9' {
		chars_d[c as usize - b'0' as usize] = c as char;
		c += 1;
	}

	const CHARS_P_SIZE: usize = 6;
	let chars_p: [char; CHARS_P_SIZE] = [',', ';', '-', '=', '+', '!'];


	let mut chars = ['a'; CHARS_LC_SIZE + CHARS_UC_SIZE + CHARS_D_SIZE + CHARS_P_SIZE];
	let mut k = 0;

	let mut i = 0;
	while i < CHARS_LC_SIZE {
		chars[k] = chars_lc[i];
		i += 1;
		k += 1;
	}

	let mut i = 0;
	while i < CHARS_UC_SIZE {
		chars[k] = chars_uc[i];
		i += 1;
		k += 1;
	}

	let mut i = 0;
	while i < CHARS_D_SIZE {
		chars[k] = chars_d[i];
		i += 1;
		k += 1;
	}

	let mut i = 0;
	while i < CHARS_P_SIZE {
		chars[k] = chars_p[i];
		i += 1;
		k += 1;
	}


	chars
}
