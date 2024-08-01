use alloc::string::{String, ToString};
use core::ops::Deref;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Representing tenths of interline staff space (positive or negative).
///
/// Both integer and decimal values are allowed, such as 5 for a half space and 2.5 for a quarter space.
/// Interline space is measured from the middle of a staff line.
///
/// Distances in a MusicXML file are measured in tenths of staff space. Tenths are then scaled to millimeters
/// within the scaling element, used in the defaults element at the start of a score. Individual staves can
/// apply a scaling factor to adjust staff size.
///
/// When a MusicXML element or attribute refers to tenths, it means the global tenths defined by
/// the [Scaling][crate::elements::Scaling] element, not the local tenths as adjusted by the
/// [StaffSize][crate::elements::StaffSize] element.
///
/// The value of an instance of this type may be accessed by dereferencing the struct: `*datatype_val`.
#[derive(Debug, PartialEq, DatatypeDeserialize, DatatypeSerialize)]
pub struct Tenths(pub f64);

impl Eq for Tenths {}

impl Deref for Tenths {
  type Target = f64;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[cfg(test)]
mod tenths_tests {
  use super::*;

  #[test]
  fn deserialize_valid1() {
    let result = Tenths::deserialize("1");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Tenths(1.0));
  }

  #[test]
  fn deserialize_valid2() {
    let result = Tenths::deserialize("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Tenths(0.0));
  }

  #[test]
  fn deserialize_valid3() {
    let result = Tenths::deserialize("2.5");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Tenths(2.5));
  }

  #[test]
  fn deserialize_invalid1() {
    let result = Tenths::deserialize("asdf");
    assert!(result.is_err());
  }
}
