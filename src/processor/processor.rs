use image::{DynamicImage, imageops};

pub fn overlay_images(logo_image: DynamicImage, main_image: DynamicImage) -> DynamicImage {
    let mut main_rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = main_image.to_rgba8();

    let (main_width, main_height) = main_rgba.dimensions();
    let logo_width: u32 = 830;
    let logo_height: u32 = 142;
    let margin: u32 = 10;
    let logo_position_x: i64 = (main_width - logo_width - margin) as i64;
    let logo_position_y: i64 = (main_height - logo_height - margin) as i64;

    let resized_logo: DynamicImage = logo_image.resize_exact(
        logo_width,
        logo_height,
        imageops::FilterType::Lanczos3,
    ); 
    let logo_rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = resized_logo.to_rgba8();

    imageops::overlay(&mut main_rgba, &logo_rgba, logo_position_x, logo_position_y);

    DynamicImage::ImageRgba8(main_rgba)
}