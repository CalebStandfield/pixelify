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

pub fn pixelify_by_pixel_size(bytes: &[u8], pixel_size: u32) {
    
}

pub fn pixelify_by_image_size(bytes: &[u8], width: u32, height: u32) {
    
}
