use musicxml_internal::*;
use musicxml_macros::*;

/// The [VirtualLibrary] element indicates the virtual instrument library name.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("virtual-library")]
pub struct VirtualLibrary {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
