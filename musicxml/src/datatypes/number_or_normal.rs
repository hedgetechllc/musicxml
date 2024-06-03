use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Can be either a decimal number or the string "normal".
/// 
/// This is used by the `line_height` and `letter_spacing` attributes.
#[derive(Debug, PartialEq)]
pub enum NumberOrNormal {
  /// Represents the string "normal".
  Normal,
  /// Represents a decimal number.
  Decimal(f64),
}

impl Eq for NumberOrNormal {}

impl DatatypeSerializer for NumberOrNormal {
  fn serialize(element: &Self) -> String {
    match element {
      Self::Normal => String::from("normal"),
      Self::Decimal(number) => number.to_string(),
    }
  }
}

impl DatatypeDeserializer for NumberOrNormal {
  fn deserialize(value: &str) -> Result<Self, String> {
    if let Ok(dec_val) = value.parse::<f64>() {
      Ok(NumberOrNormal::Decimal(dec_val))
    } else if value.to_lowercase() == "normal" {
      Ok(NumberOrNormal::Normal)
    } else {
      Err(format!("Value {} is invalid for the <number-or-normal> data type", value))
    }
  }
}

#[cfg(test)]
mod number_or_normal_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = NumberOrNormal::deserialize("-1.2");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumberOrNormal::Decimal(-1.2));
  }

  #[test]
  fn deserialize_valid2() {
    let result = NumberOrNormal::deserialize("6");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumberOrNormal::Decimal(6.0));
  }

  #[test]
  fn deserialize_valid3() {
    let result = NumberOrNormal::deserialize("Normal");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumberOrNormal::Normal);
  }

  #[test]
  fn deserialize_valid4() {
    let result = NumberOrNormal::deserialize("normal");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NumberOrNormal::Normal);
  }

  #[test]
  fn deserialize_invalid1() {
    let result = NumberOrNormal::deserialize("sdfs");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = NumberOrNormal::deserialize("");
    assert!(result.is_err());
  }
}
