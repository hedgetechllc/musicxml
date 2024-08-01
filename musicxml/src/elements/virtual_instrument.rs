use super::{VirtualLibrary, VirtualName};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [VirtualInstrument] element.
#[derive(Debug, Default, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct VirtualInstrumentContents {
  /// The [VirtualLibrary] element specifies the virtual library used for the virtual instrument.
  pub virtual_library: Option<VirtualLibrary>,
  /// The [VirtualName] element specifies the virtual instrument used for the instrument sound.
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
