use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [StickLocation] element represents pictograms for the location of sticks, beaters, or mallets on cymbals, gongs, drums, and other instruments.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("stick-location")]
pub struct StickLocation {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::StickLocation,
}
