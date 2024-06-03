use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// Indicates the line position on a given staff.
/// 
/// Staff lines are numbered from bottom to top, with 1 being the bottom line on a staff.
/// A [StaffLinePosition] value can extend beyond the range of the lines on the current staff.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct StaffLinePosition(pub i16);

impl Deref for StaffLinePosition {
  type Target = i16;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod staff_line_position_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = StaffLinePosition::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StaffLinePosition(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = StaffLinePosition::deserialize("-1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), StaffLinePosition(-1));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = StaffLinePosition::deserialize("0.23");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = StaffLinePosition::deserialize("");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = StaffLinePosition::deserialize("as");
    assert!(result.is_err());
  }
}
