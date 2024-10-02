use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// See the definition in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#link-behaviors).
///
/// MusicXML only supports the simple type.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum XlinkType {
  /// See the definition in the [XML Linking Language recommendation](https://www.w3.org/TR/xlink11/#dt-simplelink).
  Simple,
}
