use std::io;

use crate::format::Format;

#[derive(Debug)]
pub enum ConversionError {
    IoError(io::Error),
    UnknownSourceFormat,
    Unexpected,
    UnsupportedOperation,
    IndirectConversionFailure(Format, Format, Box<ConversionError>), //(source, target, error)
}
