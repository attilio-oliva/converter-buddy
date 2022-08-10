use crate::{converter_info::ConverterInfo, format::Format};

pub struct PngConverter;

impl ConverterInfo for PngConverter {
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::Png,
            Format::Jpeg,
            Format::Tiff,
            Format::Bmp,
            Format::Gif,
            Format::Pdf,
        ]
    }
}
