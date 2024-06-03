use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// See the definition in the [W3C XML Schema standard](https://www.w3.org/TR/xmlschema-2/#nonNegativeInteger).
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct NonNegativeInteger(pub u32);

impl Deref for NonNegativeInteger {
  type Target = u32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod non_negative_integer_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = NonNegativeInteger::deserialize("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NonNegativeInteger(0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = NonNegativeInteger::deserialize("234566");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NonNegativeInteger(234566));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = NonNegativeInteger::deserialize("-1");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = NonNegativeInteger::deserialize("0.2342");
    assert!(result.is_err());
  }
}
