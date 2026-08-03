#[derive(Clone, Copy)]
pub enum PixelFormat {
    RGBA,
    Z16
}

pub struct Texture2d {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub size: u64,

    pub dirty: bool
}

impl Texture2d {
    pub fn new(data: Vec<u8>, width: u32, height: u32, format: PixelFormat) -> Self {
        let bpp = match format {
            PixelFormat::RGBA => 4,
            PixelFormat::Z16 => 2
        };

        Self {
            data,
            width,
            height,
            format,
            size: width as u64 * height as u64 * bpp,
            dirty: true
        }
    }

    pub fn update_data(&mut self, data: Vec<u8>) {
        self.data = data;
        self.dirty = true;
    }
}
