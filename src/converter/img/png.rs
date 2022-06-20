use super::img_utils::*;
use image::{ImageFormat, Pixel};

use crate::{
    converter::{ConversionError, Converter},
    format::Format,
};

pub struct PngConverter;

impl Converter for PngConverter {
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

    fn to_png(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        output.clone_from(input);
        Ok(())
    }
    fn to_jpeg(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion_with_processing(
            input,
            output,
            ImageFormat::Jpeg,
            &|image| {
                // Decide a color to replace transparent pixels with
                // TODO: Make this configurable
                // TODO: Make this support higher color depth than 8-bit (Rgb<u8>)
                // TODO: Add anti-aliasing
                let avg_color = processing::average_image_color(image);
                let white_color = image::Rgb([255, 255, 255]);
                let black_color = image::Rgb([0, 0, 0]);
                //info!("Average color: {:?}", avg_color);
                let background_color =
                    if processing::contrast_ratio(&avg_color, &white_color) >= 1.5 {
                        white_color
                    } else {
                        black_color
                    };
                Ok(processing::map_image_transparent_color(
                    image,
                    &background_color.to_rgba(),
                ))
            },
        )
    }
    fn to_tiff(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion(input, output, ImageFormat::Tiff)
    }
    fn to_bmp(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion(input, output, ImageFormat::Bmp)
    }
    fn to_gif(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        wrapper::image_crate_conversion(input, output, ImageFormat::Gif)
    }
    fn to_pdf(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        output.clone_from(&wrapper::pdfwriter_image_to_pdf(input).map_err(|_| ConversionError::Unexpected)?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::{test_utils, Converter, PngConverter};
    use crate::decoder::PdfDecoder;
    use crate::format::Format;
    use image::codecs::bmp::BmpDecoder;
    use image::codecs::gif::GifDecoder;
    use image::codecs::jpeg::JpegDecoder;
    use image::codecs::tiff::TiffDecoder;

    // Implementation of the used Converter trait
    // Converter are supposed to be stateless, so we can use this single instance
    static CONVERTER: PngConverter = PngConverter;
    // Test asset file extension
    static SOURCE_EXT: &str = "png";

    #[test]
    fn test_supported_formats() {
        let formats = CONVERTER.supported_formats();
        assert_eq!(formats.len(), 6);
        assert!(formats.contains(&Format::Png));
        assert!(formats.contains(&Format::Jpeg));
        assert!(formats.contains(&Format::Tiff));
        assert!(formats.contains(&Format::Bmp));
        assert!(formats.contains(&Format::Gif));
        assert!(formats.contains(&Format::Pdf));
    }

    #[test]
    fn test_to_png() {
        let target_ext = "png";

        test_utils::test_conversion_to(
            Format::Png,
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
    
    #[test]
    fn test_to_pdf() {
        let target_ext = "pdf";

        test_utils::test_conversion_to(
            Format::Pdf,
            &CONVERTER,
            SOURCE_EXT,
            target_ext,
            |_, target| {
                PdfDecoder::check(&target)
            },
        );
    }
}
