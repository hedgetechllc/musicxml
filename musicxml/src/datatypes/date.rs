use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use regex::Regex;

/// See the definition in the [W3C XML Schema standard](https://www.w3.org/TR/xmlschema-2/#date).
#[derive(Debug, PartialEq, Eq)]
pub struct Date {
  /// The year of the date.
  pub year: u16,
  /// The month of the date.
  pub month: u8,
  /// The day of the date.
  pub date: u8,
  /// The offset hours of the timezone from GMT.
  pub timezone_hours: i8,
  /// The offset minutes of the timezone from GMT.
  pub timezone_minutes: u8,
}

impl Date {
  /// Creates a new [Date] with the given year, month, day, and timezone offset.
  #[must_use]
  pub fn new(year: u16, month: u8, date: u8, timezone_hours: i8, timezone_minutes: u8) -> Self {
    Date {
      year,
      month,
      date,
      timezone_hours,
      timezone_minutes,
    }
  }
}

impl core::fmt::Display for Date {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "{:04}-{:02}-{:02}{:+03}:{:02}",
      self.year, self.month, self.date, self.timezone_hours, self.timezone_minutes
    )
  }
}

impl DatatypeSerializer for Date {
  fn serialize(element: &Self) -> String {
    format!(
      "{:04}-{:02}-{:02}{:+03}:{:02}",
      element.year, element.month, element.date, element.timezone_hours, element.timezone_minutes
    )
  }
}

impl DatatypeDeserializer for Date {
  fn deserialize(value: &str) -> Result<Self, String> {
    let regex = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})[T|\ ]?((([\+\-][0-2][0-9]):([0-5][0-9]))|Z)?").unwrap();
    if let Some(captures) = regex.captures(value) {
      let date = if let Some(capture) = captures.get(6) {
        Date::new(
          captures.get(1).unwrap().as_str().parse().unwrap(),
          captures.get(2).unwrap().as_str().parse().unwrap(),
          captures.get(3).unwrap().as_str().parse().unwrap(),
          capture.as_str().parse().unwrap(),
          captures.get(7).unwrap().as_str().parse().unwrap(),
        )
      } else {
        Date::new(
          captures.get(1).unwrap().as_str().parse().unwrap(),
          captures.get(2).unwrap().as_str().parse().unwrap(),
          captures.get(3).unwrap().as_str().parse().unwrap(),
          0,
          0,
        )
      };
      if date.month > 0
        && date.month < 13
        && date.date > 0
        && date.date < 32
        && date.timezone_hours > -24
        && date.timezone_hours < 24
        && date.timezone_minutes < 61
      {
        Ok(date)
      } else {
        Err(format!("Value {value} is invalid for the <date> data type"))
      }
    } else {
      Err(format!("Value {value} is invalid for the <date> data type"))
    }
  }
}

#[cfg(test)]
mod date_tests {
  use super::*;

  #[test]
  fn serialize_valid1() {
    let test = Date {
      year: 2024,
      month: 01,
      date: 23,
      timezone_hours: -2,
      timezone_minutes: 30,
    };
    let result = Date::serialize(&test);
    assert_eq!(result, "2024-01-23-02:30");
  }

  #[test]
  fn deserialize_valid1() {
    let result = Date::deserialize("2024-01-23 -02:30");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Date {
        year: 2024,
        month: 01,
        date: 23,
        timezone_hours: -2,
        timezone_minutes: 30
      }
    );
  }

  #[test]
  fn deserialize_valid2() {
    let result = Date::deserialize("1971-12-01T+03:00");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Date {
        year: 1971,
        month: 12,
        date: 01,
        timezone_hours: 3,
        timezone_minutes: 0
      }
    );
  }

  #[test]
  fn deserialize_valid3() {
    let result = Date::deserialize("1971-12-01Z");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Date {
        year: 1971,
        month: 12,
        date: 01,
        timezone_hours: 0,
        timezone_minutes: 0
      }
    );
  }

  #[test]
  fn deserialize_valid4() {
    let result = Date::deserialize("2100-09-02 Z");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Date {
        year: 2100,
        month: 09,
        date: 02,
        timezone_hours: 0,
        timezone_minutes: 0
      }
    );
  }

  #[test]
  fn deserialize_valid5() {
    let result = Date::deserialize("1971-12-01TZ");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Date {
        year: 1971,
        month: 12,
        date: 01,
        timezone_hours: 0,
        timezone_minutes: 0
      }
    );
  }
  #[test]
  fn deserialize_invalid1() {
    let result = Date::deserialize("2024");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Date::deserialize("2024-1-3");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Date::deserialize("2024-13-01");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid4() {
    let result = Date::deserialize("2024-12-41");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid5() {
    let result = Date::deserialize("2024-12-41 02:00");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid6() {
    let result = Date::deserialize("2024-12-41T2:30");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid7() {
    let result = Date::deserialize("2024-12-41 -2:00");
    assert!(result.is_err());
  }
}
