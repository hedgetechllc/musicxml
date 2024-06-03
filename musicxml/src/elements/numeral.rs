use super::{NumeralAlter, NumeralKey, NumeralRoot};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct NumeralContents {
  pub numeral_root: NumeralRoot,
  pub numeral_alter: Option<NumeralAlter>,
  pub numeral_key: Option<NumeralKey>,
}

/// The [Numeral] element represents the Roman numeral or Nashville number part of a harmony.
/// 
/// It requires that the key be specified in the encoding, either with a [Key][super::Key] or [NumeralKey] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Numeral {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: NumeralContents,
}
