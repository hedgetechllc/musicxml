use crate::datatypes::{GlassValue, SmuflPictogramGlyphName};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Glass] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct GlassAttributes {
  /// Distinguishes different SMuFL glyphs for wind chimes in the Chimes pictograms range, including those made of materials other than glass.
  pub smufl: Option<SmuflPictogramGlyphName>,
}

/// The [Glass] element represents pictograms for glass percussion instruments.
/// 
/// ![Glass](glass.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Glass {
  /// Element-specific attributes
  pub attributes: GlassAttributes,
  /// Element-specific content
  pub content: GlassValue,
}
