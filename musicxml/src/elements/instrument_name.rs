use alloc::string::String;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [InstrumentName] element is typically used within a software application, rather than appearing on the printed page of a score.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("instrument-name")]
pub struct InstrumentName {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
