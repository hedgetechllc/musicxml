use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// See the definition in the [W3C XML Schema standard](https://www.w3.org/TR/xmlschema-2/#anyURI).
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct AnyUri(pub String);

impl Deref for AnyUri {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod anyuri_tests {
  use super::*;

  #[test]
  fn serialize_valid1() {
    let test = AnyUri(String::from("http://test.url/with/path"));
    let result = AnyUri::serialize(&test);
    assert_eq!(result, "http://test.url/with/path");
  }

  #[test]
  fn deserialize_valid() {
    let result = AnyUri::deserialize("http://test.url/with/path");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), AnyUri(String::from("http://test.url/with/path")));
  }
}
