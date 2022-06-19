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

use crate::format::Format;

pub static SUPPORTED_FORMATS: [Format; 7] = [
    Format::Png,
    Format::Jpeg,
    Format::Tiff,
    Format::Bmp,
    Format::Gif,
    Format::Svg,
    Format::WebP,
];
