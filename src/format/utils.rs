use super::Format;

use strum::IntoEnumIterator;

pub fn from_extension(ext: &str) -> Option<Format> {
    Format::iter().find(|f| f.info().extensions.contains(&ext))
}
