use std::io::Write;

use crate::{
    config::{
        BmpConfig, Config, GifConfig, JpegConfig, PdfConfig, PngConfig, SvgConfig, TiffConfig,
    },
    converter::{ConversionError, ConversionStrategy, QueueConverter},
    define_converter,
    format::Format,
};

use super::common_strategies;

define_converter!(SvgConverter, Svg, Bmp, Jpeg, Png, Tiff, Gif, Pdf);

impl ConversionStrategy<SvgConfig> for SvgConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        _config: SvgConfig,
    ) -> Result<(), ConversionError> {
        output.write_all(input).map_err(ConversionError::IoError)
    }
}

impl ConversionStrategy<BmpConfig> for SvgConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: BmpConfig,
    ) -> Result<(), ConversionError> {
        SvgConverter::to_raster_format(input, output, Config::Bmp(config))
    }
}
impl ConversionStrategy<GifConfig> for SvgConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: GifConfig,
    ) -> Result<(), ConversionError> {
        SvgConverter::to_raster_format(input, output, Config::Gif(config))
    }
}
impl ConversionStrategy<TiffConfig> for SvgConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: TiffConfig,
    ) -> Result<(), ConversionError> {
        SvgConverter::to_raster_format(input, output, Config::Tiff(config))
    }
}
impl ConversionStrategy<JpegConfig> for SvgConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: JpegConfig,
    ) -> Result<(), ConversionError> {
        SvgConverter::to_raster_format(input, output, Config::Jpeg(config))
    }
}
impl ConversionStrategy<PngConfig> for SvgConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: PngConfig,
    ) -> Result<(), ConversionError> {
        let mut opt = usvg::Options::default();
        opt.fontdb.load_system_fonts();

        let tree =
            usvg::Tree::from_data(input, &opt.to_ref()).map_err(|_| ConversionError::Unexpected)?;

        let pixmap_size = tree.svg_node().size.to_screen_size();
        if let Some(new_size) = config.base.size {
            let new_pixmap_size = usvg::ScreenSize::new(new_size.width, new_size.height)
                .ok_or(ConversionError::Unexpected)?;
            pixmap_size.scale_to(new_pixmap_size);
        }
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .ok_or(ConversionError::Unexpected)?;

        resvg::render(
            &tree,
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .ok_or(ConversionError::Unexpected)?;
        let converted_image = pixmap
            .encode_png()
            .map_err(|_| ConversionError::Unexpected)?;
        output
            .write_all(converted_image.as_slice())
            .map_err(ConversionError::IoError)?;
        Ok(())
    }
}

impl ConversionStrategy<PdfConfig> for SvgConverter {
    fn process(
        &self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        config: PdfConfig,
    ) -> Result<(), ConversionError> {
        common_strategies::from_image_to_pdf(input, output, Format::Svg, Config::Pdf(config))
    }
}

impl SvgConverter {
    fn to_raster_format(
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        target_config: Config,
    ) -> Result<(), ConversionError> {
        let target_format = Format::from(target_config.clone());
        let mut converter = QueueConverter::new(Format::Svg);
        converter.push(Format::Png);
        converter.push(target_format);

        converter.process(input, output, target_config)
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::config::{
        BmpConfig, Config, GifConfig, JpegConfig, PdfConfig, PngConfig, SvgConfig, TiffConfig,
    };
    use crate::converter::test_utils;
    use crate::converter::ConverterInfo;
    use crate::format::Format;

    use super::SvgConverter;

    // Implementation of the used Converter trait
    // Converters are supposed to be stateless, so we can use this single instance
    static CONVERTER: SvgConverter = SvgConverter;
    // Test asset file extension
    static SOURCE_EXT: &str = "svg";

    #[test]
    fn supported_formats() {
        let formats = &CONVERTER.supported_formats();
        assert_eq!(formats.len(), 7);
        assert!(formats.contains(&Format::Svg));
        assert!(formats.contains(&Format::Gif));
        assert!(formats.contains(&Format::Tiff));
        assert!(formats.contains(&Format::Png));
        assert!(formats.contains(&Format::Jpeg));
        assert!(formats.contains(&Format::Bmp));
        assert!(formats.contains(&Format::Pdf));
    }

    #[test_case(SvgConfig::default() ; "to_svg")]
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
