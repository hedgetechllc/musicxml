use super::{ArrowDirection, ArrowStyle, Arrowhead, CircularArrow};
use crate::datatypes::{AboveBelow, Color, FontFamily, FontSize, FontStyle, FontWeight, SmuflGlyphName, Tenths};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Arrow] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ArrowAttributes {
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
  /// Indicates a particular Standard Music Font Layout (SMuFL) character using its canonical glyph name.
  /// Sometimes this is a formatting choice, and sometimes this is a refinement of the semantic meaning of an element.
  pub smufl: Option<SmuflGlyphName>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct StraightArrowContents {
  pub arrow_direction: ArrowDirection,
  pub arrow_style: Option<ArrowStyle>,
  pub arrowhead: Option<Arrowhead>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct CircularArrowContents {
  pub circular_arrow: CircularArrow,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArrowContents {
  Straight(StraightArrowContents),
  Circular(CircularArrowContents),
}

impl ContentDeserializer for ArrowContents {
  fn deserialize(elements: &Vec<XmlElement>) -> Result<Self, String> {
    if let Some(_) = elements.iter().find(|&el| el.name == "circular-arrow") {
      Ok(ArrowContents::Circular(CircularArrowContents::deserialize(elements)?))
    } else {
      Ok(ArrowContents::Straight(StraightArrowContents::deserialize(elements)?))
    }
  }
}

impl ContentSerializer for ArrowContents {
  fn serialize(element: &Self) -> Vec<XmlElement> {
    match element {
      ArrowContents::Circular(contents) => CircularArrowContents::serialize(contents),
      ArrowContents::Straight(contents) => StraightArrowContents::serialize(contents),
    }
  }
}

/// The [Arrow] element represents an arrow used for a musical technical indication.
///
/// ![Arrow](arrow.png)
///
/// It can represent both Unicode and Standard Music Font Layout (SMuFL) arrows.
/// The `smufl` attribute distinguishes different SMuFL glyphs that have an arrow appearance such as "arrowBlackUp,"
/// "guitarStrumUp," or "handbellsSwingUp." The specified glyph should match the descriptive representation.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Arrow {
  /// Element-specific attributes
  pub attributes: ArrowAttributes,
  #[flatten]
  /// Element-specific content
  pub content: ArrowContents,
}

#[cfg(test)]
mod arrow_tests {
  use super::*;
  use crate::elements::{ArrowDirection, ArrowStyle, CircularArrow};
  use crate::parser::parse_from_xml_str;

  #[test]
  fn test1() {
    let result = parse_from_xml_str::<Arrow>(
      "<arrow placement=\"above\">
        <circular-arrow>clockwise</circular-arrow>
      </arrow>",
    );
    assert_eq!(
      result.unwrap(),
      Arrow {
        attributes: ArrowAttributes {
          placement: Some(AboveBelow::Above),
          ..Default::default()
        },
        content: ArrowContents::Circular(CircularArrowContents {
          circular_arrow: CircularArrow {
            attributes: (),
            content: crate::datatypes::CircularArrow::Clockwise
          },
        }),
      }
    );
  }

  #[test]
  fn test2() {
    let result = parse_from_xml_str::<Arrow>(
      "<arrow placement=\"above\">
        <arrow-direction>left right</arrow-direction>
        <arrow-style>single</arrow-style>
      </arrow>",
    );
    assert_eq!(
      result.unwrap(),
      Arrow {
        attributes: ArrowAttributes {
          placement: Some(AboveBelow::Above),
          ..Default::default()
        },
        content: ArrowContents::Straight(StraightArrowContents {
          arrow_direction: ArrowDirection {
            attributes: (),
            content: crate::datatypes::ArrowDirection::LeftRight
          },
          arrow_style: Some(ArrowStyle {
            attributes: (),
            content: crate::datatypes::ArrowStyle::Single
          }),
          arrowhead: None,
        }),
      }
    );
  }
}
