use super::{ClefOctaveChange, Line, Sign};
use alloc::{string::String, vec::Vec};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [PartClef] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PartClefContents {
  /// The [Sign] element represents the clef symbol.
  pub sign: Sign,
  /// The [Line] element represents the clef line.
  pub line: Option<Line>,
  /// The [ClefOctaveChange] element indicates the octave transposition of the clef.
  pub clef_octave_change: Option<ClefOctaveChange>,
}

/// The [PartClef] element is used for transpositions from concert scores that also include a change of clef, as for instruments such as bass clarinet.
///
/// The child elements of the [PartClef] element have the same meaning as for the [Clef][super::Clef] element. However that meaning applies
/// to a transposed part created from the existing score file.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("part-clef")]
pub struct PartClef {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: PartClefContents,
}
