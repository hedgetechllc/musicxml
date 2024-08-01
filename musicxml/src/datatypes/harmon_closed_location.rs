use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates which portion of the symbol is filled in when the corresponding
/// [HarmonClosedValue][super::HarmonClosedValue] is [Half][super::HarmonClosedValue::Half].
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HarmonClosedLocation {
  /// ![Bottom](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/harmon-closed-location-bottom.png)
  Bottom,
  /// ![Left](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/harmon-closed-location-left.png)
  Left,
  /// ![Right](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/harmon-closed-location-right.png)
  Right,
  /// ![Top](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/harmon-closed-location-top.png)
  Top,
}
