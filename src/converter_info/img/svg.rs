use crate::{converter_info::ConverterInfo, format::Format};


pub struct SvgConverter;

impl ConverterInfo for SvgConverter{
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::Svg,
            Format::Jpeg,
            Format::Png,
            Format::Tiff,
            Format::Bmp,
            Format::Gif,
            Format::Pdf
        ]
    }
}