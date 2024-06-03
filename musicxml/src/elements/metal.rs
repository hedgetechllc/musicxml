use crate::datatypes::{MetalValue, SmuflPictogramGlyphName};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Metal] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MetalAttributes {
  /// Distinguishes different SMuFL stylistic alternates.
  pub smufl: Option<SmuflPictogramGlyphName>,
}

/// The [Metal] element represents pictograms for metal percussion instruments.
///
/// ![Metal](metal.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Metal {
  /// Element-specific attributes
  pub attributes: MetalAttributes,
  /// Element-specific content
  pub content: MetalValue,
}
