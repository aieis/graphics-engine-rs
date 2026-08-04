use stb_truetype::*;


const FONT_BUFFER: &[u8] = include_bytes!("../../../assets/fonts/Iosevka-Regular.ttf");
// const FONT_BUFFER: &[u8] = include_bytes!("../../../assets/fonts/IosevkaTerm-Regular.ttf");


/* 4-Channel 8-bit image */
struct RgbaImage {
    w: u32,
    h: u32,
    data: Vec<u8>,
}

fn main() {
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
    let h = h_c * chars.len();

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


    let atlas_path = "atlas/iosevka.ff";

    write_ff(atlas_path, atlas);

    match load_ff(atlas_path) {
        Ok(im) => {

            println!("Loaded Image: {}x{}", im.w, im.h);

        }


        Err(err) => {
            println!("{}", err);
        }
    };
}


/* Farbfeld images */

const FF_MAGIC_BYTES: &[u8] = b"farbfeld";
const FF_HEADER_LEN : usize = 8 + 4 + 4;
const U8_U16_MAP: [u16; 256] = make_u8_u16_map();

fn write_ff(path: &str, im: RgbaImage)
{


    let data_len = (im.w * im.h * 4 * 2) as usize;
    let mut buffer: Vec<u8> = vec![0; FF_HEADER_LEN + data_len];


    let mut ptr = 0;
    buffer[ptr..ptr+FF_MAGIC_BYTES.len()].copy_from_slice(FF_MAGIC_BYTES);
    ptr += FF_MAGIC_BYTES.len();

    buffer[ptr..ptr+4].copy_from_slice(&im.w.to_be_bytes());
    ptr += 4;

    buffer[ptr..ptr+4].copy_from_slice(&im.h.to_be_bytes());

    for r in 0..im.h as usize {
        for c in 0..im.w as usize {
            let src = (r * im.w as usize + c) * 4;
            let dst = (r * im.w as usize + c) * 4 * 2 + FF_HEADER_LEN;

            let r = U8_U16_MAP[im.data[src]   as usize].to_be_bytes();
            let g = U8_U16_MAP[im.data[src+1] as usize].to_be_bytes();
            let b = U8_U16_MAP[im.data[src+2] as usize].to_be_bytes();
            let a = U8_U16_MAP[im.data[src+3] as usize].to_be_bytes();

            let mut ptr = dst;

            buffer[ptr..ptr+2].copy_from_slice(&r);
            ptr+=2;

            buffer[ptr..ptr+2].copy_from_slice(&g);
            ptr+=2;

            buffer[ptr..ptr+2].copy_from_slice(&b);
            ptr+=2;

            buffer[ptr..ptr+2].copy_from_slice(&a);
        }
    }


    std::fs::write(path, buffer).expect("Failed to write ff test image");

}

fn test_farbfeld_image(path: &str)
{
    let h = 512 + 64;
    let w = 512 + 64;

    let mut data = vec![0; h * w * 4 * 16];

    let s = 64;

    for r in 0..h as usize {
        for c in 0..w as usize {
            let b = (r / s) % 2 == 1 && (c / s) % 2 == 1;
            let v = if b {255} else {0};

            let dst = (r * w as usize + c) * 4;
            data[dst  ] = v;
            data[dst+1] = v;
            data[dst+2] = v;
            data[dst+3] = 255;
        }
    }


    let im = RgbaImage {
        w: w as u32,
        h: h as u32,
        data
    };

    write_ff(path, im);
}


fn load_ff(path: &str) -> Result<RgbaImage, String> {
    let data = match std::fs::read(path) {
        Ok(data) => data,
        Err(err) => return Err(format!("Failed to read farbfeld image at {}: \n\t {}", path, err.to_string())),
    };


    if data.len() < FF_HEADER_LEN {
        return Err(format!("Data length ({}) is less than the FF_HEADER_LEN ({})", data.len(), FF_HEADER_LEN));
    }

    let magic = &data[0..FF_MAGIC_BYTES.len()];

    if magic != FF_MAGIC_BYTES {
        return Err(format!("Magic bytes do not match farbfeld : \n\t Got {:?} \n\t Exp {:?}", magic, FF_MAGIC_BYTES));
    }

    let mut ptr = FF_MAGIC_BYTES.len();

    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[ptr..ptr+4]);
    ptr += 4;

    let w = u32::from_be_bytes(bytes);

    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[ptr..ptr+4]);
    let h = u32::from_be_bytes(bytes);

    let mut buffer = vec![0u8; (w * h * 4) as usize];

    for r in 0..w as usize {
        for c in 0..h as usize {
            let src = r * c * 4 * 2 + FF_MAGIC_BYTES.len();
            let dst = r * c * 4;


            // We are ignoring the second byte
            let r = u8::from_be(data[src]   as u8);
            let g = u8::from_be(data[src+1] as u8);
            let b = u8::from_be(data[src+2] as u8);
            let a = u8::from_be(data[src+3] as u8);

            buffer[dst + 0] = r;
            buffer[dst + 1] = g;
            buffer[dst + 2] = b;
            buffer[dst + 3] = a;
        }
    }


    Ok( RgbaImage {
        w,
        h,
        data: buffer,
    })

}

const U8_U16_F: f32 = u16::MAX as f32 / u8::MAX as f32;
const fn make_u8_u16_map() -> [u16; 256] {
    let mut map = [0; 256];
    let mut i = 0;
    while i < 256 {
        map[i] = (i as f32 * U8_U16_F) as u16;
        i += 1;
    }
    map
}
