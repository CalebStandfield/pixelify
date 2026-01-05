pub struct ImageDimensions {
    width: u32,
    height: u32,
}

pub struct PixelifyImage {
    pixels: Vec<u8>,
    dimensions: ImageDimensions,
}

impl PixelifyImage {
    pub fn new(bytes: Vec<u8>, width: u32, height: u32) -> PixelifyImage {
        Self {
            pixels: bytes,
            dimensions: ImageDimensions { width, height },
        }
    }

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.pixels
    }

    pub fn get_width(&self) -> u32 {
        self.dimensions.width
    }

    pub fn get_height(&self) -> u32 {
        self.dimensions.height
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.pixels
    }
}
