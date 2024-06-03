use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Indicates which portion of the hole is filled in when the corresponding
/// [HoleClosedValue][super::HoleClosedValue] is [Half][super::HoleClosedValue::Half].
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum HoleClosedLocation {
  /// ![Bottom](hole-closed-location-bottom.png)
  Bottom,
  /// ![Left](hole-closed-location-left.png)
  Left,
  /// ![Right](hole-closed-location-right.png)
  Right,
  /// ![Top](hole-closed-location-top.png)
  Top,
}
