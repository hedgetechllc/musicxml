use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [SemiPitched] element represents categories of indefinite pitch for percussion instruments.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("semi-pitched")]
pub struct SemiPitched {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::SemiPitched,
}
