use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// See the definition in the [W3C XML Schema standard](https://www.w3.org/TR/xmlschema-2/#decimal).
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, DatatypeDeserialize, DatatypeSerialize)]
pub struct Decimal(pub f64);

impl Eq for Decimal {}

impl Deref for Decimal {
  type Target = f64;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod decimal_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Decimal::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Decimal(1.0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Decimal::deserialize("-0.3");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Decimal(-0.3));
  }

  #[test]
  fn deserialize_valid3() {
    let result = Decimal::deserialize("3.234234");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Decimal(3.234234));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Decimal::deserialize("0s");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid2() {
    let result = Decimal::deserialize("4.a");
    assert!(result.is_err());
  }

  #[test]
  fn deserialize_invalid3() {
    let result = Decimal::deserialize("-a.2");
    assert!(result.is_err());
  }
}
