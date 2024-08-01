use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// See the definition in the [W3C Extensible Markup Language recommendation](https://www.w3.org/TR/xml/#sec-white-space).
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum XmlSpace {
  /// See the definition in the [W3C Extensible Markup Language recommendation](https://www.w3.org/TR/xml/#sec-white-space).
  Default,
  /// See the definition in the [W3C Extensible Markup Language recommendation](https://www.w3.org/TR/xml/#sec-white-space).
  Preserve,
}
