use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::DatatypeSerialize;

/// May have values of 1, 2, or 3, corresponding to having 1 to 3 dots in the middle section of the accordion registration symbol.
///
/// This type is not used if no dots are present.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
///
/// **Minimum allowed value:** 1
///
/// **Maximum allowed value:** 3
#[derive(Debug, PartialEq, Eq, DatatypeSerialize)]
pub struct AccordionMiddle(pub u8);

impl Deref for AccordionMiddle {
  type Target = u8;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DatatypeDeserializer for AccordionMiddle {
  fn deserialize(value: &str) -> Result<Self, String> {
    match value.parse::<u8>() {
      Ok(val) => match val {
        1..=3 => Ok(AccordionMiddle(val)),
        _ => Err(format!("Value {val} is invalid for the <accordion-middle> data type")),
      },
      Err(_) => Err(format!("Invalid value {} for <accordion-middle>", value)),
    }
  }
}

#[cfg(test)]
mod accordion_middle_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = AccordionMiddle::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), AccordionMiddle(1));
  }

  #[test]
  fn deserialize_valid2() {
    let result = AccordionMiddle::deserialize("3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), AccordionMiddle(3));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = AccordionMiddle::deserialize("0");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = AccordionMiddle::deserialize("4");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = AccordionMiddle::deserialize("-1");
    assert!(result.is_err());
  }
}
