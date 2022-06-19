use super::FormatInfo;
use crate::format::Format;
use once_cell::sync::Lazy;

pub static LIST: &[&Lazy<FormatInfo>] = &[
    &PNG, &JPEG, &GIF, &WEBP, &PNM, &TIFF, &TGA, &DDS, &BMP, &ICO, &HDR, &OPENEXR, &FARBFELD, &AVIF,
];

pub static PNG: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Png,
    extensions: vec!["png"],
    preferred_extension: "png",
    mime: "image/png",
});
pub static JPEG: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Jpeg,
    extensions: vec!["jpg", "jpeg"],
    preferred_extension: "jpg",
    mime: "image/jpeg",
});
pub static GIF: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Gif,
    extensions: vec!["gif"],
    preferred_extension: "gif",
    mime: "image/gif",
});
pub static WEBP: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::WebP,
    extensions: vec!["webp"],
    preferred_extension: "webp",
    mime: "image/webp",
});
pub static PNM: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Pnm,
    extensions: vec!["pnm", "pgm", "ppm", "pfm", "pam"],
    preferred_extension: "pnm",
    mime: "image/x-portable-anymap",
});
pub static TIFF: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Tiff,
    extensions: vec!["tiff", "tif"],
    preferred_extension: "tiff",
    mime: "image/tiff",
});
pub static TGA: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Tga,
    extensions: vec!["tga"],
    preferred_extension: "tga",
    mime: "image/x-tga",
});
pub static DDS: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Dds,
    extensions: vec!["dds"],
    preferred_extension: "dds",
    mime: "image/vnd.ms-dds",
});
pub static BMP: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Bmp,
    extensions: vec!["bmp"],
    preferred_extension: "bmp",
    mime: "image/bmp",
});
pub static ICO: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Ico,
    extensions: vec!["ico"],
    preferred_extension: "ico",
    mime: "image/x-icon",
});
pub static HDR: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Hdr,
    extensions: vec!["hdr"],
    preferred_extension: "hdr",
    mime: "image/vnd.radiance",
});
pub static OPENEXR: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::OpenExr,
    extensions: vec!["exr"],
    preferred_extension: "exr",
    mime: "image/vnd.openexr",
});
pub static FARBFELD: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Farbfeld,
    extensions: vec!["farbfeld"],
    preferred_extension: "farbfeld",
    mime: "image/x-farbfeld",
});
pub static AVIF: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Avif,
    extensions: vec!["avif"],
    preferred_extension: "avif",
    mime: "image/avif",
});
pub static SVG: Lazy<FormatInfo> = Lazy::new(|| FormatInfo {
    format: Format::Svg,
    extensions: vec!["svg"],
    preferred_extension: "svg",
    mime: "image/svg+xml",
});
