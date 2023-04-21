use super::ConversionError;
use crate::format::Format;

pub trait ConversionStrategy<F> {
    fn process(
        &self,
        _input: &Vec<u8>,
        _output: &mut Vec<u8>,
        _config: F,
    ) -> Result<(), ConversionError> {
        Err(ConversionError::UnsupportedOperation)
    }
}

pub trait ConverterInfo {
    fn supported_formats(&self) -> Vec<Format> {
        vec![]
    }
}
