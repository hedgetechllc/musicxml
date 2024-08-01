use crate::datatypes::Color;
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [BarStyle] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct BarStyleAttributes {
  /// Indicates the color of an element.
  pub color: Option<Color>,
}

/// The [BarStyle] element contains barline style and color information.
///
/// ![BarStyle](https://hedgetechllc.github.io/musicxml/musicxml/elements/bar-style.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("bar-style")]
pub struct BarStyle {
  /// Element-specific attributes
  pub attributes: BarStyleAttributes,
  /// Element-specific content
  pub content: crate::datatypes::BarStyle,
}
