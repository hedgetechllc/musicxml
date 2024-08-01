use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to represent millimeters.
///
/// This is used in the [Scaling][crate::elements::Scaling] element to provide a default scaling from tenths to physical units.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, DatatypeDeserialize, DatatypeSerialize)]
pub struct Millimeters(pub f64);

impl Eq for Millimeters {}

impl Deref for Millimeters {
  type Target = f64;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod millimeters_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Millimeters::deserialize("-1234");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Millimeters(-1234.0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Millimeters::deserialize("0.23456");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Millimeters(0.23456));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Millimeters::deserialize("asd");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Millimeters::deserialize("");
    assert!(result.is_err());
  }
}
