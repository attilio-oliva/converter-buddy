# Converter Buddy

Converter Buddy provides a simple to use way to convert file from a format to another.

Currently only the most popular image formats are supported, but the goal is to extend to documents, audio and video formats.

## Basic usage

ConvertibleFile is a conversion utility wrapper for std::fs::File: 
```rust
let src_path = "tests/assets/test.png";

let file = ConvertibleFile::new(src_path);
let format = file.format().expect("No format found");
let target_format = Format::Jpeg;

println!("Found source format: {}", format);
println!("Converting to {} format", target_format);

match file.convert(target_format) {
    Ok(_) => println!("Conversion successful"),
    Err(e) => println!("Conversion failed: {:?}", e),
}
```

You can use the underneath converters if you don't want to use std::fs, but pure data:
```rust
fn get_input_data() -> Vec<u8> {
    ...
}

fn main() {
    let input = get_input_data();
    let mut output = Vec::<u8>::new();

    PngConverter.to_jpeg(&input, &mut output).expect("Conversion error");
    
    // or in a more generic way
    let source_format = Format::Png;
    let target_format = Format::Jpeg;

    let converter = converter::from_format(source_format);
    converter.process(&input, &mut output, target_format).expect("Conversion error");

    // use output ...
}
```

## Compatibility

| From\To | PNG                | JPEG               | BMP                | TIFF               | GIF                | SVG                | WEBP               |
|---------|--------------------|--------------------|--------------------|--------------------|--------------------|--------------------|--------------------|
| PNG     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                |
| JPEG    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                |
| BMP     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                |
| TIFF    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                |
| GIF     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                |
| SVG     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                |
| WEBP    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.