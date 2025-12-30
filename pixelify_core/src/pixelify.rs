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
use image::RgbaImage;
use crate::PixelifyImage;

pub fn pixelify_downscale_by_pixel_size(
    bytes: &[u8],
    pixel_size: u32,
) -> Result<PixelifyImage, ImageProcessingError> {
    if pixel_size == 0 {
        return Err(ImageProcessingError::failed(
            "pixelify_downscale_by_pixel_size",
            "Pixel size must be a positive number",
        ));
    }

    let image = image::load_from_memory(&bytes).map_err(|_| {
        ImageProcessingError::failed("pixelify_downscale_by_pixel_size", "Failed to decode PNG")
    })?;

    let image = image.to_rgba8();

    let (width, height) = image.dimensions();

    // New number of pixels by width with truncation
    let new_width = width / pixel_size;

    // New number of pixels by height with truncation
    let new_height = height / pixel_size;

    if new_width == 0 || new_height == 0 {
        return Err(ImageProcessingError::failed(
            "pixelify_downscale_by_pixel_size",
            "Pixel size is larger than the image dimensions",
        ));
    }

    let mut downscaled = vec![0u8; (new_width * new_height * 4) as usize];

    // Take the average color and map that to the downscaled image
    for by in 0..new_height {
        for bx in 0..new_width {
            let x = bx * pixel_size;
            let y = by * pixel_size;

            let (r, g, b, a) = get_average_rgba(&image, x, y, pixel_size)?;

            let out_i = ((by * new_width + bx) * 4) as usize;
            downscaled[out_i] = r;
            downscaled[out_i + 1] = g;
            downscaled[out_i + 2] = b;
            downscaled[out_i + 3] = a;
        }
    }
    
    Ok(PixelifyImage::new(downscaled, new_width, new_height))
}

pub fn pixelify_false_downscale_by_pixel_size(
    bytes: &[u8],
    pixel_size: u32,
) -> Result<PixelifyImage, ImageProcessingError> {
    if pixel_size == 0 {
        return Err(ImageProcessingError::failed(
            "pixelify_downscale_by_pixel_size",
            "Pixel size must be a positive number",
        ));
    }

    let image = image::load_from_memory(&bytes).map_err(|_| {
        ImageProcessingError::failed("pixelify_downscale_by_pixel_size", "Failed to decode PNG")
    })?;

    let image = image.to_rgba8();

    let (width, height) = image.dimensions();

    let mut false_downscaled = image.as_raw().to_vec();

    let blocks_x = width / pixel_size;
    let blocks_y = height / pixel_size;

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let x0 = bx * pixel_size;
            let y0 = by * pixel_size;

            let (r, g, b, a) = get_average_rgba(&image, x0, y0, pixel_size)?;

            for dy in 0..pixel_size {
                for dx in 0..pixel_size {
                    let x = x0 + dx;
                    let y = y0 + dy;

                    let out_i = ((y * width + x) * 4) as usize;
                    false_downscaled[out_i] = r;
                    false_downscaled[out_i + 1] = g;
                    false_downscaled[out_i + 2] = b;
                    false_downscaled[out_i + 3] = a;
                }
            }
        }
    }
    
    Ok(PixelifyImage::new(false_downscaled, width, height))
}

fn get_average_rgba(
    image: &RgbaImage,
    x: u32,
    y: u32,
    pixel_size: u32,
) -> Result<(u8, u8, u8, u8), ImageProcessingError> {
    if x + pixel_size > image.width() || y + pixel_size > image.height() {
        return Err(ImageProcessingError::failed(
            "Rgba Average",
            "Indexing would cause out of bounds error logic",
        ));
    }

    let mut red_sum: u32 = 0;
    let mut green_sum: u32 = 0;
    let mut blue_sum: u32 = 0;
    let mut alpha_sum: u32 = 0;
    let mut pixel_count: u32 = 0;

    for local_x in 0..pixel_size {
        for local_y in 0..pixel_size {
            let pixel = image.get_pixel(local_x + x, local_y + y);
            let [r, g, b, a] = pixel.0;
            red_sum += r as u32;
            green_sum += g as u32;
            blue_sum += b as u32;
            alpha_sum += a as u32;
            pixel_count += 1;
        }
    }

    Ok((
        (red_sum / pixel_count) as u8,
        (green_sum / pixel_count) as u8,
        (blue_sum / pixel_count) as u8,
        (alpha_sum / pixel_count) as u8,
    ))
}

pub fn pixelify_by_image_size(
    bytes: &[u8],
    new_width: u32,
    new_height: u32,
) -> Result<PixelifyImage, ImageProcessingError> {
    if new_width == 0 || new_height == 0 {
        return Err(ImageProcessingError::failed(
            "pixelify_by_image_size",
            "Width and height must be non-zero",
        ));
    }

    let image = image::load_from_memory(&bytes).map_err(|_| {
        ImageProcessingError::failed("pixelify_by_image_size", "Failed to decode PNG")
    })?;

    let image = image.to_rgba8();

    let (original_width, original_height) = image.dimensions();

    if new_width > original_width || new_height > original_height {
        return Err(ImageProcessingError::failed(
            "pixelify_by_image_size",
            "desired width and/or height is greater than original_width and original_height",
        ));
    }

    // This would give a rectangular box
    let pixel_size_x = original_width / new_width;
    let pixel_size_y = original_height / new_height;

    let pixel_size = pixel_size_x.min(pixel_size_y);

    let mut downscaled = vec![0u8; (new_width * new_height * 4) as usize];

    // Take the average color and map that to the downscaled image
    for by in 0..new_height {
        for bx in 0..new_width {
            let x = bx * pixel_size;
            let y = by * pixel_size;

            let (r, g, b, a) = get_average_rgba(&image, x, y, pixel_size)?;

            let out_i = ((by * new_width + bx) * 4) as usize;
            downscaled[out_i] = r;
            downscaled[out_i + 1] = g;
            downscaled[out_i + 2] = b;
            downscaled[out_i + 3] = a;
        }
    }
    
    Ok(PixelifyImage::new(downscaled, new_width, new_height))
}
