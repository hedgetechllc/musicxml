use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// See the definition in the [W3C XML Schema standard](https://www.w3.org/TR/xmlschema-2/#integer).
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct Integer(pub i32);

impl Deref for Integer {
  type Target = i32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod integer_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Integer::deserialize("2342");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Integer(2342));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Integer::deserialize("-234234");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Integer(-234234));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Integer::deserialize("0.234");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Integer::deserialize("1.23432");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Integer::deserialize("dsd");
    assert!(result.is_err());
  }
}
