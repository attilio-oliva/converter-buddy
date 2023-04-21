mod bmp;
mod gif;
mod img_utils;
mod jpeg;
mod png;
mod svg;
mod tiff;
mod webp;

pub use bmp::BmpConverter;
pub use gif::GifConverter;
pub use img_utils::*;
pub use jpeg::JpegConverter;
pub use png::PngConverter;
pub use svg::SvgConverter;
pub use tiff::TiffConverter;
pub use webp::WebPConverter;

use crate::{config::Config, format::Format};

use super::{ConversionError, ConversionStrategy};

#[non_exhaustive]
pub enum Converter {
    Jpeg(JpegConverter),
    Png(PngConverter),
    Gif(GifConverter),
    Tiff(TiffConverter),
    Bmp(BmpConverter),
    WebP(WebPConverter),
    Svg(SvgConverter),
}

impl Converter {
    pub fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: Config,
    ) -> Result<(), ConversionError> {
        match self {
            Converter::Jpeg(c) => c.process(input, output, config),
            Converter::Png(c) => c.process(input, output, config),
            Converter::Gif(c) => c.process(input, output, config),
            Converter::Tiff(c) => c.process(input, output, config),
            Converter::Bmp(c) => c.process(input, output, config),
            Converter::WebP(c) => c.process(input, output, config),
            Converter::Svg(c) => c.process(input, output, config),
        }
    }
}

impl TryFrom<Format> for Converter {
    type Error = ConversionError;
    fn try_from(value: Format) -> Result<Self, Self::Error> {
        match value {
            Format::Png => Ok(Converter::Png(PngConverter)),
            Format::Jpeg => Ok(Converter::Jpeg(JpegConverter)),
            Format::Gif => Ok(Converter::Gif(GifConverter)),
            Format::WebP => Ok(Converter::WebP(WebPConverter)),
            Format::Tiff => Ok(Converter::Tiff(TiffConverter)),
            Format::Bmp => Ok(Converter::Bmp(BmpConverter)),
            Format::Svg => Ok(Converter::Svg(SvgConverter)),
            _ => Err(ConversionError::UnsupportedOperation),
        }
    }
}
