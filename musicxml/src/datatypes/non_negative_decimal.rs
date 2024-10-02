use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// Specifies a non-negative decimal value.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
///
/// **Minimum value:** 0.0
#[derive(Debug, PartialEq, DatatypeSerialize)]
pub struct NonNegativeDecimal(pub f64);

impl Eq for NonNegativeDecimal {}

impl Deref for NonNegativeDecimal {
  type Target = f64;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for NonNegativeDecimal {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<f64>() {
      Ok(val) => match val {
        x if x >= 0.0 => Ok(NonNegativeDecimal(val)),
        _ => Err(format!("Value {val} is invalid for the NonNegativeDecimal data type")),
      },
      Err(_) => Err(format!("Invalid value {value} for NonNegativeDecimal")),
    }
  }
}

#[cfg(test)]
mod non_negative_decimal_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = NonNegativeDecimal::deserialize("0.0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NonNegativeDecimal(0.0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = NonNegativeDecimal::deserialize("123.456");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NonNegativeDecimal(123.456));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = NonNegativeDecimal::deserialize("-0.01");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = NonNegativeDecimal::deserialize("-4");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = NonNegativeDecimal::deserialize("as");
    assert!(result.is_err());
  }
}
