use crate::datatypes::{Color, FermataShape, FontFamily, FontSize, FontStyle, FontWeight, Id, Tenths, UprightInverted};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Fermata] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct FermataAttributes {
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
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// The type describes whether a fermata is upright or inverted. It is upright if not specified.
  pub r#type: Option<UprightInverted>,
}

/// The [Fermata] element content represents the shape of the fermata sign.
///
/// An empty [Fermata] element represents a normal fermata.
///
/// ![Fermata](https://hedgetechllc.github.io/musicxml/musicxml/elements/fermata.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Fermata {
  /// Element-specific attributes
  pub attributes: FermataAttributes,
  /// Element-specific content
  pub content: FermataShape,
}

#[cfg(test)]
mod fermata_tests {
  use super::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Fermata>("<fermata>square</fermata>");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Fermata {
        attributes: FermataAttributes::default(),
        content: FermataShape::Square,
      }
    );
  }
}
