use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// See the definition in the [W3C XML Schema standard](https://www.w3.org/TR/xmlschema-2/#positiveInteger).
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct PositiveInteger(pub u32);

impl Deref for PositiveInteger {
  type Target = u32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for PositiveInteger {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u32>() {
      Ok(val) => match val {
        1.. => Ok(PositiveInteger(val)),
        _ => Err(format!("Value {val} is invalid for the <positive-integer> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <positive-integer>", value)),
    }
  }
}

#[cfg(test)]
mod positive_integer_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = PositiveInteger::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PositiveInteger(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = PositiveInteger::deserialize("2345534234");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PositiveInteger(2345534234));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = PositiveInteger::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = PositiveInteger::deserialize("-23");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = PositiveInteger::deserialize("s");
    assert!(result.is_err());
  }
}
