use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Represents a Roman numeral or Nashville number value as a positive integer from 1 to 7.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
///
/// **Minimum value**: 1
///
/// **Maximum value**: 7
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct NumeralValue(pub u8);

impl Deref for NumeralValue {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for NumeralValue {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        1..=7 => Ok(NumeralValue(val)),
        _ => Err(format!("Value {val} is invalid for the <numeral-value> data type")),
      },
      Err(_) => Err(format!("Invalid value {value} for <numeral-value>")),
    }
  }
}

#[cfg(test)]
mod numeral_value_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = NumeralValue::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumeralValue(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = NumeralValue::deserialize("7");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumeralValue(7));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = NumeralValue::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = NumeralValue::deserialize("8");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = NumeralValue::deserialize("-2");
    assert!(result.is_err());
  }
}
