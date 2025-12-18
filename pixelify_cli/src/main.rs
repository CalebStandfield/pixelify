//! Main file and execution point for Pixelify.
//! Pixelify is a Rust + WebAssembly or CLI tool that converts normal images into pixel-art sprites.
//! As well as having some more basic editing features like a crop or grayscale functionality, for example.

use clap::{Parser, Subcommand};
use pixelify_core::crop::crop_png;
use pixelify_core::grayscale::grayscale_png;
use pixelify_core::pixelify_image;
use std::path::Path;
use std::{fs, io};

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Pixelify {
            input,
            output,
            width,
            height,
        } => {
            // This does not pixelify an image as of now.
            // Just a simple copy function until pixelify is implemented.
            let bytes = fs::read(&input).expect("failed to read input");
            let out_png = pixelify_image::PixelifyImage::new(bytes, width, height);
            fs::write(&output, out_png.as_bytes()).expect("failed to write output");
        }

        Command::ClearOutputs => {
            if let Err(e) = clear_outputs() {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }

        Command::Grayscale { input, output } => {
            let bytes = fs::read(&input).expect("failed to read input");
            let out_png = grayscale_png(&bytes).expect("failed to encode PNG");
            fs::write(&output, out_png).expect("failed to write output");
        }

        Command::Crop {
            input,
            output,
            x,
            y,
            w,
            h,
        } => {
            let bytes = fs::read(&input).expect("failed to read input");
            let out_png = crop_png(&bytes, x, y, w, h).expect("failed to crop");
            fs::write(&output, out_png).expect("failed to write output");
        }
    }
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    Pixelify {
        input: String,
        output: String,
        #[arg(long)]
        width: u32,
        #[arg(long)]
        height: u32,
    },
    Grayscale {
        input: String,
        output: String,
    },
    Crop {
        input: String,
        output: String,
        #[arg(long)]
        x: u32,
        #[arg(long)]
        y: u32,
        #[arg(long)]
        w: u32,
        #[arg(long)]
        h: u32,
    },
    #[command(
        visible_alias = "clear_outputs",
        visible_alias = "clearoutputs",
        alias = "clear"
    )]
    ClearOutputs,
}

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
fn clear_outputs() -> io::Result<()> {
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
