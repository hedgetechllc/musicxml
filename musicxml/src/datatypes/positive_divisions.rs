use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use std::ops::Deref;

/// Represents how many integer divisions per quarter note represent a note duration.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
/// 
/// **Minimum value**: 1
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct PositiveDivisions(pub u32);

impl Deref for PositiveDivisions {
  type Target = u32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for PositiveDivisions {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u32>() {
      Ok(val) => match val {
        1.. => Ok(PositiveDivisions(val)),
        _ => Err(format!("Value {val} is invalid for the <positive-divisions> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <positive-divisions>", value)),
    }
  }
}
#[cfg(test)]
mod positive_divisions_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = PositiveDivisions::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PositiveDivisions(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = PositiveDivisions::deserialize("32343");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PositiveDivisions(32343));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = PositiveDivisions::deserialize("a");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = PositiveDivisions::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = PositiveDivisions::deserialize("2d");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid4() {
    let result = PositiveDivisions::deserialize("-3234");
    assert!(result.is_err());
  }
}
