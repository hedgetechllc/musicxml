use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use regex::Regex;

/// Represents dates in the yyyy-mm-dd format, following ISO 8601.
///
/// This is a W3C XML Schema date type, but without the optional timezone data.
#[derive(Debug, PartialEq, Eq)]
pub struct YyyyMmDd {
  /// The year.
  pub year: u16,
  /// The month.
  pub month: u8,
  /// The date.
  pub date: u8,
}

impl YyyyMmDd {
  /// Creates a new `YyyyMmDd` instance.
  #[must_use]
  pub fn new(year: u16, month: u8, date: u8) -> Self {
    YyyyMmDd { year, month, date }
  }
}

impl core::fmt::Display for YyyyMmDd {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "{:02}-{:02}-{:02})", self.year, self.month, self.date)
  }
}

impl DatatypeSerializer for YyyyMmDd {
  fn serialize(element: &Self) -> String {
    format!("{:04}-{:02}-{:02}", element.year, element.month, element.date)
  }
}

impl DatatypeDeserializer for YyyyMmDd {
  fn deserialize(value: &str) -> Result<Self, String> {
    let regex = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})$").unwrap();
    if let Some(captures) = regex.captures(value) {
      let date = YyyyMmDd::new(
        captures.get(1).unwrap().as_str().parse::<u16>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
        captures.get(3).unwrap().as_str().parse::<u8>().unwrap(),
      );
      if date.month > 0 && date.month < 13 && date.date > 0 && date.date < 32 {
        Ok(date)
      } else {
        Err(format!("Value {value} is invalid for the <yyyy-mm-dd> data type"))
      }
    } else {
      Err(format!("Value {value} is invalid for the <yyyy-mm-dd> data type"))
    }
  }
}

#[cfg(test)]
mod yyyy_mm_dd_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = YyyyMmDd::deserialize("2024-01-23");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      YyyyMmDd {
        year: 2024,
        month: 01,
        date: 23
      }
    );
  }

  #[test]
  fn deserialize_valid2() {
    let result = YyyyMmDd::deserialize("1971-12-01");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      YyyyMmDd {
        year: 1971,
        month: 12,
        date: 01
      }
    );
  }

  #[test]
  fn deserialize_invalid1() {
    let result = YyyyMmDd::deserialize("2024");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = YyyyMmDd::deserialize("2024-1-3");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = YyyyMmDd::deserialize("2024-13-01");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid4() {
    let result = YyyyMmDd::deserialize("2024-12-41");
    assert!(result.is_err());
  }
}
