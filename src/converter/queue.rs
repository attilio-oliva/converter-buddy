use std::{collections::VecDeque, io::Write};

use crate::{config::Config, format::Format};

use super::{ConversionError, Converter};

pub struct QueueConverter {
    queue: VecDeque<Format>,
    source_format: Format,
}

impl QueueConverter {
    pub fn new(source_format: Format) -> Self {
        Self {
            queue: VecDeque::default(),
            source_format,
        }
    }
    pub fn push(&mut self, format: Format) {
        self.queue.push_back(format);
    }
    #[allow(dead_code)]
    fn poll(&mut self) -> Option<Format> {
        self.queue.pop_front()
    }

    pub fn process(
        &mut self,
        input: &Vec<u8>,
        output: &mut Vec<u8>,
        target_config: Config,
    ) -> Result<(), ConversionError> {
        let mut source_format = self.source_format;
        let mut current_input = input.clone();
        let mut current_output = Vec::<u8>::new();
        let total_steps = self.queue.len();
        // The default VecDeque behavior is to operate as a queue, so the iterable should follow a FIFO order
        for (step, current_target_format) in self.queue.iter().enumerate() {
            let converter = Converter::try_from(source_format)?;
            let config = if step + 1 != total_steps {
                Config::try_from(*current_target_format)?
            } else {
                target_config.clone()
            };
            current_output = vec![];
            //info!("Converting from {:?} to {:?}, with {:?}", source_format, target_format, converter.supported_formats());
            converter
                .process(&current_input, &mut current_output, config)
                .map_err(|e| {
                    ConversionError::IndirectConversionFailure(
                        source_format,
                        *current_target_format,
                        Box::new(e),
                    )
                })?;
            //info!("Converted from {:?} to {:?}", source_format, target_format);
            source_format = *current_target_format;
            current_input = current_output.clone();
        }
        output
            .write_all(&mut current_output)
            .map_err(ConversionError::IoError)
    }
}

#[cfg(test)]
mod tests {
    use image::codecs::tiff::TiffDecoder;

    #[test]
    fn queued_conversion() {
        use super::*;
        use crate::format::Format;
        use std::{
            fs::File,
            io::{Read, Write},
            path::PathBuf,
        };

        let source_format = Format::Jpeg;
        let intermediate_format = Format::Png;
        let target_format = Format::Tiff;

        let mut queue_converter = QueueConverter::new(source_format);
        queue_converter.push(intermediate_format);
        queue_converter.push(target_format);

        let source_path = PathBuf::from("./tests/assets/test.jpg");
        let target_path = PathBuf::from("/tmp/test_jpeg_to_png_to_tiff.tiff");

        let mut source_file = File::open(&source_path).unwrap();
        let mut target_file = File::create(&target_path).unwrap();

        let mut input = Vec::<u8>::new();
        let mut output = Vec::<u8>::new();

        source_file.read_to_end(&mut input).unwrap();

        let conversion_operation =
            queue_converter.process(&input, &mut output, target_format.try_into().unwrap());
        assert!(conversion_operation.is_ok());

        target_file.write_all(&output).unwrap();

        let readable_output = File::open(&target_path).unwrap();
        let decoding_operation = TiffDecoder::new(readable_output);
        assert!(decoding_operation.is_ok());

        std::fs::remove_file(target_path).unwrap();
    }
}
