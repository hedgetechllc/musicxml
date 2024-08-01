use super::positive_integer::PositiveInteger;
use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Indicates the line on a given staff.
///
/// Staff lines are numbered from bottom to top, with 1 being the bottom line on a staff.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct StaffLine(pub u8);

impl Deref for StaffLine {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for StaffLine {
  fn deserialize(value: &str) -> Result<Self, String> {
    match PositiveInteger::deserialize(value) {
      Ok(val) => match *val {
        1..=255 => Ok(StaffLine(*val as u8)),
        _ => Err(format!("Value {} is invalid for the <staff-line> data type", *val)),
      },
      Err(_) => Err(format!("Invalid value {} for <staff-line>", value)),
    }
  }
}

#[cfg(test)]
mod staff_line_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = StaffLine::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StaffLine(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = StaffLine::deserialize("255");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StaffLine(255));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = StaffLine::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = StaffLine::deserialize("12345");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = StaffLine::deserialize("-1");
    assert!(result.is_err());
  }
}
