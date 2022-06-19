use super::img_utils::*;
use image::ImageFormat;

use crate::{
    converter::{ConversionError, Converter},
    format::Format,
};

pub struct WebPConverter;

impl Converter for WebPConverter {
    fn supported_formats(&self) -> Vec<Format> {
        vec![
            Format::WebP,
            Format::Jpeg,
            Format::Png,
            Format::Tiff,
            Format::Bmp,
            Format::Gif,
        ]
    }

    fn to_webp(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        output.clone_from(input);
        Ok(())
    }
    fn to_bmp(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion(input, output, ImageFormat::Bmp)
    }
    fn to_tiff(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion(input, output, ImageFormat::Tiff)
    }
    fn to_png(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion(input, output, ImageFormat::Png)
    }
    fn to_jpeg(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion(input, output, ImageFormat::Jpeg)
    }
    fn to_gif(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion(input, output, ImageFormat::Gif)
    }
}

#[cfg(test)]
mod tests {

    use image::codecs::bmp::BmpDecoder;
    use image::codecs::gif::GifDecoder;
    use image::codecs::jpeg::JpegDecoder;
    use image::codecs::png::PngDecoder;
    use image::codecs::tiff::TiffDecoder;

    use crate::converter::{test_utils, Converter, WebPConverter};
    use crate::format::Format;

    // Implementation of the used Converter trait
    // Converter are supposed to be stateless, so we can use this single instance
    static CONVERTER: WebPConverter = WebPConverter;
    // Test asset file extension
    static SOURCE_EXT: &str = "webp";

    #[test]
    fn test_supported_formats() {
        let formats = CONVERTER.supported_formats();
        assert_eq!(formats.len(), 6);
        assert!(formats.contains(&Format::WebP));
        assert!(formats.contains(&Format::Tiff));
        assert!(formats.contains(&Format::Png));
        assert!(formats.contains(&Format::Jpeg));
        assert!(formats.contains(&Format::Bmp));
        assert!(formats.contains(&Format::Gif));
    }

    #[test]
    fn test_to_webp() {
        let target_ext = "webp";

        test_utils::test_conversion_to(
            Format::WebP,
            &CONVERTER,
            SOURCE_EXT,
            target_ext,
            |source, target| {
                // Check if file was created has same dimensions as the source file
                let input_size = source.metadata().unwrap().len();
                let output_size = target.metadata().unwrap().len();

                input_size == output_size
            },
        );
    }

    #[test]
    fn test_to_bmp() {
        let target_ext = "bmp";

        test_utils::test_conversion_to(
            Format::Bmp,
            &CONVERTER,
            SOURCE_EXT,
            target_ext,
            |_, target| {
                let decoding = BmpDecoder::new(target);
                decoding.is_ok()
            },
        );
    }

    #[test]
    fn test_to_tiff() {
        let target_ext = "tiff";

        test_utils::test_conversion_to(
            Format::Tiff,
            &CONVERTER,
            SOURCE_EXT,
            target_ext,
            |_, target| {
                let decoding = TiffDecoder::new(target);
                decoding.is_ok()
            },
        );
    }

    #[test]
    fn test_to_png() {
        let target_ext = "png";

        test_utils::test_conversion_to(
            Format::Png,
            &CONVERTER,
            SOURCE_EXT,
            target_ext,
            |_, target| {
                let decoding = PngDecoder::new(target);
                decoding.is_ok()
            },
        );
    }

    #[test]
    fn test_to_jpeg() {
        let target_ext = "jpeg";

        test_utils::test_conversion_to(
            Format::Jpeg,
            &CONVERTER,
            SOURCE_EXT,
            target_ext,
            |_, target| {
                let decoding = JpegDecoder::new(target);
                decoding.is_ok()
            },
        );
    }

    #[test]
    fn test_to_gif() {
        let target_ext = "gif";

        test_utils::test_conversion_to(
            Format::Gif,
            &CONVERTER,
            SOURCE_EXT,
            target_ext,
            |_, target| {
                let decoding = GifDecoder::new(target);
                decoding.is_ok()
            },
        );
    }
}
