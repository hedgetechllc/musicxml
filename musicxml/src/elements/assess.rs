use crate::datatypes::{IdRef, TimeOnly, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Assess] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct AssessAttributes {
  /// If yes, the note should be assessed; if no, it should not be assessed.
  /// If not specified, it is no for notes with a [Cue][super::Cue] child element and yes otherwise.
  pub r#type: YesNo,
  /// Restricts the type to apply to a single player. If missing, the type applies to all players.
  /// It references the `id` attribute of a [Player][super::Player] element defined within the
  /// matching [ScorePart][super::ScorePart].
  pub player: Option<IdRef>,
  /// Restricts the type to apply to a set of times through a repeated section.
  /// If missing, the type applies all times through the repeated section.
  pub time_only: Option<TimeOnly>,
}

impl Default for AssessAttributes {
  fn default() -> Self {
    AssessAttributes {
      r#type: YesNo::Yes,
      player: None,
      time_only: None,
    }
  }
}

/// The [Assess] element allows default [Cue][super::Cue] assessments to be overridden for individual notes.
///
/// By default, an assessment application should assess all notes without a [Cue][super::Cue] child element,
/// and not assess any note with a [Cue][super::Cue] child element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Assess {
  /// Element-specific attributes
  pub attributes: AssessAttributes,
  /// Element-specific content
  pub content: (),
}

#[cfg(test)]
mod assess_tests {
  use super::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Assess>("<assess type=\"yes\" time-only=\"1,2\"></assess>");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Assess {
        attributes: AssessAttributes {
          r#type: YesNo::Yes,
          time_only: Some(TimeOnly(vec![1, 2])),
          ..Default::default()
        },
        content: ()
      }
    );
  }
}
