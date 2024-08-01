use alloc::string::String;
use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Used to indicate where a key signature cancellation appears relative to a new key signature.
///
/// Possible locations are: to the [Left][CancelLocation::Left], to the [Right][CancelLocation::Right], or
/// [BeforeBarline][CancelLocation::BeforeBarline] and to the left. It is [Left][CancelLocation::Left] if not specified.
///
/// For mid-measure key elements, [BeforeBarline][CancelLocation::BeforeBarline] should be treated like [Left][CancelLocation::Left].
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum CancelLocation {
  /// ![Left](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/cancel-location-left.png)
  Left,
  /// ![Right](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/cancel-location-right.png)
  Right,
  /// ![BeforeBarline](https://hedgetechllc.github.io/musicxml/musicxml/datatypes/cancel-location-before-barline.png)
  #[rename("before-barline")]
  BeforeBarline,
}
