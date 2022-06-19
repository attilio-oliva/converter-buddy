use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use super::DecodingError;
use crate::converter::{from_format, ConversionError};
use crate::{format, format::Format};

/// Utility struct to convert a file from one format to another using std::fs::File.
pub struct ConvertibleFile {
    pub path: PathBuf,
    format: OnceCell<Option<Format>>,
}

impl ConvertibleFile {
    pub fn new(path: &str) -> ConvertibleFile {
        ConvertibleFile {
            path: PathBuf::from(path),
            format: OnceCell::new(),
        }
    }

    fn guess_format(path: &PathBuf) -> Result<Format, DecodingError> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(format::from_extension)
            .ok_or(DecodingError::UnknownFormat)
    }

    pub fn format(&self) -> &Option<Format> {
        self.format
            .get_or_init(|| ConvertibleFile::guess_format(&self.path).ok())
    }

    pub fn convert(&self, target_format: Format) -> Result<File, ConversionError> {
        let source_format = self.format().ok_or(ConversionError::UnknownSourceFormat)?;

        let target_format_ext = target_format.info().preferred_extension;
        let full_target_ext = String::from("cb.") + target_format_ext;
        let full_path = &self.path.with_extension(full_target_ext);

        let mut src_file = File::open(&self.path).map_err(ConversionError::IoError)?;
        let mut target_file = File::create(full_path).map_err(ConversionError::IoError)?;

        let mut input_buffer = Vec::<u8>::new();
        let mut output_buffer = Vec::<u8>::new();

        // read buffer from source file
        src_file
            .read_to_end(&mut input_buffer)
            .map_err(ConversionError::IoError)?;

        let converter = from_format(source_format);
        converter.process(&input_buffer, &mut output_buffer, target_format)?;

        target_file
            .write_all(&output_buffer)
            .map_err(ConversionError::IoError)?;
        Ok(target_file)
    }
}
