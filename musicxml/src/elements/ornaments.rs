use super::{
  AccidentalMark, DelayedInvertedTurn, DelayedTurn, Haydn, InvertedMordent, InvertedTurn, InvertedVerticalTurn,
  Mordent, OtherOrnament, Schleifer, Shake, Tremolo, TrillMark, Turn, VerticalTurn, WavyLine,
};
use crate::datatypes::Id;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Ornaments] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OrnamentsAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
}

#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum OrnamentType {
  #[rename("trill-mark")]
  TrillMark(TrillMark),
  Turn(Turn),
  #[rename("delayed-turn")]
  DelayedTurn(DelayedTurn),
  #[rename("inverted-turn")]
  InvertedTurn(InvertedTurn),
  #[rename("delayed-inverted-turn")]
  DelayedInvertedTurn(DelayedInvertedTurn),
  #[rename("vertical-turn")]
  VerticalTurn(VerticalTurn),
  #[rename("inverted-vertical-turn")]
  InvertedVerticalTurn(InvertedVerticalTurn),
  Shake(Shake),
  #[rename("wavy-line")]
  WavyLine(WavyLine),
  Mordent(Mordent),
  #[rename("inverted-mordent")]
  InvertedMordent(InvertedMordent),
  Schleifer(Schleifer),
  Tremolo(Tremolo),
  Haydn(Haydn),
  #[rename("other-ornament")]
  OtherOrnament(OtherOrnament),
}

#[derive(Debug, Default, PartialEq, Eq, ContentSerialize)]
pub struct OrnamentContents {
  pub ornaments: Vec<OrnamentType>,
  pub accidental_mark: Vec<AccidentalMark>,
}

impl ContentDeserializer for OrnamentContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    Ok(OrnamentContents {
      ornaments: elements
        .iter()
        .filter_map(|el| OrnamentType::deserialize(el).ok())
        .collect::<Vec<_>>(),
      accidental_mark: elements
        .iter()
        .filter_map(|el| {
          if el.name == "accidental-mark" {
            AccidentalMark::deserialize(el).ok()
          } else {
            None
          }
        })
        .collect::<Vec<_>>(),
    })
  }
}

/// [Ornaments] can be any of several types, followed optionally by accidentals.
///
/// The [AccidentalMark] element's content is represented the same as an [Accidental][super::Accidental] element, but with a different
/// name to reflect the different musical meaning.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ornaments {
  /// Element-specific attributes
  pub attributes: OrnamentsAttributes,
  #[flatten]
  /// Element-specific content
  pub content: OrnamentContents,
}

#[cfg(test)]
mod ornaments_tests {
  use crate::datatypes::{TremoloMarks, TrillBeats, YesNo};
  use crate::elements::*;
  use crate::parser::{parse_from_xml_str, parse_to_xml_str};

  #[test]
  fn serialize_valid1() {
    let test = Ornaments {
      attributes: OrnamentsAttributes::default(),
      content: OrnamentContents {
        ornaments: vec![
          OrnamentType::Schleifer(Schleifer {
            attributes: SchleiferAttributes::default(),
            content: (),
          }),
          OrnamentType::Tremolo(Tremolo {
            attributes: TremoloAttributes::default(),
            content: TremoloMarks(2),
          }),
        ],
        accidental_mark: vec![],
      },
    };
    let expected = "<ornaments><schleifer/><tremolo>2</tremolo></ornaments>";
    let result = parse_to_xml_str(&test, false);
    assert_eq!(result, expected);
  }

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Ornaments>(
      "<ornaments>
        <delayed-turn beats=\"2.1\"/>
        <shake accelerate=\"no\"/>
        <accidental-mark>natural-down</accidental-mark>
        <accidental-mark>double-sharp-down</accidental-mark>
      </ornaments>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Ornaments {
        attributes: OrnamentsAttributes::default(),
        content: OrnamentContents {
          ornaments: vec![
            OrnamentType::DelayedTurn(DelayedTurn {
              attributes: DelayedTurnAttributes {
                beats: Some(TrillBeats(2.1)),
                ..Default::default()
              },
              content: (),
            }),
            OrnamentType::Shake(Shake {
              attributes: ShakeAttributes {
                accelerate: Some(YesNo::No),
                ..Default::default()
              },
              content: (),
            }),
          ],
          accidental_mark: vec![
            AccidentalMark {
              attributes: AccidentalMarkAttributes::default(),
              content: crate::datatypes::AccidentalValue::NaturalDown,
            },
            AccidentalMark {
              attributes: AccidentalMarkAttributes::default(),
              content: crate::datatypes::AccidentalValue::DoubleSharpDown,
            },
          ],
        }
      }
    );
  }
}
