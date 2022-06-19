use image::{DynamicImage, GenericImageView, Pixel, Rgba};

// Given an color, get the contrast ratio between the color and white
//
// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#contrast-ratiodef
//
// The contrast ratio is calculated as follows:
//
// C = (L1 + 0.05) / (L2 + 0.05)
//
// Where L1 is the luminance of the lighter color, and L2 is the luminance of the darker color.
//
pub fn contrast_ratio(first_color: &image::Rgb<u8>, second_color: &image::Rgb<u8>) -> f64 {
    let first_luminance = luminance(first_color);
    let second_luminance = luminance(second_color);

    if first_luminance > second_luminance {
        (first_luminance + 0.05) / (second_luminance + 0.05)
    } else {
        (second_luminance + 0.05) / (first_luminance + 0.05)
    }
}

// Given a color, get the luminance
//
// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
//
// The relative luminance of a color is calculated as follows:
//
// L = 0.2126 * R + 0.7152 * G + 0.0722 * B
//
// Where R, G and B are the red, green and blue components of the color, respectively.
//
pub fn luminance(color: &image::Rgb<u8>) -> f64 {
    let red = color[0];
    let green = color[1];
    let blue = color[2];
    luminance_from_rgb(red, green, blue)
}

pub fn luminance_from_rgb(red: u8, green: u8, blue: u8) -> f64 {
    let red_coef = 0.2126;
    let green_coef = 0.7152;
    let blue_coef = 0.0722;
    let red = red as f64;
    let green = green as f64;
    let blue = blue as f64;
    red_coef * red + green_coef * green + blue_coef * blue
}

// Given a color, get the brightness
//
// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
//
// The brightness of a color is calculated as follows:
//
// B = 0.2126 * R + 0.7152 * G + 0.0722 * B
//
// Where R, G and B are the red, green and blue components of the color, respectively.
//
pub fn brightness(color: image::Rgb<u8>) -> f64 {
    let red = color[0];
    let green = color[1];
    let blue = color[2];
    brightness_from_rgb(red, green, blue)
}
pub fn brightness_from_rgb(red: u8, green: u8, blue: u8) -> f64 {
    let red_coef = 0.299;
    let green_coef = 0.587;
    let blue_coef = 0.114;
    let red = red as f64;
    let green = green as f64;
    let blue = blue as f64;
    red_coef * red + green_coef * green + blue_coef * blue
}

// Given a color, get the average color
//
// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#relativeluminancedef
//
// The average color of a color is calculated as follows:
//
// A = (R + G + B) / 3
//
// Where R, G and B are the red, green and blue components of the color, respectively.
//
pub fn average_color(color: image::Rgb<u8>) -> f64 {
    let red = color[0];
    let green = color[1];
    let blue = color[2];
    average_color_from_rgb(red, green, blue)
}
pub fn average_color_from_rgb(red: u8, green: u8, blue: u8) -> f64 {
    let red = red as f64;
    let green = green as f64;
    let blue = blue as f64;
    (red + green + blue) / 3.0
}

pub fn average_image_color(image: &image::DynamicImage) -> image::Rgb<u8> {
    //let pixel_count = (image.width() * image.height()) as usize;
    let mut pixel_count = 0;
    let mut red_pixels = 0;
    let mut green_pixels = 0;
    let mut blue_pixels = 0;

    //for each non-transparent pixel
    for pixel in image.pixels().filter(|p| p.2.channels()[3] > 0) {
        let ch = pixel.2.channels();
        red_pixels += ch[0] as usize;
        green_pixels += ch[1] as usize;
        blue_pixels += ch[2] as usize;
        pixel_count += 1;
    }
    let red_avg = red_pixels / pixel_count;
    let green_avg = green_pixels / pixel_count;
    let blue_avg = blue_pixels / pixel_count;
    image::Rgb([red_avg as u8, green_avg as u8, blue_avg as u8])
}
// Map a color to a new color in an image
pub fn map_image_color(image: &mut DynamicImage, source: &Rgba<u8>, target: &Rgba<u8>) {
    let mut image = DynamicImage::to_rgba8(image);
    for pixel in image.pixels_mut() {
        if pixel == source {
            pixel[0] = target[0];
            pixel[1] = target[1];
            pixel[2] = target[2];
        }
    }
}

pub fn map_image_transparent_color(image: &DynamicImage, target: &Rgba<u8>) -> DynamicImage {
    let mut image = DynamicImage::to_rgba8(image);
    for pixel in image.pixels_mut() {
        if pixel[3] == 0 {
            pixel[0] = target[0];
            pixel[1] = target[1];
            pixel[2] = target[2];
            pixel[3] = target[3];
        }
    }
    image::DynamicImage::ImageRgba8(image)
}
