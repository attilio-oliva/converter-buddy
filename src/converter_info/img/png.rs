use crate::{
    format::Format, converter_info::ConverterInfo,
};

pub struct PngConverter;

impl ConverterInfo for PngConverter{
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::Png,
            Format::Jpeg,
            Format::Tiff,
            Format::Bmp,
            Format::Gif,
            Format::Pdf
        ]
    }
}