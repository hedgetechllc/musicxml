use super::{BarStyle, Coda, Ending, Fermata, Footnote, Level, Repeat, Segno, WavyLine};
use crate::datatypes::{Divisions, Id, RightLeftMiddle, Token};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Barline] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BarlineAttributes {
  /// Used for playback when there is a [Coda] child element.
  /// Indicates the end point for a forward jump to a coda sign.
  /// If there are multiple jumps, the value of these parameters can be used to name and distinguish them.
  pub coda: Option<Token>,
  /// If the `segno` or `coda` attributes are used, the `divisions` attribute can be used to indicate the
  /// number of divisions per quarter note. Otherwise sound and MIDI generating programs may have to recompute this.
  pub divisions: Option<Divisions>,
  /// Specifies an ID that is unique to the entire document.
  pub id: Option<Id>,
  /// Barlines have a `location` attribute to make it easier to process barlines independently of the other musical data in a score.
  /// It is often easier to set up measures separately from entering notes. The `location` attribute must match where the [Barline]
  /// element occurs within the rest of the musical data in the score. If `location` is "left," it should be the first element in the measure,
  /// aside from the [Print][super::Print], [Bookmark][super::Bookmark], and [Link][super::Link] elements.
  /// If `location` is "right," it should be the last element, again with the possible exception of the [Print][super::Print],
  /// [Bookmark][super::Bookmark], and [Link][super::Link] elements. The default value is "right."
  pub location: Option<RightLeftMiddle>,
  /// Used for playback when there is a [Segno] child element. Indicates the end point for a backward jump to a segno sign.
  /// If there are multiple jumps, the value of these parameters can be used to name and distinguish them.
  pub segno: Option<Token>,
}

/// Contents of the [Barline] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct BarlineContents {
  /// The [BarStyle] element indicates the style of the barline.
  pub bar_style: Option<BarStyle>,
  /// The [Footnote] element specifies editorial information or lyrics content.
  pub footnote: Option<Footnote>,
  /// The [Level] element specifies the editorial level of a score or part.
  pub level: Option<Level>,
  /// The [WavyLine] element represents a wavy line symbol.
  pub wavy_line: Option<WavyLine>,
  /// The [Segno] element indicates a segno sign.
  pub segno: Option<Segno>,
  /// The [Coda] element indicates a coda sign.
  pub coda: Option<Coda>,
  /// The [Fermata] element represents a fermata symbol.
  pub fermata: Vec<Fermata>,
  /// The [Ending] element indicates a multiple ending.
  pub ending: Option<Ending>,
  /// The [repeat] element specifies a repeat barline.
  pub repeat: Option<Repeat>,
}

/// If a barline is other than a normal single barline, it should be represented by a [Barline] element that describes it.
///
/// ![Barline](barline.png)
///
/// This includes information about repeats and multiple endings, as well as line style. Barline data is on the same level as the other musical data
/// in a score - a child of a measure in a partwise score, or a part in a timewise score. This allows for barlines within measures, as in dotted barlines
/// that subdivide measures in complex meters. The two [Fermata] elements allow for fermatas on both sides of the barline (the lower one inverted).
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Barline {
  /// Element-specific attributes
  pub attributes: BarlineAttributes,
  #[flatten]
  /// Element-specific content
  pub content: BarlineContents,
}

#[cfg(test)]
mod barline_tests {
  use super::*;
  use crate::datatypes::FermataShape;
  use crate::elements::{BarStyleAttributes, Fermata, FermataAttributes};
  use crate::parser::parse_from_xml_str;

  #[test]
  fn deserialize_valid1() {
    let result = parse_from_xml_str::<Barline>(
      "
    <barline location=\"right\">
      <bar-style>heavy-heavy</bar-style>
      <fermata>angled</fermata>
      <fermata>square</fermata>
    </barline>",
    );
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      Barline {
        attributes: BarlineAttributes {
          location: Some(RightLeftMiddle::Right),
          ..Default::default()
        },
        content: BarlineContents {
          bar_style: Some(BarStyle {
            attributes: BarStyleAttributes::default(),
            content: crate::datatypes::BarStyle::HeavyHeavy
          }),
          fermata: vec![
            Fermata {
              attributes: FermataAttributes::default(),
              content: FermataShape::Angled
            },
            Fermata {
              attributes: FermataAttributes::default(),
              content: FermataShape::Square
            }
          ],
          ..Default::default()
        }
      }
    );
  }
}
