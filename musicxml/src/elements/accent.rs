use crate::datatypes::{AboveBelow, Color, FontFamily, FontSize, FontStyle, FontWeight, Tenths};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Accent] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct AccentAttributes {
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
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
}

/// The [Accent] element indicates a regular horizontal accent mark.
///
/// ![Accent](https://hedgetechllc.github.io/musicxml/musicxml/elements/accent.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Accent {
  /// Element-specific attributes
  pub attributes: AccentAttributes,
  /// Element-specific content
  pub content: (),
}

#[cfg(test)]
mod accent_tests {
  use super::*;
  use crate::parser::{parse_from_xml_str, parse_to_xml_str};

  #[test]
  fn serialize_valid1() {
    let test = Accent {
      attributes: AccentAttributes {
        color: Some(Color::new(35, 69, 103, 161)),
        default_x: Some(Tenths(0.2)),
        default_y: Some(Tenths(1.234)),
        font_family: Some(FontFamily(vec![String::from("Ariel"), String::from("Courier")])),
        ..Default::default()
      },
      content: (),
    };
    let expected = "<accent color=\"#A1234567\" default-x=\"0.2\" default-y=\"1.234\" font-family=\"Ariel,Courier\"/>";
    let result = parse_to_xml_str(&test, false);
    assert_eq!(result, expected);
  }

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Accent>(
      "<accent color=\"#a1234567\" default-x=\"0.2\" default-y=\"1.234\" font-family=\"Ariel,Courier\"></accent>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Accent {
        attributes: AccentAttributes {
          color: Some(Color::new(35, 69, 103, 161)),
          default_x: Some(Tenths(0.2)),
          default_y: Some(Tenths(1.234)),
          font_family: Some(FontFamily(vec![String::from("Ariel"), String::from("Courier")])),
          ..Default::default()
        },
        content: (),
      }
    );
  }
}
