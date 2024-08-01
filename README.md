# MusicXML

MusicXML is a library for reading and writing MusicXML files in Rust. MusicXML is a standard format for
representing Western musical notation and is widely used in music notation software such as Finale, Sibelius,
and MuseScore. This library provides a simple interface for parsing MusicXML files and converting them into
a structured data format that can be easily manipulated and analyzed programmatically.

The library is designed to be easy to use and flexible, allowing you to read and write MusicXML files with
minimal effort. It provides functions for reading MusicXML files and converting them into a structured
data format, as well as functions for writing structured data back to MusicXML files. It is also designed to
be robust and reliable, handling a wide variety of MusicXML files and formats with ease.

## Features

- Implements the full official [MusicXML Standard](https://www.w3.org/2021/06/musicxml40/)
- Supports both raw `.musicxml` and compressed `.mxl` file formats
- Supports both "partwise" and "timewise" data representations
- Allows for transparent conversion between the partwise and timewise representations
- Is provided as a Rust-format library so that it can be link-time optimized with your own code

## Getting Started

To use this library in your Rust project, simply add the following line to your `Cargo.toml` file:

```toml
[dependencies]
musicxml = "1.0"
```

You can then parse any regular MusicXML file or compressed MXL file into a structured data format using:

```rust
use musicxml::*;

match read_score_partwise("path/to/file.musicxml") {
  Ok(score) => {}, // Do something with the score
  Err(e) => println!("Error reading MusicXML file: {}", e),
}
```

or

```rust
use musicxml::*;

match read_score_timewise("path/to/file.musicxml") {
  Ok(score) => {}, // Do something with the score
  Err(e) => println!("Error reading MusicXML file: {}", e),
}
```

Please refer to the [library documentation](https://docs.rs/musicxml/latest/) for full usage instructions.
You may also want to consult the official [MusicXML Standard](https://www.w3.org/2021/06/musicxml40/) for additional
details.

## License

This library is licensed under the [MIT license](http://opensource.org/licenses/MIT).


## TODO

Add documentation about "std" feature, add options to parse MusicXML files directly from string (both compressed and uncompressed), same for writing to string
