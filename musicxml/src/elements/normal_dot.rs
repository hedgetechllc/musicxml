use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [NormalDot] element is used to specify dotted normal tuplet types.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("normal-dot")]
pub struct NormalDot {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
