use musicxml_internal::*;
use musicxml_macros::*;

/// The [VirtualName] element indicates the library-specific name for the virtual instrument.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("virtual-name")]
pub struct VirtualName {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
