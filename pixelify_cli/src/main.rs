use std::env::args;
use std::fs;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 5 {
        eprintln!("usage: pixelify_cli <input> <output.png> <width> <height>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let width: u8 = args[3].parse().expect("width must be a number");
    let height: u8 = args[4].parse().expect("height must be a number");

    let input_bytes = fs::read(input_path).expect("failed to read input file");

    let out = pixelify_core::pixelify_png(input_bytes, width, height);

    fs::write(output_path, out.pixels).expect("failed to write output file");
    println!("wrote {}", output_path);
}
