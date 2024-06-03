use super::positive_integer::PositiveInteger;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use std::ops::Deref;

/// Indicates staff numbers within a multi-staff part.
/// 
/// Staves are numbered from top to bottom, with 1 being the top staff on a part.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct StaffNumber(pub u8);

impl Deref for StaffNumber {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for StaffNumber {
  fn deserialize(value: &str) -> Result<Self, String> {
    match PositiveInteger::deserialize(value) {
      Ok(val) => match *val {
        1..=255 => Ok(StaffNumber(*val as u8)),
        _ => Err(format!("Value {} is invalid for the <staff-number> data type", *val)),
      },
      Err(_) => Err(format!("Invalid value {} for <staff-number>", value)),
    }
  }
}

#[cfg(test)]
mod staff_number_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = StaffNumber::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StaffNumber(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = StaffNumber::deserialize("255");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StaffNumber(255));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = StaffNumber::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = StaffNumber::deserialize("12345");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = StaffNumber::deserialize("-1");
    assert!(result.is_err());
  }
}
