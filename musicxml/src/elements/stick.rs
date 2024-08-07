use super::{StickMaterial, StickType};
use crate::datatypes::{TipDirection, YesNo};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Stick] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct StickAttributes {
  /// Indicates the presence of a dashed circle around the round beater part of a pictogram. The value is no if not specified.
  pub dashed_circle: Option<YesNo>,
  /// Specifies whether or not parentheses are put around a symbol for an editorial indication. If not specified, it is left to application defaults.
  pub parentheses: Option<YesNo>,
  /// Represents the direction in which the tip of a stick or beater points, using Unicode arrow terminology.
  pub tip: Option<TipDirection>,
}

/// Contents of the [Stick] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct StickContents {
  /// The [StickType] element indicates the pictogram type of the stick, mallet, or beater.
  pub stick_type: StickType,
  /// The [StickMaterial] element indicates the material of the stick, mallet, or beater.
  pub stick_material: StickMaterial,
}

/// The [Stick] element represents pictograms where the material of the stick, mallet, or beater is included.
///
/// ![Stick](https://hedgetechllc.github.io/musicxml/musicxml/elements/stick.png)
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Stick {
  /// Element-specific attributes
  pub attributes: StickAttributes,
  #[flatten]
  /// Element-specific content
  pub content: StickContents,
}
