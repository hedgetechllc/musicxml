use super::{DisplayOctave, DisplayStep};
use crate::datatypes::YesNo;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Rest] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct RestAttributes {
  /// If yes, this indicates this is a complete measure rest.
  pub measure: Option<YesNo>,
}

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct RestContents {
  pub display_step: Option<DisplayStep>,
  pub display_octave: Option<DisplayOctave>,
}

/// The [Rest] element indicates notated rests or silences.
/// 
/// A [Rest] element is usually empty, but placement on the staff can be specified using [DisplayStep] and [DisplayOctave] elements.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Rest {
  /// Element-specific attributes
  pub attributes: RestAttributes,
  #[flatten]
  /// Element-specific content
  pub content: RestContents,
}
