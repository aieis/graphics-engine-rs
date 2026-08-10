use stb_truetype::*;
use crate::FONT_ATLAS_PATH_PFX;


use farbfeld_image::{load_ff, write_ff, RgbaImage};

const CHARS_LEN: usize = 96;
const CHARS: [char; CHARS_LEN] = create_target_chars();


pub struct FontAtlasDescription {
    pub info: FontAtlasInfo,
	pub glyphs: [Glyph; CHARS_LEN],
    pub scale: f32,
    pub ascent: i32,
    pub advance_map: [[i32; CHARS_LEN]; CHARS_LEN]
}

impl FontAtlasDescription {

    pub fn new(font: &FontInfo, pixel_info: f32) -> Self {

        let scale = ScaleFontForPixelHeight(&font, pixel_info);

        let glyphs: [Glyph; CHARS_LEN] = CHARS.map(|c| { GetGlyph(&font, c, scale) });

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

        let mut advance_map = [[0; CHARS_LEN]; CHARS_LEN];
        for i in 0..CHARS_LEN {
            for j in 0..CHARS_LEN {
                advance_map[i][j] = GetCodepointKernAdvance(&font, CHARS[i], CHARS[j]);
            }
        }

		let ascent = GetFontVMetrics(&font);

		println!("Atlas Info: \n\t Char Dims: {}x{} \n\t Ascent: {} \n\t Scale: {}",
				 atlas_info.char_width,
				 atlas_info.char_height,
				 ascent,
				 scale,
		);

        Self {
            info: atlas_info,
            ascent,
            advance_map,
            scale,
            glyphs
        }
    }


	pub fn pack_message(&self, text: &str) -> RgbaImage {

		let message = text.as_bytes();

		let width = self.info.chars_per_row * self.info.char_width * message.len() as u32;
		let height = self.info.char_height * 30;

		let mut im = vec![0; (width * height * 4) as usize];
		let stride = width as usize * 4;

		let baseline = self.ascent as f32 * self.scale;
		println!("Baseline: {}", baseline);

		let py_base = baseline as i32 + 80;

		let mut x_pos = 0;
		for i in 1..message.len() - 1 {
			let c = message[i];
			let c_i = c as usize - 32;
			let glyph = &self.glyphs[c_i];

            let px = ((x_pos as i32 + (glyph.advance as f32 * self.scale) as i32) + glyph.x) as usize;
			println!("Glyph.Y: {}", glyph.y);
            let py = (py_base + glyph.y) as usize;

			Self::pack_char(&mut im, glyph, (px, py), stride);
			let c_i_1 = message[i+1] as usize - 32;
			x_pos += self.advance_map[c_i][c_i_1];
		}

		RgbaImage {
			w: width,
			h: height,
			data: im,
		}
	}

	pub fn pack_char(im: &mut [u8], glyph: &Glyph, point: (usize, usize), stride: usize) {
		let (px, py) = point;
        for y in 0..glyph.h {
			for x in 0..glyph.w {
                let dst = ((py + y) * stride + (px + x)) * 4;
                im[dst+0] = 255;
                im[dst+3] = glyph.bitmap[y*glyph.w + x];
			}
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
