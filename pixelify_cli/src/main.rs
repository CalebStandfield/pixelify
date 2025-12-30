//! Main file and execution point for Pixelify.
//! Pixelify is a Rust + WebAssembly or CLI tool that converts normal images into pixel-art sprites.
//! As well as having some more basic editing features like a crop or grayscale functionality, for example.

use clap::{Parser, Subcommand};
use pixelify_core::crop::crop_png;
use pixelify_core::grayscale::grayscale_png;
use pixelify_core::pixelify::*;
mod cli_utils;
use cli_utils::*;

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::PixelifyDownscaleByPixelSize {
            input,
            output,
            pixel_size,
        } => {
            run_op(&input, &output, |b| pixelify_downscale_by_pixel_size(b, pixel_size))
        }
        Command::PixelifyFalseDownscaleByPixelSize {
            input,
            output,
            pixel_size,
        } => {
            run_op(&input, &output, |b| pixelify_false_downscale_by_pixel_size(b, pixel_size))
        }
        Command::PixelifyDownscaleByImageSize {
            input,
            output,
            width,
            height,
        } => {
            run_op(&input, &output, |b| pixelify_by_image_size(b, width, height))
        }
        Command::ClearOutputs => {
            if let Err(e) = clear_outputs() {
                eprintln!("{e}");
                std::process::exit(1);
            }
        }

        Command::Grayscale { input, output } => {
            run_op(&input, &output, |b| grayscale_png(b));
        }

        Command::Crop {
            input,
            output,
            x,
            y,
            w,
            h,
        } => {
            run_op(&input, &output, |b| crop_png(b, x, y, w, h));
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
    PixelifyDownscaleByPixelSize {
        input: String,
        output: String,
        #[arg(long)]
        pixel_size: u32,
    },
    PixelifyFalseDownscaleByPixelSize {
        input: String,
        output: String,
        #[arg(long)]
        pixel_size: u32,
    },
    PixelifyDownscaleByImageSize {
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
