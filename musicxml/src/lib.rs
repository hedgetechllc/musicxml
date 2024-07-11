#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(html_favicon_url = "https://hedgetechllc.github.io/musicxml/musicxml/favicon.ico")]
#![doc(html_logo_url = "https://hedgetechllc.github.io/musicxml/musicxml/MusicXML-Logo-Square.png")]
#![doc(html_playground_url = "https://playground.example.com/")]

//! # Overview
//!
//! `musicxml` is a library for reading and writing MusicXML files in Rust. MusicXML is a standard format for
//! representing Western musical notation and is widely used in music notation software such as Finale, Sibelius,
//! and MuseScore. This library provides a simple interface for parsing MusicXML files and converting them into
//! a structured data format that can be easily manipulated and analyzed programmatically.
//!
//! The library is designed to be easy to use and flexible, allowing you to read and write MusicXML files with
//! minimal effort. It provides functions for reading MusicXML files and converting them into a structured
//! data format, as well as functions for writing structured data back to MusicXML files. It is also designed to
//! be robust and reliable, handling a wide variety of MusicXML files and formats with ease.
//!
//! # Installation
//!
//! To use the `musicxml` library in your Rust project, simply add the following line to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! musicxml = "1.0"
//! ```
//!
//! # Parsing MusicXML Files
//!
//! The recommended way to parse a MusicXML file is to use the [read_score_partwise] and [read_score_timewise] functions.
//! The difference between these two functions is based on the internal representation of the MusicXML data that you would
//! like to work with. The [read_score_partwise] function returns a [ScorePartwise] object, which is a representation in
//! which the score is divided into "parts," typically corresponding to either individual musical instruments or even
//! individual hands in multi-hand instruments. The [read_score_timewise] function returns a [ScoreTimewise] object,
//! in which the representation is divided into "measures" instead of "parts."
//!
//! The data encoded in either of these representations is identical, but the way in which the data is structured is different.
//! The primary difference is that in the [ScorePartwise] representation, the top-level data structure is a list of parts,
//! each of which contain a list of measures containing the actual notes to be played by the enclosing part. In the
//! [ScoreTimewise] representation, the top-level data structure is a list of measures, each of which contain a list of parts
//! indicating what each part is supposed to play in the enclosing measure.
//!
//! Transitioning between these two representations is handled internally and transparently to the user, meaning that
//! regardless of how the original MusicXML file was encoded, you can parse the data using whichever representation you prefer,
//! simply by calling the appropriate function. To read a MusicXML file and get a [ScorePartwise] object, simply do the
//! following:
//!
//! ```no_run
//! use musicxml::read_score_partwise;
//!
//! match read_score_partwise("path/to/file.musicxml") {
//!   Ok(score) => {}, // Do something with the score
//!   Err(e) => println!("Error reading MusicXML file: {}", e),
//! }
//! ```
//!
//! Likewise, to read a MusicXML file and get a [ScoreTimewise] object, you can do the following:
//!
//! ```no_run
//! use musicxml::read_score_timewise;
//!
//! match read_score_timewise("path/to/file.musicxml") {
//!   Ok(score) => {}, // Do something with the score
//!   Err(e) => println!("Error reading MusicXML file: {}", e),
//! }
//! ```
//!
//! Note that this library is able to read both `.musicxml` files and compressed `.mxl` files. The file type being read is
//! determined internally, and decoding will be handled automatically when calling either of the above functions.
//!
//! # Writing MusicXML Files
//!
//! Assuming you have a [ScorePartwise] or [ScoreTimewise] object that you would like to write to a MusicXML file, you can
//! use the [write_partwise_score] and [write_timewise_score] functions to do so. The difference between these two functions
//! is similar to the difference between the [read_score_partwise] and [read_score_timewise] functions, in that the former
//! writes a [ScorePartwise] object to a file, while the latter writes a [ScoreTimewise] object.
//!
//! As with reading MusicXML files, the representation in which you would like to store the output file can be specified as
//! a parameter to either of these functions to allow you to explicitly convert from your original representation to the
//! alternate representation when storing. Additionally, a `compressed` parameter exists to indicate whether the output file
//! should be compressed as an `.mxl` file or stored as a plain `.musicxml` file.
//!
//! To write a [ScorePartwise] object to a standard MusicXML file in its default representation, you can do the following:
//!
//! ```ignore
//! use musicxml::write_partwise_score;
//!
//! // Assume ScorePartwise "score" object created elsewhere
//! match write_partwise_score("path/to/output.musicxml", &score, false, false) {
//!   Ok(_) => println!("Successfully wrote MusicXML file"),
//!   Err(e) => println!("Error writing MusicXML file: {}", e),
//! }
//! ```
//!
//! Alternately, to write a [ScoreTimewise] object to a compressed MusicXML file using a partwise representation, you can do the following:
//!
//! ```ignore
//! use musicxml::write_timewise_score;
//!
//! // Assume ScoreTimewise "score" object created elsewhere
//! match write_timewise_score("path/to/output.musicxml", &score, true, true) {
//!   Ok(_) => println!("Successfully wrote compressed MusicXML file"),
//!   Err(e) => println!("Error writing compressed MusicXML file: {}", e),
//! }
//! ```

/// Contains the main data types used by the MusicXML format.
///
/// Note that these data types correspond to the textual contents of an XML tag. For example, the XML string
/// `<element>123</element>` might specify that the `123` value be of data type `UnsignedInteger`.
/// This module defines all the various data types that a MusicXML text field can have.
pub mod datatypes;

/// Contains the main elements used in the MusicXML format.
///
/// Note that these elements correspond to the XML tags themselves and are usually containers for other
/// MusicXML elements or data types. For example, take the following contrived MusicXML string:
///
/// ```xml
/// <note>
///   <color>#FF0000</color>
///   <type>quarter</type>
///   <duration>1</duration>
/// </note>
/// ```
///
/// The `note` element is a container for the `color`, `type`, and `duration` elements, which in turn contain
/// data types such as `Color`, `NoteType`, and `PositiveInteger`, respectively.
///
/// This module defines all the various container elements that a MusicXML file can have.
pub mod elements;

/// Contains functions for parsing and writing MusicXML files.
///
/// It is recommended that the top-level [MusicXML][crate] functions be used to read and write MusicXML files;
/// however, the functions in this module can be used directly for more fine-grained control over the parsing
/// and writing process.
pub mod parser;

use elements::{ScorePartwise, ScoreTimewise};

/// Reads a MusicXML file and returns a [ScorePartwise] object.
///
/// The specified file can be either a `.musicxml` file or a compressed `.mxl` file.
pub fn read_score_partwise(path: &str) -> Result<ScorePartwise, String> {
  parser::parse_score_partwise_from_file(path)
}

/// Reads a MusicXML file and returns a [ScoreTimewise] object.
///
/// The specified file can be either a `.musicxml` file or a compressed `.mxl` file.
pub fn read_score_timewise(path: &str) -> Result<ScoreTimewise, String> {
  parser::parse_score_timewise_from_file(path)
}

/// Writes a [ScorePartwise] object into a MusicXML file.
///
/// If the `compressed` parameter is set to `true`, the MusicXML file will be written as a compressed `.mxl` file.
/// If the `write_as_timewise` parameter is set to `true`, the MusicXML file will be converted into a timewise format and
/// written as a `<score-timewise>` element.
pub fn write_partwise_score(
  path: &str,
  score: &ScorePartwise,
  compressed: bool,
  write_as_timewise: bool,
) -> Result<(), String> {
  parser::parse_score_partwise_to_file(path, score, compressed, true, write_as_timewise)
}

/// Writes a [ScoreTimewise] object into a MusicXML file.
///
/// If the `compressed` parameter is set to `true`, the MusicXML file will be written as a compressed `.mxl` file.
/// If the `write_as_partwise` parameter is set to `true`, the MusicXML file will be converted into a partwise format and
/// written as a `<score-partwise>` element.
pub fn write_timewise_score(
  path: &str,
  score: &ScoreTimewise,
  compressed: bool,
  write_as_partwise: bool,
) -> Result<(), String> {
  parser::parse_score_timewise_to_file(path, score, compressed, true, write_as_partwise)
}
