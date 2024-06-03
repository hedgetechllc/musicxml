use musicxml_internal::*;
use musicxml_macros::*;

/// The presence of a [ConcertScore] element indicates that a score is displayed in concert pitch.
///
/// It is used for scores that contain parts for transposing instruments.
///
/// A document with a [ConcertScore] element may not contain any [Transpose][super::Transpose] elements that have non-zero values for
/// either the [Diatonic][super::Diatonic] or [Chromatic][super::Chromatic] elements. Concert scores may include octave transpositions,
/// so [Transpose][super::Transpose] elements with a [Double][super::Double] element or a non-zero [OctaveChange][super::OctaveChange]
/// element value are permitted.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("concert-score")]
pub struct ConcertScore {
  /// Element-specific attributes
  pub attributes: (),
  /// Element-specific content
  pub content: (),
}
