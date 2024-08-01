use crate::datatypes::{BeaterValue, TipDirection};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Beater] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BeaterAttributes {
  /// Indicates the direction in which the tip of the beater points.
  pub tip: Option<TipDirection>,
}

/// The [Beater] element represents pictograms for beaters, mallets, and sticks that do not have different materials represented in the pictogram.
///
/// ![Beater](https://hedgetechllc.github.io/musicxml/musicxml/elements/beater.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Beater {
  /// Element-specific attributes
  pub attributes: BeaterAttributes,
  /// Element-specific content
  pub content: BeaterValue,
}
