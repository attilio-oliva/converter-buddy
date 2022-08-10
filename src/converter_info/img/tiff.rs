use crate::{converter_info::ConverterInfo, format::Format};
pub struct TiffConverter;

impl ConverterInfo for TiffConverter {
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::Tiff,
            Format::Png,
            Format::Jpeg,
            Format::Bmp,
            Format::Gif,
            Format::Pdf,
        ]
    }
}
