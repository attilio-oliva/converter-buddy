//! Contains a list of information about a format.
//!
//! A list of supported formats is provided, such as:
//! - converter-buddy::format::info::PNG
//! - converter-buddy::format::info::JPEG
//!
//! Every element of the list is defined as a FormatInfo struct, accessible in converter-buddy::format::info::FormatInfo.
//!
//! # Examples
//! Get all the extension of a JPEG image format:
//! ```
//! //let jpegExt = converter_buddy::format::info::JPEG.extension;
//! //jpegExt.iter().for_each(|ext| print!("{} ", ext));
//! ```
//! Output: `jpg jpeg`

mod data;
mod structure;

pub use self::data::*;
pub use self::structure::FormatInfo;
