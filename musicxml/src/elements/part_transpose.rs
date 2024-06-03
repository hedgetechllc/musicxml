use super::{Chromatic, Diatonic, Double, OctaveChange};
use musicxml_internal::*;
use musicxml_macros::*;

#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PartTransposeContents {
  pub diatonic: Option<Diatonic>,
  pub chromatic: Chromatic,
  pub octave_change: Option<OctaveChange>,
  pub double: Option<Double>,
}

/// The child elements of the [PartTranspose] element have the same meaning as for the [Transpose][super::Transpose] element.
/// 
/// However that meaning applies to a transposed part created from the existing score file.
/// 
/// The [Chromatic] element in a [PartTranspose] element will usually have a non-zero value, since octave transpositions can be represented in concert
/// scores using the [Transpose][super::Transpose] element.
#[derive(Debug, PartialEq, Eq, ElementDeserialize, ElementSerialize)]
#[rename("part-transpose")]
pub struct PartTranspose {
  /// Element-specific attributes
  pub attributes: (),
  #[flatten]
  /// Element-specific content
  pub content: PartTransposeContents,
}
