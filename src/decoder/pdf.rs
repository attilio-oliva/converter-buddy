use std::{fs::File, os::unix::prelude::FileExt};

pub struct PdfDecoder;

impl PdfDecoder {
    fn check_magic_bytes(bytes: &[u8]) -> bool {
        bytes[0] == 0x25 && bytes[1] == 0x50 && bytes[2] == 0x44 && bytes[3] == 0x46
    }

    pub fn check(file: &File) -> bool {
        let mut signature: [u8;4] = [0;4];
        file.read_at(&mut signature, 0).unwrap();
        Self::check_magic_bytes(&signature)
    }
}