use image::{DynamicImage, imageops};

pub fn overlay_images(logo_image: DynamicImage, main_image: DynamicImage) -> DynamicImage {
    let mut main_rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = main_image.to_rgba8();

    // logo dimensions are set to take 10% of the main image dimensions
    let (main_width, main_height) = main_rgba.dimensions();
    let logo_width: u32 = (main_width as f32 * 0.1) as u32; 
    let logo_height: u32 = (main_height as f32 * 0.1) as u32;

    let margin: u32 = 10; // pixels bottom right offset


    let logo_rgba: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = logo_image.resize_exact(
          logo_width,
         logo_height,
          imageops::FilterType::Lanczos3
        ).to_rgba8();

    let logo_pos_x: i64 = (main_width - logo_width - margin) as i64;
    let logo_pos_y: i64 = (main_height - logo_height - margin) as i64;
    imageops::overlay(&mut main_rgba, &logo_rgba, logo_pos_x, logo_pos_y);

    DynamicImage::ImageRgba8(main_rgba)
}