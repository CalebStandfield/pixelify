//! Utility file for pixelify_cli

use std::path::Path;
use std::{fs, io};
use image::{ImageFormat, RgbaImage};
use pixelify_core::pixelify_errors::ImageProcessingError;
use pixelify_core::PixelifyImage;
use pixelify_core::into_png::into_png;

/// Clears the `outputs/` directory's contents.
///
/// This function expects the `outputs/` directory to contain only files produced by this program.
/// Thus, no subdirectories.
///
/// # Errors
///
/// Returns an error if:
/// - `outputs/` is not a directory or missing, returns `ErrorKind::NotFound`,
/// - a subdirectory is found inside `outputs/`, returns `ErrorKind::InvalidData`.
pub fn clear_outputs() -> io::Result<()> {
    let dir = Path::new("outputs");

    // Outputs should exist
    if !dir.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "outputs/ missing"));
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Outputs should consist of only image files outputted by this program
        // If any directories exist, that is not the intended behavior
        if entry.file_type()?.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("unexpected directory inside outputs/: {:?}", path),
            ));
        }

        fs::remove_file(&path)
            .map_err(|e| io::Error::new(e.kind(), format!("failed to delete {:?}: {}", path, e)))?;
    }
    Ok(())
}

/// Runs an image-processing operation on an input file and writes the result to an output file.
///
/// This helper reads the entire input file into memory, applies the provided operation
/// to the file bytes, and writes the resulting PNG bytes to the output path.
///
/// The operation is provided as a function or closure that takes the input bytes
/// and returns either encoded PNG bytes or an error.
///
/// # Errors
///
/// - Exits the process with a non-zero status if the input file cannot be read
/// - Exits the process if the operation returns an error
/// - Panics if the output file cannot be written
pub fn run_op<F, E>(input: &str, output: &str, op: F)
where
    F: FnOnce(&[u8]) -> Result<PixelifyImage, E>,
    E: std::fmt::Display,
{
    let mut bytes = fs::read(input).expect("failed to read input");

    // Check if the image type is a png
    // If not, then convert and then run operation
    let format = image::guess_format(&bytes).unwrap_or_else(|e| {
        eprintln!("Format guess failed {}", e);
        // if the guess fails, default to png
        ImageFormat::Png
    });

    // If the bytes are not of type png, convert them into a png format
    if format != ImageFormat::Png {
        bytes = into_png(bytes).expect("").into_bytes();
    }

    let image = match op(&bytes) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("operation failed: {e}");
            std::process::exit(1);
        }
    };

    let png_bytes = match write_to_png_format(image) {
        Ok(png_bytes) => png_bytes,
        Err(e) => {
            eprintln!("operation failed: {e}");
            std::process::exit(1);
        }
    };

    fs::write(output, png_bytes.as_bytes()).expect("failed to write output");
}

/// Encodes a `PixelifyImage` (raw RGBA pixels) into PNG file bytes.
///
/// This function treats `pixelify_image.as_bytes()` as a **raw RGBA buffer**
/// (4 bytes per pixel, row-major order) and encodes it into the **PNG file
/// format**. The returned `PixelifyImage` contains **PNG-encoded bytes**, not
/// raw pixel bytes.
///
/// The dimensions are preserved (`width`/`height` are copied over).
///
/// # Errors
///
/// Returns `Err(ImageProcessingError)` if:
/// - the input buffer length does not match `width * height * 4` (invalid RGBA buffer),
/// - PNG encoding fails.
///
/// # Notes
///
/// - This does **not** modify the original image in-place; it creates a new
///   byte buffer containing a PNG file (headers + compressed image data).
/// - After calling this, `PixelifyImage::as_bytes()` is **not** raw pixels
///   anymore, so only use this at the “output boundary” (save/send/download)
///   unless your type explicitly tracks the encoding.
fn write_to_png_format(
    pixelify_image: PixelifyImage
) -> Result<PixelifyImage, ImageProcessingError> {
    let rgba = RgbaImage::from_raw(pixelify_image.get_width(), pixelify_image.get_height(), pixelify_image.as_bytes().to_vec())
        .ok_or_else(|| ImageProcessingError::failed("pixelify", "Bad buffer length"))?;

    let mut cursor = io::Cursor::new(Vec::new());

    rgba.write_to(&mut cursor, ImageFormat::Png)
        .map_err(|_| ImageProcessingError::failed("crop", "Failed to encode PNG"))?;


    Ok(PixelifyImage::new(cursor.into_inner(), pixelify_image.get_width(), pixelify_image.get_height()))
}
