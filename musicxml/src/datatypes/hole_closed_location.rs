use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates which portion of the hole is filled in when the corresponding
/// [HoleClosedValue][super::HoleClosedValue] is [Half][super::HoleClosedValue::Half].
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HoleClosedLocation {
  /// ![Bottom](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/hole-closed-location-bottom.png)
  Bottom,
  /// ![Left](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/hole-closed-location-left.png)
  Left,
  /// ![Right](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/hole-closed-location-right.png)
  Right,
  /// ![Top](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/hole-closed-location-top.png)
  Top,
}
