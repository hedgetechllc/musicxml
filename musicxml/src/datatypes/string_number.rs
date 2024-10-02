use super::positive_integer::PositiveInteger;
use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Indicates a string number.
///
/// Strings are numbered from high to low, with 1 being the highest pitched full-length string.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct StringNumber(pub u8);

impl Deref for StringNumber {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for StringNumber {
  fn deserialize(value: &str) -> Result<Self, String> {
    match PositiveInteger::deserialize(value) {
      Ok(val) => match *val {
        1..=255 => Ok(StringNumber(*val as u8)),
        _ => Err(format!("Value {} is invalid for the <string> data type", *val)),
      },
      Err(_) => Err(format!("Invalid value {value} for <string>")),
    }
  }
}

#[cfg(test)]
mod string_number_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = StringNumber::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StringNumber(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = StringNumber::deserialize("255");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StringNumber(255));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = StringNumber::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = StringNumber::deserialize("12345");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = StringNumber::deserialize("-1");
    assert!(result.is_err());
  }
}
