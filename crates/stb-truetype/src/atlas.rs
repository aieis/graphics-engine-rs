use crate::{FontAtlasInfo, FontInfo, GetGlyph, FONT_ATLAS_PATH_PFX, get_font_atlas_path, ScaleFontForPixelHeight, GetCodepointKernAdvance};

use farbfeld_image::{load_ff, write_ff, RgbaImage};

const CHARS_LEN: usize = 96;
const CHARS: [char; CHARS_LEN] = create_target_chars();
const FONT_HEIGHT_TARGET: f32 = 40.0;


pub struct FontAtlasDescription {
    info: FontAtlasInfo,
    scale: f32,
    internal_advance: [i32; CHARS_LEN],
    advance_map: [[i32; CHARS_LEN]; CHARS_LEN]
}

impl FontAtlasDescription {

    pub fn new(font: &FontInfo, scale: f32) -> Self {

        let scale = ScaleFontForPixelHeight(&font, FONT_HEIGHT_TARGET);

        let glyphs = CHARS.iter().map(|c| { GetGlyph(&font, *c, scale) }).collect::<Vec<_>>();

        let mut char_width  = 0;
        let mut char_height = 0;

        for c in glyphs.iter() {
            char_width  = if c.w > char_width {c.w} else { char_width };
            char_height = if c.h > char_height {c.h} else { char_height };
        }

        println!("Character dimensions: {}x{}", char_width, char_height);

        let chars_per_row = 12;
        let chars_per_col = glyphs.len() / chars_per_row;

        let w = char_width  * chars_per_row;
        let h = char_height * chars_per_col;

        let mut image_data = vec![0; w * h * 4];

        for (i, c) in glyphs.iter().enumerate() {
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
        write_ff(&atlas_output_path, atlas);

        match load_ff(&atlas_output_path) {
            Ok(im) => {
                println!("Loaded Image: {}x{}", im.w, im.h);
            }

            Err(err) => {
                println!("{}", err);
            }
        };

        let mut internal_advance: [i32; CHARS_LEN] = [0; CHARS_LEN];

        for i in 0..CHARS_LEN {
            internal_advance[i] = glyphs[i].advance;
        }

        let mut advance_map = [[0; CHARS_LEN]; CHARS_LEN];
        for i in 0..CHARS_LEN {
            for j in 0..CHARS_LEN {
                advance_map[i][j] = GetCodepointKernAdvance(&font, CHARS[i], CHARS[j]);
            }
        }
        
        Self {
            info: atlas_info,
            internal_advance,
            advance_map,
            scale,
        }
    }


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
