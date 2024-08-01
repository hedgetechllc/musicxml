use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Specifies the beats used in a `trill-sound` or `bend-sound` attribute group.
///
/// It is a decimal value with a minimum value of 2.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
///
/// **Minimum value**: 2.0
#[derive(Debug, PartialEq, DatatypeSerialize)]
pub struct TrillBeats(pub f64);

impl Eq for TrillBeats {}

impl Deref for TrillBeats {
  type Target = f64;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for TrillBeats {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<f64>() {
      Ok(val) => match val {
        x if x >= 2.0 => Ok(TrillBeats(val)),
        _ => Err(format!("Value {val} is invalid for the <trill-beats> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <trill-beats>", value)),
    }
  }
}

#[cfg(test)]
mod trill_beats_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = TrillBeats::deserialize("2");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_valid2() {
    let result = TrillBeats::deserialize("3.4");
    assert!(result.is_ok());
  }

  #[test]
  fn deserialize_invalid1() {
    let result = TrillBeats::deserialize("1");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = TrillBeats::deserialize("-0.2");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = TrillBeats::deserialize("a");
    assert!(result.is_err());
  }
}
