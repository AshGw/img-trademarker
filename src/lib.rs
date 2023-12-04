//! # img_watermarker
//! Minimal lib for watermarking images

mod processor;
mod decoder;
use std::env;
use crate::processor::processor::overlay_images;
use crate::decoder::decoder::decode;


const DEFAULT_OUTPUT: &str = "output.jpg"; 
/// The `watermark` function overlays a logo image onto another image and saves the resulting image.
///
/// # Arguments
///
/// * `logo_path` - The path to the logo image file.
/// * `image_path` - The path to the main image file.
/// * `debug` - Optional. Enables debug mode if set.
/// * `output_path` - Optional. The path to save the resulting image. If not provided, defaults to "output.jpg".
///
/// # Example
/// 
/// ```rust
/// use img_watermarker::watermark;
/// 
/// fn main() {
///     watermark();
/// }
/// ```
///
/// # Command Line Example
///
/// ```bash
/// cargo run <logo> <image> <debug?> <output?>
/// ```
///
/// ## Command Line Usage
///
/// ```bash
/// cargo run logo.png image.jpg debug output.jpg
/// ```
///
/// - The `debug` mode is optional and can be set to enable debugging output.
/// - The `output` path is optional. If not provided, the result will be saved to "output.jpg".

pub fn watermark() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: <logo> <image> [debug] [output]");
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
        decode(&args[1], "main", debug),
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

