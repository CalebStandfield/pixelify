

pub struct PixelifyOptions {
    pub width: u8,
    pub height: u8
}

pub struct PixelifyImage {
    pub pixels: Vec<u8>,
    pub options: PixelifyOptions,
}

impl PixelifyImage {
    pub fn new(bytes: Vec<u8>, height: u8, width: u8) -> PixelifyImage {
        Self {
            pixels: bytes,
            options: PixelifyOptions {
                width,
                height,
            }
        }
    }
}

pub fn pixelify_png(bytes: Vec<u8>, height: u8, width: u8) -> PixelifyImage {
    PixelifyImage::new(bytes, height, width)
}