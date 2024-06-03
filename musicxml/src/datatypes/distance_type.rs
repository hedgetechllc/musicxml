use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};

/// Defines what type of distance is being defined in a [Distance][crate::elements::Distance] element.
///
/// Values include:
///
/// - [Beam][DistanceType::Beam]: The distance between beams.
/// - [Hyphen][DistanceType::Hyphen]: The distance between hyphens in lyrics.
/// - [Other][DistanceType::Other]: Left as a string so that other application-specific types can be defined.
#[derive(Debug, PartialEq, Eq)]
pub enum DistanceType {
  /// The distance between beams.
  Beam,
  /// The distance between hyphens in lyrics.
  Hyphen,
  /// Another type of distance.
  Other(String),
}

impl DatatypeSerializer for DistanceType {
  fn serialize(element: &Self) -> String {
    match element {
      Self::Beam => String::from("beam"),
      Self::Hyphen => String::from("hyphen"),
      Self::Other(text) => text.clone(),
    }
  }
}

impl DatatypeDeserializer for DistanceType {
  fn deserialize(value: &str) -> Result<Self, String> {
    Ok(match value {
      "beam" => Self::Beam,
      "hyphen" => Self::Hyphen,
      _ => Self::Other(String::from(value)),
    })
  }
}

#[cfg(test)]
mod distance_type_tests {
  use super::*;

  #[test]
  fn deserialize_valid() {
    let result = DistanceType::deserialize("beam");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DistanceType::Beam);
  }
}
