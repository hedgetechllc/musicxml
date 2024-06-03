use super::{ClefOctaveChange, Line, Sign};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PartClefContents {
  pub sign: Sign,
  pub line: Option<Line>,
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
