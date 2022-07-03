use crate::{
    format::Format, converter_info::ConverterInfo,
};

pub struct BmpConverter;

impl ConverterInfo for BmpConverter{
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::Bmp,
            Format::Jpeg,
            Format::Png,
            Format::Tiff,
            Format::Gif,
            Format::Pdf
        ]
    }
}