use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Represents the number of tremolo marks by a number from 0 to 8.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
///
/// **Minimum value:** 0
///
/// **Maximum value:** 8
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct TremoloMarks(pub u8);

impl Deref for TremoloMarks {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for TremoloMarks {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        0..=8 => Ok(TremoloMarks(val)),
        _ => Err(format!("Value {val} is invalid for the <tremolo-marks> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <tremolo-marks>", value)),
    }
  }
}

#[cfg(test)]
mod tremolo_marks_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = TremoloMarks::deserialize("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TremoloMarks(0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = TremoloMarks::deserialize("8");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TremoloMarks(8));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = TremoloMarks::deserialize("-1");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = TremoloMarks::deserialize("9");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = TremoloMarks::deserialize("a");
    assert!(result.is_err());
  }
}
