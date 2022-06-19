use super::info;
use super::Format;
use once_cell::sync::Lazy;

pub fn from_extension(ext: &str) -> Option<Format> {
    info::LIST
        .iter()
        .find(|info| info.extensions.contains(&ext))
        .map(|info| Lazy::force(info).format)
}
