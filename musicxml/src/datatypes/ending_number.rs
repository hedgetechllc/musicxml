use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;
use regex::Regex;
use std::ops::Deref;

/// Used to specify either a comma-separated list of positive integers without leading zeros, or a string of zero or more spaces.
/// 
/// It is used for the `number` attribute of the [Ending][crate::elements::Ending] element.
/// 
/// The "zero or more spaces" version is used when software knows that an ending is present, but cannot determine the type of the ending.
/// 
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct EndingNumber(pub String);

impl Deref for EndingNumber {
  type Target = String;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for EndingNumber {
  fn deserialize(value: &str) -> Result<Self, String> {
    let space_regex = Regex::new(r"^[ ]*$").unwrap();
    let integer_regex = Regex::new(r"^[1-9][0-9]*(, ?[1-9][0-9]*)*$").unwrap();
    if space_regex.is_match(value) {
      Ok(EndingNumber(String::from(value)))
    } else if integer_regex.is_match(value) {
      Ok(EndingNumber(value.split(' ').collect::<Vec<_>>().join("")))
    } else {
      Err(format!("Value {} is invalid for the <ending-number> data type", value))
    }
  }
}

#[cfg(test)]
mod ending_number_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = EndingNumber::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), EndingNumber(String::from("1")));
  }

  #[test]
  fn deserialize_valid2() {
    let result = EndingNumber::deserialize("33");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), EndingNumber(String::from("33")));
  }

  #[test]
  fn deserialize_valid3() {
    let result = EndingNumber::deserialize("348,21");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), EndingNumber(String::from("348,21")));
  }

  #[test]
  fn deserialize_valid4() {
    let result = EndingNumber::deserialize("9, 23, 34,34");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), EndingNumber(String::from("9,23,34,34")));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = EndingNumber::deserialize("02");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = EndingNumber::deserialize("4,");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = EndingNumber::deserialize("a");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid4() {
    let result = EndingNumber::deserialize(",33");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid5() {
    let result = EndingNumber::deserialize("23,33,  2");
    assert!(result.is_err());
  }
}
