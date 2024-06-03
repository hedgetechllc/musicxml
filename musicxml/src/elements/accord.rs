use super::{TuningAlter, TuningOctave, TuningStep};
use crate::datatypes::StringNumber;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Accord] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct AccordAttributes {
  /// Strings are numbered from high to low.
  pub string: Option<StringNumber>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct AccordContents {
  pub tuning_step: TuningStep,
  pub tuning_alter: Option<TuningAlter>,
  pub tuning_octave: TuningOctave,
}

/// The [Accord] element represents the tuning of a single string in the [Scordatura][super::Scordatura] element.
///
/// It uses the same group of elements as the [StaffTuning][super::StaffTuning] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Accord {
  /// Element-specific attributes
  pub attributes: AccordAttributes,
  #[flatten]
  /// Element-specific content
  pub content: AccordContents,
}

#[cfg(test)]
mod accord_tests {
  use super::*;
  use crate::datatypes::{Octave, Semitones, Step};
  use crate::parser::{parse_from_xml_str, parse_to_xml_str};

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Accord>(
      "
      <accord string=\"1\">
        <tuning-step>D</tuning-step>
        <tuning-alter>-1</tuning-alter>
        <tuning-octave>4</tuning-octave>
      </accord>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Accord {
        attributes: AccordAttributes {
          string: Some(StringNumber(1)),
        },
        content: AccordContents {
          tuning_step: TuningStep {
            attributes: (),
            content: Step::D
          },
          tuning_alter: Some(TuningAlter {
            attributes: (),
            content: Semitones(-1)
          }),
          tuning_octave: TuningOctave {
            attributes: (),
            content: Octave(4)
          },
        }
      }
    );
  }

  #[test]
  fn deserialize_invalid1() {
    let result = parse_from_xml_str::<Accord>(
      "
      <accord string=\"1\">
        <tuning-step>D</tuning-step>
        <tuning-alter>-1</tuning-alter>
        <tuning-octave>10</tuning-octave>
      </accord>",
    );
    assert!(result.is_err());
  }

  #[test]
  fn serialize_valid1() {
    let expected = "<accord string=\"1\">
  <tuning-step>D</tuning-step>
  <tuning-alter>-1</tuning-alter>
  <tuning-octave>4</tuning-octave>
</accord>";
    let test = Accord {
      attributes: AccordAttributes {
        string: Some(StringNumber(1)),
      },
      content: AccordContents {
        tuning_step: TuningStep {
          attributes: (),
          content: Step::D,
        },
        tuning_alter: Some(TuningAlter {
          attributes: (),
          content: Semitones(-1),
        }),
        tuning_octave: TuningOctave {
          attributes: (),
          content: Octave(4),
        },
      },
    };
    let result = parse_to_xml_str(&test, true);
    assert_eq!(result, expected);
  }
}
