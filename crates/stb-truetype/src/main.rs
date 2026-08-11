/*
 * This little program is used to generate the font atlas to be used by the main application
 */

mod atlas;

use farbfeld_image::{load_ff, write_ff, RgbaImage};
use atlas::FontAtlas;

use stb_truetype::*;

const FONT_BUFFER: &[u8]    = include_bytes!("../../../assets/fonts/Iosevka-Regular.ttf");
const FONT_ATLAS_PATH_PFX: &str = "atlas/Atlas_Iosevka_Regular";
const FONT_HEIGHT_TARGET: f32 = 64.0;

const CHARS: [char; 96] = create_target_chars();

fn main() {

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 && args[1] == "--tight" {
		println!("The following functionality is deprecated");
        pack_atlas_tight();
	} else if args.len() > 1 && args[1] == "--with-info" {
		println!("Making a full atlas");
		pack_atlas_with_info();
    } else {
        pack_atlas_sparse();
    }
}

pub fn pack_atlas_with_info() {
    let font = InitFont(FONT_BUFFER);

	let font_atlas = FontAtlas::new(&font, FONT_HEIGHT_TARGET);
    font_atlas.write_atlas(FONT_ATLAS_PATH_PFX);

    let (h, m, s) = get_time_of_day();

    let msg = format!("Hello, my Quick Brown Fox!: {:02}:{:02}:{:02}", h, m, s);
	let im: RgbaImage = font_atlas.pack_message(&msg);

	write_ff("./test_outputs/message.ff", &im);

}

fn pack_atlas_sparse() {
    let font = InitFont(FONT_BUFFER);

    let code_points = CHARS.iter().map(|c| { GetCodepointBitmap(&font, *c, FONT_HEIGHT_TARGET) }).collect::<Vec<_>>();

    let mut char_width  = 0;
    let mut char_height = 0;

    for c in code_points.iter() {
        char_width  = if c.w > char_width {c.w} else { char_width };
        char_height = if c.h > char_height {c.h} else { char_height };
    }

    println!("Character dimensions: {}x{}", char_width, char_height);

    let chars_per_row = 12;
    let chars_per_col = code_points.len() / chars_per_row;

    let w = char_width  * chars_per_row;
    let h = char_height * chars_per_col;

    let mut image_data = vec![0; w * h * 4];

    for (i, c) in code_points.iter().enumerate() {
        let c_i = CHARS[i] as usize - 32;

        let c_pos_x = c_i % chars_per_row;
        let c_pos_y = c_i / chars_per_row;

        let px = c_pos_x * char_width;
        let py = (c_pos_y+1) * char_height - c.h;

        for y in 0..c.h {
			for x in 0..c.w {
                let dst = ((py + y) * w + (px + x)) * 4;
                image_data[dst+0] = 255;
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
    write_ff(&atlas_output_path, &atlas);

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
    let code_points = CHARS.iter().map(|c| { GetCodepointBitmap(&font, *c, FONT_HEIGHT_TARGET) }).collect::<Vec<_>>();

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
    write_ff(&atlas_output_path, &atlas);

    match load_ff(&atlas_output_path) {
        Ok(im) => {

            println!("Loaded Image: {}x{}", im.w, im.h);

        }


        Err(err) => {
            println!("{}", err);
        }
    };
}

const fn create_target_chars() -> [char; 96] {
	let mut chars = ['a'; 128 - 32];
	let mut i = 32u8;
	while i < 128u8 {
		chars[(i - 32) as usize] = i as char;
		i+=1;
	}
	chars
}


fn get_time_of_day() -> (u64, u64, u64) {
    let duration = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    let total_seconds = duration.as_secs();
    let secs_in_day = total_seconds % 86400;
    let h = secs_in_day / 3600;
    let m = (secs_in_day % 3600) / 60;
    let s = secs_in_day % 60;

    (h, m, s)
}
