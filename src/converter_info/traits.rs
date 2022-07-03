use crate::format::Format;

pub trait ConverterInfo{
    fn supported_formats(&self) -> Vec<Format> {
        vec![]
    }
}
