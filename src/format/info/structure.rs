use crate::format::Format;

/// A structure containing all additional informations about a format.
/// The objective in future is to take advantage of infos for conversion strategies.
/// An example would an indirect conversion of lossless images, it would be ideal to use only other lossless formats.

#[non_exhaustive]
pub struct FormatInfo {
    /// Reference of format type
    pub format: Format,

    /// List of extension that identifies the format
    pub extensions: Vec<&'static str>,

    /// The preferred extension for the format contained in the extension list
    pub preferred_extension: &'static str,

    /// Mime type of the format
    pub mime: &'static str,
}
