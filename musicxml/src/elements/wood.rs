use crate::datatypes::{SmuflPictogramGlyphName, WoodValue};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Wood] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct WoodAttributes {
  /// Distinguishes different SMuFL stylistic alternates.
  pub smufl: Option<SmuflPictogramGlyphName>,
}

/// The [Wood] element represents pictograms for wood percussion instruments.
///
/// ![Wood](https://hedgetechllc.github.io/musicxml/musicxml/elements/wood.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Wood {
  /// Element-specific attributes
  pub attributes: WoodAttributes,
  /// Element-specific content
  pub content: WoodValue,
}
