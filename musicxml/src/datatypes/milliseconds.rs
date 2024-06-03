use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// Represents an integral number of milliseconds.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct Milliseconds(pub u32);

impl Deref for Milliseconds {
  type Target = u32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod milliseconds_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Milliseconds::deserialize("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Milliseconds(0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Milliseconds::deserialize("23423");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Milliseconds(23423));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Milliseconds::deserialize("-2343");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Milliseconds::deserialize("0.23");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Milliseconds::deserialize("fds");
    assert!(result.is_err());
  }
}
