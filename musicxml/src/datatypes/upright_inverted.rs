use musicxml_internal::{DatatypeDeserializer, DatatypeSerializer};
use musicxml_macros::{DatatypeDeserialize, DatatypeSerialize};

/// Describes the appearance of a [Fermata][crate::elements::Fermata] element.
///
/// The value is [Upright][UprightInverted::Upright] if not specified.
#[derive(Debug, PartialEq, Eq, DatatypeDeserialize, DatatypeSerialize)]
pub enum UprightInverted {
  /// <span class="smufl">&#xE4C0;</span>
  Upright,
  /// <span class="smufl">&#xE4C1;</span>
  Inverted,
}
