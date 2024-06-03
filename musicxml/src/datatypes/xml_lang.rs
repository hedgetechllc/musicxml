use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// See the definition in the [W3C Extensible Markup Language recommendation](https://www.w3.org/TR/xml/#sec-lang-tag).
///
/// Language names come from ISO 639, with optional country subcodes from ISO 3166.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct XmlLang(pub String);

impl Deref for XmlLang {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
