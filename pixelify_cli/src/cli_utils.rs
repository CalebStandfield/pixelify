//! Utility file for pixelify_cli

use std::{fs, io};
use std::path::Path;

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