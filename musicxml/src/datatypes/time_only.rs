use super::positive_integer::PositiveInteger;
use alloc::{string::{String, ToString}, vec::Vec};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Used to indicate that a particular playback- or listening-related element only applies particular times
/// through a repeated section.
///
/// The value is a comma-separated list of positive integers arranged in ascending order, indicating which times
/// through the repeated section that the element applies.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, Eq)]
pub struct TimeOnly(pub Vec<u8>);

impl Deref for TimeOnly {
  type Target = Vec<u8>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeSerializer for TimeOnly {
  fn serialize(element: &Self) -> String {
    element
      .0
      .iter()
      .map(ToString::to_string)
      .collect::<Vec<_>>()
      .join(",")
  }
}

impl DatatypeDeserializer for TimeOnly {
  fn deserialize(value: &str) -> Result<Self, String> {
    let mut errs = false;
    let mut res: Vec<u8> = Vec::new();
    value.split(',').for_each(|item| {
      if let Ok(token) = PositiveInteger::deserialize(item) {
        match *token {
          #[allow(clippy::cast_possible_truncation)]
          1..=255 => res.push(*token as u8),
          _ => errs = true,
        }
      } else {
        errs = true;
      }
    });
    if errs {
      Err(format!("Value {value} is invalid for the <time-only> data type"))
    } else {
      Ok(TimeOnly(res))
    }
  }
}

#[cfg(test)]
mod time_only_tests {
  use super::*;

  #[test]
  fn serialize_valid1() {
    let test = TimeOnly(vec![1, 2, 4]);
    let result = TimeOnly::serialize(&test);
    assert_eq!(result, "1,2,4");
  }

  #[test]
  fn deserialize_valid1() {
    let result = TimeOnly::deserialize("1,2,3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TimeOnly(vec![1, 2, 3]));
  }

  #[test]
  fn deserialize_valid2() {
    let result = TimeOnly::deserialize("3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TimeOnly(vec![3]));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = TimeOnly::deserialize(",1,2,");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = TimeOnly::deserialize("0,3234");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = TimeOnly::deserialize("0,1");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid4() {
    let result = TimeOnly::deserialize("2, 4 ");
    assert!(result.is_err());
  }
}
