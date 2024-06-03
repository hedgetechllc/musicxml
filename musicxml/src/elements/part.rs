use super::{
  Attributes, Backup, Barline, Bookmark, Direction, FiguredBass, Forward, Grouping, Harmony, Link, Listening, Measure,
  Note, Print, Sound,
};
use crate::datatypes::IdRef;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Part] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PartAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: IdRef,
}

/// The [PartElement] specifies all possible elements available for use in a [Part] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum PartElement {
  /// The [Measure] element includes the basic musical data containers within a document.
  Measure(Measure),
  /// The [Note] element includes the basic musical data such as pitch, duration, and instrument.
  Note(Note),
  /// The [Backup] element is used to indicate that the music following it is a repetition of music that already appeared earlier in the score.
  Backup(Backup),
  /// The [Forward] element is used to indicate that the music following it is not a repetition of music that already appeared earlier in the score.
  Forward(Forward),
  /// The [Direction] element is a musical indication that is not necessarily attached to a specific note.
  Direction(Direction),
  /// The [Attributes] element contains all the musical information that is not pitch-related.
  Attributes(Attributes),
  /// The [Harmony] element is used to represent harmony symbols.
  Harmony(Harmony),
  /// The [FiguredBass] element is used to specify figured bass symbols.
  #[rename("figured-bass")]
  FiguredBass(FiguredBass),
  /// The [Print] element contains score layout information.
  Print(Print),
  /// The [Sound] element contains MIDI instrument information.
  Sound(Sound),
  /// The [Listening] element specifies the MIDI device that should be used for playback.
  Listening(Listening),
  /// The [Barline] element is used to set barline information.
  Barline(Barline),
  /// The [Grouping] element is used to indicate editorial information about how to group parts of a score.
  Grouping(Grouping),
  /// The [Link] element is used to establish a link to a URL.
  Link(Link),
  /// The [Bookmark] element is used to establish a bookmark for the current score location.
  Bookmark(Bookmark),
}

/// The [Part] element includes the basic musical data such as [Notes][super::Note] or [Measures][Measure] within a document.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Part {
  /// Element-specific attributes
  pub attributes: PartAttributes,
  /// Element-specific content
  pub content: Vec<PartElement>,
}

#[cfg(test)]
mod part_tests {
  use super::*;
  use crate::elements::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid() {
    let result = parse_from_xml_str::<Part>(
      "<part id=\"P1\">
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
      </part>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Part {
        attributes: PartAttributes {
          id: IdRef(String::from("P1")),
        },
        content: vec![PartElement::Note(Note {
          attributes: NoteAttributes {
            default_x: Some(crate::datatypes::Tenths(148.50)),
            dynamics: Some(crate::datatypes::NonNegativeDecimal(2.22)),
            time_only: Some(crate::datatypes::TimeOnly(vec![1, 2, 3])),
            default_y: Some(crate::datatypes::Tenths(-35.00)),
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
