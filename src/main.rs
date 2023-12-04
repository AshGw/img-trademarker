mod processor;
mod decoder;
use std::env;
use crate::processor::processor::overlay_images;
use crate::decoder::decoder::decode;


const DEFAULT_OUTPUT: &str = "output.jpg"; 

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <logo> <image> [debug] [output]", args[0]);
        std::process::exit(1);
    }

    let debug: bool = args.contains(&String::from("debug"));
    if debug {
        println!("Debug mode enabled.");
    }
    let output_path: &str = args
        .iter()
        .position(|arg: &String| arg == "debug")
        .map_or(DEFAULT_OUTPUT, |index: usize| args.get(index + 1)
        .map_or(DEFAULT_OUTPUT, String::as_str));

    let result_image: image::DynamicImage = overlay_images(
        decode(&args[2], "main", debug),
        decode(&args[2], "main", debug)
    );

    result_image.save(output_path).unwrap_or_else(|e| {
        if debug {
            eprintln!("Failed to save result image: {}", e);
        }
        std::process::exit(1);
    });

    if debug {
        println!("Result image saved to {}", output_path);
    }
}


