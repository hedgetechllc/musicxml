use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use std::ops::Deref;

/// Used to specify the number of lines in text decoration attributes.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
/// 
/// **Minimum value**: 0
/// 
/// **Maximum value**: 3
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct NumberOfLines(pub u8);

impl Deref for NumberOfLines {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for NumberOfLines {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        0..=3 => Ok(NumberOfLines(val)),
        _ => Err(format!("Value {val} is invalid for the <number-of-lines> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <number-of-lines>", value)),
    }
  }
}

#[cfg(test)]
mod number_of_lines_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = NumberOfLines::deserialize("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumberOfLines(0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = NumberOfLines::deserialize("3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumberOfLines(3));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = NumberOfLines::deserialize("-1");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = NumberOfLines::deserialize("4");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = NumberOfLines::deserialize("");
    assert!(result.is_err());
  }
}
