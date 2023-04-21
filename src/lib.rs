//TODO: rework the API, because clippy is right on this. For the moment ignore the warning.
#![allow(clippy::ptr_arg)]

pub mod config;
#[cfg(feature = "converters")]
pub mod converter;
pub mod format;

#[cfg(feature = "io")]
pub mod io;

#[cfg(feature = "decoders")]
pub mod decoder;
