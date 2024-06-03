use crate::datatypes::NonNegativeInteger;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [ActualNotes] element describes how many notes are played in the time usually occupied by the number in the [NormalNotes][super::NormalNotes] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("actual-notes")]
pub struct ActualNotes {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: NonNegativeInteger,
}

#[cfg(test)]
mod actual_notes_tests {
  use super::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<ActualNotes>("<actual-notes>3</actual-notes>");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      ActualNotes {
        attributes: (),
        content: NonNegativeInteger(3)
      }
    );
  }

  #[test]
  fn deserialize_invalid1() {
    let result = parse_from_xml_str::<ActualNotes>("<actual-notes>-1</actual-notes>");
    assert!(result.is_err());
  }
}
