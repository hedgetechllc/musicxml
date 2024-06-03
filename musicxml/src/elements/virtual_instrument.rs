use super::{VirtualLibrary, VirtualName};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct VirtualInstrumentContents {
  pub virtual_library: Option<VirtualLibrary>,
  pub virtual_name: Option<VirtualName>,
}

/// The [VirtualInstrument] element defines a specific virtual instrument used for an instrument sound.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("virtual-instrument")]
pub struct VirtualInstrument {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: VirtualInstrumentContents,
}
