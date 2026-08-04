/* 4-Channel 8-bit image */
pub struct RgbaImage {
    pub w: u32,
    pub h: u32,
    pub data: Vec<u8>,
}


const FF_MAGIC_BYTES: &[u8] = b"farbfeld";
const FF_HEADER_LEN : usize = 8 + 4 + 4;
const U8_U16_MAP: [u16; 256] = make_u8_u16_map();

pub fn write_ff(path: &str, im: RgbaImage)
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

pub fn load_ff(path: &str) -> Result<RgbaImage, String> {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_load_farbfeld_image()
    {
        std::fs::create_dir("./test_output");

        let path = "./test_output/output.ff";

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

    match load_ff(path) {
        Ok(im) => {
            assert!(im.w as usize == w && im.h as usize == h);
        }

        Err(_err) => {
            assert!(false)
        }
    };

    }

}
