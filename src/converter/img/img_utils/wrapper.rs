use std::io::Cursor;

use image::{io::Reader as ImageReader, DynamicImage, ImageFormat};

use crate::converter::ConversionError;

/// Use image crate for the conversion
pub fn image_crate_conversion(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    target_format: ImageFormat,
) -> Result<(), ConversionError> {
    image_crate_conversion_with_processing(input, output, target_format, &|img| Ok(img.to_owned()))
}

/// Use image crate for the conversion and apply a processing function
/// to the image before writing it to the output
pub fn image_crate_conversion_with_processing(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    target_format: ImageFormat,
    processing: &dyn Fn(&DynamicImage) -> Result<DynamicImage, ConversionError>,
) -> Result<(), ConversionError> {
    let reader = ImageReader::new(Cursor::new(input))
        .with_guessed_format()
        .expect("Cursor io never fails");

    let mut image = reader
        .decode()
        .map_err(|_| ConversionError::UnknownSourceFormat)?;

    image = processing(&mut image)?;

    image
        .write_to(&mut Cursor::new(output), target_format)
        .map_err(|_| ConversionError::Unexpected)
}
