use super::img_utils::*;

use crate::{
    config::{BmpConfig, Config, PdfConfig},
    converter::{ConversionError, ConversionStrategy},
    define_converter,
    format::Format,
    impl_common_image_conversions,
};

define_converter!(GifConverter, Bmp, Jpeg, Png, Tiff, Gif, Pdf);
impl_common_image_conversions!(GifConverter, Bmp, Jpeg, Png, Tiff, Gif);

impl ConversionStrategy<PdfConfig> for GifConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: PdfConfig,
    ) -> Result<(), ConversionError> {
        common_strategies::from_image_to_pdf(input, output, Format::Gif, config.into())
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::config::{
        BmpConfig, Config, GifConfig, JpegConfig, PdfConfig, PngConfig, TiffConfig,
    };
    use crate::converter::ConverterInfo;
    use crate::converter::{test_utils, GifConverter};
    use crate::format::Format;

    // Implementation of the used Converter trait
    // Converters are supposed to be stateless, so we can use this single instance
    static CONVERTER: GifConverter = GifConverter;
    // Test asset file extension
    static SOURCE_EXT: &str = "gif";

    #[test]
    fn test_supported_formats() {
        let formats = CONVERTER.supported_formats();
        assert_eq!(formats.len(), 6);
        assert!(formats.contains(&Format::Gif));
        assert!(formats.contains(&Format::Tiff));
        assert!(formats.contains(&Format::Png));
        assert!(formats.contains(&Format::Jpeg));
        assert!(formats.contains(&Format::Bmp));
        assert!(formats.contains(&Format::Pdf));
    }

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
