use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Represents octaves by the numbers 0 to 9, where 4 indicates the octave started by middle C.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct Octave(pub u8);

impl Deref for Octave {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for Octave {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        0..=9 => Ok(Octave(val)),
        _ => Err(format!("Value {val} is invalid for the <octave> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <octave>", value)),
    }
  }
}

#[cfg(test)]
mod octave_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Octave::deserialize("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Octave(0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Octave::deserialize("9");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Octave(9));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Octave::deserialize("-1");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Octave::deserialize("10");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Octave::deserialize("");
    assert!(result.is_err());
  }
}
