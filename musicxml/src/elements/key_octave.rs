use crate::datatypes::{Octave, PositiveInteger, YesNo};
use musicxml_internal::*;
use musicxml_macros::*;

/// Attributes pertaining to the [KeyOctave] element.
#[derive(Debug, PartialEq, Eq, AttributeDeserialize, AttributeSerialize)]
pub struct KeyOctaveAttributes {
  /// A positive integer that refers to the key signature element in left-to-right order.
  pub number: PositiveInteger,
  /// If set to yes, then the number refers to the canceling key signature specified by the [Cancel][super::Cancel] element
  /// in the parent [Key][super::Key] element. It cannot be set to yes if there is no corresponding [Cancel][super::Cancel] element
  /// within the parent [Key][super::Key] element. It is no if absent.
  pub cancel: Option<YesNo>,
}

/// The [KeyOctave] element specifies in which octave an element of a key signature appears.
///
/// The content specifies the octave value using the same values as the [DisplayOctave][super::DisplayOctave] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("key-octave")]
pub struct KeyOctave {
  /// Element-specific attributes
  pub attributes: KeyOctaveAttributes,
  /// Element-specific content
  pub content: Octave,
}
