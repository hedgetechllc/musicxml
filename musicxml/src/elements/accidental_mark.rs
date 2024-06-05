use crate::datatypes::{
  AboveBelow, AccidentalValue, Color, FontFamily, FontSize, FontStyle, FontWeight, Id, SmuflAccidentalGlyphName,
  SymbolSize, Tenths, YesNo,
};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [AccidentalMark] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct AccidentalMarkAttributes {
  /// Specifies whether or not brackets are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub bracket: Option<YesNo>,
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
  /// Specifies whether or not parentheses are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub parentheses: Option<YesNo>,
  /// Indicates whether something is above or below another element, such as a note or a notation.
  pub placement: Option<AboveBelow>,
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

/// An [AccidentalMark] element can be used as a separate notation or as part of an ornament.
///
/// ![AccidentalMark](https://hedgetechllc.github.io/musicxml/musicxml/elements/accidental-mark.png)
///
/// When used in an ornament, position and placement are relative to the ornament, not relative to the note.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("accidental-mark")]
pub struct AccidentalMark {
  /// Element-specific attributes
  pub attributes: AccidentalMarkAttributes,
  /// Element-specific content
  pub content: AccidentalValue,
}

#[cfg(test)]
mod accidental_mark_tests {
  use super::*;
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<AccidentalMark>(
      "<accidental-mark font-weight=\"bold\" font-style=\"italic\" font-size=\"32\" id=\"RandomID\">sharp</accidental-mark>");
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      AccidentalMark {
        attributes: AccidentalMarkAttributes {
          font_weight: Some(FontWeight::Bold),
          font_style: Some(FontStyle::Italic),
          font_size: Some(FontSize::Decimal(32.0)),
          id: Some(Id(String::from("RandomID"))),
          ..Default::default()
        },
        content: AccidentalValue::Sharp,
      }
    );
  }
}
