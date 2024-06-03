use super::{BeatType, Beats, TimeRelation};
use crate::datatypes::{TimeSeparator, TimeSymbol};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Interchangeable] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct InterchangeableAttributes {
  /// Indicates how to display the arrangement between the [Beats] and [BeatType] values in the second of the dual time signatures.
  pub separator: Option<TimeSeparator>,
  /// Indicates how to display the second of the dual time signatures, such as by using common and cut time symbols or a single number display.
  pub symbol: Option<TimeSymbol>,
}

/// Contents of the [InterchangeableBeatData] element.
#[derive(Debug, PartialEq, Eq)]
pub struct InterchangeableBeatData {
  /// The number of beats in the second of the dual time signatures.
  pub beats: Beats,
  /// The beat type of the second of the dual time signatures.
  pub beat_type: BeatType,
}

/// Contents of the [Interchangeable] element.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct InterchangeableContents {
  /// The time relation between the two dual time signatures.
  pub time_relation: Option<TimeRelation>,
  /// The beat data for the second of the dual time signatures.
  pub beat_data: Vec<InterchangeableBeatData>,
}

impl ContentDeserializer for InterchangeableContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut temp_beats: Option<Beats> = None;
    let mut time_relation = None;
    let mut beat_data = Vec::new();
    for element in elements {
      match element.name.as_str() {
        "time-relation" => {
          time_relation = Some(TimeRelation::deserialize(element)?);
        }
        "beats" => {
          temp_beats = Some(Beats::deserialize(element)?);
        }
        "beat-type" => {
          match &temp_beats {
            Some(temp_beat) => beat_data.push(InterchangeableBeatData {
              beats: temp_beat.clone(),
              beat_type: BeatType::deserialize(element)?,
            }),
            _ => Err(format!("Missing required \"beat\" element prior to \"beat-type\""))?,
          };
          temp_beats = None;
        }
        _ => (),
      }
    }
    Ok(InterchangeableContents {
      time_relation,
      beat_data,
    })
  }
}

impl ContentSerializer for InterchangeableContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    if let Some(el) = &element.time_relation {
      elements.push(TimeRelation::serialize(el));
    }
    for el in &element.beat_data {
      elements.push(Beats::serialize(&el.beats));
      elements.push(BeatType::serialize(&el.beat_type))
    }
    elements
  }
}

/// The [Interchangeable] element is used to represent the second in a pair of interchangeable dual time signatures, such as the 6/8 in 3/4 (6/8).
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Interchangeable {
  /// Element-specific attributes
  pub attributes: InterchangeableAttributes,
  #[flatten]
  /// Element-specific content
  pub content: InterchangeableContents,
}

#[cfg(test)]
mod interchangeable_tests {
  use super::*;
  use crate::datatypes::TimeSymbol;
  use crate::elements::{BeatType, Beats};
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Interchangeable>(
      "<interchangeable symbol=\"dotted-note\">
        <time-relation>parentheses</time-relation>
        <beats>3</beats>
        <beat-type>8</beat-type>
        <beats>2</beats>
        <beat-type>4</beat-type>
      </interchangeable>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Interchangeable {
        attributes: InterchangeableAttributes {
          symbol: Some(TimeSymbol::DottedNote),
          ..Default::default()
        },
        content: InterchangeableContents {
          time_relation: Some(TimeRelation {
            attributes: (),
            content: crate::datatypes::TimeRelation::Parentheses
          }),
          beat_data: vec![
            InterchangeableBeatData {
              beats: Beats {
                attributes: (),
                content: String::from("3")
              },
              beat_type: BeatType {
                attributes: (),
                content: String::from("8")
              },
            },
            InterchangeableBeatData {
              beats: Beats {
                attributes: (),
                content: String::from("2")
              },
              beat_type: BeatType {
                attributes: (),
                content: String::from("4")
              },
            },
          ],
        },
      }
    );
  }
}
