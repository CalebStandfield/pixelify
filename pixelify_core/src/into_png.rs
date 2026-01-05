//! Take in bytes from any image type and produced a png that can be used for pixelify modifications.
//! The goal is to make other image types `usable` with pixelify, instead of `best effort`.
//! Keeping in mind the process of conversation between types will increase overhead memory and computation costs.

use image::{GenericImageView, ImageFormat};
use crate::pixelify_errors::ImageProcessingError;
use crate::PixelifyImage;

pub fn into_png(bytes: Vec<u8>) -> Result<PixelifyImage, ImageProcessingError> {
    let image = image::load_from_memory(&bytes).expect("");

    let (width, height) = image.dimensions();

    let mut cursor = std::io::Cursor::new(Vec::new());
    image
        .write_to(&mut cursor, ImageFormat::Png)
        .map_err(|_| ImageProcessingError::failed("crop", "Failed to encode PNG"))?;

    Ok(PixelifyImage::new(cursor.into_inner(), width, height))
}