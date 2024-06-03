use crate::datatypes::Semitones;
use musicxml_internal::*;
use musicxml_macros::*;

/// The [Alter] element represents chromatic alteration in number of semitones (e.g., -1 for flat, 1 for sharp).
/// 
/// Decimal values like 0.5 (quarter tone sharp) are used for microtones.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Alter {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: Semitones,
}
