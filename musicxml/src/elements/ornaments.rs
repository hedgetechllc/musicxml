use super::{
  AccidentalMark, DelayedInvertedTurn, DelayedTurn, Haydn, InvertedMordent, InvertedTurn, InvertedVerticalTurn,
  Mordent, OtherOrnament, Schleifer, Shake, Tremolo, TrillMark, Turn, VerticalTurn, WavyLine,
};
use crate::datatypes::Id;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Ornaments] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct OrnamentsAttributes {
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
}

/// The [OrnamentType] element specifies the ornaments available to be performed.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub enum OrnamentType {
  /// The [TrillMark] element represents the trill-mark symbol.
  #[rename("trill-mark")]
  TrillMark(TrillMark),
  /// The [Turn] element represents the turn symbol.
  Turn(Turn),
  /// The [DelayedTurn] element represents the delayed-turn symbol.
  #[rename("delayed-turn")]
  DelayedTurn(DelayedTurn),
  /// The [InvertedTurn] element represents the inverted-turn symbol.
  #[rename("inverted-turn")]
  InvertedTurn(InvertedTurn),
  /// The [DelayedInvertedTurn] element represents the delayed-inverted-turn symbol.
  #[rename("delayed-inverted-turn")]
  DelayedInvertedTurn(DelayedInvertedTurn),
  /// The [VerticalTurn] element represents the vertical-turn symbol.
  #[rename("vertical-turn")]
  VerticalTurn(VerticalTurn),
  /// The [InvertedVerticalTurn] element represents the inverted-vertical-turn symbol.
  #[rename("inverted-vertical-turn")]
  InvertedVerticalTurn(InvertedVerticalTurn),
  /// The [Shake] element represents the shake symbol.
  Shake(Shake),
  /// The [WavyLine] element represents the wavy-line symbol.
  #[rename("wavy-line")]
  WavyLine(WavyLine),
  /// The [Mordent] element represents the mordent symbol.
  Mordent(Mordent),
  /// The [InvertedMordent] element represents the inverted-mordent symbol.
  #[rename("inverted-mordent")]
  InvertedMordent(InvertedMordent),
  /// The [Schleifer] element represents the schleifer symbol.
  Schleifer(Schleifer),
  /// The [Tremolo] element represents the tremolo symbol.
  Tremolo(Tremolo),
  /// The [Haydn] element represents the Haydn symbol.
  Haydn(Haydn),
  /// The [OtherOrnament] element represents an ornament not covered by other elements.
  #[rename("other-ornament")]
  OtherOrnament(OtherOrnament),
}

/// Contents of the [Ornaments] element.
#[derive(Debug, Default, PartialEq, Eq, ContentSerialize)]
pub struct OrnamentContents {
  /// The [OrnamentType] elements specify the ornaments to be performed.
  pub ornaments: Vec<OrnamentType>,
  /// The [AccidentalMark] elements specify the accidentals to be performed.
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
