use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};
use std::ops::Deref;

/// Used to express values in terms of the musical divisions defined by the divisions element.
/// 
/// It is preferred that these be integer values both for MIDI interoperability and to avoid roundoff errors.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub struct Divisions(pub i32);

impl Deref for Divisions {
  type Target = i32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod divisions_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Divisions::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Divisions(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Divisions::deserialize("32343");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Divisions(32343));
  }

  #[test]
  fn deserialize_valid3() {
    let result = Divisions::deserialize("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Divisions(0));
  }

  #[test]
  fn deserialize_valid4() {
    let result = Divisions::deserialize("223234");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Divisions(223234));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Divisions::deserialize("a");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Divisions::deserialize("2d");
    assert!(result.is_err());
  }
}
