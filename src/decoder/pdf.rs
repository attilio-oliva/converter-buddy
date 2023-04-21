use std::{fs::File, io::Read};

pub struct PdfDecoder;

impl PdfDecoder {
    fn check_magic_bytes(bytes: &[u8]) -> bool {
        bytes[0] == 0x25 && bytes[1] == 0x50 && bytes[2] == 0x44 && bytes[3] == 0x46
    }
    /// The first 4 bytes are used to check if the header if from a PNG file
    pub fn check(file: &File) -> bool {
        let extracted_bytes = file.bytes().take(4).collect::<Result<Vec<u8>, _>>();

        match extracted_bytes {
            Ok(signature) => Self::check_magic_bytes(signature.as_slice()),
            Err(_) => false,
        }
    }
}
