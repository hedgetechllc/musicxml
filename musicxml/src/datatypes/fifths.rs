use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Represents the number of flats or sharps in a traditional key signature.
///
/// Negative numbers are used for flats and positive numbers for sharps, reflecting the
/// key's placement within the circle of fifths (hence the type name).
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct Fifths(pub i8);

impl Deref for Fifths {
  type Target = i8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for Fifths {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<i8>() {
      Ok(val) => match val {
        -7..=7 => Ok(Fifths(val)),
        _ => Err(format!("Value {value} is invalid for the <fifths> data type")),
      },
      _ => Err(format!("Value {value} is invalid for the <fifths> data type")),
    }
  }
}

#[cfg(test)]
mod fifths_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Fifths::deserialize("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Fifths(0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Fifths::deserialize("-7");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Fifths(-7));
  }

  #[test]
  fn deserialize_valid3() {
    let result = Fifths::deserialize("7");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Fifths(7));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Fifths::deserialize("-9");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Fifths::deserialize("9");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Fifths::deserialize("2,");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid4() {
    let result = Fifths::deserialize("a");
    assert!(result.is_err());
  }
}
