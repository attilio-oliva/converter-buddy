use crate::{
    format::Format, converter_info::ConverterInfo,
};

pub struct JpegConverter;

impl ConverterInfo for JpegConverter{
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::Jpeg,
            Format::Png,
            Format::Tiff,
            Format::Bmp,
            Format::Gif,
            Format::Pdf
        ]
    }
}