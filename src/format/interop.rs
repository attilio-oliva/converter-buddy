use image::ImageFormat;

use super::Format;

impl From<ImageFormat> for Format {
    fn from(format: ImageFormat) -> Self {
        match format {
            ImageFormat::Png => Format::Png,
            ImageFormat::Jpeg => Format::Jpeg,
            ImageFormat::Gif => Format::Gif,
            ImageFormat::WebP => Format::WebP,
            ImageFormat::Pnm => Format::Pnm,
            ImageFormat::Tiff => Format::Tiff,
            ImageFormat::Tga => Format::Tga,
            ImageFormat::Dds => Format::Dds,
            ImageFormat::Bmp => Format::Bmp,
            ImageFormat::Ico => Format::Ico,
            ImageFormat::Hdr => Format::Hdr,
            ImageFormat::OpenExr => Format::OpenExr,
            ImageFormat::Farbfeld => Format::Farbfeld,
            ImageFormat::Avif => Format::Avif,
            _ => todo!(),
        }
    }
}

impl Into<ImageFormat> for Format {
    fn into(self) -> ImageFormat {
        match self {
            Format::Png => ImageFormat::Png,
            Format::Jpeg => ImageFormat::Jpeg,
            Format::Gif => ImageFormat::Gif,
            Format::WebP => ImageFormat::WebP,
            Format::Pnm => ImageFormat::Pnm,
            Format::Tiff => ImageFormat::Tiff,
            Format::Tga => ImageFormat::Tga,
            Format::Dds => ImageFormat::Dds,
            Format::Bmp => ImageFormat::Bmp,
            Format::Ico => ImageFormat::Ico,
            Format::Hdr => ImageFormat::Hdr,
            Format::OpenExr => ImageFormat::OpenExr,
            Format::Farbfeld => ImageFormat::Farbfeld,
            Format::Avif => ImageFormat::Avif,
            _ => todo!(),
        }
    }
}
