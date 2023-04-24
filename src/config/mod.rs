use derive_builder::Builder;
use image::codecs::{jpeg::PixelDensity, png::CompressionType};
use image::imageops::FilterType;
use smart_default::SmartDefault;

use crate::converter::ConversionError;
use crate::format::Format;

#[derive(Default, Debug, Clone, Copy, Builder, Eq, PartialEq)]
pub struct SizeSetting {
    pub width: u32,
    pub height: u32,
}
#[derive(Default, Debug, Clone, Builder, PartialEq)]
#[builder(default)]
pub struct ImageOperations {
    pub blur: Option<f32>,
    pub contrast: Option<f32>,
    pub grayscale: Option<bool>,
    pub invert: Option<bool>,
}
#[derive(Default, Debug, Clone, Builder, PartialEq)]
#[builder(default)]
pub struct ImageConfig {
    pub size: Option<SizeSetting>,
    pub filter: Option<FilterType>,
    pub color_type: Option<image::ColorType>,
    pub operations: Option<ImageOperations>,
}
#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(default)]
pub struct PngConfig {
    pub base: ImageConfig,
    pub compression: CompressionType,
    pub filter: image::codecs::png::FilterType,
}
#[derive(Clone, SmartDefault, Debug, Builder, PartialEq)]
#[builder(default)]
pub struct JpegConfig {
    pub base: ImageConfig,
    #[default = 80]
    pub quality: u8,
    pub pixel_density: PixelDensity,
}
#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(default)]
pub struct BmpConfig {
    pub base: ImageConfig,
}

#[derive(Clone, SmartDefault, Builder, Debug)]
#[builder(default)]
pub struct GifConfig {
    pub base: ImageConfig,
    #[default = 1]
    pub speed: i32,
    pub repeat: Option<image::codecs::gif::Repeat>,
}

#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(default)]
pub struct TiffConfig {
    pub base: ImageConfig,
}

#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(default)]
pub struct WebPConfig {
    pub base: ImageConfig,
}
#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(default)]
pub struct PdfConfig {}
#[derive(Clone, Default, Debug, Builder, PartialEq)]
#[builder(default)]
pub struct SvgConfig {}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Config {
    Jpeg(JpegConfig),
    Png(PngConfig),
    Bmp(BmpConfig),
    Tiff(TiffConfig),
    Gif(GifConfig),
    WebP(WebPConfig),
    Svg(SvgConfig),
    Pdf(PdfConfig),
}
// TODO: create macro to implement all the convertions from this line

impl TryFrom<Format> for Config {
    type Error = ConversionError;
    fn try_from(value: Format) -> Result<Self, Self::Error> {
        match value {
            Format::Png => Ok(Config::Png(PngConfig::default())),
            Format::Jpeg => Ok(Config::Jpeg(JpegConfig::default())),
            Format::Bmp => Ok(Config::Bmp(BmpConfig::default())),
            Format::Tiff => Ok(Config::Tiff(TiffConfig::default())),
            Format::Gif => Ok(Config::Gif(GifConfig::default())),
            Format::Pdf => Ok(Config::Pdf(PdfConfig::default())),
            Format::Svg => Ok(Config::Svg(SvgConfig::default())),
            _ => Err(ConversionError::UnsupportedOperation),
        }
    }
}

impl From<Config> for Format {
    fn from(value: Config) -> Format {
        match value {
            Config::Jpeg(_) => Format::Jpeg,
            Config::Png(_) => Format::Png,
            Config::Bmp(_) => Format::Bmp,
            Config::Tiff(_) => Format::Tiff,
            Config::Gif(_) => Format::Gif,
            Config::WebP(_) => Format::WebP,
            Config::Svg(_) => Format::Svg,
            Config::Pdf(_) => Format::Pdf,
        }
    }
}

// inner config structure conversions to Config enum
impl From<JpegConfig> for Config {
    fn from(value: JpegConfig) -> Self {
        Config::Jpeg(value)
    }
}
impl From<PngConfig> for Config {
    fn from(value: PngConfig) -> Self {
        Config::Png(value)
    }
}
impl From<PdfConfig> for Config {
    fn from(value: PdfConfig) -> Self {
        Config::Pdf(value)
    }
}
impl From<GifConfig> for Config {
    fn from(value: GifConfig) -> Self {
        Config::Gif(value)
    }
}
impl From<BmpConfig> for Config {
    fn from(value: BmpConfig) -> Self {
        Config::Bmp(value)
    }
}
impl From<TiffConfig> for Config {
    fn from(value: TiffConfig) -> Self {
        Config::Tiff(value)
    }
}

impl From<WebPConfig> for Config {
    fn from(value: WebPConfig) -> Self {
        Config::WebP(value)
    }
}

impl From<SvgConfig> for Config {
    fn from(value: SvgConfig) -> Self {
        Config::Svg(value)
    }
}

// inner config structure to format it refers to
impl From<JpegConfig> for Format {
    fn from(_value: JpegConfig) -> Self {
        Format::Jpeg
    }
}
impl From<PngConfig> for Format {
    fn from(_value: PngConfig) -> Self {
        Format::Png
    }
}
impl From<TiffConfig> for Format {
    fn from(_value: TiffConfig) -> Self {
        Format::Tiff
    }
}
impl From<SvgConfig> for Format {
    fn from(_value: SvgConfig) -> Self {
        Format::Svg
    }
}
impl From<WebPConfig> for Format {
    fn from(_value: WebPConfig) -> Self {
        Format::WebP
    }
}
impl From<GifConfig> for Format {
    fn from(_value: GifConfig) -> Self {
        Format::Gif
    }
}
impl From<PdfConfig> for Format {
    fn from(_value: PdfConfig) -> Self {
        Format::Pdf
    }
}
