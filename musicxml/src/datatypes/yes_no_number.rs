use super::decimal::Decimal;
use alloc::string::{String, ToString};
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Used for attributes that can be either boolean or numeric values.
#[derive(Debug, PartialEq)]
pub enum YesNoNumber {
  /// The value is `yes`.
  Yes,
  /// The value is `no`.
  No,
  /// The value is a decimal number.
  Decimal(f32),
}

impl Eq for YesNoNumber {}

impl DatatypeSerializer for YesNoNumber {
  fn serialize(element: &Self) -> String {
    match element {
      Self::Yes => String::from("yes"),
      Self::No => String::from("no"),
      Self::Decimal(number) => number.to_string(),
    }
  }
}

impl DatatypeDeserializer for YesNoNumber {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(dec_val) = Decimal::deserialize(value) {
      Ok(YesNoNumber::Decimal(*dec_val as f32))
    } else if value.to_lowercase() == "yes" {
      Ok(YesNoNumber::Yes)
    } else if value.to_lowercase() == "no" {
      Ok(YesNoNumber::No)
    } else {
      Err(format!("Value {} is invalid for the <yes-no-number> data type", value))
    }
  }
}

#[cfg(test)]
mod yes_no_number_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = YesNoNumber::deserialize("yes");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), YesNoNumber::Yes);
  }

  #[test]
  fn deserialize_valid2() {
    let result = YesNoNumber::deserialize("No");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), YesNoNumber::No);
  }

  #[test]
  fn deserialize_valid3() {
    let result = YesNoNumber::deserialize("3.234");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), YesNoNumber::Decimal(3.234));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = YesNoNumber::deserialize("other");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = YesNoNumber::deserialize("true");
    assert!(result.is_err());
  }
}
