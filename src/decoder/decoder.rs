use image::io::Reader as ImageReader;


pub fn decode(path: &str, image_type: &str, debug: bool) -> image::DynamicImage {
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