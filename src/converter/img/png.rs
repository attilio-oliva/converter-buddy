use super::img_utils::*;
use image::{DynamicImage, Pixel};

use crate::{
    config::{BmpConfig, Config, PdfConfig},
    converter::{ConversionError, ConversionStrategy},
    define_converter,
    format::Format,
    impl_common_image_conversions,
};

define_converter!(PngConverter, Bmp, Jpeg, Png, Tiff, Gif, Pdf);
impl_common_image_conversions!(PngConverter, Bmp, Jpeg, Png, Tiff, Gif);

impl ConversionStrategy<PdfConfig> for PngConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        _config: PdfConfig,
    ) -> Result<(), ConversionError> {
        let generated_output =
            wrapper::pdfwriter_image_to_pdf(input).map_err(|_| ConversionError::Unexpected)?;
        output.clone_from(&generated_output);
        Ok(())
    }
}

impl PngConverter {
    fn tranparent_to_color(image: &DynamicImage) -> Result<DynamicImage, ConversionError> {
        // Decide a color to replace transparent pixels with
        // TODO: Make this configurable
        // TODO: Make this support higher color depth than 8-bit (Rgb<u8>)
        // TODO: Add anti-aliasing
        let avg_color = processing::average_image_color(image);
        let white_color = image::Rgb([255, 255, 255]);
        let black_color = image::Rgb([0, 0, 0]);
        //info!("Average color: {:?}", avg_color);
        let background_color = if processing::contrast_ratio(&avg_color, &white_color) >= 1.5 {
            white_color
        } else {
            black_color
        };
        Ok(processing::map_image_transparent_color(
            image,
            &background_color.to_rgba(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::config::{
        BmpConfig, Config, GifConfig, JpegConfig, PdfConfig, PngConfig, TiffConfig,
    };
    use crate::converter::test_utils;
    use crate::converter::ConverterInfo;
    use crate::format::Format;

    use super::PngConverter;

    // Implementation of the used Converter trait
    // Converters are supposed to be stateless, so we can use this single instance
    static CONVERTER: PngConverter = PngConverter;
    // Test asset file extension
    static SOURCE_EXT: &str = "png";

    #[test]
    fn test_supported_formats() {
        let formats = &CONVERTER.supported_formats();
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
