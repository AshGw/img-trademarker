use image::{DynamicImage, imageops};

pub fn overlay_images(logo_image: DynamicImage, main_image: DynamicImage) -> DynamicImage {
    let mut main_rgba = main_image.to_rgba8();

    let (main_width, main_height) = main_rgba.dimensions();
    let logo_width = 830;
    let logo_height = 142;
    let margin = 10;
    let logo_position_x = (main_width - logo_width - margin) as i64;
    let logo_position_y = (main_height - logo_height - margin) as i64;

    let resized_logo = logo_image.resize_exact(
        logo_width,
        logo_height,
        imageops::FilterType::Lanczos3,
    ); 
    let logo_rgba = resized_logo.to_rgba8();

    imageops::overlay(&mut main_rgba, &logo_rgba, logo_position_x, logo_position_y);

    DynamicImage::ImageRgba8(main_rgba)
}