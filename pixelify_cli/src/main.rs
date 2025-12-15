use clap::{Parser, Subcommand};
use std::fs;
use pixelify_core::pixelify_image;
use pixelify_core::grayscale::grayscale_png;
use pixelify_core::crop::crop_png;


fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Pixelify { input, output, width, height } => {
            let bytes = fs::read(&input).expect("failed to read input");
            let out_png = pixelify_image::PixelifyImage::new(
                bytes,
                width,
                height,
            );
            fs::write(&output, out_png.as_bytes()).expect("failed to write output");
        }

        Command::Delete => {
            clear_outputs();
        }

        // Command::Grayscale { input, output } => {
        //     let bytes = fs::read(&input).expect("failed to read input");
        //     let out_png = grayscale_png(bytes);
        //     fs::write(&output, out_png).expect("failed to write output");
        // }
        //
        // Command::Crop { input, output, x, y, w, h } => {
        //     let bytes = fs::read(&input).expect("failed to read input");
        //     let out_png = crop_png(&bytes, x, y, w, h);
        //     fs::write(&output, out_png).expect("failed to write output");
        _ => {}
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
    Delete,

}

fn clear_outputs() {
    let path = "outputs";

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        fs::remove_file(path).unwrap();
    }
}
