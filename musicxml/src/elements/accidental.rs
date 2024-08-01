use crate::datatypes::{
  AccidentalValue, Color, FontFamily, FontSize, FontStyle, FontWeight, SmuflAccidentalGlyphName, SymbolSize, Tenths,
  YesNo,
};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [AccidentalAttributes] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct AccidentalAttributes {
  /// Specifies whether or not brackets are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub bracket: Option<YesNo>,
  /// If yes, indicates that this is a cautionary accidental. The value is no if not present.
  pub cautionary: Option<YesNo>,
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
  /// If yes, indicates that this is an editorial accidental. The value is no if not present.
  pub editorial: Option<YesNo>,
  /// A comma-separated list of font names.
  pub font_family: Option<FontFamily>,
  /// One of the CSS sizes or a numeric point size.
  pub font_size: Option<FontSize>,
  /// Normal or italic style.
  pub font_style: Option<FontStyle>,
  /// Normal or bold weight.
  pub font_weight: Option<FontWeight>,
  /// Specifies whether or not parentheses are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub parentheses: Option<YesNo>,
  /// Changes the horizontal position relative to the default position, either as computed by the individual program, or as overridden by the `default_x` attribute.
  /// Positive x is right and negative x is left. It should be interpreted in the context of the [Offset][super::Offset] element or directive attribute if those are present.
  pub relative_x: Option<Tenths>,
  /// Changes the vertical position relative to the default position, either as computed by the individual program, or as overridden by the `default_y` attribute.
  /// Positive y is up and negative y is down. It should be interpreted in the context of the `placement` attribute if that is present.
  pub relative_y: Option<Tenths>,
  /// Specifies the symbol size to use for an editorial indication. If not specified, it is left to application defaults.
  pub size: Option<SymbolSize>,
  /// References a specific Standard Music Font Layout (SMuFL) accidental glyph.
  /// This is used both with the other accidental value and for disambiguating cases where a single MusicXML accidental value could be represented by multiple SMuFL glyphs.
  pub smufl: Option<SmuflAccidentalGlyphName>,
}

/// The [Accidental] element represents actual notated accidentals.
///
/// ![Accidental](https://hedgetechllc.github.io/musicxml/musicxml/elements/accidental.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Accidental {
  /// Element-specific attributes
  pub attributes: AccidentalAttributes,
  /// Element-specific content
  pub content: AccidentalValue,
}

#[cfg(test)]
mod accidental_tests {
  use super::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Accidental>(
      "<accidental bracket=\"yes\" parentheses=\"yes\" size=\"grace-cue\" smufl=\"medRenFla\">flat-flat</accidental>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Accidental {
        attributes: AccidentalAttributes {
          bracket: Some(YesNo::Yes),
          parentheses: Some(YesNo::Yes),
          size: Some(SymbolSize::GraceCue),
          smufl: Some(SmuflAccidentalGlyphName(String::from("medRenFla"))),
          ..Default::default()
        },
        content: AccidentalValue::FlatFlat,
      }
    );
  }
}
