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

You can use the underneath converters if you don't want to use bytes vectors instead of std::fs:
```rust
fn get_input_data() -> Vec<u8> {
    ...
}

fn main() {
    let input = get_input_data();
    let mut output = Vec::<u8>::new();

    PngConverter.process(&input, &mut output, Format::Jpeg).expect("Conversion error");
    
    // or in a more generic way
    let source_format = Format::Png;
    let target_format = Format::Jpeg;

    let converter = converter::from_format(source_format);
    converter.process(&input, &mut output, target_format).expect("Conversion error");

    // use output ...
}
```

## Compatibility

| From\To | PNG                | JPEG               | BMP                | TIFF               | GIF                | SVG                | WEBP               | PDF                |
|---------|--------------------|--------------------|--------------------|--------------------|--------------------|--------------------|--------------------|--------------------|
| PNG     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                | :heavy_check_mark: |
| JPEG    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                | :heavy_check_mark: |
| BMP     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                | :heavy_check_mark: |
| TIFF    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                | :heavy_check_mark: |
| GIF     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :x:                | :heavy_check_mark: |
| SVG     | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:                | :heavy_check_mark: |
| WEBP    | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
