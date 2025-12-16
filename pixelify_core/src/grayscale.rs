use crate::pixelify_errors::ImageProcessingError;

pub fn grayscale_png(bytes: &[u8]) -> Result<Vec<u8>, ImageProcessingError> {
    let image = image::load_from_memory(bytes)
        .map_err(|_| ImageProcessingError::failed("grayscale", "Failed to decode input image"))?;

    let luma = image.to_luma8();

    let mut out = Vec::new();
    luma.write_to(&mut std::io::Cursor::new(&mut out), image::ImageFormat::Png)
        .map_err(|_| ImageProcessingError::failed("grayscale", "Failed to encode PNG"))?;

    Ok(out)
}
