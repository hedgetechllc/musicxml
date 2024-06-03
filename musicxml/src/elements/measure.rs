use super::{
  Attributes, Backup, Barline, Bookmark, Direction, FiguredBass, Forward, Grouping, Harmony, Link, Listening, Note,
  Part, Print, Sound,
};
use crate::datatypes::{Id, MeasureText, Tenths, Token, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Measure] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MeasureAttributes {
  /// The attribute that identifies the measure. Going from partwise to timewise, measures are grouped via this attribute.
  /// In partwise files, it should be the same for measures in different parts that share the same left barline.
  ///
  /// While often numeric, it does not have to be. Non-numeric values are typically used together with the `implicit` or
  /// `non_controlling` attributes being set to "yes". For a pickup measure, the `number` attribute is typically set to "0"
  /// and the `implicit` attribute is typically set to "yes".
  pub number: Token,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Set to "yes" for measures where the measure number should never appear, such as pickup measures and the last half
  /// of mid-measure repeats. The value is "no" if not specified.
  pub implicit: Option<YesNo>,
  /// Intended for use in multimetric music like the Don Giovanni minuet. If set to "yes", the left barline in this measure
  /// does not coincide with the left barline of measures in other parts. The value is "no" if not specified.
  pub non_controlling: Option<YesNo>,
  /// If measure numbers are not unique within a part, this can cause problems for conversions between partwise and
  /// timewise formats. The `text` attribute allows specification of displayed measure numbers that are different than what
  /// is used in the `number` attribute. This attribute is ignored for measures where the `implicit` attribute is set to "yes".
  /// Further details about measure numbering can be specified using the [MeasureNumbering][super::MeasureNumbering] element.
  pub text: Option<MeasureText>,
  /// Used to size and scale an image. The image should be scaled independently in X and Y if both `height` and `width` are specified.
  /// If only `width` is specified, the image should be scaled proportionally to fit in the specified X dimension.
  pub width: Option<Tenths>,
}

#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum MeasureElement {
  Part(Part),
  Note(Note),
  Backup(Backup),
  Forward(Forward),
  Direction(Direction),
  Attributes(Attributes),
  Harmony(Harmony),
  #[rename("figured-bass")]
  FiguredBass(FiguredBass),
  Print(Print),
  Sound(Sound),
  Listening(Listening),
  Barline(Barline),
  Grouping(Grouping),
  Link(Link),
  Bookmark(Bookmark),
}

/// The [Measure] element includes the basic musical data such as [Notes][super::Note] within a document.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Measure {
  /// Element-specific attributes
  pub attributes: MeasureAttributes,
  /// Element-specific content
  pub content: Vec<MeasureElement>,
}

#[cfg(test)]
mod measure_tests {
  use super::*;
  use crate::elements::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid() {
    let result = parse_from_xml_str::<Measure>(
      "<measure number=\"2\">
        <note default-x=\"148.50\" dynamics=\"2.22\" time-only=\"1,2,3\" default-y=\"-35.00\">
          <pitch>
            <step>F</step>
            <octave>4</octave>
          </pitch>
          <duration>6</duration>
          <voice>1</voice>
          <type>eighth</type>
          <stem>up</stem>
          <staff>1</staff>
          <beam number=\"1\">continue</beam>
          <notations>
            <articulations>
              <staccato/>
            </articulations>
          </notations>
        </note>
      </measure>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Measure {
        attributes: MeasureAttributes {
          number: Token(String::from("2")),
          id: None,
          implicit: None,
          non_controlling: None,
          text: None,
          width: None,
        },
        content: vec![MeasureElement::Note(Note {
          attributes: NoteAttributes {
            default_x: Some(Tenths(148.50)),
            dynamics: Some(crate::datatypes::NonNegativeDecimal(2.22)),
            time_only: Some(crate::datatypes::TimeOnly(vec![1, 2, 3])),
            default_y: Some(Tenths(-35.00)),
            ..Default::default()
          },
          content: NoteContents {
            info: NoteType::Normal(NormalInfo {
              chord: None,
              audible: AudibleType::Pitch(Pitch {
                attributes: (),
                content: PitchContents {
                  step: Step {
                    attributes: (),
                    content: crate::datatypes::Step::F
                  },
                  alter: None,
                  octave: Octave {
                    attributes: (),
                    content: crate::datatypes::Octave(4)
                  },
                }
              }),
              duration: Duration {
                attributes: (),
                content: crate::datatypes::PositiveDivisions(6)
              },
              tie: vec![],
            }),
            instrument: vec![],
            footnote: None,
            level: None,
            voice: Some(Voice {
              attributes: (),
              content: String::from("1")
            }),
            r#type: Some(Type {
              attributes: TypeAttributes::default(),
              content: crate::datatypes::NoteTypeValue::Eighth
            }),
            dot: vec![],
            accidental: None,
            time_modification: None,
            stem: Some(Stem {
              attributes: StemAttributes::default(),
              content: crate::datatypes::StemValue::Up
            }),
            notehead: None,
            notehead_text: None,
            staff: Some(Staff {
              attributes: (),
              content: crate::datatypes::PositiveInteger(1)
            }),
            beam: vec![Beam {
              attributes: BeamAttributes {
                number: Some(crate::datatypes::BeamLevel(1)),
                ..Default::default()
              },
              content: crate::datatypes::BeamValue::Continue,
            }],
            notations: vec![Notations {
              attributes: NotationsAttributes::default(),
              content: NotationsContents {
                footnote: None,
                level: None,
                notations: vec![NotationContentTypes::Articulations(Articulations {
                  attributes: ArticulationsAttributes::default(),
                  content: vec![ArticulationsType::Staccato(Staccato {
                    attributes: StaccatoAttributes::default(),
                    content: ()
                  })],
                })]
              }
            }],
            lyric: vec![],
            play: None,
            listen: None,
          },
        })]
      }
    );
  }
}
