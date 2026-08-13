
use crate::common::*;
use anyhow::{anyhow, Result, Error};


use farbfeld_image::{load_ff_from_memory, write_ff, RgbaImage};

pub const CHARS_LEN: usize = 96;
const CHARS: [char; CHARS_LEN] = create_target_chars();

#[derive(Debug)]
pub struct FontAtlasDescription {
    pub scale: f32,
    pub ascent: i32,
    pub info: FontAtlasInfo,
    pub advance_map: [[i32; CHARS_LEN]; CHARS_LEN],
    pub glyph_info: [GlyphInfo; CHARS_LEN],
}


pub struct FontAtlas {
	pub atlas: RgbaImage,
    pub desc: FontAtlasDescription
}

impl FontAtlas {

    pub fn new(font: &FontInfo, pixel_height: f32) -> Self {

        let scale = ScaleFontForPixelHeight(&font, pixel_height);

        let glyphs: [Glyph; CHARS_LEN] = CHARS.map(|c| { GetGlyph(&font, c, scale) });

        let mut char_width  = 0;
        let mut char_height = 0;

        for glyph in glyphs.iter() {
            char_width  = if glyph.info.w > char_width {glyph.info.w} else { char_width };
            char_height = if glyph.info.h > char_height {glyph.info.h} else { char_height };
        }

        println!("Character dimensions: {}x{}", char_width, char_height);

        let chars_per_row = 12;
        let chars_per_col = glyphs.len() / chars_per_row;

        let w = char_width  * chars_per_row;
        let h = char_height * chars_per_col;

        let mut image_data = vec![0; w * h * 4];

        for (i, glyph) in glyphs.iter().enumerate() {
            let c_i = CHARS[i] as usize - 32;

            let c_pos_x = c_i % chars_per_row;
            let c_pos_y = c_i / chars_per_row;

            let px = c_pos_x * char_width;
            let py = (c_pos_y+1) * char_height - glyph.info.h;

            for y in 0..glyph.info.h {
			    for x in 0..glyph.info.w {
                    let dst = ((py + y) * w + (px + x)) * 4;
                    image_data[dst+0] = 255;
                    image_data[dst+3] = glyph.bitmap[y*glyph.info.w + x];
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

        let glyph_info = glyphs.each_ref().map(|g| { g.info.clone() });

        let desc = FontAtlasDescription {
            info: atlas_info,
            ascent,
            advance_map,
            scale,
            glyph_info
        };

        Self {
            atlas,
            desc,
        }
    }

    pub fn pack_kerning_data(&self, text: &[u8], kerning_data: &mut [(i32, i32)]) {
        if text.len() == 0 {
            return;
        }

        let baseline = self.desc.ascent as f32 * self.desc.scale;

        let mut prev_char_advance = self.get_char_advance(text[0] as char);
        let (x, y) = self.get_glyph_inner(text[0] as char);

        kerning_data[0] = (x, baseline as i32 + y);

        let mut x_pos = 0.0;

		for i in 1..text.len(){
			let c_i   = text[i-1] as usize - 32;
			let c_i_1 = text[i] as usize - 32;
			let advance_amount = self.desc.advance_map[c_i][c_i_1] as f32 * self.desc.scale + prev_char_advance;
			x_pos = x_pos + advance_amount;

            let (x, y) = self.get_glyph_inner(text[i] as char);

            kerning_data[i] = (x_pos as i32 + x, baseline as i32 + y);

			prev_char_advance = self.get_char_advance(text[i] as char);
		}
    }

    pub fn pack_message_with_kerning_data(&self, message: &[u8], kerning_data: &[(i32, i32)]) -> RgbaImage {
		let width  = self.desc.info.char_width * message.len() as u32 * 2;
		let height = self.desc.info.char_height * 2;

		let mut im = vec![0; (width * height * 4) as usize];
		let stride = width as usize * 4;

		for i in 0..message.len(){
            let (px, py) = kerning_data[i];
            let c_i = message[i] as usize  - 32;

		    let glyph_info = &self.desc.glyph_info[c_i];

            let px = px as usize;
            let py = py as usize;

            let glyph_col = c_i % self.desc.info.chars_per_row as usize;
            let glyph_row = c_i / self.desc.info.chars_per_row as usize;

            let atlas_px =   glyph_col   * self.desc.info.char_width as usize;
            let atlas_py = (glyph_row+1) * self.desc.info.char_height as usize - glyph_info.h;
            let atlas_stride = (self.desc.info.chars_per_row * self.desc.info.char_width * 4) as usize;

            for y in 0..glyph_info.h {
			    for x in 0..glyph_info.w {
                    let src = (atlas_py + y) * atlas_stride + (atlas_px + x) *4;
                    let dst = (py + y) * stride + (px + x) * 4;
                    im[dst+2] = 255;
                    im[dst+3] = self.atlas.data[src+3];
			    }
		    }
		}

		RgbaImage {
			w: width,
			h: height,
			data: im,
		}

    }

	pub fn pack_message(&self, text: &str) -> RgbaImage {

		let message = text.as_bytes();

		if message.len() == 0 {
			return RgbaImage {
				w: 0,
				h: 0,
				data: vec![],
			};
		}

		let width  = self.desc.info.char_width * message.len() as u32 * 2;
		let height = self.desc.info.char_height * 2;

		let mut im = vec![0; (width * height * 4) as usize];
		let stride = width as usize * 4;

		let baseline = self.desc.ascent as f32 * self.desc.scale;

		let mut char_advance = self.pack_char(&mut im, message[0] as char, (0, baseline as usize), stride);

		let mut x_pos = 0.0f32;

		for i in 1..message.len(){
			let c_i   = message[i-1] as usize - 32;
			let c_i_1 = message[i] as usize - 32;
			let advance_amount = self.desc.advance_map[c_i][c_i_1] as f32 * self.desc.scale + char_advance;
			x_pos = x_pos + advance_amount;

			char_advance = self.pack_char(&mut im, message[i] as char, (x_pos as usize, baseline as usize), stride);
		}

		RgbaImage {
			w: width,
			h: height,
			data: im,
		}
	}

	pub fn pack_char(&self, im: &mut [u8], c: char, point: (usize, usize), stride: usize) -> f32 {
		let c_i = c as usize - 32;

		let glyph_info = &self.desc.glyph_info[c_i];

		let (px, py) = point;

        let px = (px as i32 + glyph_info.x) as usize;
        let py = (py as i32 + glyph_info.y as i32) as usize;

        let glyph_col = c_i % self.desc.info.chars_per_row as usize;
        let glyph_row = c_i / self.desc.info.chars_per_row as usize;

        let atlas_px =   glyph_col   * self.desc.info.char_width as usize;
        let atlas_py = (glyph_row+1) * self.desc.info.char_height as usize - glyph_info.h;
        let atlas_stride = (self.desc.info.chars_per_row * self.desc.info.char_width * 4) as usize;

        for y in 0..glyph_info.h {
			for x in 0..glyph_info.w {
                let src = (atlas_py + y) * atlas_stride + (atlas_px + x) *4;
                let dst = (py + y) * stride + (px + x) * 4;
                im[dst+0] = 255;
                im[dst+3] = self.atlas.data[src+3];
			}
		}

		self.get_char_advance(c)
	}

    pub fn get_char_advance(&self, c: char) -> f32 {
        self.desc.glyph_info[c as usize - 32].advance as f32 * self.desc.scale
    }

    pub fn get_glyph_inner(&self, c: char) -> (i32, i32) {
        let glyph_info = &self.desc.glyph_info[c as usize - 32];
        (glyph_info.x, glyph_info.y)
    }

    pub fn write_atlas_files(&self, pfx: &str) {

	    let atlas_output_path = get_font_atlas_path(pfx, &self.desc.info);
        write_ff(&atlas_output_path, &self.atlas);

        let atlas_desc_output_path = get_font_atlas_desc_path(pfx, &self.desc.info);

        let mut ptr = 0;
        let mut buf = [0; S_BUF];

        buf[ptr..ptr+S_F32].copy_from_slice(&self.desc.scale.to_be_bytes());
        ptr += S_F32;

        buf[ptr..ptr+S_I32].copy_from_slice(&self.desc.ascent.to_be_bytes());
        ptr += S_I32;

        // Atlas Info Time
        buf[ptr..ptr+S_U32].copy_from_slice(&self.desc.info.chars_per_row.to_be_bytes());
        ptr += S_U32;

        buf[ptr..ptr+S_U32].copy_from_slice(&self.desc.info.chars_per_col.to_be_bytes());
        ptr += S_U32;

        buf[ptr..ptr+S_U32].copy_from_slice(&self.desc.info.char_width.to_be_bytes());
        ptr += S_U32;

        buf[ptr..ptr+S_U32].copy_from_slice(&self.desc.info.char_height.to_be_bytes());
        ptr += S_U32;

        // Advance map time
        let mut i = 0;
        while i < CHARS_LEN {
            let mut j = 0;
            while j < CHARS_LEN {
                buf[ptr..ptr+S_I32].copy_from_slice(&self.desc.advance_map[i][j].to_be_bytes());
				ptr += S_I32;
				j += 1;
            }
			i += 1;
        }

		// Write the glyph info

        let mut i = 0;
        while i < CHARS_LEN {
            buf[ptr..ptr+S_I32].copy_from_slice(&self.desc.glyph_info[i].x.to_be_bytes());
			ptr += S_I32;

            buf[ptr..ptr+S_I32].copy_from_slice(&self.desc.glyph_info[i].y.to_be_bytes());
			ptr += S_I32;

            buf[ptr..ptr+S_I32].copy_from_slice(&self.desc.glyph_info[i].advance.to_be_bytes());
			ptr += S_I32;

            buf[ptr..ptr+S_USZ].copy_from_slice(&self.desc.glyph_info[i].w.to_be_bytes());
			ptr += S_USZ;

            buf[ptr..ptr+S_USZ].copy_from_slice(&self.desc.glyph_info[i].h.to_be_bytes());
			ptr += S_USZ;

			i += 1;
        }

		match std::fs::write(&atlas_desc_output_path, buf) {
			Ok(_) => {},
			Err(err) => println!("Error writing atlas file '{}': \n\t {}", atlas_desc_output_path, err),
		}
    }


    pub fn parse_atlas_from_files(path: &str) -> Result<Self, Error> {

        let atlas_desc_output_path = get_font_atlas_desc_from_path(path);

        let buf: Vec<u8> = match std::fs::read(atlas_desc_output_path) {
            Ok(buf) => buf,
            Err(err) => { return Err(anyhow!(err)); }
        };

        let atlas_image_data = match std::fs::read(path) {
            Ok(buf) => buf,
            Err(err) => { return Err(anyhow!(err)); }
        };

        Self::parse_atlas_from_memory(&buf, &atlas_image_data)

    }

    pub fn parse_atlas_from_memory(atlas_desc_buf: &[u8], atlas_image_data: &[u8]) -> Result<Self, Error> {

		let atlas = match load_ff_from_memory(&atlas_image_data) {
			Ok(atlas) => atlas,
			Err(err) => { return Err(anyhow!(err)); }
		};

        if atlas_desc_buf.len() != S_BUF {
            return Err(anyhow!(format!("Expected buffer size is: {} byte(s). Read {} byte(s).", S_BUF, atlas_desc_buf.len())))
        }

        let mut ptr = 0;

		let scale = f32::from_be_bytes(*<&[u8; S_F32]>::try_from(&atlas_desc_buf[ptr..ptr+S_F32]).unwrap());
        ptr += S_F32;


        let ascent = i32::from_be_bytes(*<&[u8; S_I32]>::try_from(&atlas_desc_buf[ptr..ptr+S_I32]).unwrap());
        ptr += S_I32;

        // Atlas Info Time
        let chars_per_row = u32::from_be_bytes(*<&[u8; S_U32]>::try_from(&atlas_desc_buf[ptr..ptr+S_U32]).unwrap());
        ptr += S_U32;

        let chars_per_col = u32::from_be_bytes(*<&[u8; S_U32]>::try_from(&atlas_desc_buf[ptr..ptr+S_U32]).unwrap());
        ptr += S_U32;

        let char_width = u32::from_be_bytes(*<&[u8; S_U32]>::try_from(&atlas_desc_buf[ptr..ptr+S_U32]).unwrap());
        ptr += S_U32;

        let char_height = u32::from_be_bytes(*<&[u8; S_U32]>::try_from(&atlas_desc_buf[ptr..ptr+S_U32]).unwrap());
        ptr += S_U32;

		let info = FontAtlasInfo {
			chars_per_row,
			chars_per_col,
			char_width,
			char_height,
		};

        // Advance map time

		let mut advance_map = [[0i32; CHARS_LEN]; CHARS_LEN];

        let mut i = 0;
        while i < CHARS_LEN {
            let mut j = 0;
            while j < CHARS_LEN {
                advance_map[i][j] = i32::from_be_bytes(*<&[u8; S_I32]>::try_from(&atlas_desc_buf[ptr..ptr+S_I32]).unwrap());
				ptr += S_I32;
				j += 1;
            }
			i += 1;
        }

		// Write the glyph info

		let mut glyph_info = [0; CHARS_LEN].map(|_x| { GlyphInfo { x: 0, y: 0, advance: 0, w: 0, h: 0 } });

        let mut i = 0;
        while i < CHARS_LEN {
            let x = i32::from_be_bytes(*<&[u8; S_I32]>::try_from(&atlas_desc_buf[ptr..ptr+S_I32]).unwrap());
			ptr += S_I32;

            let y = i32::from_be_bytes(*<&[u8; S_I32]>::try_from(&atlas_desc_buf[ptr..ptr+S_I32]).unwrap());
			ptr += S_I32;

            let advance = i32::from_be_bytes(*<&[u8; S_I32]>::try_from(&atlas_desc_buf[ptr..ptr+S_I32]).unwrap());
			ptr += S_I32;

            let w = usize::from_be_bytes(*<&[u8; S_USZ]>::try_from(&atlas_desc_buf[ptr..ptr+S_USZ]).unwrap());
			ptr += S_USZ;

            let h = usize::from_be_bytes(*<&[u8; S_USZ]>::try_from(&atlas_desc_buf[ptr..ptr+S_USZ]).unwrap());
			ptr += S_USZ;

			glyph_info[i] = GlyphInfo { x, y, advance, w, h };

			i += 1;
        }

		let desc = FontAtlasDescription {
			scale,
			ascent,
			info,
			advance_map,
			glyph_info,
		};

		Ok(Self {
			atlas,
			desc,
		})

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

const S_U32: usize = std::mem::size_of::<u32>();
const S_I32: usize = std::mem::size_of::<i32>();
const S_F32: usize = std::mem::size_of::<f32>();
const S_USZ: usize = std::mem::size_of::<usize>();

const S_ATLAS_INFO: usize  = S_U32 * 4;
const S_ADVANCE_MAP: usize = S_I32 * CHARS_LEN * CHARS_LEN;

const S_GLYPH_SIZE: usize = S_I32 * 3 + S_USZ * 2;
const S_GLYPH_INFOS_SIZE: usize = S_GLYPH_SIZE * CHARS_LEN;

const S_BUF: usize = S_F32 + S_I32 + S_ATLAS_INFO + S_ADVANCE_MAP + S_GLYPH_INFOS_SIZE;
