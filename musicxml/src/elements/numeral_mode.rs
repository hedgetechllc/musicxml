use musicxml_internal::*;
use musicxml_macros::*;

/// The [NumeralMode] specifies the scale that is used to interpret the [NumeralRoot][super::NumeralRoot] element values.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("numeral-mode")]
pub struct NumeralMode {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: crate::datatypes::NumeralMode,
}
