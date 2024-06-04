// TODO: #![deny(missing_docs)]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
// TODO: Fix icon and logo
#![doc(html_favicon_url = "favicon.ico")]
#![doc(html_logo_url = "logo.jpg")]
#![doc(html_playground_url = "https://playground.example.com/")]

//! TODO: Documentation of entire crate goes here.
//!
//! Description, etc.

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

pub fn read_score_partwise(path: &str) -> Result<ScorePartwise, String> {
  parser::parse_score_partwise_from_file(path)
}

pub fn read_score_timewise(path: &str) -> Result<ScoreTimewise, String> {
  parser::parse_score_timewise_from_file(path)
}

pub fn write_partwise_score(
  path: &str,
  score: &ScorePartwise,
  compressed: bool,
  write_as_timewise: bool,
) -> Result<(), String> {
  parser::parse_score_partwise_to_file(path, score, compressed, true, write_as_timewise)
}

pub fn write_timewise_score(
  path: &str,
  score: &ScoreTimewise,
  compressed: bool,
  write_as_partwise: bool,
) -> Result<(), String> {
  parser::parse_score_timewise_to_file(path, score, compressed, true, write_as_partwise)
}
