use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use std::ops::Deref;

/// Specifies rotation, pan, and elevation values in degrees.
/// 
/// Values range from -180 to 180.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
/// 
/// **Minimum value:** -180.0
/// 
/// **Maximum value:** 180.0
#[derive(Debug, PartialEq, DatatypeSerialize)]
pub struct RotationDegrees(pub f32);

impl Eq for RotationDegrees {}

impl Deref for RotationDegrees {
  type Target = f32;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for RotationDegrees {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<f32>() {
      Ok(val) => match val {
        x if x >= -180.0 && x <= 180.0 => Ok(RotationDegrees(val)),
        _ => Err(format!("Value {val} is invalid for the <rotation-degrees> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <rotation-degrees>", value)),
    }
  }
}

#[cfg(test)]
mod rotation_degrees_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = RotationDegrees::deserialize("-180.0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RotationDegrees(-180.0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = RotationDegrees::deserialize("180.0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), RotationDegrees(180.0));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = RotationDegrees::deserialize("-181");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = RotationDegrees::deserialize("182.2");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = RotationDegrees::deserialize("sd");
    assert!(result.is_err());
  }
}
