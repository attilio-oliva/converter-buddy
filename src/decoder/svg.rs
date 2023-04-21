use std::{fs, io::Read};

use crate::converter::ConversionError;

pub struct SvgDecoder;
impl SvgDecoder {
    pub fn check(file: fs::File) -> bool {
        let file_data = file
            .bytes()
            .collect::<Result<Vec<u8>, _>>()
            .map_err(ConversionError::IoError);

        match file_data {
            Ok(data) => usvg::Tree::from_data(&data, &usvg::Options::default().to_ref()).is_ok(),
            Err(_) => false,
        }
    }
}
