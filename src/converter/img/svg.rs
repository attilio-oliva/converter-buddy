use crate::{
    converter::{ConversionError, Converter, ConverterImpl, QueueConverter},
    format::Format,
};

pub use crate::converter_info::SvgConverter;

impl Converter for SvgConverter {}

impl ConverterImpl for SvgConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        target_format: Format,
    ) -> Result<(), ConversionError> {
        match target_format {
            Format::Svg => self.to_same_format(input, output),
            Format::Tiff | Format::Png | Format::Jpeg | Format::Bmp | Format::Gif | Format::Pdf => {
                SvgConverter::to_raster_format(input, output, target_format)
            }
            _ => Err(ConversionError::UnsupportedOperation),
        }
    }
}

impl SvgConverter {
    fn to_raster_format(
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        target_format: Format,
    ) -> Result<(), ConversionError> {
        let mut converter = QueueConverter::new();
        converter.push(Format::Png);
        converter.push(target_format);

        converter.process(input, output, Format::Svg)
    }
}

#[cfg(test)]
mod tests {

    use image::codecs::bmp::BmpDecoder;
    use image::codecs::gif::GifDecoder;
    use image::codecs::jpeg::JpegDecoder;
    use image::codecs::png::PngDecoder;
    use image::codecs::tiff::TiffDecoder;

    use crate::converter::{test_utils, SvgConverter};
    use crate::converter_info::ConverterInfo;
    use crate::decoder::PdfDecoder;
    use crate::format::Format;

    // Implementation of the used Converter trait
    // Converter are supposed to be stateless, so we can use this single instance
    static CONVERTER: SvgConverter = SvgConverter;
    // Test asset file extension
    static SOURCE_EXT: &str = "svg";

    #[test]
    fn test_supported_formats() {
        let formats = CONVERTER.supported_formats();
        assert_eq!(formats.len(), 7);
        assert!(formats.contains(&Format::Svg));
        assert!(formats.contains(&Format::Tiff));
        assert!(formats.contains(&Format::Png));
        assert!(formats.contains(&Format::Jpeg));
        assert!(formats.contains(&Format::Bmp));
        assert!(formats.contains(&Format::Gif));
        assert!(formats.contains(&Format::Pdf));
    }

    #[test]
    fn test_to_svg() {
        let target_ext = "svg";

        test_utils::test_conversion_to(
            Format::Svg,
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
    #[test]
    fn test_to_pdf() {
        let target_ext = "pdf";

        test_utils::test_conversion_to(
            Format::Pdf,
            &CONVERTER,
            SOURCE_EXT,
            target_ext,
            |_, target| PdfDecoder::check(&target),
        );
    }
}
