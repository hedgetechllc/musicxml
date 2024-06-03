use super::{
  Clef, Directive, Divisions, Footnote, ForPart, Instruments, Key, Level, MeasureStyle, PartSymbol, StaffDetails,
  Staves, Time, Transpose,
};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct AttributesContents {
  pub footnote: Option<Footnote>,
  pub level: Option<Level>,
  pub divisions: Option<Divisions>,
  pub key: Vec<Key>,
  pub time: Vec<Time>,
  pub staves: Option<Staves>,
  pub part_symbol: Option<PartSymbol>,
  pub instruments: Option<Instruments>,
  pub clef: Vec<Clef>,
  pub staff_details: Vec<StaffDetails>,
  pub transpose: Vec<Transpose>,
  pub for_part: Vec<ForPart>,
  pub directive: Vec<Directive>,
  pub measure_style: Vec<MeasureStyle>,
}

/// The [Attributes] element contains musical information that typically changes on measure boundaries.
///
/// ![Attributes](attributes.png)
///
/// This includes key and time signatures, clefs, transpositions, and staving. When attributes are changed mid-measure,
/// it affects the music in score order, not in MusicXML document order.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Attributes {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: AttributesContents,
}

#[cfg(test)]
mod attributes_tests {
  use crate::datatypes::{
    AccidentalValue, Octave, PositiveDivisions, Semitones, ShowFrets, StaffLine, StaffLinePosition, Step, YesNo,
  };
  use crate::elements::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Attributes>(
      "<attributes>
        <footnote>Some footnote</footnote>
        <divisions>2</divisions>
        <key>
          <cancel>-2</cancel>
          <fifths>2</fifths>
          <mode>major</mode>
        </key>
        <key>
          <key-step>B</key-step>
          <key-alter>2</key-alter>
          <key-accidental>natural-sharp</key-accidental>
        </key>
        <time><senza-misura>M</senza-misura></time>
        <part-symbol>brace</part-symbol>
        <clef>
          <sign>G</sign>
          <line>2</line>
        </clef>
        <clef additional=\"yes\"><sign>F</sign></clef>
        <staff-details show-frets=\"letters\">
          <staff-type>ossia</staff-type>
          <staff-tuning line=\"1\">
            <tuning-step>B</tuning-step>
            <tuning-alter>2</tuning-alter>
            <tuning-octave>4</tuning-octave>
          </staff-tuning>
        </staff-details>
        <transpose><chromatic>2</chromatic></transpose>
        <directive>Some directive</directive>
        <directive>Other directive</directive>
        <measure-style><slash type=\"start\"/></measure-style>
      </attributes>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Attributes {
        attributes: (),
        content: AttributesContents {
          footnote: Some(Footnote {
            attributes: FootnoteAttributes::default(),
            content: String::from("Some footnote")
          }),
          divisions: Some(Divisions {
            attributes: (),
            content: PositiveDivisions(2)
          }),
          key: vec![
            Key {
              attributes: KeyAttributes { ..Default::default() },
              content: KeyContents::Explicit(ExplicitKeyContents {
                cancel: Some(Cancel {
                  attributes: CancelAttributes { ..Default::default() },
                  content: crate::datatypes::Fifths(-2)
                }),
                fifths: Fifths {
                  attributes: (),
                  content: crate::datatypes::Fifths(2)
                },
                mode: Some(Mode {
                  attributes: (),
                  content: crate::datatypes::Mode::Major
                }),
                key_octave: Vec::new(),
              })
            },
            Key {
              attributes: KeyAttributes { ..Default::default() },
              content: KeyContents::Relative(RelativeKeyContents {
                key_step: KeyStep {
                  attributes: (),
                  content: Step::B
                },
                key_alter: KeyAlter {
                  attributes: (),
                  content: Semitones(2)
                },
                key_accidental: Some(KeyAccidental {
                  attributes: KeyAccidentalAttributes { ..Default::default() },
                  content: AccidentalValue::NaturalSharp
                }),
                key_octave: Vec::new(),
              }),
            }
          ],
          time: vec![Time {
            attributes: TimeAttributes { ..Default::default() },
            content: TimeContents {
              senza_misura: Some(SenzaMisura {
                attributes: (),
                content: String::from("M")
              }),
              ..Default::default()
            }
          }],
          part_symbol: Some(PartSymbol {
            attributes: PartSymbolAttributes::default(),
            content: crate::datatypes::GroupSymbolValue::Brace
          }),
          clef: vec![
            Clef {
              attributes: ClefAttributes::default(),
              content: ClefContents {
                sign: Sign {
                  attributes: (),
                  content: crate::datatypes::ClefSign::G
                },
                line: Some(Line {
                  attributes: (),
                  content: StaffLinePosition(2)
                }),
                clef_octave_change: None
              }
            },
            Clef {
              attributes: ClefAttributes {
                additional: Some(YesNo::Yes),
                ..Default::default()
              },
              content: ClefContents {
                sign: Sign {
                  attributes: (),
                  content: crate::datatypes::ClefSign::F
                },
                line: None,
                clef_octave_change: None
              }
            },
          ],
          staff_details: vec![StaffDetails {
            attributes: StaffDetailsAttributes {
              show_frets: Some(ShowFrets::Letters),
              ..Default::default()
            },
            content: StaffDetailsContents {
              staff_type: Some(StaffType {
                attributes: (),
                content: crate::datatypes::StaffType::Ossia
              }),
              staff_tuning: vec![StaffTuning {
                attributes: StaffTuningAttributes { line: StaffLine(1) },
                content: StaffTuningContents {
                  tuning_step: TuningStep {
                    attributes: (),
                    content: Step::B
                  },
                  tuning_alter: Some(TuningAlter {
                    attributes: (),
                    content: Semitones(2)
                  }),
                  tuning_octave: TuningOctave {
                    attributes: (),
                    content: Octave(4)
                  }
                }
              }],
              ..Default::default()
            }
          },],
          transpose: vec![Transpose {
            attributes: TransposeAttributes::default(),
            content: TransposeContents {
              chromatic: Chromatic {
                attributes: (),
                content: Semitones(2)
              },
              diatonic: None,
              octave_change: None,
              double: None
            }
          }],
          directive: vec![
            Directive {
              attributes: DirectiveAttributes::default(),
              content: String::from("Some directive")
            },
            Directive {
              attributes: DirectiveAttributes::default(),
              content: String::from("Other directive")
            },
          ],
          measure_style: vec![MeasureStyle {
            attributes: MeasureStyleAttributes::default(),
            content: MeasureStyleContents::Slash(Slash {
              attributes: SlashAttributes {
                r#type: crate::datatypes::StartStop::Start,
                use_dots: None,
                use_stems: None
              },
              content: SlashContents { ..Default::default() }
            })
          }],
          ..Default::default()
        },
      }
    );
  }
}
