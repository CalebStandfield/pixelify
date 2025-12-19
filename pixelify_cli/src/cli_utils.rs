//! Utility file for pixelify_cli

use std::path::Path;
use std::{fs, io};

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
    F: FnOnce(&[u8]) -> Result<Vec<u8>, E>,
    E: std::fmt::Display,
{
    let bytes = fs::read(input).expect("failed to read input");
    let out_png = match op(&bytes) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("operation failed: {e}");
            std::process::exit(1);
        }
    };
    fs::write(output, out_png).expect("failed to write output");
}
