use musicxml_internal::*;
use musicxml_macros::*;

/// The [Ipa] element represents International Phonetic Alphabet (IPA) sounds for vocal music.
/// 
/// String content is limited to IPA 2015 symbols represented in Unicode 13.0.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
pub struct Ipa {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: String,
}
