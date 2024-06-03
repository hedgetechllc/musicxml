use musicxml_internal::*;
use musicxml_macros::*;

/// The [InstrumentAbbreviation] element is typically used within a software application, rather than appearing on the printed page of a score.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("instrument-abbreviation")]
pub struct InstrumentAbbreviation {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
