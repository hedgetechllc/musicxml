use super::{NumeralAlter, NumeralKey, NumeralRoot};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [Numeral] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct NumeralContents {
  /// The [NumeralRoot] element specifies the root of the Roman numeral or Nashville number.
  pub numeral_root: NumeralRoot,
  /// The [NumeralAlter] element specifies the alteration of the Roman numeral or Nashville number.
  pub numeral_alter: Option<NumeralAlter>,
  /// The [NumeralKey] element specifies the key of the Roman numeral or Nashville number.
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
