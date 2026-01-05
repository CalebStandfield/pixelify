use crate::PixelifyImage;
use crate::pixelify_errors::ImageProcessingError;
use image::GenericImageView;

/// Crops a rectangular portion of an image.
///
/// The crop rectangle is defined by its top-left corner (`x`, `y`)
/// and its width (`width`) and height (`height`), all in pixels.
///
/// If the requested crop size x + w and y + h are outside the bounds of the image,
/// then the values will be clamped to fit within the image.
///
/// # Errors
///
/// Returns an error if:
/// - loading the bytes from memory fails,
/// - x or y is outside the image dimensions,
/// - w or h is 0,
/// - writing the bytes into the output fails.
/// Each error is of the type `ImageProcessingError` with a related message.
pub fn crop_png(
    bytes: &[u8],
    x: u32,
    y: u32,
    width: u32,
    height: u32,
) -> Result<PixelifyImage, ImageProcessingError> {
    let image = image::load_from_memory(bytes)
        .map_err(|_| ImageProcessingError::failed("crop", "Failed to decode input image"))?;

    let (img_w, img_h) = image.dimensions();

    if x >= img_w || y >= img_h {
        return Err(ImageProcessingError::failed(
            "crop",
            "Crop origin out of bounds",
        ));
    }

    let max_w = img_w - x;
    let max_h = img_h - y;

    let width = width.min(max_w);
    let height = height.min(max_h);

    if width == 0 || height == 0 {
        return Err(ImageProcessingError::failed("crop", "Crop size is zero"));
    }

    let cropped = image.crop_imm(x, y, width, height);

    let mut cursor = std::io::Cursor::new(Vec::new());
    cropped
        .write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|_| ImageProcessingError::failed("crop", "Failed to encode PNG"))?;

    Ok(PixelifyImage::new(cursor.into_inner(), width, height))
}
