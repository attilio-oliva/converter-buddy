use std::io::Cursor;

use image::{
    error::{ImageFormatHint, UnsupportedError, UnsupportedErrorKind},
    io::Reader as ImageReader,
    ColorType, DynamicImage, GenericImageView, ImageError, ImageFormat,
};
use miniz_oxide::deflate::{compress_to_vec_zlib, CompressionLevel};
use pdf_writer::{Content, Filter, Finish, Name, PdfWriter, Rect, Ref};

use crate::converter::ConversionError;

/// use image crate to read an image from a buffer
pub fn image_crate_read(input: &Vec<u8>) -> Result<DynamicImage, ImageError> {
    let reader = ImageReader::new(Cursor::new(input))
        .with_guessed_format()
        .expect("Cursor io never fails");

    reader.decode()
}

/// Use image crate for the conversion
pub fn image_crate_conversion(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    target_format: ImageFormat,
) -> Result<(), ConversionError> {
    image_crate_conversion_with_processing(input, output, target_format, &|img| Ok(img.to_owned()))
}

/// Use image crate for the conversion and apply a processing function
/// to the image before writing it to the output
pub fn image_crate_conversion_with_processing(
    input: &Vec<u8>,
    output: &mut Vec<u8>,
    target_format: ImageFormat,
    processing: &dyn Fn(&DynamicImage) -> Result<DynamicImage, ConversionError>,
) -> Result<(), ConversionError> {
    let mut image = image_crate_read(input).map_err(|_| ConversionError::UnknownSourceFormat)?;

    image = processing(&mut image)?;

    image
        .write_to(&mut Cursor::new(output), target_format)
        .map_err(|_| ConversionError::Unexpected)
}

pub fn pdfwriter_image_to_pdf(input: &Vec<u8>) -> Result<Vec<u8>, ImageError> {
    let mut writer = PdfWriter::new();
    // From the pdf_writer crate example:

    // Define some indirect reference ids we'll use.
    let catalog_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let page_id = Ref::new(3);
    let image_id = Ref::new(4);
    let s_mask_id = Ref::new(5);
    let content_id = Ref::new(6);
    let image_name = Name(b"Im1");

    // Set up the page tree
    writer.catalog(catalog_id).pages(page_tree_id);
    writer.pages(page_tree_id).kids([page_id]).count(1);

    // Specify one A4 page and map the image name "Im1" to the id of the
    // embedded image stream.
    let mut page = writer.page(page_id);
    let a4 = Rect::new(0.0, 0.0, 595.0, 842.0);
    page.media_box(a4);
    page.parent(page_tree_id);
    page.contents(content_id);
    page.resources().x_objects().pair(image_name, image_id);
    page.finish();

    // Decode the image.
    let format = image::guess_format(&input).unwrap();
    let dynamic = image::load_from_memory(&input).unwrap();

    // Now, there are multiple considerations:
    // - Writing an XObject with just the raw samples would work, but lead to
    //   huge file sizes since the image would be embedded without any
    //   compression.
    // - We can encode the samples with a filter. However, which filter is best
    //   depends on the file format. For example, for JPEGs you should use
    //   DCT-Decode and for PNGs you should use Deflate.
    // - When the image has transparency, we need to provide that separately
    //   through an extra linked SMask image.
    let (filter, encoded, mask) = match format {
        // A JPEG is already valid DCT-encoded data.
        ImageFormat::Jpeg => {
            assert!(dynamic.color() == ColorType::Rgb8);
            (Filter::DctDecode, input.to_owned(), None)
        }

        // While PNGs uses deflate internally, we need to re-encode to get just
        // the raw coded samples without metadata. Also, we need to encode the
        // RGB and alpha data separately.
        ImageFormat::Png => {
            let level = CompressionLevel::DefaultLevel as u8;
            let encoded = compress_to_vec_zlib(dynamic.to_rgb8().as_raw(), level);

            // If there's an alpha channel, extract the pixel alpha values.
            let mask = dynamic.color().has_alpha().then(|| {
                let alphas: Vec<_> = dynamic.pixels().map(|p| (p.2).0[3]).collect();
                compress_to_vec_zlib(&alphas, level)
            });

            (Filter::FlateDecode, encoded, mask)
        }

        // You could handle other image formats similarly or just recode them to
        // JPEG or PNG, whatever best fits your use case.
        unsupported_format => {
            let format_error_hint = ImageFormatHint::Exact(unsupported_format);
            return Err(ImageError::Unsupported(
                UnsupportedError::from_format_and_kind(
                    format_error_hint.clone(),
                    UnsupportedErrorKind::Format(format_error_hint),
                ),
            ));
        }
    };

    // Write the stream for the image we want to embed.
    let mut image = writer.image_xobject(image_id, &encoded);
    image.filter(filter);
    image.width(dynamic.width() as i32);
    image.height(dynamic.height() as i32);
    image.color_space().device_rgb();
    image.bits_per_component(8);
    if mask.is_some() {
        image.s_mask(s_mask_id);
    }
    image.finish();

    // Add SMask if the image has transparency.
    if let Some(encoded) = &mask {
        let mut s_mask = writer.image_xobject(s_mask_id, &encoded);
        s_mask.filter(filter);
        s_mask.width(dynamic.width() as i32);
        s_mask.height(dynamic.height() as i32);
        s_mask.color_space().device_gray();
        s_mask.bits_per_component(8);
    }

    // Size the image at 1pt per pixel.
    let image_width = dynamic.width() as f32;
    let image_height = dynamic.height() as f32;

    // Auto fit image to a4 paper
    let image_aspect_ratio = image_width / image_height;
    let screen_aspect_ratio = a4.x2 / a4.y2;

    let w;
    let h;

    if screen_aspect_ratio > image_aspect_ratio {
        w = image_width * a4.y2 / image_height;
        h = image_height;
    } else {
        w = image_width;
        h = image_height;
    }

    let w = w.min(a4.x2);
    let h = h.min(a4.y2);

    // Center the image on the page.
    let x = (a4.x2 - w) / 2.0;
    let y = (a4.y2 - h) / 2.0;

    // Place and size the image in a content stream.
    //
    // By default, PDF XObjects always have a size of 1x1 user units (and 1 user
    // unit is one 1pt if you don't change that). To position and size them, you
    // have to change the current transformation matrix, which is structured as
    // [scale_x, skew_x, skew_y, scale_y, translate_x, translate_y]. Also,
    // remember that the PDF coordinate system starts at the bottom left! When
    // you have other elements after the image, it's also important to save &
    // restore the state so that they are not affected by the transformation.
    let mut content = Content::new();
    content.save_state();
    content.transform([w, 0.0, 0.0, h, x, y]);
    content.x_object(image_name);
    content.restore_state();
    writer.stream(content_id, &content.finish());

    // Write the document to a buffer.
    Ok(writer.finish())
}
