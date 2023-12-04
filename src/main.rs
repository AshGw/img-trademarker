mod processor;

use std::env;
use image::io::Reader as ImageReader;
use crate::processor::processor::overlay_images;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <logo> <image>", args[0]);
        std::process::exit(1);
    }

    let logo_path: &String = &args[1];
    let main_path: &String = &args[2];

    let logo_image: image::DynamicImage = ImageReader::open(logo_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open the logo image: {}", e);
            std::process::exit(1);
        })
        .decode()
        .unwrap_or_else(|e| {
            eprintln!("Failed to decode the logo image: {}", e);
            std::process::exit(1);
        });

    let main_image: image::DynamicImage = ImageReader::open(main_path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to open the main image: {}", e);
            std::process::exit(1);
        })
        .decode()
        .unwrap_or_else(|e| {
            eprintln!("Failed to decode the main image: {}", e);
            std::process::exit(1);
        });

    let result_image: image::DynamicImage = overlay_images(logo_image, main_image);

    let output_path: &str = "./result_of_both.jpg";
    result_image.save(output_path).unwrap_or_else(|e| {
        eprintln!("Failed to save result image: {}", e);
        std::process::exit(1);
    });
}
