use crate::datatypes::SmuflPictogramGlyphName;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Timpani] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct TimpaniAttributes {
  /// Distinguishes different SMuFL stylistic alternates.
  pub smufl: Option<SmuflPictogramGlyphName>,
}

/// The [Timpani] element represents the timpani pictogram.
///
/// ![Timpani](https://hedgetechllc.github.io/musicxml/musicxml/elements/timpani.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Timpani {
  /// Element-specific attributes
  pub attributes: TimpaniAttributes,
  /// Element-specific content
  pub content: (),
}
