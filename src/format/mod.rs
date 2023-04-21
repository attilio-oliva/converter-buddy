//! Contains the formats and their info used by the converter.
//!
//! Mainly used to get the list of supported formats and their info.
//! - crate::format::Format is an enumeration of supported formats.
//! - crate::format::info contains all additional informations about a format.
//!
//! # Examples

mod enumerator;
pub mod info;
mod interop;
mod utils;

pub use enumerator::*;
pub use utils::*;
