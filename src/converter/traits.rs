use super::ConversionError;
use crate::{format::Format, converter_info::ConverterInfo};

pub trait Converter: ConverterImpl + ConverterInfo{}
pub trait ConverterImpl {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        target_format: Format,
    ) -> Result<(), ConversionError> {
        match target_format {
            Format::Tiff => self.to_tiff(input, output),
            Format::Png => self.to_png(input, output),
            Format::Jpeg => self.to_jpeg(input, output),
            Format::Bmp => self.to_bmp(input, output),
            Format::Gif => self.to_gif(input, output),
            Format::WebP => self.to_webp(input, output),
            Format::Svg => self.to_svg(input, output),
            Format::Pdf => self.to_pdf(input, output),
            _ => Err(ConversionError::UnsupportedOperation),
        }
    }

    fn to_png(&self, _input: &Vec<u8>, _output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
    fn to_jpeg(&self, _input: &Vec<u8>, _output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
    fn to_tiff(&self, _input: &Vec<u8>, _output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
    fn to_bmp(&self, _input: &Vec<u8>, _output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
    fn to_gif(&self, _input: &Vec<u8>, _output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
    fn to_webp(&self, _input: &Vec<u8>, _output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
    fn to_svg(&self, _input: &Vec<u8>, _output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
    fn to_pdf(&self, _input: &Vec<u8>, _output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
}