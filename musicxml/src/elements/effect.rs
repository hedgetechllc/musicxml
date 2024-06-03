use crate::datatypes::{EffectValue, SmuflPictogramGlyphName};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Effect] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct EffectAttributes {
  /// Distinguishes different SMuFL stylistic alternates.
  pub smufl: Option<SmuflPictogramGlyphName>,
}

/// The [Effect] element represents pictograms for sound effect percussion instruments.
/// 
/// ![Effect](effect.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Effect {
  /// Element-specific attributes
  pub attributes: EffectAttributes,
  /// Element-specific content
  pub content: EffectValue,
}
