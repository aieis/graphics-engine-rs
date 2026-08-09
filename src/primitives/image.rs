#[derive(Clone, Copy)]
pub enum PixelFormat {
    RGBA,
    Z16
}

pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub size: u64,

    pub dirty: bool
}

impl Image {
    pub fn new(data: Vec<u8>, width: u32, height: u32, format: PixelFormat) -> Self {
        let bpp = pixel_format_bpp(format) as u64;
        Self {
            data,
            width,
            height,
            format,
            size: width as u64 * height as u64 * bpp,
            dirty: true
        }
    }

    pub fn copy_to_data(&mut self, data: &[u8]) {
        if data.len() == self.data.len() {
            self.data.copy_from_slice(data);
            self.dirty = true;
        }
    }
}

pub fn pixel_format_bpp(format: PixelFormat) -> usize {
	match format {
            PixelFormat::RGBA => 4,
            PixelFormat::Z16 => 2
    }
}
