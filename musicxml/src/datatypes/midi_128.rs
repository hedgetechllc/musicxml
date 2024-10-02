use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Used to express MIDI 1.0 values that range from 1 to 128.
///
/// MusicXML uses 1-based numbers rather than 0-based numbers often found in MIDI 1.0 documentation.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
///
/// **Minimum Value**: 1
///
/// **Maximum Value**: 128
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct Midi128(pub u8);

impl Deref for Midi128 {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for Midi128 {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        1..=128 => Ok(Midi128(val)),
        _ => Err(format!("Value {val} is invalid for the <midi-128> data type")),
      },
      Err(_) => Err(format!("Invalid value {value} for <midi-128>")),
    }
  }
}

#[cfg(test)]
mod midi128_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Midi128::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Midi128(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Midi128::deserialize("128");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Midi128(128));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Midi128::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Midi128::deserialize("129");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Midi128::deserialize("-8");
    assert!(result.is_err());
  }
}
