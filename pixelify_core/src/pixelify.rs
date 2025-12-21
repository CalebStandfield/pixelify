//! The core functionality of Pixelify.
//!
//! Should take in an image and turn it into a sprite style image.
//! This can either be done with downscale or largescale pixel writing.
//! For example, downscale might take a 1000 x 1000 image and turn it into 100 x 100.
//! Largescale might take that same image, keep the same dimensions, but make it look pixelized.
//! I.e., each pixel essentially becomes 10x the size to look similar to the 100 x 100 downscaled version, while retaining the original dimensions.
//!
//! Users should have the choice between setting the pixel size, ex, pixel_size = 8.
//! Or they should be able to enter in their desired image size, ex, w = 128, h = 72, and then the backed determine pixel size from that.

use crate::pixelify_errors::ImageProcessingError;
use image::{GenericImageView, RgbaImage};

pub fn pixelify_downscale_by_pixel_size(
    bytes: &[u8],
    pixel_size: u32,
) -> Result<Vec<u8>, ImageProcessingError> {
    if pixel_size == 0 {
        return Err(ImageProcessingError::failed(
            "pixelify_downscale_by_pixel_size",
            "Pixel size must be a positive number",
        ));
    }

    // Clone bytes since we can't modify them
    // Nor should we since this should be non-destructive
    let bytes = bytes.to_vec();

    let image = image::load_from_memory(&bytes).map_err(|_| {
        ImageProcessingError::failed("pixelify_downscale_by_pixel_size", "Failed to decode PNG")
    })?;

    let image = image.to_rgba8();

    let (width, height) = image.dimensions();

    // New number of pixels by width with truncation
    let new_width = width / pixel_size;

    // New number of pixels by height with truncation
    let new_height = height / pixel_size;

    // Create a new byte array of this size
    let downscaled_image = Vec::with_capacity(new_width as usize * new_height as usize);

    // For each pixel_size * pixel_size block in the original image calculate the average color

    // Take the average color and map that to the downscaled image
}

pub fn pixelify_by_image_size(bytes: &[u8], width: u32, height: u32) {
    // Clone bytes since we can't modify them
    // Nor should we since this should be non-destructive
}

fn get_average_rgba(image: RgbaImage, x: u32, y: u32, pixel_size: u32) -> Result<Vec<u8>, ImageProcessingError> {
    
}
