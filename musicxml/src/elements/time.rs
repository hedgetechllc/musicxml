use super::{BeatType, Beats, Interchangeable, SenzaMisura};
use crate::datatypes::{
  Color, FontFamily, FontSize, FontStyle, FontWeight, Id, LeftCenterRight, StaffNumber, Tenths, TimeSeparator,
  TimeSymbol, Valign, YesNo,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Time] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TimeAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// Changes the computation of the default horizontal position.
  /// The origin is changed relative to the left-hand side of the note or the musical position within the bar.
  /// Positive x is right and negative x is left.
  ///
  /// This attribute provides higher-resolution positioning data than the [Offset][super::Offset] element.
  /// Applications reading a MusicXML file that can understand both features should generally rely on this attribute for its greater accuracy.
  pub default_x: Option<Tenths>,
  /// Changes the computation of the default vertical position.
  /// The origin is changed relative to the top line of the staff. Positive y is up and negative y is down.
  ///
  /// This attribute provides higher-resolution positioning data than the `placement` attribute.
  /// Applications reading a MusicXML file that can understand both attributes should generally rely on this attribute for its greater accuracy.
  pub default_y: Option<Tenths>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// In cases where text extends over more than one line, horizontal alignment and justify values can be different.
  /// The most typical case is for credits, such as:
  ///
  /// ```text
  /// Words and music by
  ///   Pat Songwriter
  /// ```
  /// Typically this type of credit is aligned to the right, so that the position information refers to the right-most part of the text.
  /// But in this example, the text is center-justified, not right-justified.
  ///
  /// The `halign` attribute is used in these situations. If it is not present, its value is the same as for the `justify` attribute.
  /// For elements where a justify attribute is not allowed, the default is implementation-dependent.
  pub halign: Option<LeftCenterRight>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Allows a time signature to apply to only the specified staff in the part. If absent, the time signature applies to all staves in the part.
  pub number: Option<StaffNumber>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Indicates how to display the arrangement between the [Beats] and [BeatType] values in a time signature.
  pub separator: Option<TimeSeparator>,
  /// Indicates how to display a time signature, such as by using common and cut time symbols or a single number display.
  pub symbol: Option<TimeSymbol>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TimeBeatContents {
  pub beats: Beats,
  pub beat_type: BeatType,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TimeContents {
  pub beats: Vec<TimeBeatContents>,
  pub interchangeable: Option<Interchangeable>,
  pub senza_misura: Option<SenzaMisura>,
}

impl ContentDeserializer for TimeContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut beats = Vec::new();
    let mut time_beats = None;
    let mut interchangeable = None;
    let mut senza_misura = None;
    for element in elements {
      match element.name.as_str() {
        "beats" => time_beats = Some(Beats::deserialize(element)?),
        "beat-type" => {
          beats.push(TimeBeatContents {
            beats: time_beats.unwrap(),
            beat_type: BeatType::deserialize(element)?,
          });
          time_beats = None;
        }
        "interchangeable" => interchangeable = Some(Interchangeable::deserialize(element)?),
        "senza-misura" => senza_misura = Some(SenzaMisura::deserialize(element)?),
        _ => return Err(format!("Invalid element name: {}", element.name)),
      }
    }
    Ok(TimeContents {
      beats,
      interchangeable,
      senza_misura,
    })
  }
}

impl ContentSerializer for TimeContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    for el in &element.beats {
      elements.push(Beats::serialize(&el.beats));
      elements.push(BeatType::serialize(&el.beat_type));
    }
    if let Some(content) = &element.interchangeable {
      elements.push(Interchangeable::serialize(content));
    }
    if let Some(content) = &element.senza_misura {
      let mut xml_element = SenzaMisura::serialize(content);
      xml_element.name = String::from("senza-misura");
      elements.push(xml_element);
    }
    elements
  }
}

/// Time signatures are represented by the [Beats] element for the numerator and the [BeatType] element for the denominator.
///
/// ![Time](time.png)
///
/// Multiple pairs of [Beats] and [BeatType] elements are used for composite time signatures with multiple denominators, such as 2/4 + 3/8.
/// A composite such as 3+2/8 requires only one [Beats]/[BeatType] pair.
///
/// The `print_object` attribute allows a time signature to be specified but not printed, as is the case for excerpts from the middle of a score.
/// The value is "yes" if not present.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Time {
  /// Element-specific attributes
  pub attributes: TimeAttributes,
  #[flatten]
  /// Element-specific content
  pub content: TimeContents,
}

#[cfg(test)]
mod time_tests {
  use crate::elements::*;
  use crate::{
    elements::InterchangeableContents,
    parser::{parse_from_xml_str, parse_to_xml_str},
  };

  #[test]
  fn serialize_valid1() {
    let test = Time {
      attributes: TimeAttributes::default(),
      content: TimeContents {
        beats: vec![
          TimeBeatContents {
            beats: Beats {
              attributes: (),
              content: String::from("3"),
            },
            beat_type: BeatType {
              attributes: (),
              content: String::from("4"),
            },
          },
          TimeBeatContents {
            beats: Beats {
              attributes: (),
              content: String::from("6"),
            },
            beat_type: BeatType {
              attributes: (),
              content: String::from("8"),
            },
          },
        ],
        interchangeable: Some(Interchangeable {
          attributes: InterchangeableAttributes::default(),
          content: InterchangeableContents {
            time_relation: Some(TimeRelation {
              attributes: (),
              content: crate::datatypes::TimeRelation::Bracket,
            }),
            beat_data: vec![InterchangeableBeatData {
              beats: Beats {
                attributes: (),
                content: String::from("4"),
              },
              beat_type: BeatType {
                attributes: (),
                content: String::from("4"),
              },
            }],
          },
        }),
        senza_misura: None,
      },
    };
    let expected = "<time>
  <beats>3</beats>
  <beat-type>4</beat-type>
  <beats>6</beats>
  <beat-type>8</beat-type>
  <interchangeable>
    <time-relation>bracket</time-relation>
    <beats>4</beats>
    <beat-type>4</beat-type>
  </interchangeable>
</time>";
    let result = parse_to_xml_str(&test, true);
    assert_eq!(result, expected);
  }

  #[test]
  fn serialize_valid2() {
    let test = Time {
      attributes: TimeAttributes::default(),
      content: TimeContents {
        beats: Vec::new(),
        interchangeable: None,
        senza_misura: Some(SenzaMisura {
          attributes: (),
          content: String::from("Senza Test"),
        }),
      },
    };
    let expected = "<time><senza-misura>Senza Test</senza-misura></time>";
    let result = parse_to_xml_str(&test, false);
    assert_eq!(result, expected);
  }

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Time>(
      "<time>
      <beats>3</beats>
      <beat-type>4</beat-type>
      <beats>6</beats>
      <beat-type>8</beat-type>
      <interchangeable>
        <time-relation>bracket</time-relation>
        <beats>4</beats>
        <beat-type>4</beat-type>
      </interchangeable>
    </time>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Time {
        attributes: TimeAttributes::default(),
        content: TimeContents {
          beats: vec![
            TimeBeatContents {
              beats: Beats {
                attributes: (),
                content: String::from("3")
              },
              beat_type: BeatType {
                attributes: (),
                content: String::from("4")
              },
            },
            TimeBeatContents {
              beats: Beats {
                attributes: (),
                content: String::from("6")
              },
              beat_type: BeatType {
                attributes: (),
                content: String::from("8")
              },
            },
          ],
          interchangeable: Some(Interchangeable {
            attributes: InterchangeableAttributes::default(),
            content: InterchangeableContents {
              time_relation: Some(TimeRelation {
                attributes: (),
                content: crate::datatypes::TimeRelation::Bracket,
              }),
              beat_data: vec![InterchangeableBeatData {
                beats: Beats {
                  attributes: (),
                  content: String::from("4")
                },
                beat_type: BeatType {
                  attributes: (),
                  content: String::from("4")
                },
              }],
            },
          }),
          senza_misura: None,
        }
      }
    );
  }
}
