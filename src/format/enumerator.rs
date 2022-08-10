use super::info;
use once_cell::sync::Lazy;
use strum_macros::{Display, EnumIter, EnumString};

/// An enumeration of supported file formats.
/// Not all formats support both encoding and decoding.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Display, EnumIter, EnumString)]
#[strum(serialize_all = "PascalCase")]
pub enum Format {
    //----------------------------------Image formats------------------------------------------------------
    /// An Image in PNG Format
    Png,

    /// An Image in JPEG Format
    Jpeg,

    /// An Image in GIF Format
    Gif,

    /// An Image in WEBP Format
    WebP,

    /// An Image in general PNM Format
    Pnm,

    /// An Image in TIFF Format
    Tiff,

    /// An Image in TGA Format
    Tga,

    /// An Image in DDS Format
    Dds,

    /// An Image in BMP Format
    Bmp,

    /// An Image in ICO Format
    Ico,

    /// An Image in Radiance HDR Format
    Hdr,

    /// An Image in OpenEXR Format
    OpenExr,

    /// An Image in farbfeld Format
    Farbfeld,

    /// An Image in AVIF format.
    Avif,

    /// An Image in SVG Format
    Svg,

    // ----------------------------------Document formats---------------------------------------------------------
    /// A Document in PDF Format
    Pdf,
    // ----------------------------------Audio formats------------------------------------------------------------

    // ----------------------------------Video formats------------------------------------------------------------

    // ----------------------------------Archive formats----------------------------------------------------------

    // ----------------------------------Other formats-----------------------------------------------------------
}

impl Format {
    /// Returns the list of supported formats.
    ///
    /// This list is generated on function call and should contain all Format values.
    /// Update this list if you add a new Format.
    pub fn supported_formats() -> Vec<Self> {
        use self::Format::*;

        vec![
            Png, Jpeg, Gif, WebP, Pnm, Tiff, Tga, Dds, Bmp, Ico, Hdr, OpenExr, Farbfeld, Avif,
        ]
    }

    /// Returns all the information about the format as a FormatInfo struct.
    /// The returned FormatInfo struct is reference to a lazy static.
    pub fn info(&self) -> &Lazy<info::FormatInfo> {
        match *self {
            Format::Png => &info::PNG,
            Format::Jpeg => &info::JPEG,
            Format::Gif => &info::GIF,
            Format::WebP => &info::WEBP,
            Format::Pnm => &info::PNM,
            Format::Tiff => &info::TIFF,
            Format::Tga => &info::TGA,
            Format::Dds => &info::DDS,
            Format::Bmp => &info::BMP,
            Format::Ico => &info::ICO,
            Format::Hdr => &info::HDR,
            Format::OpenExr => &info::OPENEXR,
            Format::Farbfeld => &info::FARBFELD,
            Format::Avif => &info::AVIF,
            Format::Svg => &info::SVG,
            Format::Pdf => &info::PDF,
        }
    }
}
