use super::{Chromatic, Diatonic, Double, OctaveChange};
use musicxml_internal::*;
use musicxml_macros::*;

/// Contents of the [PartTranspose] element.
#[derive(Debug, PartialEq, Eq, ContentDeserialize, ContentSerialize)]
pub struct PartTransposeContents {
  /// The [Diatonic] element specifies the number of diatonic steps up or down from the written key signature.
  pub diatonic: Option<Diatonic>,
  /// The [Chromatic] element specifies the number of chromatic steps up or down from the written key signature.
  pub chromatic: Chromatic,
  /// The [OctaveChange] element specifies the number of octaves up or down from the written key signature.
  pub octave_change: Option<OctaveChange>,
  /// The [Double] element specifies the number of double accidentals up or down from the written key signature.
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
