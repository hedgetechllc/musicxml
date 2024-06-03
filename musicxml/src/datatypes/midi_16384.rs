use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use std::ops::Deref;

/// Used to express MIDI 1.0 values that range from 1 to 16,384.
/// 
/// MusicXML uses 1-based numbers rather than 0-based numbers often found in MIDI 1.0 documentation.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
/// 
/// **Minimum Value**: 1
/// 
/// **Maximum Value**: 16384
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct Midi16384(pub u16);

impl Deref for Midi16384 {
  type Target = u16;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for Midi16384 {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u16>() {
      Ok(val) => match val {
        1..=16384 => Ok(Midi16384(val)),
        _ => Err(format!("Value {val} is invalid for the <midi-16384> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <midi-16384>", value)),
    }
  }
}

#[cfg(test)]
mod midi16384_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Midi16384::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Midi16384(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Midi16384::deserialize("16384");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Midi16384(16384));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Midi16384::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Midi16384::deserialize("16385");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Midi16384::deserialize("-10");
    assert!(result.is_err());
  }
}
