mod error;
mod img;
mod queue;
mod traits;
mod utils;

pub use error::ConversionError;
pub use img::*;
pub use queue::QueueConverter;
pub use utils::*;

pub use traits::{Converter, ConverterImpl};


#[cfg(test)]
pub mod test_utils {
    use std::{
        env, fs,
        io::{Read, Write},
        path::PathBuf,
    };

    use crate::format::Format;

    use super::Converter;

    pub fn get_assets_path() -> PathBuf {
        PathBuf::from("./tests/assets/test")
    }

    pub fn get_converted_file_path() -> PathBuf {
        env::temp_dir().join("converted_file")
    }

    pub fn test_conversion_to(
        format: Format,
        converter: &dyn Converter,
        source_ext: &str,
        target_ext: &str,
        decodable_tester: fn(fs::File, fs::File) -> bool,
    ) {
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

        let conversion_operation = converter.process(&input, &mut output, format);
        assert!(conversion_operation.is_ok());

        target_file.write_all(&output).unwrap();

        // Check if target is decodable with the given decodable_tester
        let readable_output = fs::File::open(&target_path).unwrap();
        assert!(decodable_tester(source_file, readable_output));

        fs::remove_file(target_path).unwrap();
    }
}
