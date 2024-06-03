use super::Accord;
use crate::datatypes::Id;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Scordatura] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ScordaturaAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
}

/// [Scordatura] string tunings are represented by a series of [Accord] elements, similar to the [StaffTuning][super::StaffTuning] elements.
///
/// ![Scordatura](scordatura.png)
///
/// Strings are numbered from high to low.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Scordatura {
  /// Element-specific attributes
  pub attributes: ScordaturaAttributes,
  /// Element-specific content
  pub content: Vec<Accord>,
}

#[cfg(test)]
mod scordatura_tests {
  use crate::datatypes::{Octave, Semitones, Step, StringNumber};
  use crate::elements::*;
  use crate::parser::{parse_from_xml_str, parse_to_xml_str};

  #[test]
  fn serialize_valid1() {
    let test = Scordatura {
      attributes: ScordaturaAttributes::default(),
      content: vec![
        Accord {
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
        },
        Accord {
          attributes: AccordAttributes {
            string: Some(StringNumber(2)),
          },
          content: AccordContents {
            tuning_step: TuningStep {
              attributes: (),
              content: Step::E,
            },
            tuning_alter: Some(TuningAlter {
              attributes: (),
              content: Semitones(4),
            }),
            tuning_octave: TuningOctave {
              attributes: (),
              content: Octave(3),
            },
          },
        },
      ],
    };
    let expected = "<scordatura>
  <accord string=\"1\">
    <tuning-step>D</tuning-step>
    <tuning-alter>-1</tuning-alter>
    <tuning-octave>4</tuning-octave>
  </accord>
  <accord string=\"2\">
    <tuning-step>E</tuning-step>
    <tuning-alter>4</tuning-alter>
    <tuning-octave>3</tuning-octave>
  </accord>
</scordatura>";
    let result = parse_to_xml_str(&test, true);
    assert_eq!(result, expected);
  }

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Scordatura>(
      "
    <scordatura>
      <accord string=\"1\">
        <tuning-step>D</tuning-step>
        <tuning-alter>-1</tuning-alter>
        <tuning-octave>4</tuning-octave>
      </accord>
      <accord string=\"2\">
        <tuning-step>E</tuning-step>
        <tuning-alter>4</tuning-alter>
        <tuning-octave>3</tuning-octave>
      </accord>
    </scordatura>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Scordatura {
        attributes: ScordaturaAttributes::default(),
        content: vec![
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
          },
          Accord {
            attributes: AccordAttributes {
              string: Some(StringNumber(2)),
            },
            content: AccordContents {
              tuning_step: TuningStep {
                attributes: (),
                content: Step::E
              },
              tuning_alter: Some(TuningAlter {
                attributes: (),
                content: Semitones(4)
              }),
              tuning_octave: TuningOctave {
                attributes: (),
                content: Octave(3)
              },
            }
          },
        ]
      }
    );
  }
}
