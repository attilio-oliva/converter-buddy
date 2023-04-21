mod error;
mod img;
mod queue;
mod traits;

pub use error::ConversionError;
pub use img::*;
pub use queue::QueueConverter;

pub use traits::{ConversionStrategy, ConverterInfo};

/// Macro used to define a dynamic converter.
/// To achieve this ConversionStrategy<Config> is implemented by relegating the actual conversion to its implementation using the actual format config structure as a generic.
/// Furthermore, ConversionInfo traits is automatically implemented to provide a handy function in a dynamic environment.
/// The first element is the converter and then a comma separated supported formats have to be provided.

#[macro_export]
macro_rules! define_converter {
    ($converter:ident, $($format:ident),*) => {
        use $crate::converter::ConverterInfo;

        pub struct $converter;

        impl ConversionStrategy<Config> for $converter{
            fn process(
                &self,
                input: &Vec<u8>,
                output: &mut Vec<u8>,
                config: Config
            ) -> Result<(), ConversionError> {
                match config {
                    $(Config::$format(format_config) => self.process(input, output, format_config),)*
                    _ => Err(ConversionError::UnsupportedOperation)
                }
            }
        }
        impl ConverterInfo for $converter {
            fn supported_formats(&self) -> Vec<Format> {
                vec![
                    $(Format::$format,)*
                ]
            }
        }
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::{
        env, fs,
        io::{Read, Write},
        path::PathBuf,
    };

    use image::codecs::{
        bmp::BmpDecoder, gif::GifDecoder, jpeg::JpegDecoder, png::PngDecoder, tiff::TiffDecoder,
        webp::WebPDecoder,
    };

    use crate::{
        config::Config,
        decoder::{PdfDecoder, SvgDecoder},
        format::Format,
    };

    use super::ConversionStrategy;

    pub fn get_assets_path() -> PathBuf {
        PathBuf::from("./tests/assets/test")
    }

    pub fn get_converted_file_path() -> PathBuf {
        env::temp_dir().join("converted_file")
    }

    pub fn test_conversion_to<Conv>(
        config: Config,
        converter: &Conv,
        source_ext: &str,
        target_ext: &str,
    ) where
        Conv: ConversionStrategy<Config>,
    {
        let target_format = Format::from(config.clone());
        let file_name = format!("test_{}_to_{}", source_ext, target_ext);
        let source_path = get_assets_path().with_extension(source_ext);
        let target_path = get_converted_file_path()
            .with_file_name(file_name)
            .with_extension(target_ext);

        let mut source_file = fs::File::open(&source_path).unwrap();
        let mut target_file = fs::File::create(&target_path).unwrap();

        let mut input = Vec::<u8>::new();
        let mut output = Vec::<u8>::new();

        source_file.read_to_end(&mut input).unwrap();

        let conversion_operation = converter.process(&input, &mut output, config);
        assert!(conversion_operation.is_ok());

        target_file.write_all(&output).unwrap();

        // Check if target is decodable with the given decodable_tester
        let generated_output = fs::File::open(&target_path).unwrap();
        assert!(check_output_format(
            target_format,
            source_file,
            generated_output
        ));

        fs::remove_file(target_path).unwrap();
    }

    pub fn check_output_format(
        target_format: Format,
        _source_file: fs::File,
        output_file: fs::File,
    ) -> bool {
        match target_format {
            Format::Png => PngDecoder::new(output_file).is_ok(),
            Format::Jpeg => JpegDecoder::new(output_file).is_ok(),
            Format::Gif => GifDecoder::new(output_file).is_ok(),
            Format::WebP => WebPDecoder::new(output_file).is_ok(),
            Format::Tiff => TiffDecoder::new(output_file).is_ok(),
            Format::Bmp => BmpDecoder::new(output_file).is_ok(),
            Format::Pdf => PdfDecoder::check(&output_file),
            Format::Svg => SvgDecoder::check(output_file),
            _ => panic!("This target format in not supported"),
        }
    }
}
