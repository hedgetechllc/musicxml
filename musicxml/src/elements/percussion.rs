use super::{Beater, Effect, Glass, Membrane, Metal, OtherPercussion, Pitched, Stick, StickLocation, Timpani, Wood};
use crate::datatypes::{
  Color, EnclosureShape, FontFamily, FontSize, FontStyle, FontWeight, Id, LeftCenterRight, Tenths, Valign,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Percussion] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PercussionAttributes {
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
  /// Formatting of an enclosure around text or symbols.
  pub enclosure: Option<EnclosureShape>,
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
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Indicates vertical alignment to the top, middle, bottom, or baseline of the text. The default is implementation-dependent.
  pub valign: Option<Valign>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PercussionContents {
  Glass(Glass),
  Metal(Metal),
  Wood(Wood),
  Pitched(Pitched),
  Membrane(Membrane),
  Effect(Effect),
  Timpani(Timpani),
  Beater(Beater),
  Stick(Stick),
  StickLocation(StickLocation),
  OtherPercussion(OtherPercussion),
}

/// The [Percussion] element is used to define percussion pictogram symbols.
/// 
/// ![Percussion](percussion.png)
/// 
/// The organization of these symbols follows the definitions in Kurt Stone's "Music Notation in the Twentieth Century" on pages 206-212 and 223.
/// More pictograms have been added to the ones listed in Stone, based on how usage has evolved since the book was published in 1980.
#[derive(Debug, PartialEq, Eq)]
pub struct Percussion {
  /// Element-specific attributes
  pub attributes: PercussionAttributes,
  /// Element-specific content
  pub content: PercussionContents,
}

impl ElementDeserializer for Percussion {
  fn deserialize(element: &XmlElement) -> Result<Self, String> {
    let el = element.elements.first().unwrap();
    Ok(Percussion {
      attributes: PercussionAttributes::deserialize(&element.attributes)?,
      content: match el.name.as_str() {
        "glass" => PercussionContents::Glass(Glass::deserialize(el)?),
        "metal" => PercussionContents::Metal(Metal::deserialize(el)?),
        "wood" => PercussionContents::Wood(Wood::deserialize(el)?),
        "pitched" => PercussionContents::Pitched(Pitched::deserialize(el)?),
        "membrane" => PercussionContents::Membrane(Membrane::deserialize(el)?),
        "effect" => PercussionContents::Effect(Effect::deserialize(el)?),
        "timpani" => PercussionContents::Timpani(Timpani::deserialize(el)?),
        "beater" => PercussionContents::Beater(Beater::deserialize(el)?),
        "stick" => PercussionContents::Stick(Stick::deserialize(el)?),
        "stick-location" => PercussionContents::StickLocation(StickLocation::deserialize(el)?),
        "other-percussion" => PercussionContents::OtherPercussion(OtherPercussion::deserialize(el)?),
        other => Err(format!("Unknown sub-element type for <percussion>: {}", other))?,
      }
    })
  }
}

impl ElementSerializer for Percussion {
  fn serialize(element: &Self) -> XmlElement {
    let name;
    let mut xml_element = XmlElement {
      name: String::new(),
      attributes: PercussionAttributes::serialize(&element.attributes),
      elements: vec![match &element.content {
        PercussionContents::Glass(content) => { name = String::from("glass"); Glass::serialize(content) },
        PercussionContents::Metal(content) => { name = String::from("metal"); Metal::serialize(content) },
        PercussionContents::Wood(content) => { name = String::from("wood"); Wood::serialize(content) },
        PercussionContents::Pitched(content) => { name = String::from("pitched"); Pitched::serialize(content) },
        PercussionContents::Membrane(content) => { name = String::from("membrane"); Membrane::serialize(content) },
        PercussionContents::Effect(content) => { name = String::from("effect"); Effect::serialize(content) },
        PercussionContents::Timpani(content) => { name = String::from("timpani"); Timpani::serialize(content) },
        PercussionContents::Beater(content) => { name = String::from("beater"); Beater::serialize(content) },
        PercussionContents::Stick(content) => { name = String::from("stick"); Stick::serialize(content) },
        PercussionContents::StickLocation(content) => { name = String::from("stick-location"); StickLocation::serialize(content) },
        PercussionContents::OtherPercussion(content) => { name = String::from("other-percussion"); OtherPercussion::serialize(content) },
      }],
      text: String::new(),
    };
    xml_element.elements.last_mut().unwrap().name = name;
    xml_element
  }
}

#[cfg(test)]
mod percussion_tests {
  use crate::elements::*;
  use crate::datatypes::SmuflPictogramGlyphName;
  use crate::parser::{parse_from_xml_str, parse_to_xml_str};

  #[test]
  fn serialize_valid1() {
    let test = Percussion {
      attributes: PercussionAttributes::default(),
      content: PercussionContents::Timpani(Timpani {
        attributes: TimpaniAttributes { smufl: Some(SmuflPictogramGlyphName(String::from("Glyph"))) },
        content: ()
      }),
    };
    let expected = "<><timpani smufl=\"Glyph\"/></>";
    let result = parse_to_xml_str(&test, false);
    assert_eq!(result, expected);
  }

  #[test]
  fn serialize_valid2() {
    let test = Percussion {
      attributes: PercussionAttributes::default(),
      content: PercussionContents::StickLocation(StickLocation {
        attributes: (),
        content: crate::datatypes::StickLocation::CymbalBell,
      }),
    };
    let expected = "<><stick-location>cymbal bell</stick-location></>";
    let result = parse_to_xml_str(&test, false);
    assert_eq!(result, expected);
  }

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Percussion>(
      "<percussion><membrane>Chinese tomtom</membrane></percussion>"
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Percussion {
        attributes: PercussionAttributes::default(),
        content: PercussionContents::Membrane(Membrane {
          attributes: MembraneAttributes::default(),
          content: crate::datatypes::MembraneValue::ChineseTomtom,
        }),
      }
    );
  }

  #[test]
  fn deserialize_valid2() {
    let result = parse_from_xml_str::<Percussion>(
      "<percussion><effect>duck call</effect></percussion>"
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Percussion {
        attributes: PercussionAttributes::default(),
        content: PercussionContents::Effect(Effect {
          attributes: EffectAttributes::default(),
          content: crate::datatypes::EffectValue::DuckCall,
        }),
      }
    );
  }
}
