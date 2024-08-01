use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Software] element specifies what software created the digital encoding.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Software {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
