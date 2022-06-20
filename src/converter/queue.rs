use std::collections::VecDeque;

use crate::{converter::from_format, format::Format};

use super::ConversionError;

#[derive(Default)]
pub struct QueueConverter {
    queue: VecDeque<Format>,
}

impl QueueConverter {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::default(),
        }
    }
    pub fn with_queue(queue: VecDeque<Format>) -> Self {
        Self { queue }
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
        source_format: Format,
    ) -> Result<(), ConversionError> {
        let mut source_format = source_format.clone();

        let mut current_input = input.to_owned();

        // The default VecDeque behavior is to operate as a queue, so the iterable should follow a FIFO order
        for target_format in self.queue.iter() {
            let converter = from_format(source_format.clone());
            //info!("Converting from {:?} to {:?}, with {:?}", source_format, target_format, converter.supported_formats());
            converter
                .process(&current_input, output, *target_format)
                .map_err(|e| {
                    ConversionError::IndirectConversionFailure(
                        source_format,
                        *target_format,
                        Box::new(e),
                    )
                })?;
            //info!("Converted from {:?} to {:?}", source_format, target_format);
            source_format = *target_format;
            current_input = output.clone();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use image::codecs::tiff::TiffDecoder;

    #[test]
    fn test_queue_converter() {
        use super::*;
        use crate::format::Format;
        use std::{
            fs::File,
            io::{Read, Write},
            path::PathBuf,
        };

        let source_format = Format::Jpeg;

        let mut queue_converter = QueueConverter::new();
        queue_converter.push(Format::Png);
        queue_converter.push(Format::Tiff);

        let source_path = PathBuf::from("./tests/assets/test.jpg");
        let target_path = PathBuf::from("/tmp/test_jpeg_to_png_to_tiff.tiff");

        let mut source_file = File::open(&source_path).unwrap();
        let mut target_file = File::create(&target_path).unwrap();

        let mut input = Vec::<u8>::new();
        let mut output = Vec::<u8>::new();

        source_file.read_to_end(&mut input).unwrap();

        let conversion_operation = queue_converter.process(&input, &mut output, source_format);
        assert!(conversion_operation.is_ok());

        target_file.write_all(&output).unwrap();

        let readable_output = File::open(&target_path).unwrap();
        let decoding_operation = TiffDecoder::new(readable_output);
        assert!(decoding_operation.is_ok());

        std::fs::remove_file(target_path).unwrap();
    }
}
