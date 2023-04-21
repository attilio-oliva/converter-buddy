use image::ImageEncoder;
use std::io::Cursor;

use crate::{
    config::{BmpConfig, Config, GifConfig, JpegConfig, PngConfig, TiffConfig},
    converter::{ConversionError, QueueConverter},
    format::Format,
};

use super::wrapper;

#[macro_export]
macro_rules! impl_common_image_conversions {

    ($converter:ident, $($format:ident),*) => {
        use paste::paste;
        use $crate::config::*;

        $(paste!{
            impl ConversionStrategy<[<$format Config>]> for $converter{
                fn process(
                &self,
                input: &Vec<u8>,
                output: &mut Vec<u8>,
                config: [<$format Config>]
                ) -> Result<(), ConversionError> {
                    common_strategies::[<from_raster_to_ $format:lower>](input, output, config)
                }
            }
        })*
    }
}

pub fn from_raster_to_bmp(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    config: BmpConfig,
) -> Result<(), ConversionError> {
    let image = wrapper::image_crate_conversion(input, output, &config.base, Format::Bmp.into())?;
    let width = image.width();
    let height = image.height();
    let color = image.color();

    let mut encoder = image::codecs::bmp::BmpEncoder::new(output);
    encoder
        .encode(image.into_bytes().as_slice(), width, height, color)
        .map_err(|_| ConversionError::Unexpected)
}

pub fn from_raster_to_jpeg(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    config: JpegConfig,
) -> Result<(), ConversionError> {
    let image = wrapper::image_crate_conversion(input, output, &config.base, Format::Jpeg.into())?;
    let width = image.width();
    let height = image.height();
    let color = image.color();

    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(output, config.quality);
    encoder
        .encode(image.into_bytes().as_slice(), width, height, color)
        .map_err(|_| ConversionError::Unexpected)
}

pub fn from_raster_to_tiff(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    config: TiffConfig,
) -> Result<(), ConversionError> {
    let image = wrapper::image_crate_conversion(input, output, &config.base, Format::Tiff.into())?;
    let width = image.width();
    let height = image.height();
    let color = image.color();

    let encoder = image::codecs::tiff::TiffEncoder::new(Cursor::new(output));
    encoder
        .encode(image.into_bytes().as_slice(), width, height, color)
        .map_err(|_| ConversionError::Unexpected)
}

pub fn from_raster_to_gif(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    config: GifConfig,
) -> Result<(), ConversionError> {
    let image = wrapper::image_crate_conversion(input, output, &config.base, Format::Gif.into())?;
    let width = image.width();
    let height = image.height();
    let color = image.color();
    let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(output, config.speed);
    encoder
        .encode(image.into_bytes().as_slice(), width, height, color)
        .map_err(|_| ConversionError::Unexpected)
}

pub fn from_raster_to_png(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    config: PngConfig,
) -> Result<(), ConversionError> {
    let image = wrapper::image_crate_conversion(input, output, &config.base, Format::Png.into())?;
    let width = image.width();
    let height = image.height();
    let color = image.color();
    let encoder =
        image::codecs::png::PngEncoder::new_with_quality(output, config.compression, config.filter);
    encoder
        .write_image(image.into_bytes().as_slice(), width, height, color)
        .map_err(|_| ConversionError::Unexpected)
}

pub fn from_image_to_pdf(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    source_format: Format,
    config: Config,
) -> Result<(), ConversionError> {
    let mut converter = QueueConverter::new(source_format);
    converter.push(Format::Png);
    converter.push(Format::Pdf);

    converter.process(input, output, config)
}
