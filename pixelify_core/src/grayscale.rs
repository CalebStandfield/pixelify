use image::GenericImageView;
use crate::pixelify_errors::ImageProcessingError;
use crate::PixelifyImage;

/// Converts image into a grayscale format.
///
/// Take in image bytes, loads them into memory, then transforms it into a luma8 image.
/// Writes those into a resultant vector.
///
/// # Errors
///
/// Returns an error if:
/// - loading the bytes from memory fails,
/// - writing the bytes into the output fails.
/// Both failures result in an `ImageProcessingError` with a relevant message.
pub fn grayscale_png(bytes: &[u8]) -> Result<PixelifyImage, ImageProcessingError> {
    let image = image::load_from_memory(bytes)
        .map_err(|_| ImageProcessingError::failed("grayscale", "Failed to decode input image"))?;
    
    let (width, height) = image.dimensions();

    let luma = image.to_luma8();

    let mut out = Vec::new();
    luma.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
        .map_err(|_| ImageProcessingError::failed("grayscale", "Failed to encode PNG"))?;

    Ok(PixelifyImage::new(out, width, height))
}
