use musicxml_internal::*;
use musicxml_macros::*;

/// The [StickMaterial] element represents the material being displayed in a stick pictogram.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("stick-material")]
pub struct StickMaterial {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::StickMaterial,
}
