use super::{BeatRepeat, MeasureRepeat, MultipleRest, Slash};
use crate::datatypes::{Color, FontFamily, FontSize, FontStyle, FontWeight, Id, StaffNumber};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [MeasureStyle] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MeasureStyleAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Allows a measure style to apply to only the specified staff in the part.
  /// If absent, the measure style applies to all staves in the part.
  pub number: Option<StaffNumber>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MeasureStyleContents {
  MultipleRest(MultipleRest),
  MeasureRepeat(MeasureRepeat),
  BeatRepeat(BeatRepeat),
  Slash(Slash),
}

/// The [MeasureStyle] element indicates a special way to print partial to multiple measures within a part.
/// 
/// This includes multiple rests over several measures, repeats of beats, single, or multiple measures, and use of slash notation.
/// 
/// The [MultipleRest] and [MeasureRepeat] elements indicate the number of measures covered in the element content.
/// The [BeatRepeat] and [Slash] elements can cover partial measures. All but the [MultipleRest] element use a `type` attribute to indicate
/// starting and stopping the use of the style.
#[derive(Debug, PartialEq, Eq)]
pub struct MeasureStyle {
  /// Element-specific attributes
  pub attributes: MeasureStyleAttributes,
  /// Element-specific content
  pub content: MeasureStyleContents,
}

impl ElementDeserializer for MeasureStyle {
  fn deserialize(element: &XmlElement) -> Result<Self, String> {
    Ok(MeasureStyle {
      attributes: MeasureStyleAttributes::deserialize(&element.attributes)?,
      content: match element.elements.first().ok_or("Missing required content elements in <measure-style>")? {
        el if el.name == "multiple-rest" => MeasureStyleContents::MultipleRest(MultipleRest::deserialize(el)?),
        el if el.name == "measure-repeat" => MeasureStyleContents::MeasureRepeat(MeasureRepeat::deserialize(el)?),
        el if el.name == "beat-repeat" => MeasureStyleContents::BeatRepeat(BeatRepeat::deserialize(el)?),
        el if el.name == "slash" => MeasureStyleContents::Slash(Slash::deserialize(el)?),
        el => return Err(format!("Invalid <measure-style> element name: {}", el.name)),
      }
    })
  }
}

impl ElementSerializer for MeasureStyle {
  fn serialize(element: &Self) -> XmlElement {
    XmlElement {
      name: String::from("measure-style"),
      attributes: MeasureStyleAttributes::serialize(&element.attributes),
      elements: vec![match &element.content {
        MeasureStyleContents::MultipleRest(content) => MultipleRest::serialize(content),
        MeasureStyleContents::MeasureRepeat(content) => MeasureRepeat::serialize(content),
        MeasureStyleContents::BeatRepeat(content) => BeatRepeat::serialize(content),
        MeasureStyleContents::Slash(content) => Slash::serialize(content),
      }],
      text: String::new(),
    }
  }
}
