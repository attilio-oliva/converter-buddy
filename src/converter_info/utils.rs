use crate::format::Format;

use super::*;

pub fn from_format(format: Format) -> Box<dyn ConverterInfo> {
    match format {
        Format::Png => Box::new(PngConverter),
        Format::Jpeg => Box::new(JpegConverter),
        Format::Tiff => Box::new(TiffConverter),
        Format::Bmp => Box::new(BmpConverter),
        Format::Gif => Box::new(GifConverter),
        Format::WebP => Box::new(WebPConverter),
        Format::Svg => Box::new(SvgConverter),
        _ => panic!("Unsupported format"),
    }
}
