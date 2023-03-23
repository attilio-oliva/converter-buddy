use super::ConversionError;
use crate::{converter_info::ConverterInfo, format::Format};

pub trait Converter: ConverterImpl + ConverterInfo {}
pub trait ConverterImpl {
    fn process(
        &self,
        _input: &Vec<u8>,
        _output: &mut Vec<u8>,
        _target_format: Format,
    ) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }

    fn to_same_format(&self, input: &Vec<u8>, output: &mut Vec<u8>) -> Result<(), ConversionError> {
        Ok(output.clone_from(input))
    }
}
