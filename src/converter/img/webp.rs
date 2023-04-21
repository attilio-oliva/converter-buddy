use std::io::Write;

use super::img_utils::*;

use crate::{
    config::{BmpConfig, Config, PdfConfig},
    converter::{ConversionError, ConversionStrategy},
    define_converter,
    format::Format,
    impl_common_image_conversions,
};

define_converter!(WebPConverter, WebP, Bmp, Jpeg, Png, Tiff, Gif, Pdf);
impl_common_image_conversions!(WebPConverter, Bmp, Jpeg, Png, Tiff, Gif);

impl ConversionStrategy<WebPConfig> for WebPConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        _config: WebPConfig,
    ) -> Result<(), ConversionError> {
        output.write_all(input).map_err(ConversionError::IoError)
    }
}

impl ConversionStrategy<PdfConfig> for WebPConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: PdfConfig,
    ) -> Result<(), ConversionError> {
        common_strategies::from_image_to_pdf(input, output, Format::WebP, config.into())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::config::{
        BmpConfig, Config, GifConfig, JpegConfig, PdfConfig, PngConfig, TiffConfig, WebPConfig,
    };
    use crate::converter::{test_utils, ConverterInfo};

    use crate::format::Format;

    use super::WebPConverter;

    // Implementation of the used Converter trait
    // Converters are supposed to be stateless, so we can use this single instance
    static CONVERTER: WebPConverter = WebPConverter;
    // Test asset file extension
    static SOURCE_EXT: &str = "webp";

    #[test]
    fn test_supported_formats() {
        let formats = CONVERTER.supported_formats();
        assert_eq!(formats.len(), 7);
        assert!(formats.contains(&Format::WebP));
        assert!(formats.contains(&Format::Gif));
        assert!(formats.contains(&Format::Tiff));
        assert!(formats.contains(&Format::Png));
        assert!(formats.contains(&Format::Jpeg));
        assert!(formats.contains(&Format::Bmp));
        assert!(formats.contains(&Format::Pdf));
    }

    #[test_case(WebPConfig::default() ; "to_webp")]
    #[test_case(BmpConfig::default() ; "to_bmp")]
    #[test_case(JpegConfig::default() ; "to_jpeg")]
    #[test_case(TiffConfig::default() ; "to_tiff")]
    #[test_case(PngConfig::default() ; "to_png")]
    #[test_case(GifConfig::default() ; "to_gif")]
    #[test_case(PdfConfig::default() ; "to_pdf")]
    fn conversion<C>(config: C)
    where
        C: Into<Config>,
    {
        let dynamic_config = config.into();
        let target_format = Format::from(dynamic_config.clone());
        let target_ext = target_format.info().preferred_extension; //Beware that any extension could be used for this test
        test_utils::test_conversion_to(dynamic_config, &CONVERTER, SOURCE_EXT, target_ext);
    }
}
