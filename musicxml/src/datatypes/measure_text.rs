use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use std::ops::Deref;

/// Used for the `text` attribute of [Measure][crate::elements::Measure] elements.
/// 
/// It must have at least one character. The `implicit` attribute of the [Measure][crate::elements::Measure]
/// element should be set to "yes" rather than setting the `text` attribute to an empty string.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
/// 
/// **Minimum Length**: 1
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct MeasureText(pub String);

impl Deref for MeasureText {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for MeasureText {
  fn deserialize(value: &str) -> Result<Self, String> {
    if value.len() > 0 {
      Ok(MeasureText(String::from(value)))
    } else {
      Err(format!("Value {} is invalid for the <measure-text> data type", value))
    }
  }
}

#[cfg(test)]
mod measure_text_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = MeasureText::deserialize("1234");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), MeasureText(String::from("1234")));
  }

  #[test]
  fn deserialize_valid2() {
    let result = MeasureText::deserialize("test");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), MeasureText(String::from("test")));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = MeasureText::deserialize("");
    assert!(result.is_err());
  }
}
