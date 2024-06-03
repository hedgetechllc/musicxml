use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// See the definition in the [W3C XML Schema standard](https://www.w3.org/TR/xmlschema-2/#ID).
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct Id(pub String);

impl Deref for Id {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
