use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Represents pictograms for the location of sticks, beaters, or mallets on cymbals, gongs, drums, and other instruments.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum StickLocation {
  /// <span class="smufl">&#xE7FF;</span>
  Center,
  /// <span class="smufl">&#xE72A;</span>
  #[rename("cymbal bell")]
  CymbalBell,
  /// <span class="smufl">&#xE729;</span>
  #[rename("cymbal edge")]
  CymbalEdge,
  /// <span class="smufl">&#xE802;</span>
  Rim,
}
