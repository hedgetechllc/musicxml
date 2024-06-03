use crate::datatypes::Divisions;
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [Release] element.
#[derive(Debug, Default, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct ReleaseAttributes {
  /// Specifies where the release starts in terms of divisions relative to the current note.
  pub offset: Option<Divisions>,
}

/// The [Release] element indicates that a bend is a release rather than a normal bend or pre-bend.
/// 
/// The `first_beat` and `last_beat` attributes of the parent [Bend][super::Bend] element are relative to the original note position, not this offset value.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Release {
  /// Element-specific attributes
  pub attributes: ReleaseAttributes,
  /// Element-specific content
  pub content: (),
}
