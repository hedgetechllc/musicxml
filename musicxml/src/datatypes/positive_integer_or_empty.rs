use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Represents a positive integer or an empty value.
#[derive(Debug, PartialEq, Eq)]
pub enum PositiveIntegerOrEmpty {
  /// Represents an empty value.
  Empty,
  /// Represents a positive integer.
  Integer(u32),
}

impl DatatypeSerializer for PositiveIntegerOrEmpty {
  fn serialize(element: &Self) -> String {
    match element {
      Self::Empty => String::new(),
      Self::Integer(number) => number.to_string(),
    }
  }
}

impl DatatypeDeserializer for PositiveIntegerOrEmpty {
  fn deserialize(value: &str) -> Result<Self, String> {
    if value.is_empty() {
      Ok(PositiveIntegerOrEmpty::Empty)
    } else {
      match value.parse::<u32>() {
        Ok(val) => match val {
          1.. => Ok(PositiveIntegerOrEmpty::Integer(val)),
          _ => Err(format!(
            "Value {val} is invalid for the <positive-integer-or-empty> data type"
          )),
        },
        Err(_) => Err(format!(
          "Value {} is invalid for the <positive-integer-or-empty> data type",
          value
        )),
      }
    }
  }
}

#[cfg(test)]
mod positive_integer_or_empty_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = PositiveIntegerOrEmpty::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PositiveIntegerOrEmpty::Integer(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = PositiveIntegerOrEmpty::deserialize("");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PositiveIntegerOrEmpty::Empty);
  }

  #[test]
  fn deserialize_invalid1() {
    let result = PositiveIntegerOrEmpty::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = PositiveIntegerOrEmpty::deserialize("-134");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = PositiveIntegerOrEmpty::deserialize("abc");
    assert!(result.is_err());
  }
}
