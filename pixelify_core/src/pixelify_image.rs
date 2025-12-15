pub struct PixelifyOptions {
    width: u32,
    height: u32,
}

pub struct PixelifyImage {
    pixels: Vec<u8>,
    options: PixelifyOptions,
}

impl PixelifyImage {
    pub fn new(bytes: Vec<u8>,width: u32,  height: u32) -> PixelifyImage {
        Self {
            pixels: bytes,
            options: PixelifyOptions {
                width,
                height,
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.pixels
    }
}