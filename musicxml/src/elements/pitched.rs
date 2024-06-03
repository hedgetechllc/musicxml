use crate::datatypes::{PitchedValue, SmuflPictogramGlyphName};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Pitched] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct PitchedAttributes {
  /// Distinguishes different SMuFL glyphs for a particular pictogram within the Tuned mallet percussion pictograms range.
  pub smufl: Option<SmuflPictogramGlyphName>,
}

/// The [Pitched] element represents pictograms for pitched percussion instruments.
/// 
/// ![Pitched](pitched.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Pitched {
  /// Element-specific attributes
  pub attributes: PitchedAttributes,
  /// Element-specific content
  pub content: PitchedValue,
}
