// TODO: #![deny(missing_docs)]
#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

#![doc(html_favicon_url = "favicon.ico")]
#![doc(html_logo_url = "logo.jpg")]
#![doc(html_playground_url = "https://playground.example.com/")]

//! TODO: Documentation of entire crate goes here.
//!
//! Description, etc.

/// This module contains the main data types used in the MusicXML format.
/// 
/// Note that these data types correspond to the textual contents of an XML tag. For example, the XML string
/// `<element>123</element>` might specify that the `123` value be of data type `UnsignedInteger`.
/// This module defines all the various data types that a MusicXML text field can have.
pub mod datatypes;

/// This module contains the main elements used in the MusicXML format.
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

/// Information about parser module
pub mod parser;

use elements::{ScorePartwise, ScoreTimewise};

pub fn read_score_partwise(path: &str) -> Result<ScorePartwise, String> {
  parser::parse_from_xml_file::<ScorePartwise>(path)
  // TODO: Auto-convert if wrong type
}

pub fn read_score_timewise(path: &str) -> Result<ScoreTimewise, String> {
  parser::parse_from_xml_file::<ScoreTimewise>(path)
  // TODO: Auto-convert if wrong type
}

pub fn write_score_partwise(score: &ScorePartwise, path: &str) -> Result<(), String> {
  // TODO: parser::parse_to_xml_file(path, score, true)
  // TODO: Auto-convert if wrong type
  Err("Not implemented".to_string())
}

pub fn write_score_timewise(score: &ScoreTimewise, path: &str) -> Result<(), String> {
  // TODO: parser::parse_to_xml_file(path, score, true)
  // TODO: Auto-convert if wrong type
  Err("Not implemented".to_string())
}
