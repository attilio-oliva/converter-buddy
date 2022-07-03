use crate::{
    format::Format, converter_info::ConverterInfo,
};

pub struct WebPConverter;

impl ConverterInfo for WebPConverter{
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::WebP,
            Format::Jpeg,
            Format::Png,
            Format::Tiff,
            Format::Bmp,
            Format::Gif,
            Format::Pdf
        ]
    }
}