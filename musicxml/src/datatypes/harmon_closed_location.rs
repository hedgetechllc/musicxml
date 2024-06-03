use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates which portion of the symbol is filled in when the corresponding
/// [HarmonClosedValue][super::HarmonClosedValue] is [Half][super::HarmonClosedValue::Half].
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HarmonClosedLocation {
  /// ![Bottom](harmon-closed-location-bottom.png)
  Bottom,
  /// ![Left](harmon-closed-location-left.png)
  Left,
  /// ![Right](harmon-closed-location-right.png)
  Right,
  /// ![Top](harmon-closed-location-top.png)
  Top,
}
