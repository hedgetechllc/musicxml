use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Solo] element is present if performance is intended by a solo instrument.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Solo {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
