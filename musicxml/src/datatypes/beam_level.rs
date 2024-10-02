use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Supports six levels of beaming, up to 1024th notes.
///
/// Unlike the [NumberLevel][super::NumberLevel] type, the [BeamLevel] type identifies concurrent beams in a beam group.
/// It does not distinguish overlapping beams, such as grace notes within regular notes, or beams used in different voices.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct BeamLevel(pub u8);

impl Deref for BeamLevel {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for BeamLevel {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        1..=8 => Ok(BeamLevel(val)),
        _ => Err(format!("Value {val} is invalid for the <beam-level> data type")),
      },
      Err(_) => Err(format!("Invalid value {value} for <beam-level>")),
    }
  }
}

#[cfg(test)]
mod beam_level_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = BeamLevel::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BeamLevel(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = BeamLevel::deserialize("8");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), BeamLevel(8));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = BeamLevel::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = BeamLevel::deserialize("9");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = BeamLevel::deserialize("-1");
    assert!(result.is_err());
  }
}
