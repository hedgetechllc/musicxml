use super::{Distance, Glyph, LineWidth, NoteSize, OtherAppearance};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Appearance] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct AppearanceContents {
  /// The [LineWidth] element specifies the width of a line type in tenths.
  pub line_width: Vec<LineWidth>,
  /// The [NoteSize] element specifies the size of notes in a score.
  pub note_size: Vec<NoteSize>,
  /// The [Distance] element specifies the distance between two elements in tenths.
  pub distance: Vec<Distance>,
  /// The [Glyph] element specifies the SMuFL glyph name for a character in a music font.
  pub glyph: Vec<Glyph>,
  /// The [OtherAppearance] element is used to define any appearance attributes not yet supported in the MusicXML format.
  pub other_appearance: Vec<OtherAppearance>,
}

/// The [Appearance] element controls general graphical settings for the music's final form appearance on a printed page of display.
///
/// This includes support for line widths, definitions for note sizes, standard distances between notation elements, and Standard Music Font Layout (SMuFL) glyphs,
/// plus an extension element for other aspects of appearance.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Appearance {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: AppearanceContents,
}

#[cfg(test)]
mod appearance_tests {
  use super::*;
  use crate::datatypes::{DistanceType, GlyphType, LineWidthType, SmuflGlyphName, Tenths, Token};
  use crate::elements::{DistanceAttributes, GlyphAttributes, LineWidthAttributes, OtherAppearanceAttributes};
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Appearance>(
      "
    <appearance>
      <line-width type=\"leger\">2.5</line-width>
      <distance type=\"beam\">3</distance>
      <glyph type=\"f-clef\">smufl</glyph>
      <glyph type=\"percussion-clef\">test-glyph</glyph>
      <other-appearance type=\"TypeString\">Val</other-appearance>
    </appearance>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Appearance {
        attributes: (),
        content: AppearanceContents {
          line_width: vec![LineWidth {
            attributes: LineWidthAttributes {
              r#type: LineWidthType::Leger
            },
            content: Tenths(2.5)
          }],
          distance: vec![Distance {
            attributes: DistanceAttributes {
              r#type: DistanceType::Beam
            },
            content: Tenths(3.0)
          }],
          glyph: vec![
            Glyph {
              attributes: GlyphAttributes {
                r#type: GlyphType::FClef
              },
              content: SmuflGlyphName(String::from("smufl"))
            },
            Glyph {
              attributes: GlyphAttributes {
                r#type: GlyphType::PercussionClef
              },
              content: SmuflGlyphName(String::from("test-glyph"))
            }
          ],
          other_appearance: vec![OtherAppearance {
            attributes: OtherAppearanceAttributes {
              r#type: Token(String::from("TypeString"))
            },
            content: String::from("Val")
          }],
          ..Default::default()
        }
      }
    );
  }
}
