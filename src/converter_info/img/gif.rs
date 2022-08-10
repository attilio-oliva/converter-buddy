use crate::{converter_info::ConverterInfo, format::Format};

pub struct GifConverter;

impl ConverterInfo for GifConverter {
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::Gif,
            Format::Jpeg,
            Format::Png,
            Format::Tiff,
            Format::Bmp,
            Format::Pdf,
        ]
    }
}
