use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use std::ops::Deref;

/// Used to express MIDI 1.0 values that range from 1 to 16.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
/// 
/// **Minimum Value**: 1
/// 
/// **Maximum Value**: 16
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct Midi16(pub u8);

impl Deref for Midi16 {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for Midi16 {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        1..=16 => Ok(Midi16(val)),
        _ => Err(format!("Value {val} is invalid for the <midi-16> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <midi-16>", value)),
    }
  }
}

#[cfg(test)]
mod midi16_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Midi16::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Midi16(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Midi16::deserialize("16");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Midi16(16));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Midi16::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Midi16::deserialize("17");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Midi16::deserialize("-2");
    assert!(result.is_err());
  }
}
