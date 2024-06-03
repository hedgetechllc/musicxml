use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to indicate whether one element appears above or below another element.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum AboveBelow {
  /// This element appears above the reference element.
  Above,
  /// This element appears below the reference element.
  Below,
}

#[cfg(test)]
mod above_below_tests {
  use super::*;

  #[test]
  fn serialize_valid() {
    let test = AboveBelow::Below;
    let result = AboveBelow::serialize(&test);
    assert_eq!(result, "below");
  }

  #[test]
  fn deserialize_valid() {
    let result = AboveBelow::deserialize("below");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), AboveBelow::Below);
  }

  #[test]
  fn deserialize_invalid() {
    let result = AboveBelow::deserialize("belows");
    assert!(result.is_err());
  }
}
