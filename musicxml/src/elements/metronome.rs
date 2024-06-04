use super::{BeatUnit, BeatUnitDot, BeatUnitTied, MetronomeArrows, MetronomeNote, MetronomeRelation, PerMinute};
use crate::datatypes::{
  Color, FontFamily, FontSize, FontStyle, FontWeight, Id, LeftCenterRight, Tenths, Valign, YesNo,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Metronome] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MetronomeAttributes {
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
  /// Indicates left, center, or right justification. The default value varies for different elements.
  /// For elements where the `justify` attribute is present but the `halign` attribute is not,
  /// the `justify` attribute indicates horizontal alignment as well as justification.
  pub justify: Option<LeftCenterRight>,
  /// Specifies whether or not parentheses are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub parentheses: Option<YesNo>,
  /// Specifies whether or not to print an object. It is yes if not specified.
  pub print_object: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
}

/// The [BeatBasedEquation] element specifies the metronome mark in terms of beat units.
#[derive(Debug, PartialEq, Eq)]
pub struct BeatBasedEquation {
  /// The [BeatUnit] element specifies the beat unit for the metronome mark.
  pub beat_unit: BeatUnit,
  /// The [BeatUnitDot] element specifies the presence of a dot in the metronome mark.
  pub beat_unit_dot: Vec<BeatUnitDot>,
  /// The [BeatUnitTied] element specifies the presence of a tie in the metronome mark.
  pub beat_unit_tied: Vec<BeatUnitTied>,
}

/// The [BeatEquation] element specifies the metronome mark in terms of beats per minute or beat units.
#[derive(Debug, PartialEq, Eq)]
pub enum BeatEquation {
  /// The [PerMinute] element specifies the metronome mark in terms of beats per minute.
  BPM(PerMinute),
  /// The [BeatBasedEquation] element specifies the metronome mark in terms of beat units.
  Beats(BeatBasedEquation),
}

/// The [BeatBased] element specifies the metronome mark in terms of beat units.
#[derive(Debug, PartialEq, Eq)]
pub struct BeatBased {
  /// The [BeatUnit] element specifies the beat unit for the metronome mark.
  pub beat_unit: BeatUnit,
  /// The [BeatUnitDot] element specifies the presence of a dot in the metronome mark.
  pub beat_unit_dot: Vec<BeatUnitDot>,
  /// The [BeatUnitTied] element specifies the presence of a tie in the metronome mark.
  pub beat_unit_tied: Vec<BeatUnitTied>,
  /// The [BeatEquation] element specifies the metronome mark in terms of beats per minute or beat units.
  pub equals: BeatEquation,
}

impl ContentDeserializer for BeatBased {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut beat_unit: Option<BeatUnit> = None;
    let mut beat_unit_dot: Vec<BeatUnitDot> = Vec::new();
    let mut beat_unit_tied: Vec<BeatUnitTied> = Vec::new();
    let mut equals: Option<BeatEquation> = None;
    for element in elements {
      match element.name.as_str() {
        "beat-unit" => {
          if let Some(_) = beat_unit.as_mut() {
            equals = Some(BeatEquation::Beats(BeatBasedEquation {
              beat_unit: BeatUnit::deserialize(element)?,
              beat_unit_dot: Vec::new(),
              beat_unit_tied: Vec::new(),
            }))
          } else {
            beat_unit = Some(BeatUnit::deserialize(element)?);
          }
        }
        "beat-unit-dot" => {
          if let Some(equation) = equals.as_mut() {
            match equation {
              BeatEquation::Beats(ref mut beat_equation) => {
                beat_equation.beat_unit_dot.push(BeatUnitDot::deserialize(element)?);
              }
              _ => (),
            }
          } else {
            beat_unit_dot.push(BeatUnitDot::deserialize(element)?);
          }
        }
        "beat-unit-tied" => {
          if let Some(equation) = equals.as_mut() {
            match equation {
              BeatEquation::Beats(ref mut beat_equation) => {
                beat_equation.beat_unit_tied.push(BeatUnitTied::deserialize(element)?);
              }
              _ => (),
            }
          } else {
            beat_unit_tied.push(BeatUnitTied::deserialize(element)?);
          }
        }
        "per-minute" => {
          equals = Some(BeatEquation::BPM(PerMinute::deserialize(element)?));
        }
        other => Err(format!("Unknown subelement <{}> of <metronome>", other))?,
      }
    }
    Ok(BeatBased {
      beat_unit: beat_unit.ok_or_else(|| format!("Missing <beat-unit> subelement of <metronome>"))?,
      beat_unit_dot,
      beat_unit_tied,
      equals: equals.ok_or_else(|| format!("Missing some sort of <metronome> equivalency"))?,
    })
  }
}

impl ContentSerializer for BeatBased {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    elements.push(BeatUnit::serialize(&element.beat_unit));
    for el in &element.beat_unit_dot {
      elements.push(BeatUnitDot::serialize(el));
    }
    for el in &element.beat_unit_tied {
      elements.push(BeatUnitTied::serialize(el));
    }
    match &element.equals {
      BeatEquation::BPM(content) => elements.push(PerMinute::serialize(content)),
      BeatEquation::Beats(content) => {
        elements.push(BeatUnit::serialize(&content.beat_unit));
        for el in &content.beat_unit_dot {
          elements.push(BeatUnitDot::serialize(el));
        }
        for el in &content.beat_unit_tied {
          elements.push(BeatUnitTied::serialize(el));
        }
      }
    }
    elements
  }
}

/// The [AdditionalMetronomeBasedContents] element specifies additional metronome marks.
#[derive(Debug, PartialEq, Eq)]
pub struct AdditionalMetronomeBasedContents {
  /// The [MetronomeRelation] element specifies the relationship between additional metronome marks.
  pub metronome_relation: MetronomeRelation,
  /// The [MetronomeNote] element defines the appearance of a note within a metric relationship mark.
  pub metronome_note: Vec<MetronomeNote>,
}

/// The [MetronomeBased] element specifies the metronome mark in terms of beat units.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct MetronomeBased {
  /// The [MetronomeArrows] element specifies the appearance of arrows in the metronome mark.
  pub metronome_arrows: Option<MetronomeArrows>,
  /// The [MetronomeNote] element defines the appearance of a note within a metric relationship mark.
  pub metronome_note: Vec<MetronomeNote>,
  /// The [MetronomeRelation] element specifies the relationship between additional metronome marks.
  pub additional: Option<AdditionalMetronomeBasedContents>,
}

impl ContentDeserializer for MetronomeBased {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    let mut content = MetronomeBased::default();
    for element in elements {
      match element.name.as_str() {
        "metronome-arrows" => {
          content.metronome_arrows = Some(MetronomeArrows::deserialize(element)?);
        }
        "metronome-note" => {
          if let Some(additional) = content.additional.as_mut() {
            additional.metronome_note.push(MetronomeNote::deserialize(element)?);
          } else {
            content.metronome_note.push(MetronomeNote::deserialize(element)?);
          }
        }
        "metronome-relation" => {
          content.additional = Some(AdditionalMetronomeBasedContents {
            metronome_relation: MetronomeRelation::deserialize(element)?,
            metronome_note: vec![],
          })
        }
        other => Err(format!("Unknown subelement <{}> of <metronome>", other))?,
      }
    }
    Ok(content)
  }
}

impl ContentSerializer for MetronomeBased {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    let mut elements: Vec<XmlElement> = Vec::new();
    if let Some(content) = &element.metronome_arrows {
      elements.push(MetronomeArrows::serialize(content))
    }
    for el in &element.metronome_note {
      elements.push(MetronomeNote::serialize(el))
    }
    if let Some(content) = &element.additional {
      elements.push(MetronomeRelation::serialize(&content.metronome_relation));
      for note in &content.metronome_note {
        elements.push(MetronomeNote::serialize(note));
      }
    }
    elements
  }
}

/// Contents of the [Metronome] element.
///
/// The [Metronome] element may contain either [BeatBased] or [MetronomeBased] contents.
#[derive(Debug, PartialEq, Eq)]
pub enum MetronomeContents {
  /// The [BeatBased] element specifies the metronome mark in terms of beat units.
  BeatBased(BeatBased),
  /// The [MetronomeBased] element specifies the metronome mark in terms of metronome units.
  MetronomeBased(MetronomeBased),
}

impl ContentDeserializer for MetronomeContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    Ok(
      if let Some(_) = elements.iter().find(|&el| el.name == "metronome-note") {
        MetronomeContents::MetronomeBased(MetronomeBased::deserialize(elements)?)
      } else {
        MetronomeContents::BeatBased(BeatBased::deserialize(elements)?)
      },
    )
  }
}

impl ContentSerializer for MetronomeContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    match &element {
      MetronomeContents::BeatBased(content) => BeatBased::serialize(content),
      MetronomeContents::MetronomeBased(content) => MetronomeBased::serialize(content),
    }
  }
}

/// The [Metronome] element represents metronome marks and other metric relationships.
///
/// The [BeatUnit] element group and [PerMinute] element specify regular metronome marks.
/// The [MetronomeNote] and [MetronomeRelation] elements allow for the specification of metric modulations and other metric relationships,
/// such as swing tempo marks where two eighths are equated to a quarter note / eighth note triplet.
/// Tied notes can be represented in both types of metronome marks by using the [BeatUnitTied] and [MetronomeTied][super::MetronomeTied] elements.
/// The `print_object` attribute is set to "no" in cases where the [Metronome] element represents a relationship or range that is not displayed in the music notation.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Metronome {
  /// Element-specific attributes
  pub attributes: MetronomeAttributes,
  #[flatten]
  /// Element-specific content
  pub content: MetronomeContents,
}

#[cfg(test)]
mod metronome_tests {
  use super::*;
  use crate::datatypes::NoteTypeValue;
  use crate::elements::{BeatUnitTiedContents, PerMinuteAttributes};
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Metronome>(
      "<metronome>
        <beat-unit>128th</beat-unit>
        <beat-unit-dot/>
        <beat-unit-dot/>
        <beat-unit-tied>
          <beat-unit>256th</beat-unit>
          <beat-unit-dot/>
          <beat-unit-dot/>
          <beat-unit-dot/>
          <beat-unit-dot/>
        </beat-unit-tied>
        <beat-unit-tied>
          <beat-unit>whole</beat-unit>
        </beat-unit-tied>
        <per-minute>BPM Text</per-minute>
      </metronome>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Metronome {
        attributes: MetronomeAttributes::default(),
        content: MetronomeContents::BeatBased(BeatBased {
          beat_unit: BeatUnit {
            attributes: (),
            content: NoteTypeValue::OneHundredTwentyEighth
          },
          beat_unit_dot: vec![
            BeatUnitDot {
              attributes: (),
              content: ()
            },
            BeatUnitDot {
              attributes: (),
              content: ()
            },
          ],
          beat_unit_tied: vec![
            BeatUnitTied {
              attributes: (),
              content: BeatUnitTiedContents {
                beat_unit: BeatUnit {
                  attributes: (),
                  content: NoteTypeValue::TwoHundredFiftySixth
                },
                beat_unit_dot: vec![
                  BeatUnitDot {
                    attributes: (),
                    content: ()
                  },
                  BeatUnitDot {
                    attributes: (),
                    content: ()
                  },
                  BeatUnitDot {
                    attributes: (),
                    content: ()
                  },
                  BeatUnitDot {
                    attributes: (),
                    content: ()
                  },
                ]
              }
            },
            BeatUnitTied {
              attributes: (),
              content: BeatUnitTiedContents {
                beat_unit: BeatUnit {
                  attributes: (),
                  content: NoteTypeValue::Whole
                },
                beat_unit_dot: vec![]
              }
            },
          ],
          equals: BeatEquation::BPM(PerMinute {
            attributes: PerMinuteAttributes::default(),
            content: String::from("BPM Text")
          })
        })
      }
    );
  }

  #[test]
  fn deserialize_valid2() {
    let result = parse_from_xml_str::<Metronome>(
      "<metronome>
        <beat-unit>16th</beat-unit>
        <beat-unit>128th</beat-unit>
        <beat-unit-dot/>
        <beat-unit-dot/>
      </metronome>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Metronome {
        attributes: MetronomeAttributes::default(),
        content: MetronomeContents::BeatBased(BeatBased {
          beat_unit: BeatUnit {
            attributes: (),
            content: NoteTypeValue::Sixteenth
          },
          beat_unit_dot: vec![],
          beat_unit_tied: vec![],
          equals: BeatEquation::Beats(BeatBasedEquation {
            beat_unit: BeatUnit {
              attributes: (),
              content: NoteTypeValue::OneHundredTwentyEighth
            },
            beat_unit_dot: vec![
              BeatUnitDot {
                attributes: (),
                content: ()
              },
              BeatUnitDot {
                attributes: (),
                content: ()
              }
            ],
            beat_unit_tied: vec![],
          })
        })
      }
    );
  }
}
