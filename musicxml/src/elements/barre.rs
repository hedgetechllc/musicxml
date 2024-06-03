use crate::datatypes::{Color, StartStop};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Barre] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BarreAttributes {
  /// The start value indicates the lowest pitched string (e.g., the string with the highest MusicXML number).
  /// The stop value indicates the highest pitched string.
  pub r#type: StartStop,
  /// Indicates the color of an element.
  pub color: Option<Color>,
}

/// The [Barre] element indicates placing a finger over multiple strings on a single fret.
/// 
/// ![Barre](barre.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Barre {
  /// Element-specific attributes
  pub attributes: BarreAttributes,
  /// Element-specific content
  pub content: (),
}
