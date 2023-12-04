mod processor;

use std::env;
use image::io::Reader as ImageReader;
use crate::processor::processor::overlay_images;


const DEFAULT_OUTPUT: &str = "output.jpg"; 

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <logo> <image> [debug] [output]", args[0]);
        std::process::exit(1);
    }

    let logo_path: &String = &args[1];
    let main_path: &String = &args[2];
    let debug: bool = args.contains(&String::from("debug"));

    let output_path: &str = args
        .iter()
        .position(|arg: &String| arg == "debug")
        .map_or(DEFAULT_OUTPUT, |index: usize| args.get(index + 1)
        .map_or(DEFAULT_OUTPUT, String::as_str));

    if debug {
        println!("Debug mode enabled.");
    }

    let logo_image: image::DynamicImage = open_and_decode_image(logo_path, "logo", debug);
    let main_image: image::DynamicImage = open_and_decode_image(main_path, "main", debug);
    let result_image: image::DynamicImage = overlay_images(logo_image, main_image);

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

fn open_and_decode_image(path: &str, image_type: &str, debug: bool) -> image::DynamicImage {
    ImageReader::open(path)
        .unwrap_or_else(|e: std::io::Error| {
            if debug {
                eprintln!("Failed to open the {} image: {}", image_type, e);
            }
            std::process::exit(1);
        })
        .decode()
        .unwrap_or_else(|e: image::ImageError| {
            if debug {
                eprintln!("Failed to decode the {} image: {}", image_type, e);
            }
            std::process::exit(1);
        })
}
