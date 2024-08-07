use crate::datatypes::{MembraneValue, SmuflPictogramGlyphName};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Membrane] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct MembraneAttributes {
  /// Distinguishes different SMuFL stylistic alternates.
  pub smufl: Option<SmuflPictogramGlyphName>,
}

/// The [Membrane] element represents pictograms for membrane percussion instruments.
///
/// ![Membrane](https://hedgetechllc.github.io/musicxml/musicxml/elements/membrane.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Membrane {
  /// Element-specific attributes
  pub attributes: MembraneAttributes,
  /// Element-specific content
  pub content: MembraneValue,
}
