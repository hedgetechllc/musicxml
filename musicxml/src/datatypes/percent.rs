use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use std::ops::Deref;

/// Specifies a percentage from 0 to 100.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
/// 
/// **Minimum value:** 0.0
/// 
/// **Maximum value:** 100.0
#[derive(Debug, PartialEq, DatatypeSerialize)]
pub struct Percent(pub f64);

impl Eq for Percent {}

impl Deref for Percent {
  type Target = f64;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for Percent {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<f64>() {
      Ok(val) => match val {
        x if x >= 0.0 && x <= 100.0 => Ok(Percent(val)),
        _ => Err(format!("Value {val} is invalid for the <percent> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <percent>", value)),
    }
  }
}

#[cfg(test)]
mod percent_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Percent::deserialize("0.0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Percent(0.0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Percent::deserialize("100");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Percent(100.0));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Percent::deserialize("-0.01");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Percent::deserialize("100.1");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Percent::deserialize("");
    assert!(result.is_err());
  }
}
