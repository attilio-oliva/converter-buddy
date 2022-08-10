use crate::{converter_info::ConverterInfo, format::Format};

pub struct WebPConverter;

impl ConverterInfo for WebPConverter {
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::WebP,
            Format::Jpeg,
            Format::Png,
            Format::Tiff,
            Format::Bmp,
            Format::Gif,
            Format::Pdf,
        ]
    }
}
